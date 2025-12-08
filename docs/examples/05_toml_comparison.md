# TOML Comparison

diffx supports TOML configuration files.

## Basic TOML Diff

**file1.toml:**
```toml
name = "John"
age = 30
city = "New York"
items = ["apple", "banana"]
```

**file2.toml:**
```toml
name = "John"
age = 31
city = "Boston"
items = ["apple", "banana", "orange"]
```

```console
$ diffx tests/fixtures/file1.toml tests/fixtures/file2.toml
? failed
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```
