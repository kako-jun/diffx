# Getting Started with diffx

This comprehensive guide will help you get up and running with `diffx` quickly and effectively.

## What is diffx?

`diffx` is a semantic diff tool designed specifically for structured data formats. Unlike traditional text-based diff tools, `diffx` understands the structure and meaning of your data, focusing on actual changes rather than formatting differences.

### Key Benefits

- **Semantic Understanding**: Ignores formatting, key order, whitespace, and trailing commas
- **Multiple Formats**: Supports JSON, YAML, TOML, XML, INI, and CSV
- **Clean Output**: Human-readable and machine-parseable results
- **Advanced Features**: Array tracking, regex filtering, floating-point tolerance

## Prerequisites

Before starting, make sure you have `diffx` installed. See the [Installation Guide](installation.md) for detailed instructions.

Quick install:
```bash
cargo install diffx
```

## Basic Usage

### Simple File Comparison

The most basic usage is comparing two files:

```bash
# Compare JSON files
diffx config_v1.json config_v2.json

# Compare YAML files  
diffx docker-compose.yml docker-compose.new.yml

# Compare TOML files
diffx Cargo.toml Cargo.toml.backup

# Compare XML files
diffx settings.xml settings.new.xml

# Compare INI files
diffx database.ini database.prod.ini

# Compare CSV files
diffx users.csv users_updated.csv
```

### Understanding the diffx Format

By default, `diffx` outputs differences in the **diffx format** - a human-readable, structured representation designed specifically for semantic data comparison. The diffx format uses intuitive symbols to show different types of changes:

- `+` **Added**: New keys or values
- `-` **Removed**: Deleted keys or values  
- `~` **Modified**: Changed values
- `!` **Type Changed**: Value type conversion (e.g., string to number)

Example diffx format output:
```
+ database.port: 5432
- cache.ttl: 3600
~ version: "1.0" -> "1.1"
! debug: "true" -> true
```

## Format Detection

`diffx` automatically detects file formats based on file extensions:

```bash
# These commands work automatically
diffx app.json app.new.json     # Detected as JSON
diffx config.yaml config.yml    # Detected as YAML
diffx settings.toml backup.toml # Detected as TOML
```

### Manual Format Specification

If auto-detection fails or you're using pipes, specify the format explicitly:

```bash
# Force JSON interpretation
diffx --format json file1 file2

# Specify different formats for each file
diffx --format json file1.txt --format yaml file2.txt
```

## Working with Different Data Sources

### Standard Input

Use pipes to compare data from different sources:

```bash
# Compare API responses
curl -s https://api.example.com/v1/config | diffx config.json -

# Compare command outputs
docker inspect container1 | diffx - <(docker inspect container2)

# Process JSON from environment variables
echo "$CONFIG_V1" | diffx - config_v2.json --format json
```

### Directory Comparison

Compare entire directories recursively:

```bash
# Compare all files in two directories
diffx config_dir1/ config_dir2/ --recursive

# Only compare specific file types
diffx configs/ configs_backup/ --recursive --format json
```

## Advanced Features

### Ignoring Specific Keys

Use regular expressions to ignore certain keys:

```bash
# Ignore timestamp and internal fields
diffx app.json app.new.json --ignore-keys-regex "^(timestamp|_.*|createdAt)$"

# Ignore version-related fields
diffx package.json package.new.json --ignore-keys-regex "version|buildNumber"
```

### Array Element Tracking

For arrays containing objects with unique identifiers:

```bash
# Track users by ID
diffx users.json users_updated.json --array-id-key "id"

# Track products by SKU
diffx inventory.json inventory.new.json --array-id-key "sku"

# Track database records by primary key
diffx records.json records.new.json --array-id-key "pk"
```

### Floating-Point Tolerance

Handle floating-point precision differences:

```bash
# Allow small numerical differences (0.001 tolerance)
diffx metrics.json metrics.new.json --epsilon 0.001

# More lenient tolerance for scientific data
diffx measurements.json measurements.new.json --epsilon 0.01
```

### Path Filtering

Focus on specific parts of your data:

```bash
# Only show differences in the database configuration
diffx config.json config.new.json --path "database"

# Check specific array elements
diffx config.json config.new.json --path "servers[0]"

# Deep path filtering
diffx app.json app.new.json --path "microservices.auth.database.connection"
```

## Output Formats

### The diffx Format (Default)

The **diffx format** is the default output format, designed to be both human-readable and semantically precise. Unlike traditional text-based diffs, the diffx format focuses on data structure and meaning:

**Key Features of diffx Format:**
- **Semantic Focus**: Shows logical changes, not textual differences
- **Path Clarity**: Full path notation (e.g., `database.connection.host`)
- **Type Awareness**: Distinguishes between value changes and type changes
- **Hierarchical Structure**: Maintains data relationship context
- **Universal Symbols**: Consistent `+`, `-`, `~`, `!` notation across all data formats

**Standard diffx format output:**

```bash
diffx config.json config.new.json
# Output:
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

### JSON Output

Perfect for programmatic processing:

```bash
diffx config.json config.new.json --output json
```

Output format:
```json
[
  {
    "Added": ["database.port", 5432]
  },
  {
    "Modified": ["version", "1.0", "1.1"]
  },
  {
    "Removed": ["cache.enabled", true]
  }
]
```

### YAML Output

Human-readable structured output:

```bash
diffx config.json config.new.json --output yaml
```

### Unified Diff Format

Compatible with traditional diff tools:

```bash
diffx config.json config.new.json --output unified
```

## Practical Examples

### Configuration Management

```bash
# Compare Kubernetes configs
diffx k8s-prod.yaml k8s-staging.yaml --ignore-keys-regex "namespace|name"

# Check Terraform state changes
diffx terraform.tfstate terraform.tfstate.backup --path "resources"

# Compare Docker Compose files
diffx docker-compose.yml docker-compose.override.yml
```

### Data Validation

```bash
# Compare database exports
diffx users_backup.json users_current.json --array-id-key "user_id"

# Validate API responses
diffx expected_response.json actual_response.json --ignore-keys-regex "timestamp"

# Check data migrations
diffx before_migration.json after_migration.json --epsilon 0.001
```

### Development Workflow

```bash
# Compare package files
diffx package.json package.json.template --ignore-keys-regex "^(name|version)"

# Check configuration changes
diffx .env.example .env.local --format ini

# Validate build outputs
diffx build_manifest.json build_manifest.expected.json
```

## Configuration File

Create `~/.config/diffx/config.toml` for default options:

```toml
# Default output format
output = "cli"

# Default epsilon for floating-point comparison
epsilon = 0.001

# Default keys to ignore
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# Default array ID key
array_id_key = "id"

# Enable colors in output
colors = true

# Default recursive mode for directories
recursive = true
```

## Integration with Other Tools

### Git Integration

```bash
# Git alias for structured diffs
git config alias.diffx '!f() { git show "$1" | diffx - "$2"; }; f'

# Use in git hooks
diffx package.json HEAD~1:package.json --output json > package_changes.json
```

### CI/CD Pipelines

```bash
# GitHub Actions
diffx config/prod.yaml config/staging.yaml --output json > config_diff.json

# GitLab CI
diffx database_schema.json database_schema.backup.json --array-id-key "table_name"
```

### Monitoring and Alerting

```bash
# Check for configuration drift
if diffx config.json config.expected.json --output json | jq -e 'length > 0'; then
  echo "Configuration drift detected!"
  exit 1
fi
```

## Performance Tips

### Large Files

For very large files:

```bash
# Use path filtering to focus on specific sections
diffx large_config.json large_config.new.json --path "critical_section"

# Ignore non-essential fields
diffx large_data.json large_data.new.json --ignore-keys-regex "metadata|debug_info"
```

### Batch Processing

```bash
# Process multiple files efficiently
find configs/ -name "*.json" -print0 | \
  xargs -0 -I {} sh -c 'diffx {} {}.backup || echo "Differences in {}"'
```

## Common Patterns

### Environment Comparison

Compare configurations across environments:

```bash
# Development vs Production
diffx config/dev.json config/prod.json --ignore-keys-regex "host|port|password"

# Staging validation
diffx config/staging.yaml config/prod.yaml --path "database"
```

### Backup Verification

Verify backup integrity:

```bash
# Database backup verification
diffx db_export.json db_backup.json --array-id-key "id" --epsilon 0.001

# Configuration backup check
diffx app_config.toml app_config.backup.toml
```

### API Testing

Validate API responses:

```bash
# Response comparison
diffx expected_api_response.json actual_response.json --ignore-keys-regex "timestamp|request_id"

# Schema validation
diffx api_schema.json generated_schema.json --path "definitions"
```

## Troubleshooting

### Common Issues

**File not found errors:**
```bash
# Check file paths
ls -la file1.json file2.json
```

**Format detection failures:**
```bash
# Specify format explicitly
diffx file1 file2 --format json
```

**Large output:**
```bash
# Use path filtering
diffx large1.json large2.json --path "specific.section"
```

**Memory issues with large files:**
```bash
# Increase memory limit (if supported)
export DIFFX_MAX_MEMORY=2GB
diffx huge1.json huge2.json
```

## Next Steps

- Read the [Configuration Guide](configuration.md) for advanced settings
- Explore the [Examples](examples.md) for real-world use cases
- Check the [CLI Reference](../reference/cli-reference.md) for complete option documentation
- Learn about [Integration patterns](../guides/integrations.md) for CI/CD workflows

## Getting Help

If you need assistance:

1. Check the [FAQ](faq.md)
2. Browse the [Examples](examples.md)
3. Visit the [GitHub repository](https://github.com/kako-jun/diffx)
4. Create an issue for bugs or feature requests