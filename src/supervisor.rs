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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{agent::Agent, llm::MockLlm};
    use std::{collections::HashMap, sync::Arc};

    struct MockWorker {
        name: String,
        description: String,
    }

    #[async_trait]
    impl Agent for MockWorker {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn description(&self) -> String {
            self.description.clone()
        }

        async fn run(&self, task: &str) -> Result<String> {
            Ok(format!("{} completed task: {}", self.name, task))
        }
    }

    #[tokio::test]
    async fn test_supervisor_agent_routing() {
        let response = serde_json::json!({
            "worker": "Worker1",
            "task": "Do something that worker1 can do"
        })
        .to_string();
        let llm = Arc::new(MockLlm::new(&response));
        let mut workers: HashMap<String, Box<dyn Agent + Send + Sync>> = HashMap::new();

        let worker1 = MockWorker {
            name: "Worker1".to_string(),
            description: "A worker that does worker1 things.".to_string(),
        };
        workers.insert(worker1.name(), Box::new(worker1));

        let worker2 = MockWorker {
            name: "Worker2".to_string(),
            description: "A worker that does worker2 things.".to_string(),
        };
        workers.insert(worker2.name(), Box::new(worker2));

        let supervisor = SupervisorAgent::new(llm, workers);

        let task = "Do something that worker1 can do";
        let result = supervisor.run(task).await.unwrap();

        assert_eq!(result, "Worker1 completed task: Do something that worker1 can do");
    }
}
