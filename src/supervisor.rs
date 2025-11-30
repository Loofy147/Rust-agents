use crate::{agent::Agent, llm::Llm};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

#[derive(Deserialize)]
struct RoutingDecision {
    worker: String,
    task: String,
}

/// An agent responsible for supervising a team of worker agents and routing
/// tasks to the appropriate worker.
pub struct SupervisorAgent {
    llm: Arc<dyn Llm + Send + Sync>,
    workers: HashMap<String, Box<dyn Agent + Send + Sync>>,
}

impl SupervisorAgent {
    /// Creates a new `SupervisorAgent`.
    pub fn new(
        llm: Arc<dyn Llm + Send + Sync>,
        workers: HashMap<String, Box<dyn Agent + Send + Sync>>,
    ) -> Self {
        Self { llm, workers }
    }

    /// Constructs the prompt for the supervisor agent.
    fn construct_prompt(&self, task: &str) -> String {
        let worker_descriptions = self
            .workers
            .values()
            .map(|w| format!("- {}: {}", w.name(), w.description()))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "You are a supervisor agent. Your job is to route a task to the correct worker agent.

The available workers are:
{}

The task is: {}

Please respond with a JSON object containing the name of the `worker` to use and the `task` to give them. The task can be the original task, or a more specific version of it.

Example:
```json
{{
    \"worker\": \"FileSystemAgent\",
    \"task\": \"Read the content of the file src/main.rs\"
}}
```",
            worker_descriptions, task
        )
    }
}

#[async_trait]
impl Agent for SupervisorAgent {
    fn name(&self) -> String {
        "SupervisorAgent".to_string()
    }

    fn description(&self) -> String {
        "A supervisor agent that routes tasks to the correct worker agent.".to_string()
    }

    /// Runs the supervisor agent loop to route the given task.
    ///
    /// This method uses the LLM to decide which worker is best suited for the
    /// task and then delegates the task to that worker.
    ///
    /// # Arguments
    ///
    /// * `task` - The task for the agent to solve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final answer from the worker agent.
    #[tracing::instrument(skip(self))]
    async fn run(&self, task: &str) -> Result<String> {
        let prompt = self.construct_prompt(task);

        info!("---SUPERVISOR PROMPT---\n{}---END---\n", prompt);

        let llm_response = self.llm.call(&prompt).await?;
        let decision: RoutingDecision = serde_json::from_str(&llm_response)?;

        info!(
            "---SUPERVISOR DECISION---\nWorker: {}, Task: {}---END---\n",
            decision.worker, decision.task
        );

        let worker = self
            .workers
            .get(&decision.worker)
            .ok_or_else(|| anyhow::anyhow!("Worker not found: {}", decision.worker))?;

        worker.run(&decision.task).await
    }
}
