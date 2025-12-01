use crate::tools::Tool;
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

/// A tool for executing system commands.
///
/// This tool allows the agent to run arbitrary shell commands. It captures
/// both stdout and stderr and returns them as a single string.
pub struct SystemTool;

#[async_trait]
impl Tool for SystemTool {
    /// Returns the name of the tool, "SystemTool".
    fn name(&self) -> &str {
        "SystemTool"
    }

    /// Executes a system command.
    ///
    /// # Arguments
    ///
    /// * `args` - The shell command to execute.
    ///
    /// # Returns
    ///
    /// A `Result` containing the combined stdout and stderr of the command,
    /// or an error if the command fails to execute.
    #[tracing::instrument(skip(self))]
    async fn execute(&self, args: &str) -> Result<String> {
        #[cfg(feature = "sandbox")]
        {
            // Run the command inside a Docker container for sandboxing.
            // This requires Docker to be installed and running on the host.
            let output = Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("alpine")
                .arg("sh")
                .arg("-c")
                .arg(args)
                .output()
                .await?;

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
        #[cfg(not(feature = "sandbox"))]
        {
            // Run the command directly on the host.
            // This is not secure and should only be used for testing.
            let output = Command::new("sh").arg("-c").arg(args).output().await?;

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
}