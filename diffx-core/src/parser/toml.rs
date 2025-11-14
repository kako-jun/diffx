use anyhow::{anyhow, Result};
use serde_json::Value;

pub fn parse_toml(content: &str) -> Result<Value> {
    let toml_value: toml::Value = content.parse()?;
    toml_to_json_value(toml_value)
}

fn toml_to_json_value(toml_val: toml::Value) -> Result<Value> {
    match toml_val {
        toml::Value::String(s) => Ok(Value::String(s)),
        toml::Value::Integer(i) => Ok(Value::Number(i.into())),
        toml::Value::Float(f) => Ok(Value::Number(
            serde_json::Number::from_f64(f).ok_or_else(|| anyhow!("Invalid float"))?,
        )),
        toml::Value::Boolean(b) => Ok(Value::Bool(b)),
        toml::Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(toml_to_json_value(item)?);
            }
            Ok(Value::Array(json_arr))
        }
        toml::Value::Table(table) => {
            let mut json_obj = serde_json::Map::new();
            for (key, value) in table {
                json_obj.insert(key, toml_to_json_value(value)?);
            }
            Ok(Value::Object(json_obj))
        }
        toml::Value::Datetime(dt) => Ok(Value::String(dt.to_string())),
    }
}
