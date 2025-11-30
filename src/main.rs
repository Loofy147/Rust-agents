mod agent;
mod executor;
mod llm;
mod orchestrator;
mod supervisor;
mod tools;

use crate::agent::Agent;
use clap::Parser;
use config::{Config, File};
use dotenv::dotenv;
use executor::ExecutorAgent;
use llm::{Llm, MockLlm, OpenAiLlm};
use orchestrator::Orchestrator;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use supervisor::SupervisorAgent;
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace, Resource};
use tools::{
    code_writer::CodeWriterTool, directory_lister::DirectoryListerTool,
    file_reader::FileReaderTool, system::SystemTool, web_scraper::WebScraperTool,
};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracer(settings: &Settings) -> Result<(), anyhow::Error> {
    let endpoint = settings
        .otlp_endpoint
        .as_deref()
        .unwrap_or("http://localhost:4317");

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                "rust-multi-agent-framework",
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::Registry::default()
        .with(telemetry_layer)
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The task for the agent to perform
    #[arg(short, long)]
    task: String,

    /// Use the mock LLM for testing
    #[arg(long)]
    mock: bool,
}

#[derive(Deserialize, Debug)]
struct Settings {
    model: String,
    otlp_endpoint: Option<String>,
}

/// The main entry point for the application.
///
/// This function sets up the agent, including the LLM and tools, and then
/// runs the agent with a specific task.
#[tokio::main]
async fn main() {
    dotenv().ok();

    let settings = Config::builder()
        .add_source(File::with_name("config").required(false))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    init_tracer(&settings).expect("Failed to initialize tracer");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let args = Args::parse();

    let llm: Arc<dyn Llm + Send + Sync> = if args.mock {
        Arc::new(MockLlm::new(
            r#"
{
    "thought": "I am in an unexpected state. I will finish.",
    "action": { "tool": "Finish", "args": "Error: Reached an unknown state in the generation process." }
}
"#,
        ))
    } else {
        Arc::new(OpenAiLlm::new(&settings.model))
    };

    let file_system_agent = ExecutorAgent::new(
        llm.clone(),
        vec![
            Box::new(CodeWriterTool),
            Box::new(FileReaderTool),
            Box::new(DirectoryListerTool),
            Box::new(SystemTool),
        ],
        "FileSystemAgent",
        "An agent that can interact with the file system.",
    );

    let web_scraper_agent = ExecutorAgent::new(
        llm.clone(),
        vec![Box::new(WebScraperTool)],
        "WebScraperAgent",
        "An agent that can scrape web pages.",
    );

    let mut workers: HashMap<String, Box<dyn Agent + Send + Sync>> = HashMap::new();
    workers.insert(
        file_system_agent.name(),
        Box::new(file_system_agent) as Box<dyn Agent + Send + Sync>,
    );
    workers.insert(
        web_scraper_agent.name(),
        Box::new(web_scraper_agent) as Box<dyn Agent + Send + Sync>,
    );

    let supervisor = SupervisorAgent::new(llm, workers);

    let orchestrator = Orchestrator::new(Box::new(supervisor));

    info!("Task: {}\n", &args.task);

    match orchestrator.run(&args.task).await {
        Ok(result) => info!("\nFinal Answer: {}", result),
        Err(e) => error!("Error: {}", e),
    }
}