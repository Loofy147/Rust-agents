use anyhow::Result;

pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, args: &str) -> Result<String>;
}

pub mod code_writer;
pub mod system;