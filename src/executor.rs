use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tracing::info;

use crate::{
    agent::Agent,
    llm::Llm,
    tools::Tool,
};

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
pub struct ExecutorAgent {
    llm: Arc<dyn Llm + Send + Sync>,
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
    name: String,
    description: String,
}

impl ExecutorAgent {
    /// Creates a new `ExecutorAgent`.
    ///
    /// # Arguments
    ///
    /// * `llm` - A reference to an object that implements the `Llm` trait.
    /// * `tools` - A vector of objects that implement the `Tool` trait.
    /// * `name` - The name of the agent.
    /// * `description` - A description of the agent's purpose.
    ///
    /// # Returns
    ///
    /// A new instance of `ExecutorAgent`.
    pub fn new(
        llm: Arc<dyn Llm + Send + Sync>,
        tools: Vec<Box<dyn Tool + Send + Sync>>,
        name: &str,
        description: &str,
    ) -> Self {
        let mut tool_map = HashMap::new();
        for tool in tools {
            tool_map.insert(tool.name().to_string(), tool);
        }

        Self {
            llm,
            tools: tool_map,
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    /// Constructs the initial prompt for the agent.
    ///
    /// This function creates a detailed prompt that includes the task, the
    /// available tools, and instructions on how to format the response.
    ///
    /// # Arguments
    ///
    /// * `task` - The task for the agent to complete.
    ///
    /// # Returns
    ///
    /// A string containing the initial prompt.
    fn construct_initial_prompt(&self, task: &str) -> String {
        let tool_names = self
            .tools
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        format!(
            "You are a helpful assistant. Your task is to {}.

You have the following tools available: {}.

Please respond with a JSON object containing your `thought` and the `action` you want to take. The `action` should have a `tool` and `args`.

Example:
```json
{{
    \"thought\": \"I should write the code to a file.\",
    \"action\": {{
        \"tool\": \"CodeWriterTool\",
        \"args\": \"./src/main.rs 'fn main() {{ println!(\\\"hello world\\\"); }}'\"
    }}
}}
```

If you have completed the task, use the `Finish` tool with the final answer.",
            task, tool_names
        )
    }
}

#[async_trait]
impl Agent for ExecutorAgent {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

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
    #[tracing::instrument(skip(self))]
    async fn run(&self, task: &str) -> Result<String> {
        let mut prompt = self.construct_initial_prompt(task);
        loop {
            info!("---PROMPT---\n{}---END---\n", prompt);

            let llm_response = self.llm.call(&prompt).await?;
            let thought: Thought = serde_json::from_str(&llm_response)?;

            info!("---THOUGHT---\n{}---END---\n", thought.thought);
            info!(
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

            let observation = tool.execute(&thought.action.args).await?;

            info!("---OBSERVATION---\n{}---END---\n", observation);

            prompt = format!(
                "{}\nThought: {}\nAction: {}\nObservation: {}",
                prompt, thought.thought, llm_response, observation
            );
        }
    }
}