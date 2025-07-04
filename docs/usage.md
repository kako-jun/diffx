# diffx Usage Guide

This guide provides detailed examples of how to use `diffx` for various structured data comparison tasks.

## Basic Usage

Compare two JSON files:

```bash
diffx file1.json file2.json
```

Compare two YAML files:

```bash
diffx file1.yaml file2.yaml
```

Compare two TOML files:

```bash
diffx file1.toml file2.toml
```

Compare two INI files:

```bash
diffx file1.ini file2.ini
```

Compare two XML files:

```bash
diffx file1.xml file2.xml
```

Compare two CSV files:

```bash
diffx file1.csv file2.csv
```

## Specifying Input Format

If `diffx` cannot infer the format from the file extension (e.g., when using standard input or custom file extensions), you can explicitly specify the format using the `--format` option:

```bash
cat file1.json | diffx - file2.json --format json
```

## Output Formats

`diffx` supports several output formats. The default is the human-readable CLI output.

### CLI Output (Default)

This format provides a colored, indented, and symbolic representation of the differences. It focuses on semantic changes.

```bash
diffx file1.json file2.json --output cli
```

### JSON Output

For machine readability, you can output differences as a JSON array:

```bash
diffx file1.json file2.json --output json
```

Example JSON output:

```json
[
  {
    "Added": [
      "config.users[2]",
      {
        "id": 3,
        "name": "Charlie"
      }
    ]
  },
  {
    "Modified": [
      "config.users[1].name",
      "Bob",
      "Robert"
    ]
  }
]
```

### YAML Output

Similar to JSON output, but in YAML format:

```bash
diffx file1.json file2.json --output yaml
```

Example YAML output:

```yaml
- Added:
  - config.users[2]
  - id: 3
    name: Charlie
- Modified:
  - config.users[1].name
  - Bob
  - Robert
```

### Unified Format

For compatibility with `git` and other diff tools, `diffx` can output in a unified diff format. Note that this format is text-based and may show differences that `diffx` considers non-semantic (e.g., whitespace changes).

```bash
diffx file1.json file2.json --output unified
```

## Filtering Differences by Path

Use the `--path` option to display only differences within a specific path. This is useful for focusing on relevant sections of large configurations.

```bash
diffx file1.json file2.json --path "config.users[1]"
```

## Ignoring Keys by Regular Expression

Use `--ignore-keys-regex` to exclude keys matching a regular expression from the comparison. This is useful for ignoring dynamic fields like timestamps or IDs that are not relevant for semantic comparison.

```bash
diffx file1.json file2.json --ignore-keys-regex "^_.*$"
```

## Tolerance for Float Comparisons

When comparing floating-point numbers, small differences due to precision can be ignored using the `--epsilon` option.

```bash
diffx data1.json data2.json --epsilon 0.00001
```

## Identifying Array Elements by Key

For arrays of objects, `diffx` can track elements by a unique identifier key using `--array-id-key`. This allows `diffx` to correctly identify modified, added, or removed elements even if their order changes.

```bash
diffx users1.json users2.json --array-id-key "id"
```

## Directory Comparison

Compare two directories recursively using the `--recursive` option. `diffx` will compare corresponding files within the directories.

```bash
diffx dir1/ dir2/ --recursive
```

## Advanced Usage: Diffing Diff Reports (Meta-chaining)

One powerful feature of `diffx` is the ability to compare its own output. By saving `diffx` output in JSON or YAML format, you can then use `diffx` to compare these "diff reports" themselves. This is useful for tracking changes in your configuration change history, auditing, or understanding how your deployments evolve.

1.  Generate a diff report and save it to a file:

    ```bash
    diffx config_v1.json config_v2.json --output json > diff_report_v1.json
    ```

2.  Later, generate another diff report:

    ```bash
    diffx config_v2.json config_v3.json --output json > diff_report_v2.json
    ```

3.  Compare the two diff reports:

    ```bash
    diffx diff_report_v1.json diff_report_v2.json
    ```

This allows you to see the differences in the *changes themselves*, providing a meta-level view of your system's evolution.
