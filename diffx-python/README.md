# diffx-python

Python wrapper for the `diffx` CLI tool - semantic diff for structured data.

## Installation

```bash
pip install diffx-python
```

The `diffx` binary is automatically included in the wheel - no additional downloads required! This package uses [maturin](https://github.com/PyO3/maturin) to embed the native binary directly in the Python wheel, similar to tools like `ruff`.

## Usage

```python
import diffx

# Compare two JSON files
result = diffx.diff('file1.json', 'file2.json')
print(result)

# Get structured output as JSON
json_result = diffx.diff(
    'config1.yaml', 
    'config2.yaml',
    diffx.DiffOptions(format='yaml', output='json')
)

for diff_item in json_result:
    if diff_item.added:
        print(f"Added: {diff_item.added}")
    elif diff_item.modified:
        print(f"Modified: {diff_item.modified}")

# Compare directory trees
dir_result = diffx.diff(
    'dir1/', 
    'dir2/',
    diffx.DiffOptions(recursive=True, path='config')
)

# Compare strings directly
json1 = '{"name": "Alice", "age": 30}'
json2 = '{"name": "Alice", "age": 31}'
string_result = diffx.diff_string(
    json1, json2, 'json',
    diffx.DiffOptions(output='json')
)
```


## Features

- **Multiple formats**: JSON, YAML, TOML, XML, INI, CSV
- **Smart diffing**: Understands structure, not just text
- **Flexible output**: CLI, JSON, YAML, unified diff formats
- **Advanced options**: 
  - Regex-based key filtering
  - Floating-point tolerance
  - Array element identification
  - Path-based filtering
- **Cross-platform**: Native binary embedded in platform-specific wheels

## Key Benefits

- **🚀 Zero setup**: No external downloads or binary management
- **📦 Self-contained**: Everything needed is in the wheel
- **⚡ Fast installation**: No network dependencies after `pip install`
- **🔒 Secure**: No runtime downloads from external sources
- **🌐 Offline-ready**: Works in air-gapped environments

## Development

To install in development mode:

```bash
pip install -e .[dev]
```

## Verification

Verify the installation:

```python
import diffx
print("diffx available:", diffx.is_diffx_available())
print("Version:", diffx.__version__)
```

## License

This project is licensed under the MIT License.
