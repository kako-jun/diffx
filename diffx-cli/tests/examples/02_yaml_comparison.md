# YAML Comparison

diffx supports YAML files with the same semantic comparison.

## Basic YAML Diff

**file1.yaml:**
```yaml
name: John
age: 30
city: New York
items:
  - apple
  - banana
```

**file2.yaml:**
```yaml
name: John
age: 31
city: Boston
items:
  - apple
  - banana
  - orange
```

```console
$ diffx tests/fixtures/file1.yaml tests/fixtures/file2.yaml
? failed
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

The format is auto-detected from the file extension.
