use anyhow::Result;
use serde_json::json;

/// A trait for Large Language Models (LLMs).
///
/// This trait defines the interface for a large language model, which is a
/// core component of the agent. The LLM is responsible for generating responses
/// based on a given prompt.
pub trait Llm {
    /// Takes a prompt and returns the LLM's response.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A string representing the input to the LLM.
    ///
    /// # Returns
    ///
    /// A `Result` containing the LLM's response as a string, or an error if
    /// the call fails.
    fn call(&self, prompt: &str) -> Result<String>;
}

/// A mock implementation of the `Llm` trait for testing and demonstration.
///
/// `MockLlm` simulates the behavior of a real LLM by returning canned responses
/// based on the content of the prompt. This allows for predictable testing of
/// the agent's logic without making actual API calls.
pub struct MockLlm;

impl Llm for MockLlm {
    /// Simulates a call to an LLM by returning a JSON response based on the prompt.
    ///
    /// This function contains the hardcoded logic that drives the ReAct agent's
    /// behavior for the specific task of generating a SHA-256 hash program.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The input prompt from the agent.
    ///
    /// # Returns
    ///
    /// A `Result` containing a JSON string that represents the LLM's "thought"
    /// and the "action" it has decided to take.
    fn call(&self, prompt: &str) -> Result<String> {
        // This is the final, most robust version of the generative logic.
        // It now includes creating a .gitignore file to prevent sandbox errors.

        if prompt.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9") {
            let hash = prompt.lines().find(|line| line.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")).unwrap_or("").trim();
            return Ok(json!({
                "thought": "I have the output from the generated program. The task is complete.",
                "action": { "tool": "Finish", "args": hash }
            }).to_string());
        }

        if prompt.contains("Successfully wrote to ./generated_app/src/main.rs") {
            return Ok(json!({
                "thought": "Source code written. Now I will compile and run the generated program.",
                "action": { "tool": "SystemTool", "args": "cd ./generated_app && cargo run" }
            }).to_string());
        }

        if prompt.contains("Successfully wrote to ./generated_app/.gitignore") {
            let code = r#"
use sha2::{Sha256, Digest};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update("hello world");
    let result = hasher.finalize();
    println!("{:x}", result);
}
"#;
            let args = format!("./generated_app/src/main.rs {}", serde_json::to_string(code)?);
            return Ok(json!({
                "thought": ".gitignore written. Now I will write the `main.rs` file.",
                "action": { "tool": "CodeWriterTool", "args": args }
            }).to_string());
        }

        if prompt.contains("Successfully wrote to ./generated_app/Cargo.toml") {
            let gitignore_content = "target/";
            let args = format!("./generated_app/.gitignore {}", serde_json::to_string(gitignore_content)?);
            return Ok(json!({
                "thought": "Cargo.toml written. Now I will write a `.gitignore` file to exclude the target directory.",
                "action": { "tool": "CodeWriterTool", "args": args }
            }).to_string());
        }

        if prompt.contains("Created binary (application) `generated_app` package") {
            let cargo_toml_content = r#"
[package]
name = "generated_app"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10.8"
"#;
            let args = format!("./generated_app/Cargo.toml {}", serde_json::to_string(cargo_toml_content)?);
            return Ok(json!({
                "thought": "Project directory created. Now I will write the `Cargo.toml` file with the `sha2` dependency.",
                "action": { "tool": "CodeWriterTool", "args": args }
            }).to_string());
        }

        if prompt.starts_with("Task: Generate a program to find the SHA-256 hash of 'hello world'") {
            return Ok(json!({
                "thought": "I need to generate a program. I will start by creating the project directory structure myself.",
                "action": { "tool": "SystemTool", "args": "mkdir -p generated_app/src && echo 'Created binary (application) `generated_app` package'" }
            }).to_string());
        }

        // Fallback for any other state
        Ok(json!({
            "thought": "I am in an unexpected state. I will finish.",
            "action": { "tool": "Finish", "args": "Error: Reached an unknown state in the generation process." }
        }).to_string())
    }
}