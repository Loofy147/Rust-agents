use anyhow::Result;
use app::{
    agent::{Agent, ReActAgent},
    llm::MockLlm,
    tools::{code_writer::CodeWriterTool, system::SystemTool, Tool},
};

#[tokio::test]
async fn test_react_agent_e2e() -> Result<()> {
    let llm = MockLlm;
    let code_writer = CodeWriterTool;
    let system = SystemTool;

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(code_writer), Box::new(system)];

    let agent = ReActAgent::new(llm, tools);

    let task = "Generate a program to find the SHA-256 hash of 'hello world'";
    let result = agent.run(task).await?;

    assert_eq!(
        result,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );

    // Cleanup generated files
    let _ = tokio::fs::remove_dir_all("./generated_app").await;

    Ok(())
}