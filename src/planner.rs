use anyhow::Result;
use async_trait::async_trait;
use crate::{agent::Agent, llm::Llm};

/// An agent responsible for creating a plan to accomplish a given task.
pub struct PlannerAgent {
    llm: Box<dyn Llm + Sync>,
}

impl PlannerAgent {
    /// Creates a new `PlannerAgent`.
    pub fn new(llm: Box<dyn Llm + Sync>) -> Self {
        Self { llm }
    }
}

#[async_trait]
impl Agent for PlannerAgent {
    /// Creates a plan to accomplish the given task.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to create a plan for.
    ///
    /// # Returns
    ///
    /// A `Result` with a string containing the plan, or an error if the
    /// plan cannot be created.
    async fn run(&self, task: &str) -> Result<String> {
        let prompt = format!(
            "You are a planner agent. Your job is to create a step-by-step plan to accomplish the following task: {}.

Please respond with a numbered list of steps.",
            task
        );

        self.llm.call(&prompt).await
    }
}
