mod agent;
mod llm;
mod tools;

use agent::{Agent, ReActAgent};
use llm::MockLlm;
use tools::CalculatorTool;

fn main() {
    let llm = MockLlm;
    let calculator = CalculatorTool;
    let tools = vec![&calculator as &dyn tools::Tool];
    let agent = ReActAgent::new(&llm, tools);

    let task = "What is 4 * (3 + 5)?";
    println!("Task: {}\n", task);

    match agent.run(task) {
        Ok(result) => println!("\nFinal Answer: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}