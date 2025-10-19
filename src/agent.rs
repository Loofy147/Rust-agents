use anyhow::Result;
use async_trait::async_trait;

/// A trait that defines the basic functionality of an agent.
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
