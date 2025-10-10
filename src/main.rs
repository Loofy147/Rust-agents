use app::{
    agent::{Agent, ReActAgent},
    llm::OpenAiLlm,
    tools::{code_writer::CodeWriterTool, system::SystemTool, Tool},
};
use clap::Parser;
use tracing::{error, info};

/// A generative agent that can write and execute code to solve tasks.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The task for the agent to complete.
    task: String,
}

/// The main entry point for the application.
///
/// This function sets up the agent, including the LLM and tools, and then
/// runs the agent with a specific task.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let settings = app::config::Settings::new().expect("Failed to load configuration");

    let llm = OpenAiLlm::new(settings);
    let code_writer = CodeWriterTool;
    let system = SystemTool;

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(code_writer), Box::new(system)];

    let agent = ReActAgent::new(llm, tools);

    info!("Task: {}\n", &args.task);

    match agent.run(&args.task).await {
        Ok(result) => info!("\nFinal Answer: {}", result),
        Err(e) => error!("Error: {}", e),
    }
}