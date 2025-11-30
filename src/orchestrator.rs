use crate::agent::Agent;
use anyhow::Result;
use tracing::info;

/// The orchestrator is responsible for managing the agents and the overall
/// workflow of the multi-agent system.
pub struct Orchestrator {
    supervisor: Box<dyn Agent + Send + Sync>,
}

impl Orchestrator {
    /// Creates a new `Orchestrator`.
    pub fn new(supervisor: Box<dyn Agent + Send + Sync>) -> Self {
        Self { supervisor }
    }

    /// Runs the multi-agent system to complete a given task.
    ///
    /// # Arguments
    ///
    /// * `task` - The high-level task for the system to complete.
    ///
    /// # Returns
    ///
    /// A `Result` with the final result of the task, or an error if the
    /// system fails.
    #[tracing::instrument(skip(self))]
    pub async fn run(&self, task: &str) -> Result<String> {
        info!("Starting orchestrator with task: {}", task);
        let result = self.supervisor.run(task).await?;
        info!("Task completed with result: {}", result);
        Ok(result)
    }
}
