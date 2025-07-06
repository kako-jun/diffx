# diffx-python

Python wrapper for the `diffx` CLI tool - semantic diff for structured data.

## Installation

```bash
pip install diffx-python
```

This will automatically download the appropriate `diffx` binary for your system from GitHub Releases.

## Usage

### Modern API (Recommended)

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

### Legacy API (Backward Compatibility)

```python
from diffx import run_diffx

# Compare two JSON files (legacy)
result = run_diffx(["file1.json", "file2.json"])

if result.returncode == 0:
    print("No differences found.")
else:
    print("Differences found:")
    print(result.stdout)
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
- **Cross-platform**: Automatically downloads the right binary

## Development

To install in development mode with uv:

```bash
uv venv
source .venv/bin/activate
uv pip install -e .[dev]
```

## Manual Binary Installation

If automatic download fails:

```bash
diffx-download-binary
```

## License

This project is licensed under the MIT License.
