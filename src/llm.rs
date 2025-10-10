use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::Settings;

/// A trait for Large Language Models (LLMs).
///
/// This trait defines the interface for a large language model, which is a
/// core component of the agent. The LLM is responsible for generating responses
/// based on a given prompt.
#[async_trait]
pub trait Llm: Send + Sync {
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
    async fn call(&self, prompt: &str) -> Result<String>;
}

/// A mock implementation of the `Llm` trait for testing and demonstration.
///
/// `MockLlm` simulates the behavior of a real LLM by returning canned responses
/// based on the content of the prompt. This allows for predictable testing of
/// the agent's logic without making actual API calls.
pub struct MockLlm;

#[async_trait]
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
    async fn call(&self, prompt: &str) -> Result<String> {
        if prompt.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9") {
            let observation_line = prompt
                .lines()
                .find(|line| {
                    line.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
                })
                .unwrap_or("");

            // The observation line is `Observation: <hash>`, so we split and take the last part.
            let hash = observation_line.split_whitespace().last().unwrap_or("");

            return Ok(json!({
                "thought": "I have the output from the generated program. The task is complete.",
                "action": { "tool": "Finish", "args": hash }
            })
            .to_string());
        }

        if prompt.contains("Successfully wrote to ./generated_app/src/main.rs") {
            return Ok(json!({
                "thought": "Source code written. Now I will compile and run the generated program.",
                "action": { "tool": "SystemTool", "args": "cd ./generated_app && cargo run" }
            })
            .to_string());
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
            let args =
                format!("./generated_app/src/main.rs {}", serde_json::to_string(code)?);
            return Ok(json!({
                "thought": ".gitignore written. Now I will write the `main.rs` file.",
                "action": { "tool": "CodeWriterTool", "args": args }
            })
            .to_string());
        }

        if prompt.contains("Successfully wrote to ./generated_app/Cargo.toml") {
            let gitignore_content = "target/";
            let args = format!(
                "./generated_app/.gitignore {}",
                serde_json::to_string(gitignore_content)?
            );
            return Ok(json!({
                "thought": "Cargo.toml written. Now I will write a `.gitignore` file to exclude the target directory.",
                "action": { "tool": "CodeWriterTool", "args": args }
            })
            .to_string());
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
            let args = format!(
                "./generated_app/Cargo.toml {}",
                serde_json::to_string(cargo_toml_content)?
            );
            return Ok(json!({
                "thought": "Project directory created. Now I will write the `Cargo.toml` file with the `sha2` dependency.",
                "action": { "tool": "CodeWriterTool", "args": args }
            })
            .to_string());
        }

        if prompt.starts_with("Task: Generate a program to find the SHA-256 hash of 'hello world'")
        {
            return Ok(json!({
                "thought": "I need to generate a program. I will start by creating the project directory structure myself.",
                "action": { "tool": "SystemTool", "args": "mkdir -p generated_app/src && echo 'Created binary (application) `generated_app` package'" }
            })
            .to_string());
        }

        // Fallback for any other state
        Ok(json!({
            "thought": "I am in an unexpected state. I will finish.",
            "action": { "tool": "Finish", "args": "Error: Reached an unknown state in the generation process." }
        })
        .to_string())
    }
}

/// An implementation of the `Llm` trait that communicates with the OpenAI API.
pub struct OpenAiLlm {
    client: Client,
    settings: Settings,
}

impl OpenAiLlm {
    /// Creates a new `OpenAiLlm` client.
    ///
    /// # Arguments
    ///
    /// * `settings` - The application settings, containing the API key and model name.
    ///
    /// # Returns
    ///
    /// A new instance of `OpenAiLlm`.
    pub fn new(settings: Settings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChatChoice>,
}

#[derive(Deserialize, Debug)]
struct OpenAiChatChoice {
    message: ChatMessageResponse,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ChatMessageResponse {
    role: String,
    content: String,
}

#[async_trait]
impl Llm for OpenAiLlm {
    /// Makes a call to the OpenAI Chat Completions API.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt to send to the model.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of the model's response, or an error.
    async fn call(&self, prompt: &str) -> Result<String> {
        let request_body = OpenAiChatRequest {
            model: self.settings.llm.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response: OpenAiChatResponse = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.settings.llm.api_key)
            .json(&request_body)
            .send()
            .await?
            .json::<OpenAiChatResponse>()
            .await?;

        if let Some(choice) = response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No response from LLM"))
        }
    }
}