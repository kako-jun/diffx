# diffx Documentation

Welcome to the comprehensive `diffx` documentation!

`diffx` is a semantic diff tool for structured data that understands meaning, not just formatting. Unlike traditional text-based diff tools, `diffx` focuses on actual changes in your data structure.

## Quick Links

- **[Getting Started](user-guide/getting-started.md)** - Learn the basics and core concepts
- **[Installation Guide](user-guide/installation.md)** - Set up diffx on your system
- **[CLI Reference](reference/cli-reference.md)** - Complete command-line documentation
- **[Real-World Examples](user-guide/examples.md)** - Practical use cases across industries

## Documentation Structure

### 📚 User Guide
*Essential guides for getting started and daily usage*

- **[Installation](user-guide/installation.md)** - Platform-specific installation instructions
- **[Getting Started](user-guide/getting-started.md)** - Basic concepts and first steps
- **[Configuration](user-guide/configuration.md)** - Configuration files and settings
- **[Examples](user-guide/examples.md)** - Real-world examples across 8 industry categories
- **[FAQ](user-guide/faq.md)** - Frequently asked questions and troubleshooting

### 📖 Reference
*Complete technical reference documentation*

- **[CLI Reference](reference/cli-reference.md)** - Complete command-line interface documentation
- **[API Reference](reference/api-reference.md)** - Rust crate API documentation
- **[Tool Comparison](reference/comparison.md)** - How diffx compares to other tools

### 🛠️ Guides
*Advanced topics and integration guidance*

- **[Integration Guide](guides/integrations.md)** - CI/CD, development tools, and automation
- **[Performance Guide](guides/performance.md)** - Benchmarks and optimization strategies

### 📋 Project Information
*Project governance and development information*

- **[Changelog](project/changelog.md)** - Version history and release notes
- **[Roadmap](project/roadmap.md)** - Future development plans
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute to the project

## What Makes diffx Different?

### Semantic Understanding
```bash
# Traditional diff shows formatting noise
$ diff config1.json config2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }

# diffx shows only meaningful changes
$ diffx config1.json config2.json
~ version: "1.0" -> "1.1"
```

### Multi-Format Support
Supports 6 structured data formats:
- **JSON** - Web APIs, configuration files
- **YAML** - Kubernetes, Docker Compose, CI/CD
- **TOML** - Rust projects, modern config files
- **XML** - Legacy systems, SOAP APIs
- **INI** - Traditional configuration files
- **CSV** - Data exports, tabular data

### AI and Automation Friendly
- **Consistent CLI interface** across all formats
- **Machine-readable output** (JSON, YAML)
- **Flexible filtering** with regex patterns
- **Zero-configuration** operation with smart defaults

## Language Versions

- **[English Documentation](./index.md)** (Current)
- **[日本語ドキュメント](./index_ja.md)** - Japanese version

## Community and Support

- **[GitHub Repository](https://github.com/kako-jun/diffx)** - Source code and issue tracking
- **[GitHub Discussions](https://github.com/kako-jun/diffx/discussions)** - Community discussions
- **[GitHub Releases](https://github.com/kako-jun/diffx/releases)** - Download latest version

---

*Looking for something specific? Use the search function or check our [FAQ](user-guide/faq.md) for common questions.*