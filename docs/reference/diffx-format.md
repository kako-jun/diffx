# The diffx Format Specification

The **diffx format** is a human-readable, semantic diff representation designed specifically for structured data comparison. Unlike traditional text-based diff formats, the diffx format focuses on data meaning and structure rather than textual appearance.

## Overview

The diffx format addresses limitations of traditional diff tools when working with structured data:

- **Semantic Focus**: Shows logical changes, not textual differences
- **Format Agnostic**: Consistent representation across JSON, YAML, TOML, XML, INI, CSV
- **Path-Based**: Full hierarchical path notation for precise change location
- **Type Aware**: Distinguishes between value changes and type changes
- **Human Readable**: Intuitive symbols and clear formatting

## Specification

### Basic Syntax

The diffx format uses four primary symbols to indicate change types:

```
+ path: value    # Addition
- path: value    # Removal  
~ path: old -> new    # Modification
! path: old -> new    # Type change
```

### Path Notation

Paths use dot notation for objects and bracket notation for arrays:

```
database.host                    # Object property
servers[0].port                  # Array element property
config.users[2].permissions[1]   # Nested array access
```

### Value Representation

Values are displayed in their canonical JSON-like representation:

```
+ enabled: true                  # Boolean
+ port: 8080                     # Number
+ name: "production"             # String
+ tags: ["web", "api"]           # Array
+ config: {"debug": false}       # Object
+ value: null                    # Null
```

### Change Type Examples

#### Addition
When new keys or array elements are added:

```
+ database.port: 5432
+ servers[2]: {"name": "web-03", "port": 8080}
+ features[0]: "authentication"
```

#### Removal
When keys or array elements are removed:

```
- cache.ttl: 3600
- servers[1]: {"name": "web-02", "port": 8080}
- features[2]: "legacy-api"
```

#### Modification
When values change but maintain the same type:

```
~ database.host: "localhost" -> "prod-db.example.com"
~ servers[0].port: 8080 -> 9090
~ config.debug: false -> true
```

#### Type Change
When values change type (special case of modification):

```
! port: "8080" -> 8080           # String to number
! enabled: "true" -> true        # String to boolean
! config: {} -> null             # Object to null
```

### Complex Examples

#### Nested Object Changes
```
~ user.profile.settings.theme: "light" -> "dark"
+ user.profile.preferences.notifications: true
- user.profile.cache.lastLogin: "2024-01-01T00:00:00Z"
```

#### Array Modifications
```
+ items[3]: "new-item"
- items[1]: "removed-item"
~ items[0].name: "old-name" -> "new-name"
```

#### Mixed Changes
```
+ database.port: 5432
~ database.host: "localhost" -> "prod-db.example.com"
- cache.enabled: true
! debug: "false" -> false
```

## Design Principles

### 1. Semantic Clarity
The diffx format prioritizes understanding **what changed** over **how the text changed**:

- Shows `database.port: 5432 -> 6432` instead of line-by-line text differences
- Groups related changes by their semantic meaning
- Maintains data structure context

### 2. Format Independence
The same diffx format output represents changes consistently across all supported data formats:

- JSON, YAML, TOML, XML, INI, CSV all produce unified diffx format output
- Users learn one format instead of format-specific diff representations
- Tools can process diffx format output regardless of source data format

### 3. Path Precision
Full path notation eliminates ambiguity about change location:

- `config.database.connection.host` vs ambiguous line numbers
- Array indices clearly specified: `users[2].email`
- Nested changes maintain full context

### 4. Type Safety
Explicit type change detection prevents data corruption:

- Distinguishes `"8080" -> 8080` (type change) from `8080 -> 9090` (value change)
- Helps identify unintended type conversions
- Critical for API schema evolution and configuration management

## Use Cases

### DevOps and Configuration Management
```bash
# Infrastructure configuration comparison
diffx infrastructure.json infrastructure.new.json
# Output:
# ~ services.database.instance_type: "t3.micro" -> "t3.small"
# + services.cache.enabled: true
# - services.legacy.port: 3000
```

### API Schema Evolution
```bash
# OpenAPI specification comparison  
diffx api-v1.yaml api-v2.yaml --path "paths"
# Output:
# + /users.post.responses.201: {"description": "Created"}
# ~ /users/{id}.get.parameters[0].schema.type: "integer" -> "string"
```

### Data Pipeline Validation
```bash
# ETL output validation
diffx expected_output.json actual_output.json --array-id-key "id"
# Output:
# ~ records[id=123].status: "pending" -> "completed"
# + records[id=456]: {"status": "new", "timestamp": "2024-01-01T12:00:00Z"}
```

## Advantages Over Traditional Diff

| Traditional Diff | diffx Format |
|------------------|--------------|
| `- "port": 8080,`<br>`+ "port": 9090,` | `~ port: 8080 -> 9090` |
| Shows line changes | Shows semantic changes |
| Format-dependent output | Consistent across all formats |
| Sensitive to formatting | Ignores irrelevant formatting |
| No type awareness | Explicit type change detection |
| Context-poor | Full hierarchical context |

## Integration and Tooling

The diffx format is designed for both human consumption and machine processing:

### Human Consumption
- Clear, intuitive symbols
- Hierarchical path context
- Consistent formatting rules

### Machine Processing  
- Predictable syntax for parsing
- Structured change representation
- Tool-friendly output format

### Command Line Integration
```bash
# Generate diffx format output
diffx config.json config.new.json > changes.diffx

# Process diffx format with standard tools
grep "^+" changes.diffx | wc -l    # Count additions
grep "database\." changes.diffx    # Find database changes
```

## Future Extensions

The diffx format specification may be extended to support:

- **Confidence Levels**: Indicate certainty of detected changes
- **Change Metadata**: Include timestamps, authors, or change reasons
- **Semantic Annotations**: Add business context to technical changes
- **Diff Compression**: Compact representation for large changesets

## Adoption and Standardization

To establish the diffx format as an industry standard:

1. **Open Specification**: Public, versioned specification document
2. **Reference Implementation**: The `diffx` tool as canonical implementation
3. **Tool Ecosystem**: Support in editors, CI/CD tools, and analysis software
4. **Community Feedback**: Iterative improvement based on real-world usage

The goal is for "diffx format" to become as recognizable and useful as "JSON format" or "YAML format" in the developer ecosystem.

---

*This specification reflects diffx format version 1.0. For the latest updates and community discussions, see the [diffx project repository](https://github.com/kako-jun/diffx).*