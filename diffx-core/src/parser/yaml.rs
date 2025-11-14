use anyhow::{anyhow, Result};
use serde_json::Value;

pub fn parse_yaml(content: &str) -> Result<Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow!("YAML parse error: {}", e))
}
