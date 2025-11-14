use anyhow::Result;
use serde_json::Value;

pub fn parse_ini(content: &str) -> Result<Value> {
    let mut result = serde_json::Map::new();
    let mut current_section = String::new();
    let mut global_section = serde_json::Map::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            result.insert(
                current_section.clone(),
                Value::Object(serde_json::Map::new()),
            );
        } else if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();

            if current_section.is_empty() {
                global_section.insert(key, Value::String(value));
            } else if let Some(Value::Object(section)) = result.get_mut(&current_section) {
                section.insert(key, Value::String(value));
            }
        }
    }

    // Add global section if it exists
    if !global_section.is_empty() {
        result.insert("default".to_string(), Value::Object(global_section));
    }

    Ok(Value::Object(result))
}
