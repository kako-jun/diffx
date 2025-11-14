use anyhow::{anyhow, Result};
use serde_json::Value;

pub fn parse_json(content: &str) -> Result<Value> {
    serde_json::from_str(content).map_err(|e| anyhow!("JSON parse error: {}", e))
}
