# API Reference - diffx-core

Complete API documentation for the `diffx-core` Rust crate, providing semantic diff functionality for structured data.

## Overview

The `diffx-core` crate is the heart of the diffx ecosystem, providing fast and accurate semantic diff operations for structured data formats. It can be embedded in other Rust applications to add semantic comparison capabilities.

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

Primary function for computing semantic differences between two structured values.

```rust
pub fn diff(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult>
```

**Parameters:**
- `v1`: First value to compare (baseline)
- `v2`: Second value to compare (target)
- `ignore_keys_regex`: Optional regex to ignore certain keys
- `epsilon`: Optional tolerance for floating-point comparisons
- `array_id_key`: Optional key for array element identification

**Returns:** Vector of `DiffResult` representing all differences found

**Example:**
```rust
use diffx_core::{diff, DiffResult};
use serde_json::{json, Value};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = json!({
        "name": "myapp",
        "version": "1.0",
        "timestamp": "2024-01-01T00:00:00Z"
    });
    
    let v2 = json!({
        "name": "myapp",
        "version": "1.1", 
        "timestamp": "2024-01-02T00:00:00Z",
        "port": 8080
    });
    
    // Ignore timestamp changes
    let ignore_regex = Regex::new(r"^timestamp$")?;
    
    let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);
    
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

### Format Parsers

#### `parse_ini()`

Parse INI format content into a JSON Value.

```rust
pub fn parse_ini(content: &str) -> Result<Value>
```

**Example:**
```rust
use diffx_core::parse_ini;

let ini_content = r#"
[database]
host = localhost
port = 5432

[cache]
enabled = true
ttl = 3600
"#;

let parsed = parse_ini(ini_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
```

#### `parse_xml()`

Parse XML format content into a JSON Value.

```rust
pub fn parse_xml(content: &str) -> Result<Value>
```

**Example:**
```rust
use diffx_core::parse_xml;

let xml_content = r#"
<config>
    <database>
        <host>localhost</host>
        <port>5432</port>
    </database>
    <cache enabled="true">
        <ttl>3600</ttl>
    </cache>
</config>
"#;

let parsed = parse_xml(xml_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
```

#### `parse_csv()`

Parse CSV format content into a JSON Value (array of objects).

```rust
pub fn parse_csv(content: &str) -> Result<Value>
```

**Example:**
```rust
use diffx_core::parse_csv;

let csv_content = r#"
name,age,city
Alice,25,New York
Bob,30,San Francisco
Charlie,35,Chicago
"#;

let parsed = parse_csv(csv_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
// Output: [{"name": "Alice", "age": "25", "city": "New York"}, ...]
```

### Utility Functions

#### `value_type_name()`

Get a human-readable type name for a JSON value.

```rust
pub fn value_type_name(value: &Value) -> &str
```

**Returns:** String slice with type name: `"null"`, `"boolean"`, `"number"`, `"string"`, `"array"`, `"object"`

**Example:**
```rust
use diffx_core::value_type_name;
use serde_json::{json, Value};

let values = vec![
    json!(null),
    json!(true),
    json!(42),
    json!("hello"),
    json!([1, 2, 3]),
    json!({"key": "value"})
];

for value in values {
    println!("{}: {}", value, value_type_name(&value));
}
// Output:
// null: null
// true: boolean  
// 42: number
// "hello": string
// [1,2,3]: array
// {"key":"value"}: object
```

## Advanced Usage

### Custom Comparison Logic

#### Epsilon Comparison

Handle floating-point precision differences:

```rust
use diffx_core::diff;
use serde_json::json;

let v1 = json!({"pi": 3.14159});
let v2 = json!({"pi": 3.14160});

// Without epsilon - reports difference
let diffs_strict = diff(&v1, &v2, None, None, None);
assert!(!diffs_strict.is_empty());

// With epsilon - no difference
let diffs_epsilon = diff(&v1, &v2, None, Some(0.001), None);
assert!(diffs_epsilon.is_empty());
```

#### Regex Key Filtering

Ignore specific keys or patterns:

```rust
use diffx_core::diff;
use serde_json::json;
use regex::Regex;

let v1 = json!({
    "data": {"important": "value"},
    "timestamp": "2024-01-01T00:00:00Z",
    "_internal": "system_data"
});

let v2 = json!({
    "data": {"important": "new_value"},
    "timestamp": "2024-01-02T00:00:00Z", 
    "_internal": "different_system_data"
});

// Ignore timestamp and internal fields
let ignore_regex = Regex::new(r"^(timestamp|_.*)")?;
let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);

// Only reports the important data change
assert_eq!(differences.len(), 1);
```

#### Array Element Tracking

Track array elements by ID instead of position:

```rust
use diffx_core::diff;
use serde_json::json;

let v1 = json!({
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
});

let v2 = json!({
    "users": [
        {"id": 2, "name": "Bob"}, 
        {"id": 1, "name": "Alice Smith"}  // Name changed
    ]
});

// With ID tracking - detects name change
let differences = diff(&v1, &v2, None, None, Some("id"));
// Reports: Modified users[id=1].name: "Alice" -> "Alice Smith"

// Without ID tracking - reports all as changed due to position
let differences_positional = diff(&v1, &v2, None, None, None);
// Reports multiple changes due to position differences
```

### Working with Different Formats

#### Complete Format Processing Pipeline

```rust
use diffx_core::{diff, parse_ini, parse_xml, parse_csv};
use serde_json::{from_str, Value};
use std::fs;

fn compare_files(
    file1_path: &str,
    file2_path: &str,
    format: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = fs::read_to_string(file1_path)?;
    let content2 = fs::read_to_string(file2_path)?;
    
    let (value1, value2) = match format {
        "json" => {
            (from_str(&content1)?, from_str(&content2)?)
        }
        "yaml" => {
            (serde_yml::from_str(&content1)?, serde_yml::from_str(&content2)?)
        }
        "toml" => {
            (toml::from_str(&content1)?, toml::from_str(&content2)?)
        }
        "ini" => {
            (parse_ini(&content1)?, parse_ini(&content2)?)
        }
        "xml" => {
            (parse_xml(&content1)?, parse_xml(&content2)?)
        }
        "csv" => {
            (parse_csv(&content1)?, parse_csv(&content2)?)
        }
        _ => return Err(format!("Unsupported format: {}", format).into())
    };
    
    Ok(diff(&value1, &value2, None, None, None))
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
        for diff in differences {
            match diff {
                DiffResult::Added(path, value) => {
                    self.additions.push((path, value));
                }
                DiffResult::Removed(path, value) => {
                    self.removals.push((path, value));
                }
                DiffResult::Modified(path, old, new) => {
                    self.modifications.push((path, old, new));
                }
                DiffResult::TypeChanged(path, old, new) => {
                    self.type_changes.push((path, old, new));
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
    file2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = tokio::fs::read_to_string(file1).await?;
    let content2 = tokio::fs::read_to_string(file2).await?;
    
    // Parse in background task to avoid blocking
    let result = tokio::task::spawn_blocking(move || {
        let v1: Value = serde_json::from_str(&content1)?;
        let v2: Value = serde_json::from_str(&content2)?;
        Ok::<_, serde_json::Error>(diff(&v1, &v2, None, None, None))
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
use diffx_core::{diff, parse_xml};
use serde_json::json;

// Handle malformed data
fn robust_comparison(
    data1: &str,
    data2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // Attempt to parse as JSON first
    let v1 = match serde_json::from_str(data1) {
        Ok(v) => v,
        Err(_) => {
            // Try XML if JSON fails
            parse_xml(data1)?
        }
    };
    
    let v2 = match serde_json::from_str(data2) {
        Ok(v) => v,
        Err(_) => parse_xml(data2)?
    };
    
    Ok(diff(&v1, &v2, None, None, None))
}
```

## Performance Considerations

### Memory Usage

For large datasets:

```rust
use diffx_core::diff;
use serde_json::Value;

// Process large files efficiently
fn process_large_diff(
    v1: &Value,
    v2: &Value,
    focus_path: Option<&str>
) -> Vec<DiffResult> {
    // If focusing on a specific path, extract just that portion
    if let Some(path) = focus_path {
        if let (Some(sub1), Some(sub2)) = (
            extract_path(v1, path),
            extract_path(v2, path)
        ) {
            return diff(&sub1, &sub2, None, None, None);
        }
    }
    
    diff(v1, v2, None, None, None)
}

fn extract_path(value: &Value, path: &str) -> Option<Value> {
    // Implementation to extract nested path
    // This would traverse the JSON path
    todo!("Implement path extraction")
}
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
        let v1 = json!({"a": 1, "b": 2});
        let v2 = json!({"a": 1, "b": 3, "c": 4});
        
        let diffs = diff(&v1, &v2, None, None, None);
        
        assert_eq!(diffs.len(), 2);
        // Test specific differences...
    }
    
    #[test]
    fn test_epsilon_comparison() {
        let v1 = json!({"value": 1.0});
        let v2 = json!({"value": 1.0001});
        
        let diffs_strict = diff(&v1, &v2, None, None, None);
        assert!(!diffs_strict.is_empty());
        
        let diffs_epsilon = diff(&v1, &v2, None, Some(0.001), None);
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
