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
use supervisor::SupervisorAgent;
use tools::{
    code_writer::CodeWriterTool, directory_lister::DirectoryListerTool,
    file_reader::FileReaderTool, system::SystemTool, web_scraper::WebScraperTool,
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

    let llm: Box<dyn Llm + Send + Sync> = if args.mock {
        Box::new(MockLlm)
    } else {
        Box::new(OpenAiLlm::new(&settings.model))
    };
    let llm2: Box<dyn Llm + Send + Sync> = if args.mock {
        Box::new(MockLlm)
    } else {
        Box::new(OpenAiLlm::new(&settings.model))
    };

    let file_system_agent = ExecutorAgent::new(
        llm,
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
        llm2,
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

    let supervisor_llm: Box<dyn Llm + Send + Sync> = if args.mock {
        Box::new(MockLlm)
    } else {
        Box::new(OpenAiLlm::new(&settings.model))
    };

    let supervisor = SupervisorAgent::new(supervisor_llm, workers);

    let orchestrator = Orchestrator::new(Box::new(supervisor));

    info!("Task: {}\n", &args.task);

    match orchestrator.run(&args.task).await {
        Ok(result) => info!("\nFinal Answer: {}", result),
        Err(e) => error!("Error: {}", e),
    }
}