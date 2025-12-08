# Output Formats

diffx can output differences in various formats for automation and integration.

## JSON Output

Use `--output json` to get machine-readable JSON output:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json --output json
? failed
[
  {
    "Modified": [
      "age",
      30,
      31
    ]
  },
  {
    "Modified": [
      "city",
      "New York",
      "Boston"
    ]
  },
  {
    "Added": [
      "items[2]",
      "orange"
    ]
  }
]

```

Each difference is represented as an object with:
- `Modified`: `[path, old_value, new_value]`
- `Added`: `[path, new_value]`
- `Removed`: `[path, old_value]`

## YAML Output

Use `--output yaml` for YAML format:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json --output yaml
? failed
- Modified:
  - age
  - 30
  - 31
- Modified:
  - city
  - New York
  - Boston
- Added:
  - items[2]
  - orange


```
