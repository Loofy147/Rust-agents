use anyhow::Result;
use std::fs;
use crate::tools::Tool;

pub struct CodeWriterTool;

impl Tool for CodeWriterTool {
    fn name(&self) -> &str {
        "CodeWriterTool"
    }

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