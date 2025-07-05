# diffx-py

Python wrapper for [diffx](https://github.com/kako-jun/diffx) - semantic diff for structured data.

## Overview

`diffx-py` provides a Python interface to the powerful `diffx` command-line tool, enabling semantic comparison of structured data formats including JSON, YAML, TOML, XML, INI, and CSV.

## Prerequisites

This package requires the `diffx` CLI tool to be installed on your system.

### Install diffx CLI

```bash
# Using Cargo (Rust package manager)
cargo install diffx

# Or download from GitHub Releases
# https://github.com/kako-jun/diffx/releases
```

## Installation

```bash
pip install diffx-py
```

## Quick Start

```python
from diffx_py import diff

# Compare two JSON files
result = diff('config1.json', 'config2.json')
print(result)

# Compare with JSON output for programmatic use
from diffx_py import diff, DiffOptions

json_result = diff('file1.json', 'file2.json', 
                  DiffOptions(output='json'))
for diff_item in json_result:
    print(diff_item)

# Compare strings directly
from diffx_py import diff_string

json1 = '{"name": "Alice", "age": 30}'
json2 = '{"name": "Alice", "age": 31}'
string_result = diff_string(json1, json2, 'json')
print(string_result)
```

## API Reference

### `diff(input1, input2, options=None)`

Compare two files or directories.

**Parameters:**
- `input1: str` - Path to first file/directory or '-' for stdin
- `input2: str` - Path to second file/directory  
- `options: DiffOptions` - Comparison options (optional)

**Returns:** `str | List[DiffResult]`

### `diff_string(content1, content2, format, options=None)`

Compare two strings directly.

**Parameters:**
- `content1: str` - First content string
- `content2: str` - Second content string
- `format: Format` - Content format ('json', 'yaml', 'toml', 'xml', 'ini', 'csv')
- `options: DiffOptions` - Comparison options (optional)

**Returns:** `str | List[DiffResult]`

### `is_diffx_available()`

Check if diffx CLI is available.

**Returns:** `bool`

## Options

```python
@dataclass
class DiffOptions:
    format: Optional[Format] = None  # 'json', 'yaml', 'toml', 'xml', 'ini', 'csv'
    output: Optional[OutputFormat] = None  # 'cli', 'json', 'yaml', 'unified'
    recursive: bool = False
    path: Optional[str] = None
    ignore_keys_regex: Optional[str] = None
    epsilon: Optional[float] = None
    array_id_key: Optional[str] = None
```

## Examples

### Basic File Comparison

```python
from diffx_py import diff

result = diff('old-config.json', 'new-config.json')
print(result)
```

### Directory Comparison

```python
from diffx_py import diff, DiffOptions

dir_diff = diff('old-configs/', 'new-configs/', 
               DiffOptions(recursive=True, output='json'))

for diff_item in dir_diff:
    print(f"Change: {diff_item}")
```

### Filtered Comparison

```python
from diffx_py import diff, DiffOptions

# Ignore sensitive fields
filtered = diff('dev.json', 'prod.json', 
               DiffOptions(
                   ignore_keys_regex=r'^(password|secret_.*|api_key)$',
                   path='database'
               ))
print(filtered)
```

### Array Element Tracking

```python
from diffx_py import diff, DiffOptions

# Track array elements by ID
array_diff = diff('users1.json', 'users2.json', 
                 DiffOptions(array_id_key='id', output='json'))

for change in array_diff:
    if change.modified:
        key, old_val, new_val = change.modified
        print(f"Modified {key}: {old_val} -> {new_val}")
```

### Float Comparison with Tolerance

```python
from diffx_py import diff, DiffOptions

# Ignore small float differences
numeric_diff = diff('metrics1.json', 'metrics2.json', 
                   DiffOptions(epsilon=0.001))
print(numeric_diff)
```

### String Comparison

```python
from diffx_py import diff_string, DiffOptions

yaml1 = """
name: Alice
age: 30
"""

yaml2 = """
name: Alice  
age: 31
"""

result = diff_string(yaml1, yaml2, 'yaml', 
                    DiffOptions(output='json'))

for change in result:
    print(change)
```

## Working with DiffResult

When using `output='json'`, results are returned as `DiffResult` objects:

```python
from diffx_py import diff, DiffOptions

results = diff('a.json', 'b.json', DiffOptions(output='json'))

for result in results:
    if result.added:
        key, value = result.added
        print(f"Added: {key} = {value}")
    
    elif result.removed:
        key, value = result.removed
        print(f"Removed: {key} = {value}")
    
    elif result.modified:
        key, old_value, new_value = result.modified
        print(f"Modified: {key} = {old_value} -> {new_value}")
    
    elif result.type_changed:
        key, old_value, new_value = result.type_changed
        print(f"Type changed: {key} = {old_value} -> {new_value}")
```

## Error Handling

```python
from diffx_py import diff, DiffError

try:
    result = diff('file1.json', 'file2.json')
    print(result)
except DiffError as e:
    print(f"Diff failed: {e}")
    print(f"Exit code: {e.exit_code}")
    print(f"Stderr: {e.stderr}")
```

## Environment Check

```python
from diffx_py import is_diffx_available

if not is_diffx_available():
    print("diffx CLI is not installed or not in PATH")
    exit(1)

print("diffx is available!")
```

## Type Hints

This package includes full type hints for better IDE support:

```python
from diffx_py import diff, DiffOptions, DiffResult
from typing import List

options: DiffOptions = DiffOptions(
    output='json',
    ignore_keys_regex=r'^_.*'
)

results: List[DiffResult] = diff('a.json', 'b.json', options)
```

## Requirements

- Python 3.8 or higher
- `diffx` CLI tool installed and available in PATH

## License

MIT

## Related Projects

- [diffx](https://github.com/kako-jun/diffx) - The main CLI tool
- [diffx-core](https://crates.io/crates/diffx-core) - Rust library
- [diffx-js](https://www.npmjs.com/package/diffx-js) - JavaScript/TypeScript wrapper