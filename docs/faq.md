# diffx Frequently Asked Questions (FAQ)

This section provides answers to common questions about `diffx`.

## General Questions

### Q: What is diffx?

A: `diffx` is a command-line tool designed for comparing structured data formats like JSON, YAML, and TOML. Unlike traditional text-based `diff` tools, `diffx` understands the underlying structure and semantics of the data, focusing on meaningful changes rather than superficial differences like whitespace or key order.

### Q: How is diffx different from a regular `diff` command?

A: A regular `diff` command performs a line-by-line comparison of text files. This means it will report differences for changes in whitespace, key order, or comments, even if the underlying data structure remains semantically identical. `diffx`, on the other hand, parses the structured data and compares the actual values and their relationships, ignoring non-semantic changes.

### Q: What structured data formats does diffx support?

A: Currently, `diffx` supports JSON, YAML, and TOML. We plan to extend support to other formats like XML and INI in the future.

### Q: Does diffx support comparing directories?

A: Yes, `diffx` supports recursive directory comparison using the `--recursive` option. It will find corresponding files in both directories and compare them.

## Usage Questions

### Q: How can I specify the input format if diffx can't infer it?

A: You can use the `--format` option to explicitly tell `diffx` the format of your input files. This is particularly useful when reading from standard input or when files have non-standard extensions.

```bash
cat my_data.json | diffx - other_data.json --format json
```

### Q: Can I filter differences to a specific path?

A: Yes, the `--path` option allows you to filter the output to show only differences that occur within a specified data path. For example, `--path "config.users[0].name"`.

### Q: How do I ignore certain keys from the comparison?

A: You can use the `--ignore-keys-regex` option with a regular expression to specify keys that should be ignored during the comparison. This is useful for fields like timestamps or unique IDs that change frequently but are not semantically important for your comparison.

```bash
diffx file1.json file2.json --ignore-keys-regex "^id$|^timestamp$"
```

### Q: How does diffx handle floating-point number comparisons?

A: `diffx` provides an `--epsilon` option to specify a tolerance for floating-point number comparisons. If the absolute difference between two numbers is less than or equal to the epsilon value, they are considered equal.

```bash
diffx data1.json data2.json --epsilon 0.00001
```

### Q: How does diffx compare array elements, especially when their order changes?

A: By default, `diffx` compares array elements by their index. However, for arrays of objects, you can use the `--array-id-key` option to specify a key that uniquely identifies each object within the array. This allows `diffx` to correctly track additions, deletions, and modifications of array elements even if their order changes.

```bash
diffx users1.json users2.json --array-id-key "uuid"
```

## Technical Questions

### Q: What is the underlying technology used by diffx?

A: `diffx` is built with Rust, leveraging its performance, memory safety, and cross-platform capabilities. It uses `serde_json`, `serde_yaml`, and `toml` for parsing, and `colored` for CLI output, and `similar` for unified diff output.

### Q: Why is TOML output not fully supported?

A: TOML has a stricter type system compared to JSON or YAML. Directly serializing the flexible `DiffResult` enum (which can contain various data types) into a TOML-compatible structure without losing information or introducing complex workarounds is challenging. While `diffx` can parse TOML, outputting diff results *as* TOML is currently not fully supported to avoid data degradation.

### Q: Can I use diffx in my CI/CD pipeline?

A: Yes, `diffx` is designed to be easily integrated into CI/CD pipelines. You can use it to automatically check for structural changes in configuration files or data. Refer to the [CI/CD Integration Examples](ci-cd.md) for more details.

### Q: What is "Meta-chaining" in diffx?

A: Meta-chaining refers to the advanced capability of `diffx` to compare its own output. By saving `diffx`'s JSON or YAML output (which represents the differences between two files) to a file, you can then use `diffx` again to compare these "diff reports." This allows you to track changes in your change history, audit configurations, or analyze the evolution of your system at a meta-level.
