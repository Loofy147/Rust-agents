use anyhow::Result;
use async_trait::async_trait;

/// A trait representing a tool that can be executed by an agent.
///
/// Tools are the primary means by which an agent can interact with its environment.
/// Each tool has a unique name and an `execute` method that performs its action.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the name of the tool.
    ///
    /// The name should be a unique identifier for the tool.
    fn name(&self) -> &str;

    /// Executes the tool with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A string containing the arguments for the tool. The format of
    ///   the arguments is specific to each tool.
    ///
    /// # Returns
    ///
    /// A `Result` containing a string with the output of the tool's execution,
    /// or an error if the execution fails.
    async fn execute(&self, args: &str) -> Result<String>;
}

pub mod code_writer;
pub mod system;