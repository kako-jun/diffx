# diffx-python

A Python wrapper for the `diffx` CLI tool.

## Installation

```bash
pip install diffx-python
```

This will automatically download the appropriate `diffx` binary for your system from GitHub Releases.

## Usage

```python
from diffx_python import run_diffx

# Compare two JSON files
result = run_diffx(["file1.json", "file2.json"])

if result.returncode == 0:
    print("No differences found.")
else:
    print("Differences found:")
    print(result.stdout)

# You can pass any arguments supported by the diffx CLI
result = run_diffx(["file1.yaml", "file2.yaml", "--output", "json"])
print(result.stdout)
```

## Development

To install in editable mode for development:

```bash
pip install -e .
```

## License

This project is licensed under the MIT License.
