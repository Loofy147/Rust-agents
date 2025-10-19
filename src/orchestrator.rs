use crate::{
    agent::Agent,
    executor::ExecutorAgent,
    planner::PlannerAgent,
};
use anyhow::Result;
use tracing::info;

/// The orchestrator is responsible for managing the agents and the overall
/// workflow of the multi-agent system.
pub struct Orchestrator<'a> {
    planner: PlannerAgent,
    executor: ExecutorAgent<'a>,
}

impl<'a> Orchestrator<'a> {
    /// Creates a new `Orchestrator`.
    pub fn new(planner: PlannerAgent, executor: ExecutorAgent<'a>) -> Self {
        Self { planner, executor }
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
    pub async fn run(&self, task: &str) -> Result<String> {
        info!("Starting orchestrator with task: {}", task);
        let plan = self.planner.run(task).await?;
        info!("Plan created: \n{}", plan);

        let steps: Vec<&str> = plan.lines().collect();
        let mut results = Vec::new();

        for step in steps {
            let result = self.executor.run(step).await?;
            info!("Step completed: {}\nResult: {}", step, result);
            results.push(result);
        }

        Ok(results.join("\n"))
    }
}
