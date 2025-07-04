# diffx: Structured Data Diff Tool

## Motivation

In modern software development, structured data such as JSON, YAML, and TOML play an increasingly important role in configuration files, API responses, and data structures. However, existing tools for tracking changes in these data (e.g., the traditional `diff` command) are text-based and cannot understand the "structure" or "meaning" of the data.

This has led to developers facing inefficient problems such as:

*   **Exhaustion from superficial differences**: Formatting differences such as key order, whitespace, and trailing commas are displayed as "differences," making it easy to overlook essential changes.
*   **Limitations of manual verification**: For complex nested structures or large-scale data, manual diff verification is extremely difficult and can lead to errors.
*   **Difficulty in machine processing**: Text-based diffs are unsuitable for automated analysis by programs or understanding by AI.

`diffx` was born to solve these problems. We believe that in structured data diff comparison, we should focus on **"semantic changes"** rather than mere text changes. This frees developers from unnecessary exhaustion and allows them to concentrate on more essential tasks.

## Philosophy

**"Structured diffs, for everyone, everywhere, easily."**

Traditional `diff` commands are text-based and cannot understand the structure of data. `diffx` is a diff extraction tool specialized in structured data such such as JSON, YAML, and TOML. It provides output that is easy for both humans and AI to understand, clearly visualizing changes in configuration files, data, and other structured files.

`diffx` aims to detect **semantic changes** in data, not just text changes. For example, `diffx` does not detect changes in JSON key order or whitespace as differences. This allows you to focus on essential changes and avoid unnecessary exhaustion.

### Meaning of the name "diffx"
What does the "x" in diff + x mean?

*   **extended**: Extended diff (structured, semantic)
*   **exact**: Accurate diff extraction
*   **flexible**: Flexible format support
*   **indexed**: Trackable diffs with indexes
*   **next-gen**: Next-generation diff tool

## Specification

### Supported Formats
- JSON
- YAML
- TOML
- *Future: XML, INI, CSV*

### Types of Differences
- Key addition/deletion
- Value change
- Array insertion/deletion/modification
- Nested structure differences
- Value type change

### Output Formats
`diffx` recommends its own CLI display format that can most richly express structured data differences, but also supports the following alternative output formats for specific use cases and integration with existing tools:

- **Recommended CLI Display (Default)**
    *   A unique format that clearly displays structural differences (additions, changes, deletions, type changes, etc.) using universal design considerations such as color coding, symbols, and indentation, making it easy for humans to understand.
    *   Differences are represented by `+` (addition), `-` (deletion), `~` (change), `!` (type change) symbols and colors: blue, yellow, cyan, and magenta.
    *   **Feature**: Focuses on semantic changes in data, ignoring changes in key order or whitespace. This is the core value of `diffx`.

- **JSON Format**
    *   Machine-readable format. Used for CI/CD and integration with other programs.
    *   Differences detected by `diffx` are output as a JSON array.

- **YAML Format**
    *   Machine-readable format. Used for CI/CD and integration with other programs, similar to JSON.
    *   Differences detected by `diffx` are output as a YAML array.

- **diff-compatible Format (Unified Format)**
    *   Provided with the `--output unified` option.
    *   Intended for integration with `git` and existing merge tools.
    *   **Note**: This format expresses the "semantic differences" detected internally by `diffx` as line-based differences of the formatted text of the original files. Therefore, changes that `diffx` determines are not semantic differences (e.g., changes in key order, whitespace changes) may still be displayed with `+`/`-` if there are changes in the text representation. This is purely for compatibility and **differs from `diffx`'s semantic differences**.

## Architecture

### Proposed Structure
```
diffx/
├── diffx-core/      # Diff extraction library (Crate)
├── diffx-cli/       # CLI wrapper
├── tests/           # All test-related files
│   ├── fixtures/    # Test input data
│   ├── integration/ # CLI integration tests
│   ├── unit/        # Core library unit tests
│   └── output/      # Test intermediate files
├── docs/            # Documentation and specifications
└── ...
```

### Technology Stack
- **Rust** (Fast, safe, cross-platform)
- `serde_json`, `serde_yml`, `toml`, `configparser`, `quick-xml`, `csv` parsers
- `clap` (CLI argument parsing)
- `colored` (CLI output coloring)
- `similar` (Unified Format output)

## Future Prospects
- **Diff Report Diffs (Meta-chaining)**: Save `diffx` output in YAML/TOML format and compare it again as `diffx` input to detect "diffs of diff reports." This enables advanced operations such as change history management, auditing, and deployment tracking for configuration changes.
- **Interactive TUI (`diffx-tui`)**: A sample and high-performance viewer to demonstrate the power of `diffx`. It displays data side-by-side with a linked diff list, providing the experience of "understanding essential differences without being confused by formatting variations."
- Diff checking with GitHub Actions
- Integration with AI agents (diff summarization/explanation)

## Overall Distribution

### 1. Rust Crate (diffx-core)
- Provides diff extraction logic as a library
- Can be embedded in other Rust applications and CLI tools
- Fast, type-safe, extensible

### 2. CLI Tool (diffx)
- Command-line tool directly usable by users
- Easy to call from AI and CI/CD tools
- Installable with `cargo install diffx`

### 3. Wrappers for Other Languages (npm/pip)
- **npm package (diffx-bin)**
  - Wrapper to call diffx CLI from Node.js environment
  - Executes CLI using `child_process.spawn()`
- **pip package (diffx-bin)**
  - Wrapper to call diffx CLI from Python environment
  - Executes CLI using `subprocess.run()`

### Why this structure is effective?
- **AI Affinity**: CLI allows AI to operate regardless of language
- **Developer Reusability**: Rust Crate makes it easy to integrate into other tools
- **Language Ecosystem Expansion**: npm/pip reaches JS/Python users
- **Maintainability**: CLI is primary, wrappers can be kept thin
