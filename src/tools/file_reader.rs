use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;

/// A tool for reading files.
///
/// This tool allows the agent to read the content of a file at a given path.
pub struct FileReaderTool;

#[async_trait]
impl Tool for FileReaderTool {
    /// Returns the name of the tool, "FileReaderTool".
    fn name(&self) -> &str {
        "FileReaderTool"
    }

    /// Executes the file reading command.
    ///
    /// # Arguments
    ///
    /// * `args` - The path to the file to read.
    ///
    /// # Returns
    ///
    /// A `Result` with the content of the file, or an error if the file
    /// cannot be read.
    async fn execute(&self, args: &str) -> Result<String> {
        let filepath = args.trim();
        let content = fs::read_to_string(filepath).await?;
        Ok(content)
    }
}
