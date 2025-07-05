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

# Default output format
output = "cli"

# Default epsilon for floating-point comparison  
epsilon = 0.001

# Default keys to ignore (regex pattern)
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# Default array ID key for tracking elements
array_id_key = "id"

# Enable colored output
colors = true

# Default recursive mode for directories
recursive = false

# Default format for auto-detection failures
default_format = "json"

# Maximum file size to process (in bytes)
max_file_size = 104857600  # 100MB

# Enable verbose logging
verbose = false

# Enable debug mode
debug = false

[filters]
# Predefined filter sets
common_timestamps = "^(timestamp|createdAt|updatedAt|lastModified)$"
internal_fields = "^(_.*|__.*)"
version_fields = "^(version|buildNumber|revision)$"

[paths]
# Commonly ignored paths
exclude_metadata = "metadata"
exclude_debug = "debug_info"

[formats]
# Default format settings
json_pretty = true
yaml_flow = false
```

## Configuration Options

### Output Options

#### `output`
- **Type**: String
- **Default**: `"cli"`
- **Values**: `"cli"`, `"json"`, `"yaml"`, `"unified"`
- **Description**: Default output format

```toml
output = "json"
```

#### `colors`
- **Type**: Boolean
- **Default**: `true`
- **Description**: Enable colored output in CLI mode

```toml
colors = false  # Disable colors for CI/CD
```

### Comparison Options

#### `epsilon`
- **Type**: Float
- **Default**: `0.0` (exact comparison)
- **Description**: Tolerance for floating-point number comparison

```toml
epsilon = 0.001  # Allow small floating-point differences
```

#### `ignore_keys_regex`
- **Type**: String (regex pattern)
- **Default**: `""` (no keys ignored)
- **Description**: Regular expression to ignore specific keys

```toml
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"
```

#### `array_id_key`
- **Type**: String
- **Default**: `""` (no ID tracking)
- **Description**: Key to use for array element identification

```toml
array_id_key = "id"
```

### File Processing Options

#### `recursive`
- **Type**: Boolean
- **Default**: `false`
- **Description**: Enable recursive directory comparison by default

```toml
recursive = true
```

#### `default_format`
- **Type**: String
- **Default**: `"json"`
- **Values**: `"json"`, `"yaml"`, `"toml"`, `"xml"`, `"ini"`, `"csv"`
- **Description**: Default format when auto-detection fails

```toml
default_format = "yaml"
```

#### `max_file_size`
- **Type**: Integer
- **Default**: `104857600` (100MB)
- **Description**: Maximum file size to process in bytes

```toml
max_file_size = 52428800  # 50MB limit
```

### Debugging Options

#### `verbose`
- **Type**: Boolean
- **Default**: `false`
- **Description**: Enable verbose output

```toml
verbose = true
```

#### `debug`
- **Type**: Boolean
- **Default**: `false`
- **Description**: Enable debug logging

```toml
debug = true
```

## Environment Variables

You can override configuration options using environment variables:

### Common Environment Variables

```bash
# Output format
export DIFFX_OUTPUT=json

# Epsilon for floating-point comparison
export DIFFX_EPSILON=0.001

# Keys to ignore
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*)"

# Array ID key
export DIFFX_ARRAY_ID_KEY=id

# Enable colors
export DIFFX_COLORS=true

# Enable recursive mode
export DIFFX_RECURSIVE=true

# Default format
export DIFFX_DEFAULT_FORMAT=yaml

# Maximum file size
export DIFFX_MAX_FILE_SIZE=52428800

# Verbose mode
export DIFFX_VERBOSE=true

# Debug mode
export DIFFX_DEBUG=true
```

### Format-Specific Variables

```bash
# JSON formatting
export DIFFX_JSON_PRETTY=true

# YAML formatting
export DIFFX_YAML_FLOW=false

# XML formatting
export DIFFX_XML_INDENT=2
```

## Configuration Priority

Configuration options are applied in the following order (highest to lowest priority):

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Project-specific config** (`./.diffx` or `./diffx.toml`)
4. **User config** (`~/.diffx.toml`)
5. **Primary user config** (`~/.config/diffx/config.toml`)
6. **Default values** (lowest priority)

### Example Priority Resolution

```bash
# 1. Command line overrides everything
diffx file1.json file2.json --output yaml --epsilon 0.01

# 2. Environment variable is used if no CLI option
export DIFFX_OUTPUT=json
diffx file1.json file2.json --epsilon 0.01  # Uses JSON output

# 3. Config file value is used if no env var or CLI option
# ~/.config/diffx/config.toml: output = "unified"
diffx file1.json file2.json  # Uses unified output
```

## Configuration Profiles

### Development Profile

```toml
# ~/.config/diffx/config.toml
output = "cli"
colors = true
verbose = true
ignore_keys_regex = "^(timestamp|_.*|debug_.*)"
epsilon = 0.001
recursive = true

[filters]
dev_ignore = "^(timestamp|_.*|debug_.*|temp_.*)"
```

### CI/CD Profile

```toml
# CI/CD environment
output = "json"
colors = false
verbose = false
recursive = true
max_file_size = 10485760  # 10MB for faster processing

[filters]
ci_ignore = "^(timestamp|buildId|runId)"
```

### Production Monitoring Profile

```toml
# Production monitoring
output = "json"
colors = false
epsilon = 0.0001  # Strict comparison
ignore_keys_regex = "^(lastCheck|timestamp)"
verbose = true

[alerts]
max_differences = 10
critical_paths = ["config.database", "config.security"]
```

## Advanced Configuration

### Custom Filter Sets

```toml
[filters]
# Database-related filters
db_metadata = "^(created_at|updated_at|version|checksum)$"

# API response filters  
api_metadata = "^(request_id|timestamp|server_time|correlation_id)$"

# Kubernetes filters
k8s_metadata = "^(metadata.creationTimestamp|metadata.resourceVersion|status)$"

# Docker filters
docker_metadata = "^(Id|Created|State.StartedAt|NetworkSettings)$"
```

### Path-Based Configuration

```toml
[paths]
# Ignore specific paths entirely
ignore_paths = [
    "metadata",
    "debug_info", 
    "logs",
    "cache"
]

# Focus only on critical paths
critical_paths = [
    "config.database",
    "config.security",
    "config.api"
]

# Custom array ID keys for specific paths
[path_array_ids]
"users" = "user_id"
"products" = "sku"
"orders" = "order_number"
"servers" = "hostname"
```

### Format-Specific Settings

```toml
[formats.json]
pretty = true
sort_keys = false
escape_unicode = false

[formats.yaml]
flow_style = false
default_style = "block"
indent = 2

[formats.xml]
pretty_print = true
indent_size = 2
sort_attributes = false

[formats.csv]
delimiter = ","
quote_style = "necessary"
headers = true
```

## Configuration Validation

### Checking Current Configuration

```bash
# Show current configuration
diffx --show-config

# Validate configuration file
diffx --validate-config ~/.config/diffx/config.toml

# Show configuration sources
diffx --config-sources
```

### Configuration Testing

Test your configuration with sample files:

```bash
# Test with specific config
diffx test1.json test2.json --config-file ./test-config.toml

# Test environment variables
DIFFX_OUTPUT=json DIFFX_VERBOSE=true diffx test1.json test2.json
```

## Common Configuration Patterns

### Team Configuration

Share team configuration via project files:

```toml
# ./.diffx (project root)
output = "json"
ignore_keys_regex = "^(timestamp|build_.*|deploy_.*)"
array_id_key = "id"
recursive = true

[team]
name = "backend-team"
standards = "2024-v1"

[filters]
deployment = "^(deploy_time|build_number|git_commit)"
monitoring = "^(last_check|health_status|uptime)"
```

### Multi-Environment Configuration

```toml
# ~/.config/diffx/config.toml
[environments.development]
output = "cli"
colors = true
verbose = true
epsilon = 0.01

[environments.staging]
output = "json"
colors = false
epsilon = 0.001

[environments.production]
output = "json"
colors = false
epsilon = 0.0001
ignore_keys_regex = "^(timestamp|request_id)"
```

Use with:
```bash
# Set environment
export DIFFX_ENV=production
diffx config1.json config2.json
```

### Integration-Specific Configurations

#### Git Integration

```toml
[git]
output = "unified"
ignore_keys_regex = "^(timestamp|git_.*)"
colors = false  # For git diff compatibility
```

#### Docker Integration

```toml
[docker]
ignore_keys_regex = "^(Id|Created|State\\..*)$"
array_id_key = "Name"
output = "json"
```

#### Kubernetes Integration

```toml
[kubernetes]
ignore_keys_regex = "^(metadata\\.(creationTimestamp|resourceVersion)|status\\..*)"
output = "yaml"
recursive = true
```

## Troubleshooting Configuration

### Common Issues

#### Configuration Not Found

```bash
# Check configuration file locations
diffx --config-locations

# Verify file exists and is readable
ls -la ~/.config/diffx/config.toml
```

#### Invalid Configuration

```bash
# Validate TOML syntax
diffx --validate-config

# Check for typos in option names
diffx --list-config-options
```

#### Environment Variable Issues

```bash
# List all diffx environment variables
env | grep DIFFX

# Clear all diffx environment variables
unset $(env | grep '^DIFFX_' | cut -d= -f1)
```

### Reset Configuration

```bash
# Remove user configuration
rm ~/.config/diffx/config.toml

# Remove project configuration
rm ./.diffx ./diffx.toml

# Use default configuration only
diffx --no-config file1.json file2.json
```

## Configuration Examples

### Minimal Configuration

```toml
# Basic setup
output = "cli"
colors = true
epsilon = 0.001
```

### Comprehensive Configuration

```toml
# Full-featured configuration
output = "cli"
colors = true
verbose = false
debug = false
recursive = true
epsilon = 0.001
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"
array_id_key = "id"
default_format = "json"
max_file_size = 104857600

[filters]
common_timestamps = "^(timestamp|createdAt|updatedAt|lastModified)$"
internal_fields = "^(_.*|__.*)"
version_fields = "^(version|buildNumber|revision)$"
monitoring = "^(health_.*|status_.*|last_.*)"

[formats.json]
pretty = true

[formats.yaml]
flow_style = false

[paths]
ignore_paths = ["metadata", "debug_info"]
critical_paths = ["config", "security"]
```

This configuration system provides flexibility for different use cases while maintaining sensible defaults for most users.