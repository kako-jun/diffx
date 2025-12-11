# Basic JSON Comparison

Compare two JSON files and show semantic differences.

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json
? 1
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

## No Differences

When files are identical, no output is shown and exit code is 0.

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file1.json

```
