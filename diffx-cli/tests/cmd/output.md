# Output Formats

## JSON Output

Machine-readable JSON output for automation.

```console
$ diffx --output json tests/fixtures/file1.json tests/fixtures/file2.json
? 1
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

## YAML Output

```console
$ diffx --output yaml tests/fixtures/file1.json tests/fixtures/file2.json
? 1
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

## Brief Mode

Only report whether files differ.

```console
$ diffx --brief tests/fixtures/file1.json tests/fixtures/file2.json
? 1
Files tests/fixtures/file1.json and tests/fixtures/file2.json differ

```

## Quiet Mode

Exit code only, no output.

```console
$ diffx --quiet tests/fixtures/file1.json tests/fixtures/file2.json
? 1

```

