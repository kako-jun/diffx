// Utility functions - FOR INTERNAL USE ONLY
// These functions are public only for CLI and language bindings.
// External users should use the main diff() function.

use crate::{DiffOptions, DiffResult, OutputFormat};
use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value;

/// Get type name of a JSON value - FOR INTERNAL USE ONLY
pub fn value_type_name(value: &Value) -> &str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}

/// Format output to string - FOR INTERNAL USE ONLY
pub fn format_output<T: Serialize>(results: &[T], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)
            .map_err(|e| anyhow!("JSON serialization error: {}", e)),
        OutputFormat::Yaml => {
            serde_yaml::to_string(results).map_err(|e| anyhow!("YAML serialization error: {}", e))
        }
        OutputFormat::Diffx => {
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string(result)?;
                output.push_str(&json);
                output.push('\n');
            }
            Ok(output)
        }
    }
}

/// Format DiffResult output using proper Display implementation
pub fn format_diff_output(
    results: &[DiffResult],
    format: OutputFormat,
    _options: Option<&DiffOptions>,
) -> Result<String> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)
            .map_err(|e| anyhow!("JSON serialization error: {}", e)),
        OutputFormat::Yaml => {
            let mut output = String::new();
            for result in results {
                match result {
                    DiffResult::Added(path, value) => {
                        output.push_str("- Added:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::Removed(path, value) => {
                        output.push_str("- Removed:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::Modified(path, old_value, new_value) => {
                        output.push_str("- Modified:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(old_value).unwrap_or_default().trim()
                        ));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(new_value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::TypeChanged(path, old_value, new_value) => {
                        output.push_str("- TypeChanged:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(old_value).unwrap_or_default().trim()
                        ));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(new_value).unwrap_or_default().trim()
                        ));
                    }
                }
            }
            Ok(output)
        }
        OutputFormat::Diffx => {
            let mut output = String::new();
            for result in results {
                output.push_str(&result.to_string());
                output.push('\n');
            }
            Ok(output)
        }
    }
}
