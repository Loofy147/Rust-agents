use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::{llm::Llm, tools::Tool};

/// A trait that defines the basic functionality of an agent.
///
/// An agent is an entity that can perform tasks by interacting with its
/// environment through a set of tools.
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
    fn run(&self, task: &str) -> Result<String>;
}

/// Represents an action to be taken by the agent.
///
/// An action consists of a tool to be used and the arguments to pass to that
/// tool.
#[derive(Deserialize)]
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
#[derive(Deserialize)]
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
pub struct ReActAgent<'a> {
    llm: &'a dyn Llm,
    tools: HashMap<String, &'a dyn Tool>,
}

impl<'a> ReActAgent<'a> {
    /// Creates a new `ReActAgent`.
    ///
    /// # Arguments
    ///
    /// * `llm` - A reference to an object that implements the `Llm` trait.
    /// * `tools` - A vector of references to objects that implement the `Tool` trait.
    ///
    /// # Returns
    ///
    /// A new instance of `ReActAgent`.
    pub fn new(llm: &'a dyn Llm, tools: Vec<&'a dyn Tool>) -> Self {
        let mut tool_map = HashMap::new();
        for tool in tools {
            tool_map.insert(tool.name().to_string(), tool);
        }

        Self { llm, tools: tool_map }
    }
}

impl<'a> Agent for ReActAgent<'a> {
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