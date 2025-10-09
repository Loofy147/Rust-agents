use anyhow::Result;

/// A trait representing a tool that can be executed by an agent.
///
/// Tools are the primary means by which an agent can interact with its environment.
/// Each tool has a unique name and an `execute` method that performs its action.
pub trait Tool {
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
    fn execute(&self, args: &str) -> Result<String>;
}

pub mod code_writer;
pub mod system;