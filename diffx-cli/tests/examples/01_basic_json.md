# Basic JSON Comparison

diffx can compare two JSON files and show semantic differences.

## Simple Object Comparison

Given two JSON files with different values:

**file1.json:**
```json
{
  "name": "John",
  "age": 30,
  "city": "New York",
  "items": ["apple", "banana"]
}
```

**file2.json:**
```json
{
  "name": "John",
  "age": 31,
  "city": "Boston",
  "items": ["apple", "banana", "orange"]
}
```

Running diffx shows the differences:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json
? failed
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

The output shows:
- `~` indicates a **modified** value
- `+` indicates an **added** value
- `-` indicates a **removed** value (not shown in this example)

Exit code `1` means differences were found.
