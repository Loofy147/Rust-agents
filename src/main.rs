mod agent;
mod llm;
mod tools;

use agent::{Agent, ReActAgent};
use llm::MockLlm;
use tools::code_writer::CodeWriterTool;
use tools::system::SystemTool;

fn main() {
    let llm = MockLlm;
    let code_writer = CodeWriterTool;
    let system = SystemTool;

    let tools: Vec<&dyn tools::Tool> = vec![&code_writer, &system];

    let agent = ReActAgent::new(&llm, tools);

    let task = "Generate a program to find the SHA-256 hash of 'hello world'";

    println!("Task: {}\n", task);

    match agent.run(task) {
        Ok(result) => println!("\nFinal Answer: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}