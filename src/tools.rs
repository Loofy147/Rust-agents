use anyhow::Result;

pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, args: &str) -> Result<String>;
}

pub struct CalculatorTool;

impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "CalculatorTool"
    }

    fn execute(&self, args: &str) -> Result<String> {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid expression: {}", args));
        }

        let left = parts[0].parse::<f64>()?;
        let op = parts[1];
        let right = parts[2].parse::<f64>()?;

        let result = match op {
            "+" => left + right,
            "*" => left * right,
            _ => return Err(anyhow::anyhow!("Unknown operator: {}", op)),
        };

        Ok(result.to_string())
    }
}