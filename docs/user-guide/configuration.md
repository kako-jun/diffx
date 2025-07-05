# Configuration Guide

`diffx` supports configuration through configuration files, environment variables, and command-line options. This guide explains all available configuration options and their priorities.

## Configuration File Location

### Primary Configuration File

Create a configuration file at:
```
~/.config/diffx/config.toml
```

### Alternative Locations

`diffx` will search for configuration files in the following order:

1. `~/.config/diffx/config.toml` (primary)
2. `~/.diffx.toml` (alternative)
3. `./diffx.toml` (project-specific)
4. `./.diffx` (project-specific alternative)

## Configuration File Format

The configuration file uses TOML format:

```toml
# ~/.config/diffx/config.toml

# Default output format (cli, json, yaml, unified)
output = "cli"

# Default input format (json, yaml, toml, xml, ini, csv)  
format = "json"

# Default epsilon for floating-point comparison  
epsilon = 0.001

# Default keys to ignore (regex pattern)
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# Default array ID key for tracking elements
array_id_key = "id"
```

## Configuration Options

### Core Options

#### `output`
- **Type**: String
- **Default**: `"cli"`
- **Values**: `"cli"`, `"json"`, `"yaml"`, `"unified"`
- **Description**: Default output format for diff results

```toml
output = "json"  # Machine-readable output for CI/CD
```

#### `format`
- **Type**: String  
- **Default**: Auto-detected from file extension
- **Values**: `"json"`, `"yaml"`, `"toml"`, `"xml"`, `"ini"`, `"csv"`
- **Description**: Default input format when auto-detection fails

```toml
format = "json"  # Assume JSON when extension is unclear
```

#### `epsilon`
- **Type**: Float
- **Default**: No tolerance (exact comparison)
- **Description**: Tolerance for floating-point number comparisons

```toml
epsilon = 0.001  # Ignore differences smaller than 0.001
```

#### `ignore_keys_regex`
- **Type**: String (Regular Expression)
- **Default**: No keys ignored
- **Description**: Regular expression pattern for keys to ignore during comparison

```toml
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"
```

#### `array_id_key`
- **Type**: String
- **Default**: Index-based comparison
- **Description**: Key to use for identifying array elements (enables semantic array comparison)

```toml
array_id_key = "id"  # Track array elements by their "id" field
```

## Real-World Examples

### DevOps Configuration
```toml
# ~/.config/diffx/config.toml for infrastructure comparison
output = "json"
ignore_keys_regex = "^(timestamp|lastUpdated|deployedAt|_metadata)$"
epsilon = 0.001
array_id_key = "name"  # Infrastructure components identified by name
```

### API Development
```toml
# Project-specific .diffx.toml for API schema comparison  
format = "json"
ignore_keys_regex = "^(id|created_at|updated_at|_links)$"
array_id_key = "id"
```

### Data Pipeline Validation
```toml
# For comparing data processing results
output = "yaml"
epsilon = 0.01  # Allow small rounding differences
array_id_key = "record_id"
ignore_keys_regex = "^(processed_at|batch_id)$"
```

## Environment Variable Override

You can override config file settings using the `DIFFX_CONFIG_PATH` environment variable:

```bash
# Use a specific config file
export DIFFX_CONFIG_PATH="/path/to/project/diffx.toml"
diffx file1.json file2.json

# One-time override
DIFFX_CONFIG_PATH="./custom-config.toml" diffx data1.yaml data2.yaml
```

## Priority Order

Configuration options are applied in this priority order (highest to lowest):

1. **Command-line arguments** (highest priority)
2. **Configuration file** settings
3. **Built-in defaults** (lowest priority)

Example:
```bash
# config.toml has: output = "json"
# This command outputs in YAML (CLI overrides config)
diffx file1.json file2.json --output yaml
```
