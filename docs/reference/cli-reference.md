# CLI Reference

Complete reference documentation for the `diffx` command-line interface.

## Synopsis

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## Description

`diffx` is a command-line tool for semantic comparison of structured data files. Unlike traditional text-based diff tools, `diffx` understands the structure and meaning of your data, focusing on actual changes rather than formatting differences.

## Arguments

### `<INPUT1>`
- **Type**: File path, directory path, or `-` for stdin
- **Required**: Yes
- **Description**: The first input to compare

### `<INPUT2>`
- **Type**: File path, directory path, or `-` for stdin  
- **Required**: Yes
- **Description**: The second input to compare

**Examples:**
```bash
# Compare two files
diffx config.json config.new.json

# Compare with stdin
cat config.json | diffx - config.new.json

# Compare directories
diffx config_dir1/ config_dir2/
```

## Options

### Format Options

#### `-f, --format <FORMAT>`
- **Type**: String
- **Default**: Auto-detected from file extension
- **Values**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **Description**: Force a specific input file format

**Examples:**
```bash
# Force JSON interpretation
diffx --format json file1.txt file2.txt

# Force YAML interpretation
diffx -f yaml config1 config2
```

**Auto-detection mapping:**
- `.json` → `json`
- `.yaml`, `.yml` → `yaml`
- `.toml` → `toml`
- `.xml` → `xml`
- `.ini`, `.cfg`, `.conf` → `ini`
- `.csv` → `csv`

### Output Options

#### `-o, --output <FORMAT>`
- **Type**: String
- **Default**: `diffx` (human-readable diffx format)
- **Values**: `diffx`, `json`, `yaml`, `unified`
- **Description**: Output format for differences

**diffx Format (default):**
```bash
diffx config.json config.new.json
# Output:
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

**JSON Output:**
```bash
diffx config.json config.new.json --output json
# Output:
# [
#   {"Added": ["database.port", 5432]},
#   {"Modified": ["version", "1.0", "1.1"]},
#   {"Removed": ["cache.enabled", true]}
# ]
```

**YAML Output:**
```bash
diffx config.json config.new.json --output yaml
# Output:
# - Added:
#   - database.port
#   - 5432
# - Modified:
#   - version
#   - "1.0"
#   - "1.1"
```

**Unified Output:**
```bash
diffx config.json config.new.json --output unified
# Output: Traditional diff-style format
```

### Filtering Options

#### `--path <PATH>`
- **Type**: String
- **Default**: None (compare entire structure)
- **Description**: Filter differences to a specific path in the data structure

**Path Syntax:**
- Object keys: `database.host`
- Array indices: `users[0]`
- Nested paths: `config.database.connection.host`
- Complex paths: `services.web.env[0].name`

**Examples:**
```bash
# Only compare database configuration
diffx config.json config.new.json --path "database"

# Compare specific array element
diffx config.json config.new.json --path "users[0]"

# Deep nested path
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <PATTERN>`
- **Type**: Regular expression string
- **Default**: None
- **Description**: Ignore keys matching the specified regular expression

**Common Patterns:**
```bash
# Ignore timestamp fields
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# Ignore internal fields (starting with underscore)
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# Ignore multiple patterns
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# Ignore version-related fields
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**Regex Examples:**
- `^timestamp$` - Exact match for "timestamp"
- `^_.*` - Fields starting with underscore
- `.*_temp$` - Fields ending with "_temp"
- `^(id|uid|pk)$` - Match any of: id, uid, pk
- `(?i)password` - Case-insensitive match for "password"

### Comparison Options

#### `--epsilon <VALUE>`
- **Type**: Float
- **Default**: `0.0` (exact comparison)
- **Description**: Tolerance for floating-point number comparisons

**Examples:**
```bash
# Allow small differences in floating-point numbers
diffx metrics.json metrics.new.json --epsilon 0.001

# More lenient tolerance for scientific data
diffx measurements.json measurements.new.json --epsilon 0.01

# Very strict comparison
diffx financial.json financial.new.json --epsilon 0.000001
```

**Use Cases:**
- Scientific data with measurement precision
- Financial calculations with rounding differences
- Performance metrics with small variations
- Converted data with floating-point artifacts

#### `--array-id-key <KEY>`
- **Type**: String
- **Default**: None (positional comparison)
- **Description**: Key to use for identifying and tracking array elements

**Examples:**
```bash
# Track users by ID
diffx users.json users.updated.json --array-id-key "id"

# Track products by SKU
diffx inventory.json inventory.new.json --array-id-key "sku"

# Track database records by primary key
diffx records.json records.new.json --array-id-key "primary_key"
```

**Without ID tracking:**
```json
// Array comparison shows positional changes
// Old: [{"name": "Alice"}, {"name": "Bob"}]
// New: [{"name": "Bob"}, {"name": "Alice"}]
// Result: All elements appear changed
```

**With ID tracking:**
```json
// Old: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// New: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// Result: No changes detected (same elements, different order)
```

### Directory Options

#### `-r, --recursive`
- **Type**: Boolean flag
- **Default**: False
- **Description**: Enable recursive directory comparison

**Examples:**
```bash
# Compare all files in directories
diffx config_dir1/ config_dir2/ --recursive

# Recursive comparison with output format
diffx environments/dev/ environments/prod/ -r --output json

# Recursive with filtering
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**Behavior:**
- Compares corresponding files between directories
- Skips files that don't exist in both directories
- Maintains directory structure in output
- Respects format auto-detection for each file

### Information Options

#### `-h, --help`
- **Type**: Boolean flag
- **Description**: Print help information and exit

#### `-V, --version`
- **Type**: Boolean flag  
- **Description**: Print version information and exit

**Examples:**
```bash
# Show help
diffx --help
diffx -h

# Show version
diffx --version
diffx -V
```

## Exit Codes

`diffx` uses the following exit codes:

- **0**: Success, no differences found
- **1**: Success, differences found
- **2**: Error in command-line arguments
- **3**: File I/O error
- **4**: Parse error (invalid format)
- **5**: Internal error

**Examples:**
```bash
# Check if files are identical
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "Files are identical"
else
    echo "Files differ"
fi

# Capture exit code
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "No differences" ;;
    1) echo "Differences found" ;;
    *) echo "Error occurred (code: $EXIT_CODE)" ;;
esac
```

## Environment Variables

These environment variables can be used to set default values:

- `DIFFX_OUTPUT` - Default output format
- `DIFFX_FORMAT` - Default input format
- `DIFFX_EPSILON` - Default epsilon value
- `DIFFX_IGNORE_KEYS_REGEX` - Default ignore pattern
- `DIFFX_ARRAY_ID_KEY` - Default array ID key
- `DIFFX_RECURSIVE` - Default recursive mode
- `DIFFX_COLORS` - Enable/disable colored output

**Examples:**
```bash
# Set defaults via environment
export DIFFX_OUTPUT=json
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*)"
export DIFFX_EPSILON=0.001

# Commands now use these defaults
diffx config.json config.new.json
```

## Configuration File

See the [Configuration Guide](../user-guide/configuration.md) for details on using configuration files.

## Usage Patterns

### Basic Comparisons

```bash
# Simple file comparison
diffx file1.json file2.json

# Compare with different formats
diffx config.yaml config.toml --format yaml --format toml

# Compare stdin with file
curl -s https://api.example.com/config | diffx - local_config.json
```

### Advanced Filtering

```bash
# Complex ignore pattern
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# Path-specific comparison
diffx large_config.json large_config.new.json \
  --path "database.connections"

# Combine multiple options
diffx users.json users.new.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_.*)" \
  --output json
```

### Directory Operations

```bash
# Recursive directory comparison
diffx configs/ configs.backup/ --recursive

# Directory comparison with filtering
diffx env/dev/ env/prod/ \
  --recursive \
  --ignore-keys-regex "^(host|port|password)" \
  --output json > env_diff.json
```

### Integration Examples

```bash
# Git integration
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CD pipeline
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json

# Monitoring script
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)" >/dev/null; then
  echo "Configuration drift detected!"
  diffx baseline_config.json current_config.json --output json | \
    notify_alert_system.py
fi
```

## Error Handling

### Common Errors

**File not found:**
```bash
$ diffx nonexistent.json config.json
Error: No such file or directory (os error 2)
```

**Invalid format:**
```bash
$ diffx invalid.json valid.json
Error: Failed to parse JSON: expected `,` or `}` at line 1 column 15
```

**Permission denied:**
```bash
$ diffx protected.json config.json
Error: Permission denied (os error 13)
```

**Invalid regex:**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
Error: Invalid regular expression: unclosed character class
```

### Debugging

```bash
# Verbose output (if supported)
DIFFX_VERBOSE=true diffx file1.json file2.json

# Debug mode (if supported)
DIFFX_DEBUG=true diffx file1.json file2.json

# Validate format detection
diffx --format json file1.txt file2.txt
```

## Performance Considerations

### Large Files

```bash
# Use path filtering for large files
diffx huge1.json huge2.json --path "critical_section"

# Ignore non-essential data
diffx large1.json large2.json --ignore-keys-regex "logs|debug|metadata"
```

### Batch Processing

```bash
# Parallel processing of multiple files
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### Memory Usage

For very large files, consider:
- Using `--path` to focus on specific sections
- Filtering out large, irrelevant sections with `--ignore-keys-regex`
- Processing files in smaller chunks if possible

## Examples by Use Case

### Configuration Management
```bash
# Environment comparison
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)"

# Kubernetes manifests  
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)"
```

### API Testing
```bash
# Response validation
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)"

# Schema comparison
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### Data Processing
```bash
# ETL validation
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# Database export comparison
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)"
```

### Security Auditing
```bash
# Policy comparison
diffx security_policy.json security_policy.new.json --path "permissions"

# Access control validation
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

This comprehensive CLI reference covers all available options and provides practical examples for effective usage of `diffx`.