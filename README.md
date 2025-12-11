# diffx

[![CI](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/diffx.svg)](https://crates.io/crates/diffx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Semantic diff tool for structured data (JSON/YAML/TOML/XML/INI/CSV). Ignores key ordering and whitespace, shows only meaningful changes.

## Why diffx?

Traditional `diff` doesn't understand structure:

```bash
$ diff config_v1.json config_v2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }
```

A simple key reordering shows every line as changed.

`diffx` shows only semantic changes:

```bash
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

## Installation

```bash
# As CLI tool
cargo install diffx

# As library (Cargo.toml)
[dependencies]
diffx-core = "0.6"
```

## Usage

```bash
# Basic
diffx file1.json file2.json

# Output example
~ version: "1.0" -> "1.1"
+ features[0]: "new-feature"
- deprecated: "old-value"
```

## Supported Formats

JSON, YAML, TOML, XML, INI, CSV (auto-detected by extension, use `--format` to override)

## Main Options

```bash
--output json|yaml       # Machine-readable output
--quiet                  # Return only exit code (0: same, 1: diff found)
--ignore-keys-regex RE   # Ignore keys matching regex
--array-id-key KEY       # Identify array elements by KEY for comparison
--epsilon N              # Float comparison tolerance
--ignore-case            # Case-insensitive comparison
--ignore-whitespace      # Ignore whitespace differences
-r, --recursive          # Recursive directory comparison
```

## Output Symbols

- `+` Added
- `-` Removed
- `~` Modified
- `!` Type changed

## CI/CD Usage

```bash
# Detect config changes
if ! diffx config/prod.json config/staging.json --quiet; then
  echo "Config has changed"
  diffx config/prod.json config/staging.json --output json > changes.json
fi

# Compare ignoring timestamps and metadata
diffx api_v1.json api_v2.json --ignore-keys-regex "^(timestamp|updated_at)$"
```

## Documentation

- [CLI Specification](docs/specs/cli.md)
- [Core API Specification](docs/specs/core.md)
- [Examples](diffx-cli/tests/cmd/)

## License

MIT
