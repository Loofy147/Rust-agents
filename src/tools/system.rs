use anyhow::Result;
use std::process::Command;
use crate::tools::Tool;

pub struct SystemTool;

impl Tool for SystemTool {
    fn name(&self) -> &str {
        "SystemTool"
    }

    fn execute(&self, args: &str) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(args)
            .output()?;

        // Combine stdout and stderr to capture all output.
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        let combined_output = format!("{}\n{}", stdout, stderr);

        if output.status.success() {
            Ok(combined_output)
        } else {
            Err(anyhow::anyhow!(
                "Command failed: {}. Output:\n{}",
                args,
                combined_output
            ))
        }
    }
}