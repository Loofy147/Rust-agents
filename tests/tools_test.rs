use anyhow::Result;
use app::{
    tools::{code_writer::CodeWriterTool, system::SystemTool, Tool},
};
use serde_json::json;

#[tokio::test]
async fn test_code_writer_tool_success() -> Result<()> {
    let tool = CodeWriterTool;
    let filepath = "./test_output.txt";
    let content = "Hello, from test!";
    let content_json = json!(content).to_string();
    let args = format!("{} {}", filepath, content_json);

    let result = tool.execute(&args).await?;
    assert_eq!(result, format!("Successfully wrote to {}", filepath));

    let written_content = tokio::fs::read_to_string(filepath).await?;
    assert_eq!(written_content, content);

    tokio::fs::remove_file(filepath).await?;
    Ok(())
}

#[tokio::test]
async fn test_code_writer_tool_invalid_args() -> Result<()> {
    let tool = CodeWriterTool;
    let result = tool.execute("invalid args").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_system_tool_success() -> Result<()> {
    let tool = SystemTool;
    let result = tool.execute("echo 'hello system'").await?;
    assert!(result.contains("hello system"));
    Ok(())
}

#[tokio::test]
async fn test_system_tool_failure() -> Result<()> {
    let tool = SystemTool;
    let result = tool.execute("some_non_existent_command").await;
    assert!(result.is_err());
    Ok(())
}