# diffx 文档

欢迎来到 `diffx` 的全面文档！

`diffx` 是一个用于结构化数据的语义差异对比工具，能够理解含义而非仅仅是格式。与传统的基于文本的差异对比工具不同，`diffx` 专注于数据结构中的实际变更。

## 快速链接

- **[快速入门](user-guide/getting-started.md)** - 学习基础知识和核心概念
- **[安装指南](user-guide/installation.md)** - 在您的系统上设置 diffx
- **[CLI 参考](reference/cli-reference.md)** - 完整的命令行文档
- **[实际示例](user-guide/examples.md)** - 跨行业的实际用例

## 文档结构

### 📚 用户指南
*入门和日常使用的必备指南*

- **[安装](user-guide/installation.md)** - 平台特定的安装说明
- **[快速入门](user-guide/getting-started.md)** - 基础概念和第一步
- **[示例](user-guide/examples.md)** - 8 个行业类别的实际示例
- **[常见问题](user-guide/faq.md)** - 常见问题和故障排除

### 📖 参考
*完整的技术参考文档*

- **[diffx格式规范](reference/diffx-format.md)** - diffx格式的完整规范
- **[CLI参考](reference/cli-reference.md)** - 完整的命令行接口文档
- **[API参考](reference/api-reference.md)** - Rust crate API 文档
- **[工具比较](reference/comparison.md)** - diffx 与其他工具的比较

### 🛠️ 指南
*高级主题和集成指导*

- **[集成指南](guides/integrations.md)** - CI/CD、开发工具和自动化
- **[性能指南](guides/performance.md)** - 基准测试和优化策略

### 📋 项目信息
*项目治理和开发信息*

- **[贡献指南](../CONTRIBUTING.md)** - 如何为项目做贡献

## diffx 有何不同？

### 语义理解
```bash
# 传统 diff 显示格式噪音
$ diff config1.json config2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }

# diffx 仅显示有意义的变更
$ diffx config1.json config2.json
~ version: "1.0" -> "1.1"
```

### 多格式支持
支持 6 种结构化数据格式：
- **JSON** - Web API、配置文件
- **YAML** - Kubernetes、Docker Compose、CI/CD
- **TOML** - Rust 项目、现代配置文件
- **XML** - 遗留系统、SOAP API
- **INI** - 传统配置文件
- **CSV** - 数据导出、表格数据

### AI 和自动化友好
- **一致的 CLI 接口** 支持所有格式
- **机器可读输出** （JSON、YAML）
- **灵活的过滤** 使用正则表达式模式
- **零配置** 操作，智能默认值

## 语言版本

- **[English Documentation](./index.md)** - 英文版本
- **[日本語ドキュメント](./index_ja.md)** - 日文版本

## 社区和支持

- **[GitHub 仓库](https://github.com/kako-jun/diffx)** - 源代码和问题跟踪
- **[GitHub 讨论](https://github.com/kako-jun/diffx/discussions)** - 社区讨论
- **[GitHub 发布](https://github.com/kako-jun/diffx/releases)** - 下载最新版本

---

*寻找特定内容？使用搜索功能或查看我们的[常见问题](user-guide/faq.md)获取常见问题的答案。*