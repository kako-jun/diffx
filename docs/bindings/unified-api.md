# diffx Unified API Reference

*Language bindings API documentation for diffx-python and diffx-js*

## Overview

diffx provides a unified API for comparing structured data files (JSON, YAML, TOML, XML, INI, CSV). The library focuses on semantic differences rather than formatting changes.

**Unified API Design**: The core API exposes only a single main function `diff()` for all comparison operations. All functionality is accessed through this unified interface using the options parameter. This design ensures consistency and simplicity across all use cases.

## Main Function

### `diff(old, new, options)`

Compares two structured data values and returns the differences.

#### Parameters

- `old` (Value): The original/old data structure to compare
- `new` (Value): The new/updated data structure to compare
- `options` (DiffOptions, optional): Configuration options for the comparison

#### Returns

- `Result<Vec<DiffResult>, Error>`: A vector of differences found between the two structures

#### Example

```rust
use diffx_core::{diff, DiffOptions, OutputFormat};
use serde_json::json;

let old = json!({
    "name": "John",
    "age": 30,
    "city": "New York"
});

let new = json!({
    "name": "John",
    "age": 31,
    "city": "Boston"
});

let options = DiffOptions {
    output_format: Some(OutputFormat::Json),
    show_unchanged: Some(false),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

## Options

### DiffOptions Structure

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

pub struct DiffxSpecificOptions {
    pub context_lines: Option<usize>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_case: Option<bool>,
    pub brief_mode: Option<bool>,
    pub quiet_mode: Option<bool>,
}
```

### Option Details

#### Numeric Comparison
- **`epsilon`**: Float comparison tolerance. Values within this epsilon are considered equal.
  - Default: `0.0` (exact comparison)
  - Example: `0.001` for 0.1% tolerance

#### Array Comparison
- **`array_id_key`**: Key to use for identifying array elements when comparing arrays
  - Default: `None` (index-based comparison)
  - Example: `"id"` to match array elements by their `id` field

#### Filtering
- **`ignore_keys_regex`**: Regular expression for keys to ignore during comparison
  - Example: `"^(timestamp|metadata)"` to ignore timestamp and metadata fields
- **`path_filter`**: JSONPath-like filter to limit comparison to specific paths
  - Example: `"$.users[*].name"` to compare only user names

#### Output Control
- **`output_format`**: Format for the output
  - Options: `Json`, `Yaml`, `Csv`, `Diffx` (custom format)
  - Default: `Json`
- **`show_unchanged`**: Include unchanged values in output
  - Default: `false`
- **`show_types`**: Include type information in output
  - Default: `false`

#### Memory Optimization
- **`use_memory_optimization`**: Enable memory-efficient processing for large files
  - Default: `false`
- **`batch_size`**: Number of items to process in each batch when memory optimization is enabled
  - Default: `1000`

#### diffx-Specific Options (DiffxSpecificOptions)
These options are nested within the `diffx_options` field:

- **`context_lines`**: Number of context lines to show in diffx format output
  - Default: `3`
- **`ignore_whitespace`**: Ignore whitespace differences in string comparisons
  - Default: `false`
- **`ignore_case`**: Case-insensitive string comparison
  - Default: `false`
- **`brief_mode`**: Show only whether files differ, not the differences
  - Default: `false`
- **`quiet_mode`**: Suppress all normal output
  - Default: `false`

## Result Types

### DiffResult Enum

```rust
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, String, String),
}
```

- **`Added(path, value)`**: A new field/value was added at the given path
- **`Removed(path, value)`**: A field/value was removed from the given path
- **`Modified(path, old_value, new_value)`**: A value was changed at the given path
- **`TypeChanged(path, old_type, new_type)`**: The type of value changed at the given path


## Language Bindings

### Python

```python
import diffx_python

# Basic usage
results = diffx_python.diff(old_dict, new_dict)

# With options
results = diffx_python.diff(
    old_dict, 
    new_dict,
    epsilon=0.001,
    array_id_key="id",
    ignore_keys_regex="^(timestamp|metadata)",
    output_format="json",
    show_unchanged=False
)
```

### TypeScript/JavaScript

```typescript
import { diff, DiffOptions } from 'diffx-js';
import * as fs from 'fs';

// Basic usage - users parse files themselves
const oldData = JSON.parse(fs.readFileSync('old.json', 'utf8'));
const newData = JSON.parse(fs.readFileSync('new.json', 'utf8'));
const results = await diff(oldData, newData);

// With options
const options: DiffOptions = {
    epsilon: 0.001,
    arrayIdKey: 'id',
    ignoreKeysRegex: '^(timestamp|metadata)',
    outputFormat: 'json',
    showUnchanged: false
};
const results = await diff(oldData, newData, options);
```

## Error Handling

The library returns detailed error messages for:
- Parse errors (invalid JSON, YAML, etc.)
- File I/O errors
- Invalid regular expressions
- Memory allocation failures
- Invalid options

## Performance Considerations

- Use `use_memory_optimization` for files larger than 100MB
- Adjust `batch_size` based on available memory
- Use `path_filter` to limit comparison scope for better performance
- Regular expressions in `ignore_keys_regex` can impact performance on large datasets

## Examples

### Comparing JSON Files

```rust
use diffx_core::{diff, DiffOptions};
use serde_json;

// Users should parse data themselves using standard libraries
let old_content = std::fs::read_to_string("old.json")?;
let new_content = std::fs::read_to_string("new.json")?;

let old: serde_json::Value = serde_json::from_str(&old_content)?;
let new: serde_json::Value = serde_json::from_str(&new_content)?;

let results = diff(&old, &new, None)?;
```

### Ignoring Timestamps

```rust
use regex::Regex;

let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new("timestamp|updated_at|created_at")?),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### Array Comparison by ID

```rust
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### Using diffx-Specific Options

```rust
use diffx_core::{DiffOptions, DiffxSpecificOptions};

let diffx_opts = DiffxSpecificOptions {
    ignore_whitespace: Some(true),
    ignore_case: Some(true),
    brief_mode: Some(false),
    ..Default::default()
};

let options = DiffOptions {
    diffx_options: Some(diffx_opts),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```