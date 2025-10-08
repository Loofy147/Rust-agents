use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::{llm::Llm, tools::Tool};

pub trait Agent {
    fn run(&self, task: &str) -> Result<String>;
}

#[derive(Deserialize)]
struct Action {
    tool: String,
    args: String,
}

#[derive(Deserialize)]
struct Thought {
    thought: String,
    action: Action,
}

pub struct ReActAgent<'a> {
    llm: &'a dyn Llm,
    tools: HashMap<String, &'a dyn Tool>,
}

impl<'a> ReActAgent<'a> {
    pub fn new(llm: &'a dyn Llm, tools: Vec<&'a dyn Tool>) -> Self {
        let mut tool_map = HashMap::new();
        for tool in tools {
            tool_map.insert(tool.name().to_string(), tool);
        }

        Self { llm, tools: tool_map }
    }
}

impl<'a> Agent for ReActAgent<'a> {
    fn run(&self, task: &str) -> Result<String> {
        let mut prompt = format!("Task: {}\n", task);
        loop {
            println!("---PROMPT---\n{}---END---\n", prompt);

            let llm_response = self.llm.call(&prompt)?;
            let thought: Thought = serde_json::from_str(&llm_response)?;

            println!("---THOUGHT---\n{}---END---\n", thought.thought);
            println!(
                "---ACTION---\nTool: {}, Args: {}---END---\n",
                thought.action.tool, thought.action.args
            );

            if thought.action.tool == "Finish" {
                return Ok(thought.action.args);
            }

            let tool = self
                .tools
                .get(&thought.action.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", thought.action.tool))?;

            let observation = tool.execute(&thought.action.args)?;

            println!("---OBSERVATION---\n{}---END---\n", observation);

            prompt = format!(
                "{}\nThought: {}\nAction: {}\nObservation: {}",
                prompt, thought.thought, llm_response, observation
            );
        }
    }
}