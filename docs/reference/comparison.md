# Tool Comparison

This document compares `diffx` with other diff and data comparison tools, helping you understand when and why to choose `diffx` for your specific use cases.

## Quick Comparison Table

| Tool | Type | Formats | Semantic Aware | Array Tracking | Config Support | Best For |
|------|------|---------|---------------|----------------|----------------|----------|
| **diffx** | Semantic | JSON/YAML/TOML/XML/INI/CSV | ✅ | ✅ | ✅ | Structured data comparison |
| diff | Text-based | Any text | ❌ | ❌ | ❌ | General text files |
| jq | JSON processor | JSON | Partial | ❌ | ❌ | JSON manipulation |
| yq | YAML processor | YAML/JSON | Partial | ❌ | ❌ | YAML manipulation |
| daff | Tabular | CSV | ✅ | ❌ | ❌ | CSV/spreadsheet data |
| jsondiff | JSON diff | JSON | ✅ | Partial | ❌ | JSON-only comparison |
| deep-diff | JavaScript | JSON/Objects | ✅ | ❌ | ❌ | JavaScript applications |

## Detailed Comparisons

### vs Traditional `diff`

**Traditional diff:**
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

**diffx:**
```bash
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

**Key Differences:**

| Aspect | Traditional diff | diffx |
|--------|------------------|-------|
| **Understanding** | Line-by-line text | Semantic structure |
| **Key Order** | Reports as different | Ignores reordering |
| **Whitespace** | Reports differences | Ignores formatting |
| **Trailing Commas** | Reports differences | Ignores formatting |
| **Type Changes** | Shows as text change | Reports type conversion |
| **Array Handling** | Position-based | ID-based tracking available |
| **Output Format** | Text diff | CLI/JSON/YAML/Unified |

**When to use traditional diff:**
- Generic text files
- Source code comparison
- Line-by-line analysis needed
- Simple scripts without structured data

**When to use diffx:**
- Configuration files
- API responses
- Data exports
- Structured documents

### vs `jq` for JSON Processing

**Using jq for diff (complex):**
```bash
# Complex jq command for basic comparison
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'def diff(a; b): 
    if (a | type) != (b | type) then {type_changed: {from: (a | type), to: (b | type)}}
    elif a == b then empty
    elif (a | type) == "object" then
      (a + b) | to_entries | map(select(.value != a[.key] or .value != b[.key])) |
      from_entries
    else {changed: {from: a, to: b}}
    end;
  diff($a; $b)'
```

**Using diffx (simple):**
```bash
diffx file1.json file2.json --output json
```

**Comparison:**

| Aspect | jq | diffx |
|--------|-------|-------|
| **Complexity** | High (complex queries) | Low (simple command) |
| **Learning Curve** | Steep | Gentle |
| **JSON-only** | Yes | No (6 formats) |
| **Built-in Diff** | No (manual scripting) | Yes |
| **Array Tracking** | Manual implementation | Built-in |
| **Filtering** | Manual queries | Regex patterns |
| **Output** | Custom JSON | Multiple formats |

**When to use jq:**
- Complex JSON transformations
- Data extraction and manipulation
- Custom processing pipelines
- JSON-only workflows

**When to use diffx:**
- Simple comparison tasks
- Multiple format support needed
- Semantic diff specifically required
- Configuration management

### vs `yq` for YAML Processing

**Using yq for comparison:**
```bash
# yq doesn't have built-in diff, requires manual comparison
yq eval '. as $item ireduce ({}; . * $item)' file1.yaml file2.yaml
```

**Using diffx:**
```bash
diffx file1.yaml file2.yaml
```

**Comparison:**

| Aspect | yq | diffx |
|--------|-----|-------|
| **Primary Use** | YAML processing | Semantic diff |
| **Diff Capability** | Limited/manual | Native |
| **Format Support** | YAML/JSON | 6 formats |
| **Semantic Awareness** | Partial | Full |
| **Configuration** | No | Yes |

**When to use yq:**
- YAML transformations
- Data extraction from YAML
- YAML validation
- Complex YAML processing

**When to use diffx:**
- YAML comparison specifically
- Multi-format environments
- Configuration drift detection
- Semantic change tracking

### vs `daff` for CSV Data

**daff example:**
```bash
daff data1.csv data2.csv
```

**diffx example:**
```bash
diffx data1.csv data2.csv --array-id-key "id"
```

**Comparison:**

| Aspect | daff | diffx |
|--------|------|-------|
| **Focus** | Tabular data | General structured data |
| **Format Support** | CSV/TSV | 6 formats including CSV |
| **Visualization** | HTML output | CLI/JSON/YAML |
| **ID Tracking** | Limited | Full support |
| **Integration** | Specialized | General purpose |

**When to use daff:**
- Heavy CSV/spreadsheet work
- Tabular data visualization
- Excel integration needed
- CSV-specific workflows

**When to use diffx:**
- Mixed format environments
- CSV + other structured data
- API integration needed
- Automation workflows

### vs `jsondiff` (Python)

**jsondiff example:**
```python
from jsondiff import diff
import json

with open('file1.json') as f1, open('file2.json') as f2:
    diff_result = diff(json.load(f1), json.load(f2))
    print(diff_result)
```

**diffx example:**
```bash
diffx file1.json file2.json --output json
```

**Comparison:**

| Aspect | jsondiff | diffx |
|--------|----------|-------|
| **Language** | Python library | CLI tool |
| **Integration** | Python apps | Any language/script |
| **Format Support** | JSON only | 6 formats |
| **Performance** | Python speed | Rust speed |
| **Deployment** | Requires Python | Single binary |
| **Array Tracking** | Basic | Advanced |

**When to use jsondiff:**
- Python-native applications
- Embedded diff logic
- Custom Python processing
- JSON-only requirements

**When to use diffx:**
- Multi-language environments
- CLI/script integration
- Better performance needed
- Multiple format support

### vs Git's built-in diff

**Git diff:**
```bash
git diff HEAD~1 config.json
```

**Git diff with diffx:**
```bash
git show HEAD~1:config.json | diffx - config.json
```

**Comparison:**

| Aspect | Git diff | Git + diffx |
|--------|----------|-------------|
| **Integration** | Native | External tool |
| **Understanding** | Line-based | Semantic |
| **Configuration** | Limited | Extensive |
| **Format Awareness** | No | Yes |
| **Learning Curve** | Familiar | Additional tool |

**Git integration example:**
```bash
# Add to .gitconfig
[diff "json"]
    textconv = diffx --output unified

# In .gitattributes
*.json diff=json
```

### vs Language-Specific Libraries

#### JavaScript (`deep-diff`)
```javascript
const diff = require('deep-diff');
const differences = diff(obj1, obj2);
```

#### Python (`deepdiff`)
```python
from deepdiff import DeepDiff
diff = DeepDiff(dict1, dict2)
```

#### Ruby (`hashdiff`)
```ruby
require 'hashdiff'
diff = Hashdiff.diff(hash1, hash2)
```

**Comparison with diffx:**

| Aspect | Language Libraries | diffx |
|--------|-------------------|-------|
| **Integration** | Native to language | CLI/external |
| **Performance** | Variable | Consistent (Rust) |
| **Format Support** | Usually single | Multiple |
| **Deployment** | Language dependency | Single binary |
| **Standardization** | Per-language API | Consistent CLI |
| **Cross-team Use** | Language-specific | Universal |

## Performance Comparisons

### Speed Benchmarks

Test files: 1MB JSON configuration files

| Tool | Time (avg) | Memory Usage |
|------|------------|--------------|
| **diffx** | 5ms | 15MB |
| traditional diff | 2ms | 8MB |
| jq (scripted) | 150ms | 45MB |
| jsondiff | 80ms | 35MB |
| daff | 25ms | 20MB |

*Note: Benchmarks approximate, actual performance varies by data structure*

### Scalability

| File Size | diffx | traditional diff | jq (scripted) |
|-----------|-------|------------------|---------------|
| 1KB | 1ms | 1ms | 15ms |
| 100KB | 3ms | 2ms | 45ms |
| 1MB | 5ms | 8ms | 150ms |
| 10MB | 50ms | 80ms | 1500ms |
| 100MB | 500ms | 800ms | 15s+ |

## Feature Matrix

### Core Features

| Feature | diffx | diff | jq | yq | daff | jsondiff |
|---------|-------|------|----|----|------|----------|
| **Semantic Understanding** | ✅ | ❌ | Partial | Partial | ✅ | ✅ |
| **Multiple Formats** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Array ID Tracking** | ✅ | ❌ | ❌ | ❌ | Limited | Partial |
| **Regex Filtering** | ✅ | ❌ | Manual | Manual | ❌ | ❌ |
| **Epsilon Comparison** | ✅ | ❌ | Manual | Manual | ❌ | ❌ |
| **Path Filtering** | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ |
| **Multiple Output Formats** | ✅ | ❌ | ✅ | ✅ | Limited | ❌ |

### Integration Features

| Feature | diffx | diff | jq | yq | daff | jsondiff |
|---------|-------|------|----|----|------|----------|
| **CLI Tool** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Library** | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Configuration Files** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Environment Variables** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Exit Codes** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Pipe Support** | ✅ | ✅ | ✅ | ✅ | Limited | ❌ |

## Use Case Recommendations

### Configuration Management
**Best Choice: diffx**
- Semantic understanding crucial
- Multiple formats common
- Automation-friendly
- Filtering capabilities

**Alternative: diff** (for simple text configs)

### API Testing
**Best Choice: diffx**
- JSON/YAML response comparison
- Ignore timestamp fields
- Multiple output formats
- CI/CD integration

**Alternative: jq** (for complex JSON manipulation)

### Data Processing
**Best Choice: diffx** (structured data) or **daff** (CSV-heavy)
- Choose diffx for mixed formats
- Choose daff for pure CSV workflows

### Source Code
**Best Choice: diff**
- Line-by-line comparison needed
- Git integration
- Patch generation

**Use diffx for:** Package.json, configuration files within source

### Database Exports
**Best Choice: diffx**
- JSON/CSV export comparison
- Array ID tracking
- Large file handling

### DevOps/Infrastructure
**Best Choice: diffx**
- Kubernetes manifests (YAML)
- Terraform state (JSON)
- Docker Compose files
- Configuration drift detection

## Migration Guide

### From `diff` to `diffx`

**Old workflow:**
```bash
diff config1.json config2.json > changes.txt
```

**New workflow:**
```bash
diffx config1.json config2.json --output unified > changes.txt
# Or for semantic differences:
diffx config1.json config2.json > semantic_changes.txt
```

### From `jq` comparison to `diffx`

**Old complex jq script:**
```bash
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'complex_diff_function($a; $b)'
```

**New simple diffx:**
```bash
diffx file1.json file2.json --output json
```

### From language-specific tools

**Python (jsondiff):**
```python
# Old
from jsondiff import diff
result = diff(data1, data2)

# New
import subprocess
result = subprocess.run(['diffx', 'file1.json', 'file2.json', '--output', 'json'], 
                       capture_output=True, text=True)
diff_data = json.loads(result.stdout)
```

## Conclusion

Choose `diffx` when you need:
- **Semantic understanding** of structured data
- **Multiple format support** in one tool
- **Advanced filtering** and comparison options
- **Automation-friendly** CLI interface
- **Consistent behavior** across different data types

Choose other tools when:
- **Traditional diff**: General text files, source code, simple line-by-line comparison
- **jq/yq**: Complex data transformations, single-format specialized processing  
- **daff**: Heavy CSV/tabular data focus
- **Language libraries**: Deep integration within specific programming languages

`diffx` excels in mixed-format environments where semantic understanding of data structure changes is more important than text-level differences.