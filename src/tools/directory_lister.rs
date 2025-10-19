use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;

/// A tool for listing the contents of a directory.
///
/// This tool allows the agent to see what files and subdirectories exist in a
/// given directory.
pub struct DirectoryListerTool;

#[async_trait]
impl Tool for DirectoryListerTool {
    /// Returns the name of the tool, "DirectoryListerTool".
    fn name(&self) -> &str {
        "DirectoryListerTool"
    }

    /// Executes the directory listing command.
    ///
    /// # Arguments
    ///
    /// * `args` - The path to the directory to list.
    ///
    /// # Returns
    ///
    /// A `Result` with a string containing the names of the files and
    /// directories, separated by newlines, or an error if the directory
    /// cannot be read.
    async fn execute(&self, args: &str) -> Result<String> {
        let dir_path = args.trim();
        let mut entries = fs::read_dir(dir_path).await?;
        let mut entry_names = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            entry_names.push(entry.file_name().to_string_lossy().to_string());
        }

        Ok(entry_names.join("\n"))
    }
}
