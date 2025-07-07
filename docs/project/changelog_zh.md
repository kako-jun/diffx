# 更新日志

本文件记录了项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)，
本项目遵循 [语义化版本控制](https://semver.org/spec/v2.0.0.html)。

## [未发布]

### 新增
- 全面的文档重构，采用分层结构
- 性能基准测试和优化指南
- CI/CD 平台和开发工具的集成指南
- 详细功能分析的工具比较矩阵
- 8个行业类别的真实世界示例

### 变更
- 文档结构重组为用户指南、参考、指南和项目部分
- README 简化并使用更平易近人的语言
- 徽章链接更新为指向 GitHub 文档而非 docs.rs

### 修复
- 修正统一格式说明中的矛盾陈述
- 更新格式支持信息以反映当前实现状态

## [0.2.0] - 2025-01-15

### 新增
- **XML 格式支持** - 完整的 XML 文件解析和比较支持
- **INI 格式支持** - 完整的 INI/配置文件格式支持  
- **CSV 格式支持** - CSV 文件比较支持数组元素跟踪
- **配置文件支持** - 在 `~/.config/diffx/config.toml` 的 TOML 配置文件
- **环境变量支持** - 所有 CLI 选项都可通过环境变量设置
- **目录比较** - 带 `--recursive` 标志的递归目录比较
- **路径过滤** - `--path` 选项专注于特定数据部分的比较
- **浮点数容差** - `--epsilon` 选项用于带容差的数值比较
- **数组元素跟踪** - `--array-id-key` 用于智能数组元素识别
- **正则表达式过滤** - `--ignore-keys-regex` 从比较中排除键
- **多种输出格式** - JSON、YAML 和统一差异输出选项
- **标准输入支持** - 使用 `-` 作为文件名与 stdin 比较文件
- **格式自动检测** - 从文件扩展名自动检测格式
- **类型变更检测** - 明确报告数据类型变更（例如，字符串到数字）
- **全面测试套件** - 23个测试用例覆盖所有功能和边界情况
- **性能基准测试** - 基于 Criterion 的性能监控基准测试
- **跨平台支持** - Linux、macOS 和 Windows 兼容性
- **CI/CD 集成** - 用于测试和发布的 GitHub Actions 工作流

### 变更
- **主要版本升级**以反映重要功能添加
- **Rust 版本更新**至 2021 版以使用最新语言功能
- **CLI 界面重新设计**，具有一致的参数命名和更好的帮助文本
- **错误处理改进**，提供更具描述性的错误消息和退出代码
- **内存使用优化**用于大文件处理
- **解析器架构重构**以提高扩展性和性能

### 修复
- **INI 解析边界情况** - 改进对格式错误的 INI 文件的处理
- **数组比较逻辑** - 修复空数组和嵌套结构的问题
- **Unicode 处理** - 在所有文本格式中正确支持 Unicode
- **文件扩展名检测** - 更强健的文件类型推断
- **内存泄漏** - 消除解析器错误路径中的内存泄漏

### 移除
- **遗留命令行选项** - 移除 0.1.x 系列中的已弃用标志

## [0.1.2] - 2024-12-20

### 新增
- **Python 包装器包** - 用于 Python 集成的 `diffx-python` pip 包
- **Node.js 包装器包** - 用于 JavaScript 集成的 `diffx-npm` npm 包
- **日语文档** - README 和核心文档的完整日语翻译
- **真实基准数据** - 在 AMD Ryzen 7 5800X 上的实际性能测量

### 变更
- **文档改进** - 更好的示例和更清晰的说明
- **性能优化** - 大文件速度提升 15-20%
- **错误消息清晰度** - 更有用的错误描述

### 修复
- **TOML 解析边界情况** - 更好地处理复杂 TOML 结构
- **CLI 参数验证** - 改进命令行参数验证
- **文件路径处理** - 修复相对路径解析问题

## [0.1.1] - 2024-12-10

### 新增
- **TOML 格式支持** - 完整的 TOML 文件解析和比较
- **增强的 CLI 输出** - 改进的带颜色和符号的视觉格式
- **扩展测试覆盖** - TOML 和边界情况的额外测试用例

### 变更
- **输出格式** - 更好缩进的更可读 CLI 输出
- **性能改进** - 优化的 JSON 和 YAML 解析

### 修复
- **YAML 解析问题** - 修复复杂 YAML 结构的处理
- **CLI 退出代码** - 不同场景的正确退出代码处理
- **内存使用** - 减少大型 JSON 文件的内存占用

## [0.1.0] - 2024-12-01

### 新增
- **初始发布** diffx 语义差异工具
- **JSON 格式支持** - 完整的 JSON 文件解析和比较
- **YAML 格式支持** - 完整的 YAML 文件解析和比较
- **核心差异引擎** - 结构化数据的语义差异检测
- **CLI 界面** - 带基本选项的命令行工具
- **基本输出格式** - CLI 和 JSON 输出模式
- **Rust crate** - 用于 Rust 应用程序的 `diffx-core` 库
- **跨平台构建** - 支持 Linux、macOS 和 Windows
- **GitHub 发布** - 自动化二进制发布
- **基本文档** - 带使用示例的 README

### 技术细节
- 使用 Rust 2021 版本构建
- 使用 `serde_json` 进行 JSON 解析
- 使用 `serde_yaml` 进行 YAML 解析
- 使用 `clap` 进行 CLI 参数解析
- 使用 `anyhow` 进行错误处理
- 带 `cargo test` 的全面测试套件

## 版本历史摘要

| 版本 | 发布日期 | 关键功能 |
|------|----------|----------|
| **0.2.0** | 2025-01-15 | XML/INI/CSV 支持、高级过滤、配置文件 |
| **0.1.2** | 2024-12-20 | Python/Node.js 包装器、日语文档、真实基准测试 |
| **0.1.1** | 2024-12-10 | TOML 支持、增强的 CLI 输出、改进性能 |
| **0.1.0** | 2024-12-01 | JSON/YAML 支持的初始发布 |

## 迁移指南

### 从 0.1.x 到 0.2.0

**破坏性变更：**
- 无 - 0.2.0 与 0.1.x 完全向后兼容

**可用的新功能：**
```bash
# 新格式支持
diffx config.xml config.new.xml
diffx database.ini database.new.ini
diffx data.csv data.new.csv

# 新过滤选项
diffx config.json config.new.json --path "database"
diffx config.json config.new.json --ignore-keys-regex "^timestamp$"
diffx users.json users.new.json --array-id-key "id"

# 新输出格式
diffx config.json config.new.json --output yaml
diffx config.json config.new.json --output unified

# 配置文件支持
echo 'ignore_keys_regex = "^(timestamp|_.*)"' > ~/.config/diffx/config.toml
```

**推荐升级：**
1. **添加配置文件**用于频繁使用的选项
2. **使用数组 ID 键**获得更好的数组比较
3. **应用正则表达式过滤**忽略不相关字段
4. **利用路径过滤**用于大型配置文件

### 从 0.1.0 到 0.1.1

**新功能：**
```bash
# 添加了 TOML 支持
diffx config.toml config.new.toml

# 增强的输出格式
diffx config.json config.new.json  # 现在有颜色和更好的格式
```

## 贡献者

- **kako-jun** - 项目创建者和主要维护者
- **社区贡献者** - 完整列表请参见 GitHub 贡献者页面

## 致谢

特别感谢：
- **Rust 社区**提供优秀的解析库
- **serde 生态系统**提供序列化支持
- **clap** 提供 CLI 参数解析
- **criterion** 提供基准测试框架
- **GitHub Actions** 提供 CI/CD 基础设施

## 支持和反馈

- **问题**：[GitHub Issues](https://github.com/kako-jun/diffx/issues)
- **讨论**：[GitHub Discussions](https://github.com/kako-jun/diffx/discussions)
- **文档**：[GitHub Pages](https://kako-jun.github.io/diffx/)

## 许可证

本项目使用 MIT 许可证 - 详情请参见 [LICENSE](../../LICENSE) 文件。