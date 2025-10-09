use anyhow::Result;
use std::fs;
use crate::tools::Tool;

/// A tool for writing code to files.
///
/// This tool takes a filepath and a JSON string containing the content to be
/// written to the file. It is used by the agent to create or overwrite files
/// in the filesystem.
pub struct CodeWriterTool;

impl Tool for CodeWriterTool {
    /// Returns the name of the tool, "CodeWriterTool".
    fn name(&self) -> &str {
        "CodeWriterTool"
    }

    /// Executes the code writing command.
    ///
    /// # Arguments
    ///
    /// * `args` - A string containing the filepath and the content to write,
    ///   separated by a space. The content should be a JSON-encoded string.
    ///   Example: `<filepath> "<json_string_content>"`
    ///
    /// # Returns
    ///
    /// A `Result` with a success message if the file was written correctly, or
    /// an error if the arguments are invalid or the file cannot be written.
    fn execute(&self, args: &str) -> Result<String> {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid arguments for CodeWriterTool. Expected: <filepath> <json_string_content>"));
        }
        let filepath = parts[0];
        let content_json = parts[1];

        // The content is a JSON string, so we need to parse it to get the raw string.
        let content: String = serde_json::from_str(content_json)?;

        fs::write(filepath, content)?;
        Ok(format!("Successfully wrote to {}", filepath))
    }
}