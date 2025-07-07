# diffx 常见问题解答 (FAQ)

本节提供有关 `diffx` 常见问题的答案。

## 一般问题

### Q: 什么是 diffx？

A: `diffx` 是一个专为比较结构化数据格式（如 JSON、YAML 和 TOML）设计的命令行工具。与传统的基于文本的 `diff` 工具不同，`diffx` 理解数据的底层结构和语义，专注于有意义的变更，而非空白字符或键顺序等表面差异。

### Q: diffx 与常规 `diff` 命令有何不同？

A: 常规的 `diff` 命令对文本文件执行逐行比较。这意味着它会报告空白字符、键顺序或注释的变化，即使底层数据结构在语义上保持相同。另一方面，`diffx` 解析结构化数据并比较实际值及其关系，忽略非语义变更。

### Q: diffx 支持哪些结构化数据格式？

A: `diffx` 支持 JSON、YAML、TOML、XML、INI 和 CSV 格式。

### Q: diffx 支持比较目录吗？

A: 是的，`diffx` 使用 `--recursive` 选项支持递归目录比较。它会在两个目录中找到对应的文件并进行比较。

## 使用问题

### Q: 如果 diffx 无法推断输入格式，我如何指定？

A: 您可以使用 `--format` 选项明确告诉 `diffx` 输入文件的格式。这在从标准输入读取或文件具有非标准扩展名时特别有用。

```bash
cat my_data.json | diffx - other_data.json --format json
```

### Q: 我能将差异过滤到特定路径吗？

A: 是的，`--path` 选项允许您过滤输出，仅显示在指定数据路径内发生的差异。例如，`--path "config.users[0].name"`。

### Q: 如何从比较中忽略某些键？

A: 您可以使用带有正则表达式的 `--ignore-keys-regex` 选项来指定在比较期间应忽略的键。这对于时间戳或唯一 ID 等经常变化但对您的比较在语义上不重要的字段很有用。

```bash
diffx file1.json file2.json --ignore-keys-regex "^id$|^timestamp$"
```

### Q: diffx 如何处理浮点数比较？

A: `diffx` 提供 `--epsilon` 选项来指定浮点数比较的容差。如果两个数字之间的绝对差值小于或等于 epsilon 值，则认为它们相等。

```bash
diffx data1.json data2.json --epsilon 0.00001
```

### Q: diffx 如何比较数组元素，特别是当它们的顺序发生变化时？

A: 默认情况下，`diffx` 按索引比较数组元素。但是，对于对象数组，您可以使用 `--array-id-key` 选项指定一个唯一标识数组中每个对象的键。这允许 `diffx` 正确跟踪数组元素的添加、删除和修改，即使它们的顺序发生变化。

```bash
diffx users1.json users2.json --array-id-key "uuid"
```

## 技术问题

### Q: diffx 使用的底层技术是什么？

A: `diffx` 使用 Rust 构建，利用其性能、内存安全和跨平台能力。它使用 `serde_json`、`serde_yml`、`toml`、`configparser`、`quick-xml` 和 `csv` 进行解析，使用 `colored` 进行 CLI 输出，使用 `similar` 进行统一差异输出。

### Q: 为什么不完全支持 TOML 输出？

A: 与 JSON 或 YAML 相比，TOML 具有更严格的类型系统。将灵活的 `DiffResult` 枚举（可以包含各种数据类型）直接序列化为与 TOML 兼容的结构，而不丢失信息或引入复杂的变通方法是具有挑战性的。虽然 `diffx` 可以解析 TOML，但目前不完全支持将差异结果*作为* TOML 输出，以避免数据降级。

### Q: 我可以在我的 CI/CD 管道中使用 diffx 吗？

A: 是的，`diffx` 设计为易于集成到 CI/CD 管道中。您可以使用它自动检查配置文件或数据中的结构变更。有关更多详细信息，请参考[集成指南](../guides/integrations_zh.md)。

### Q: diffx 中的"元链式"是什么？

A: 元链式是指 `diffx` 比较自己输出的高级功能。通过将 `diffx` 的 JSON 或 YAML 输出（表示两个文件之间的差异）保存到文件中，您可以再次使用 `diffx` 来比较这些"差异报告"。这允许您跟踪变更历史中的变化、审计配置或在元级别分析系统的演进。