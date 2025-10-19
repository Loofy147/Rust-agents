mod agent;
mod executor;
mod llm;
mod orchestrator;
mod planner;
mod tools;

use clap::Parser;
use config::{Config, File};
use dotenv::dotenv;
use executor::ExecutorAgent;
use llm::{Llm, MockLlm, OpenAiLlm};
use orchestrator::Orchestrator;
use planner::PlannerAgent;
use serde::Deserialize;
use tools::{
    code_writer::CodeWriterTool, directory_lister::DirectoryListerTool,
    file_reader::FileReaderTool, system::SystemTool, web_scraper::WebScraperTool, Tool,
};
use tracing::{error, info};
use tracing_subscriber;

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
}

/// The main entry point for the application.
///
/// This function sets up the agent, including the LLM and tools, and then
/// runs the agent with a specific task.
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let settings = Config::builder()
        .add_source(File::with_name("config").required(false))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let args = Args::parse();

    let llm: Box<dyn Llm + Sync> = if args.mock {
        Box::new(MockLlm)
    } else {
        Box::new(OpenAiLlm::new(&settings.model))
    };

    let code_writer = CodeWriterTool;
    let file_reader = FileReaderTool;
    let directory_lister = DirectoryListerTool;
    let system = SystemTool;
    let web_scraper = WebScraperTool;

    let tools: Vec<&(dyn Tool + Sync)> = vec![
        &code_writer,
        &file_reader,
        &directory_lister,
        &system,
        &web_scraper,
    ];

    let planner_llm: Box<dyn Llm + Sync> = if args.mock {
        Box::new(MockLlm)
    } else {
        Box::new(OpenAiLlm::new(&settings.model))
    };

    let planner = PlannerAgent::new(planner_llm);
    let executor = ExecutorAgent::new(llm, tools);

    let orchestrator = Orchestrator::new(planner, executor);

    info!("Task: {}\n", &args.task);

    match orchestrator.run(&args.task).await {
        Ok(result) => info!("\nFinal Answer: {}", result),
        Err(e) => error!("Error: {}", e),
    }
}