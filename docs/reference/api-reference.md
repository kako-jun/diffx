# API Reference - diffx-core

Complete API documentation for the `diffx-core` Rust crate, providing semantic diff functionality for structured data.

## Overview

The `diffx-core` crate is the heart of the diffx ecosystem, providing fast and accurate semantic diff operations for structured data formats. It can be embedded in other Rust applications to add semantic comparison capabilities.

**Unified API Design**: The core API exposes only a single main function `diff()` for all comparison operations. All functionality is accessed through this unified interface using the options parameter. This design ensures consistency and simplicity across all use cases.

## Installation

Add `diffx-core` to your `Cargo.toml`:

```toml
[dependencies]
diffx-core = "0.2.0"
```

### Feature Flags

```toml
[dependencies]
diffx-core = { version = "0.2.0", features = ["all-formats"] }
```

Available features:
- `json` (default) - JSON format support
- `yaml` (default) - YAML format support  
- `toml` (default) - TOML format support
- `xml` - XML format support
- `ini` - INI format support
- `csv` - CSV format support
- `all-formats` - Enable all format parsers

## Public API

### Core Types

#### `DiffResult`

Represents a single semantic difference between two structured values.

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),           // New key/value added
    Removed(String, Value),         // Key/value removed
    Modified(String, Value, Value), // Value changed (old, new)
    TypeChanged(String, Value, Value), // Type changed (old, new)
}
```

**Fields:**
- **Path** (`String`): JSON path to the changed element (e.g., `"config.database.port"`)
- **Values** (`Value`): serde_json::Value representing the data

**Examples:**
```rust
use diffx_core::DiffResult;
use serde_json::Value;

// Key addition
let added = DiffResult::Added(
    "database.port".to_string(),
    Value::Number(5432.into())
);

// Value modification  
let modified = DiffResult::Modified(
    "version".to_string(),
    Value::String("1.0".to_string()),
    Value::String("1.1".to_string())
);

// Type change
let type_changed = DiffResult::TypeChanged(
    "debug".to_string(),
    Value::String("true".to_string()),
    Value::Bool(true)
);
```

### Core Functions

#### `diff()`

Primary function for computing semantic differences between two structured values. This is the unified API entry point for all comparison operations.

```rust
pub fn diff(
    old: &Value,
    new: &Value,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>, Error>
```

**Parameters:**
- `old`: Original/baseline value to compare
- `new`: New/target value to compare  
- `options`: Optional configuration options for the comparison

**Returns:** `Result<Vec<DiffResult>, Error>` representing all differences found

#### DiffOptions Structure

```rust
pub struct DiffOptions {
    // Core comparison options
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub ignore_keys_regex: Option<Regex>,
    pub path_filter: Option<String>,
    
    // Output control
    pub output_format: Option<OutputFormat>,
    pub show_unchanged: Option<bool>,
    pub show_types: Option<bool>,
    
    // Memory optimization
    pub use_memory_optimization: Option<bool>,
    pub batch_size: Option<usize>,
    
    // diffx-specific options
    pub diffx_options: Option<DiffxSpecificOptions>,
}
```

**Example:**
```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::{json, Value};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old = json!({
        "name": "myapp",
        "version": "1.0",
        "timestamp": "2024-01-01T00:00:00Z"
    });
    
    let new = json!({
        "name": "myapp",
        "version": "1.1", 
        "timestamp": "2024-01-02T00:00:00Z",
        "port": 8080
    });
    
    // Configure options to ignore timestamp changes
    let options = DiffOptions {
        ignore_keys_regex: Some(Regex::new(r"^timestamp$")?),
        show_unchanged: Some(false),
        ..Default::default()
    };
    
    let differences = diff(&old, &new, Some(&options))?;
    
    for diff in differences {
        match diff {
            DiffResult::Added(path, value) => {
                println!("Added {}: {}", path, value);
            }
            DiffResult::Modified(path, old, new) => {
                println!("Modified {}: {} -> {}", path, old, new);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```



## Advanced Usage

### Custom Comparison Logic

#### Epsilon Comparison

Handle floating-point precision differences:

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;

let old = json!({"pi": 3.14159});
let new = json!({"pi": 3.14160});

// Without epsilon - reports difference
let diffs_strict = diff(&old, &new, None)?;
assert!(!diffs_strict.is_empty());

// With epsilon - no difference
let options = DiffOptions {
    epsilon: Some(0.001),
    ..Default::default()
};
let diffs_epsilon = diff(&old, &new, Some(&options))?;
assert!(diffs_epsilon.is_empty());
```

#### Regex Key Filtering

Ignore specific keys or patterns:

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;
use regex::Regex;

let old = json!({
    "data": {"important": "value"},
    "timestamp": "2024-01-01T00:00:00Z",
    "_internal": "system_data"
});

let new = json!({
    "data": {"important": "new_value"},
    "timestamp": "2024-01-02T00:00:00Z", 
    "_internal": "different_system_data"
});

// Ignore timestamp and internal fields
let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new(r"^(timestamp|_.*)")?),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;

// Only reports the important data change
assert_eq!(differences.len(), 1);
```

#### Array Element Tracking

Track array elements by ID instead of position:

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;

let old = json!({
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
});

let new = json!({
    "users": [
        {"id": 2, "name": "Bob"}, 
        {"id": 1, "name": "Alice Smith"}  // Name changed
    ]
});

// With ID tracking - detects name change
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;
// Reports: Modified users[id=1].name: "Alice" -> "Alice Smith"

// Without ID tracking - reports all as changed due to position
let differences_positional = diff(&old, &new, None)?;
// Reports multiple changes due to position differences
```

### Working with Different Formats

#### Complete Format Processing Pipeline

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::{from_str, Value};
use std::fs;

fn compare_files(
    file1_path: &str,
    file2_path: &str,
    format: &str,
    options: Option<&DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = fs::read_to_string(file1_path)?;
    let content2 = fs::read_to_string(file2_path)?;
    
    // Users should use standard parsers for their formats
    let (old, new) = match format {
        "json" => {
            (from_str(&content1)?, from_str(&content2)?)
        }
        "yaml" => {
            (serde_yml::from_str(&content1)?, serde_yml::from_str(&content2)?)
        }
        "toml" => {
            (toml::from_str(&content1)?, toml::from_str(&content2)?)
        }
        _ => return Err(format!("Unsupported format: {}", format).into())
    };
    
    Ok(diff(&old, &new, options)?)
}
```

### Integration Patterns

#### Custom Diff Processing

```rust
use diffx_core::{diff, DiffResult};
use serde_json::Value;

struct DiffProcessor {
    pub additions: Vec<(String, Value)>,
    pub removals: Vec<(String, Value)>,
    pub modifications: Vec<(String, Value, Value)>,
    pub type_changes: Vec<(String, Value, Value)>,
}

impl DiffProcessor {
    pub fn new() -> Self {
        Self {
            additions: Vec::new(),
            removals: Vec::new(),
            modifications: Vec::new(),
            type_changes: Vec::new(),
        }
    }
    
    pub fn process(&mut self, differences: Vec<DiffResult>) {
        for diff_result in differences {
            match diff_result {
                DiffResult::Added(path, value) => {
                    self.additions.push((path, value));
                }
                DiffResult::Removed(path, value) => {
                    self.removals.push((path, value));
                }
                DiffResult::Modified(path, old, new) => {
                    self.modifications.push((path, old, new));
                }
                DiffResult::TypeChanged(path, old_type, new_type) => {
                    // Note: TypeChanged now contains type strings, not values
                    self.type_changes.push((path, old_type.into(), new_type.into()));
                }
            }
        }
    }
    
    pub fn has_critical_changes(&self) -> bool {
        // Define what constitutes "critical" changes
        !self.removals.is_empty() || 
        !self.type_changes.is_empty() ||
        self.modifications.iter().any(|(path, _, _)| {
            path.contains("security") || path.contains("database")
        })
    }
}
```

#### Async Processing

```rust
use diffx_core::{diff, DiffResult};
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = vec![
        process_diff_async("file1.json", "file2.json"),
        process_diff_async("file3.json", "file4.json"),
    ];
    
    let results = futures::future::try_join_all(tasks).await?;
    
    for (i, diffs) in results.into_iter().enumerate() {
        println!("File pair {}: {} differences", i + 1, diffs.len());
    }
    
    Ok(())
}

async fn process_diff_async(
    file1: &str,
    file2: &str,
    options: Option<DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = tokio::fs::read_to_string(file1).await?;
    let content2 = tokio::fs::read_to_string(file2).await?;
    
    // Parse in background task to avoid blocking
    let result = tokio::task::spawn_blocking(move || {
        let old: Value = serde_json::from_str(&content1)?;
        let new: Value = serde_json::from_str(&content2)?;
        diff(&old, &new, options.as_ref())
    }).await??;
    
    Ok(result)
}
```

## Error Handling

### Error Types

The library uses `anyhow::Error` for error handling:

```rust
use diffx_core::parse_ini;
use anyhow::Result;

fn handle_parse_errors() -> Result<()> {
    let invalid_ini = "invalid [section syntax";
    
    match parse_ini(invalid_ini) {
        Ok(value) => println!("Parsed successfully: {}", value),
        Err(e) => {
            eprintln!("Parse error: {}", e);
            
            // Chain of error causes
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("Caused by: {}", err);
                source = err.source();
            }
        }
    }
    
    Ok(())
}
```

### Common Error Scenarios

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// Handle multiple format possibilities
fn robust_comparison(
    data1: &str,
    data2: &str,
    options: Option<&DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // Attempt to parse as JSON first
    let old = serde_json::from_str::<Value>(data1)
        .or_else(|_| serde_yml::from_str::<Value>(data1))
        .or_else(|_| toml::from_str::<Value>(data1))?;
    
    let new = serde_json::from_str::<Value>(data2)
        .or_else(|_| serde_yml::from_str::<Value>(data2))
        .or_else(|_| toml::from_str::<Value>(data2))?;
    
    Ok(diff(&old, &new, options)?)
}
```

## Performance Considerations

### Memory Usage

For large datasets:

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// Process large files efficiently
fn process_large_diff(
    old: &Value,
    new: &Value,
    focus_path: Option<&str>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let options = if let Some(path) = focus_path {
        DiffOptions {
            path_filter: Some(path.to_string()),
            use_memory_optimization: Some(true),
            ..Default::default()
        }
    } else {
        DiffOptions {
            use_memory_optimization: Some(true),
            ..Default::default()
        }
    };
    
    Ok(diff(old, new, Some(&options))?)
```

### Optimization Tips

1. **Use regex filtering** to ignore large, irrelevant sections
2. **Specify epsilon** for floating-point heavy data
3. **Use array ID keys** for large arrays with identifiable elements
4. **Consider path filtering** for very large objects

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_basic_diff() {
        let old = json!({"a": 1, "b": 2});
        let new = json!({"a": 1, "b": 3, "c": 4});
        
        let diffs = diff(&old, &new, None).unwrap();
        
        assert_eq!(diffs.len(), 2);
        // Test specific differences...
    }
    
    #[test]
    fn test_epsilon_comparison() {
        let old = json!({"value": 1.0});
        let new = json!({"value": 1.0001});
        
        let diffs_strict = diff(&old, &new, None).unwrap();
        assert!(!diffs_strict.is_empty());
        
        let options = DiffOptions {
            epsilon: Some(0.001),
            ..Default::default()
        };
        let diffs_epsilon = diff(&old, &new, Some(&options)).unwrap();
        assert!(diffs_epsilon.is_empty());
    }
}
```

## Version Compatibility

- **0.2.x**: Current stable version
- **Minimum Rust version**: 1.70.0
- **Dependencies**: See `Cargo.toml` for current versions

## See Also

- [CLI Reference](cli-reference.md) for command-line usage
- [Getting Started Guide](../user-guide/getting-started.md) for basic concepts
- [Examples](../user-guide/examples.md) for practical use cases
- [Unified API Reference](../bindings/unified-api.md) for language bindings
