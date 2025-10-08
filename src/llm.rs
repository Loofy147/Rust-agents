use anyhow::Result;

pub trait Llm {
    fn call(&self, prompt: &str) -> Result<String>;
}

pub struct MockLlm;

impl Llm for MockLlm {
    fn call(&self, prompt: &str) -> Result<String> {
        // The order of these checks is critical. We must check for the
        // latest observations first to avoid getting stuck in a loop.
        if prompt.contains("Observation: 32") {
             // After calculating 4*8=32, the task is done.
             Ok(r#"
{
    "thought": "I have the final answer, which is 32. I will now output the final answer.",
    "action": {
        "tool": "Finish",
        "args": "32"
    }
}
"#.to_string())
        } else if prompt.contains("Observation: 8") {
            // After calculating 3+5=8, the next step is to multiply by 4.
            Ok(r#"
{
    "thought": "Now I have the result of the parenthesis, which is 8. I need to multiply 4 by 8 to get the final answer.",
    "action": {
        "tool": "CalculatorTool",
        "args": "4 * 8"
    }
}
"#.to_string())
        } else if prompt.contains("Task: What is 4 * (3 + 5)?") {
            // This is the initial prompt. First step is to calculate inside the parenthesis.
            Ok(r#"
{
    "thought": "I need to evaluate the expression in the parenthesis first, which is 3 + 5. Then I will multiply the result by 4.",
    "action": {
        "tool": "CalculatorTool",
        "args": "3 + 5"
    }
}
"#.to_string())
        } else {
            // Fallback to prevent infinite loops during development.
            Ok(r#"
{
    "thought": "I am lost and in an unknown state. I will finish to avoid a loop.",
    "action": {
        "tool": "Finish",
        "args": "Error: Unknown state in MockLlm."
    }
}
"#.to_string())
        }
    }
}