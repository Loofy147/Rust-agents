use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tracing::{debug, info};

use crate::{
    llm::Llm,
    tools::Tool,
};

/// A trait that defines the basic functionality of an agent.
///
/// An agent is an entity that can perform tasks by interacting with its
/// environment through a set of tools.
#[async_trait]
pub trait Agent {
    /// Runs the agent to complete a given task.
    ///
    /// # Arguments
    ///
    /// * `task` - A string describing the task for the agent to complete.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final answer or result of the task, or an
    /// error if the agent fails to complete the task.
    async fn run(&self, task: &str) -> Result<String>;
}

/// Represents an action to be taken by the agent.
///
/// An action consists of a tool to be used and the arguments to pass to that
/// tool.
#[derive(Deserialize, Debug)]
pub struct Action {
    /// The name of the tool to execute.
    pub tool: String,
    /// The arguments to pass to the tool.
    pub args: String,
}

/// Represents a thought process and the resulting action.
///
/// This struct is used to deserialize the JSON output from the LLM, which
/// contains the agent's "thought" about what to do next and the "action" it
/// plans to take.
#[derive(Deserialize, Debug)]
pub struct Thought {
    /// The reasoning behind the action.
    pub thought: String,
    /// The action to be taken.
    pub action: Action,
}

/// An implementation of the `Agent` trait that uses the ReAct (Reasoning and Acting) framework.
///
/// The `ReActAgent` works by iteratively reasoning about a task, taking an
/// action, observing the outcome, and then repeating the cycle until the task
/// is complete.
pub struct ReActAgent<L: Llm> {
    llm: L,
    tools: HashMap<String, Box<dyn Tool>>,
}

impl<L: Llm> ReActAgent<L> {
    /// Creates a new `ReActAgent`.
    ///
    /// # Arguments
    ///
    /// * `llm` - An object that implements the `Llm` trait.
    /// * `tools` - A vector of objects that implement the `Tool` trait.
    ///
    /// # Returns
    ///
    /// A new instance of `ReActAgent`.
    pub fn new(llm: L, tools: Vec<Box<dyn Tool>>) -> Self {
        let mut tool_map = HashMap::new();
        for tool in tools {
            tool_map.insert(tool.name().to_string(), tool);
        }

        Self { llm, tools: tool_map }
    }
}

#[async_trait]
impl<L: Llm> Agent for ReActAgent<L> {
    /// Runs the ReAct agent loop to complete the given task.
    ///
    /// This method implements the core ReAct logic:
    /// 1. The agent is prompted with the current task and history.
    /// 2. The LLM generates a `Thought` and an `Action`.
    /// 3. The `Action` is executed using the appropriate tool.
    /// 4. The result of the action (`Observation`) is added to the prompt history.
    /// 5. The loop continues until the LLM outputs a "Finish" action.
    ///
    /// # Arguments
    ///
    /// * `task` - The task for the agent to solve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final answer from the "Finish" action, or an
    /// error if something goes wrong.
    async fn run(&self, task: &str) -> Result<String> {
        let mut prompt = format!("Task: {}\n", task);
        loop {
            debug!("Prompt: {}", prompt);

            let llm_response = self.llm.call(&prompt).await?;
            let thought: Thought = serde_json::from_str(&llm_response)?;

            info!(thought = %thought.thought, "Llm thought");
            info!(
                tool = %thought.action.tool,
                args = %thought.action.args,
                "Llm action"
            );

            if thought.action.tool == "Finish" {
                return Ok(thought.action.args);
            }

            let tool = self
                .tools
                .get(&thought.action.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", thought.action.tool))?;

            let observation = tool.execute(&thought.action.args).await?;

            info!(observation = %observation, "Tool observation");

            prompt = format!(
                "{}\nThought: {}\nAction: {}\nObservation: {}",
                prompt, thought.thought, llm_response, observation
            );
        }
    }
}