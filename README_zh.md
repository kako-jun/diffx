# diffx

[![build status](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions)
[![Crates.io](https://img.shields.io/crates/v/diffx)](https://crates.io/crates/diffx)
[![npm version](https://badge.fury.io/js/diffx.svg)](https://badge.fury.io/js/diffx)
[![PyPI version](https://badge.fury.io/py/diffx.svg)](https://badge.fury.io/py/diffx)

下一代差异比较工具，能理解您数据的含义

[English](README.md) | [日本語](README_ja.md) | [中文](README_zh.md)

`diffx` 是一款功能强大且速度极快的命令行工具，专为比较 JSON、YAML、TOML 等结构化数据文件的语义差异而设计。它能理解数据的结构和含义，忽略格式和无关紧要的变动，仅高亮显示核心的修改。

![diffx-demo](https://github.com/user-attachments/assets/b3333184-d375-482e-95a3-431f53857c2c)

## 主要特性

- **语义比较**: 忽略空白、注释和键顺序的差异，只关注数据在语义层面的真实变化。
- **多格式支持**: 原生支持 JSON、YAML、TOML、XML、INI 和 CSV。
- **高级过滤**: 使用正则表达式或类 JSONPath 的语法，忽略或仅关注特定的键和路径。
- **灵活的输出**: 提供人类可读的 CLI 视图、`unified` 格式、JSON 和 YAML 等多种输出选项。
- **卓越性能**: 基于 Rust 构建，速度极快，并通过内存高效的流式处理来处理大文件。
- **基于ID的数组比较**: 能够区分数组中元素的顺序变化和元素本身的修改。
- **数值容差**: 允许在比较浮点数时设置一个小的容差范围。

## 为何选择 diffx？

传统的 `diff` 工具按行比较文本，不适合比较结构化数据（尤其是 JSON 或 YAML）。键的顺序变化或格式调整等无关紧要的修改，都会被 `diff` 视为大量差异，从而掩盖了真正的核心变更。

`diffx` 首先将文件解析为结构化数据，然后在语义层面进行比较。这使得开发人员可以忽略噪音，专注于重要的变更。

## 安装

### Homebrew (macOS / Linux)

```bash
brew install kako-jun/tap/diffx
```

### Cargo (Rust)

```bash
cargo install diffx
```

### npm (Node.js)

```bash
npm install -g diffx
```

### PyPI (Python)

```bash
pip install diffx
```

### 手动下载

您可以从 [Releases 页面](https://github.com/kako-jun/diffx/releases) 下载适用于您操作系统的预编译二进制文件。

## 基本用法

比较两个文件非常简单：

```bash
diffx file1.json file2.json
```

### 支持的格式

`diffx` 会根据文件扩展名自动检测文件格式：

- **JSON**: `.json`
- **YAML**: `.yaml`, `.yml`
- **TOML**: `.toml`
- **XML**: `.xml`
- **INI**: `.ini`, `.cfg`, `.conf`
- **CSV**: `.csv`

您甚至可以比较不同格式的文件：

```bash
diffx config.json config.backup.toml
```

## 高级用法

### 更改输出格式

使用 `--output` 或 `-o` 选项，将输出格式更改为 `json` 或 `yaml`。

```bash
diffx file1.json file2.json --output json
```

### 忽略特定键

使用 `--ignore-keys-regex`，可以通过正则表达式忽略特定的键。

```bash
# 忽略以 "timestamp" 或 "id" 结尾的键
diffx data1.json data2.json --ignore-keys-regex "(timestamp|id)$"
```

### 比较特定路径

使用 `--path` 选项，可以将比较范围限制在特定的数据路径下。

```bash
# 只显示 "database.connections" 下的差异
diffx config1.json config2.json --path "database.connections"
```

### 指定数组ID键

使用 `--array-id-key`，可以根据指定的键值而不是位置来匹配数组中的对象。这有助于区分顺序变化和内容修改。

```bash
# 使用 "user_id" 作为键来比较数组元素
diffx users1.json users2.json --array-id-key "user_id"
```

更多选项请参阅 [CLI 参考文档](docs/reference/cli-reference_zh.md)。

## 贡献

我们欢迎任何形式的贡献，包括错误报告、功能建议或拉取请求。详情请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

本项目基于 [MIT 许可证](LICENSE) 发布。
