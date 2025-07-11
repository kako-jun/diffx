# Contributing to diffx

Thank you for your interest in contributing to `diffx`! This document provides comprehensive guidelines for contributing to the project.

## 🎯 Project Vision

`diffx` aims to be the definitive semantic diff tool for structured data. We focus on:

- **Semantic accuracy**: Understanding data meaning, not just text formatting
- **Performance**: Fast processing of large files
- **Usability**: Clean CLI interface for humans and automation
- **Reliability**: Comprehensive testing and type safety

## 🚀 Quick Start

### Development Environment Setup

1. **Prerequisites**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Clone and Build**
   ```bash
   git clone https://github.com/your-username/diffx.git
   cd diffx
   cargo build --workspace
   ```

3. **Run Tests**
   ```bash
   # Run all tests (58 tests total)
   cargo test --workspace
   
   # Run specific test categories
   cargo test --package diffx-core     # Core library tests (29 tests)
   cargo test integration              # CLI integration tests (29 tests)
   ```

4. **Development Tools**
   ```bash
   # Format code (required for CI)
   cargo fmt --all
   
   # Run linter (required for CI)
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   
   # Run benchmarks
   cargo bench --package diffx-core
   ```

## 🔧 Development Workflow (GitHub Flow)

We use GitHub Flow for all development. Main branch is protected and requires PR reviews.

### 1. Fork and Create Feature Branch
```bash
# Always branch from latest main
git checkout main
git pull origin main
git checkout -b feature/your-feature-name
# Branch naming conventions:
# - feature/add-binary-support
# - fix/yaml-parsing-error
# - docs/update-chinese-translation
# - refactor/optimize-diff-algorithm
# - test/add-xml-benchmarks
```

### 2. Make Changes
- Follow Rust conventions and project patterns
- Add comprehensive tests for new features
- Update relevant documentation
- Keep commits atomic and focused

### 3. Pre-Push Validation
```bash
# Run our CI validation script (required)
./scripts/ci-local.sh

# This script runs exactly what GitHub CI runs:
# - cargo fmt --all --check
# - cargo clippy --workspace --all-targets --all-features -- -D warnings  
# - cargo build --workspace
# - cargo test --workspace
# - diffx CLI smoke tests
```

### 4. Commit and Push
```bash
# Stage changes
git add .

# Commit with conventional format
git commit -m "feat(core): add binary format support"

# Push to your fork
git push origin feature/your-feature-name
```

### 5. Create Pull Request
```bash
# Using GitHub CLI (recommended)
gh pr create --title "feat: add binary format support" \
  --body "Description of changes"

# Or via GitHub web interface
```

**PR Guidelines:**
- Title should follow conventional commit format
- Reference related issues with "Fixes #123" or "Closes #123"
- Fill out the PR template completely
- Ensure all CI checks pass before requesting review
- Link to relevant documentation updates

### 6. Code Review Process
- At least 1 approval required before merge
- Address all feedback comments
- Keep PR updated with main branch
- Squash commits if requested

### 7. After Merge
- Delete your feature branch
- Pull latest main to stay updated
- Celebrate your contribution! 🎉

## 📁 Project Structure

```
diffx/
├── diffx-core/           # Core diff logic library
│   ├── src/lib.rs       # Main diff algorithms
│   ├── benches/         # Performance benchmarks
│   └── Cargo.toml
├── diffx-cli/           # Command-line interface
│   ├── src/main.rs      # CLI implementation
│   └── Cargo.toml
├── tests/               # Comprehensive test suite
│   ├── fixtures/        # Test data files
│   ├── integration/     # CLI integration tests (29 tests)
│   └── unit/           # Unit tests (29 tests)
├── docs/               # Documentation
│   ├── user-guide/     # User documentation
│   ├── reference/      # Technical reference
│   ├── guides/         # Integration guides
│   └── project/        # Project information
└── .github/            # CI/CD workflows
```

## 🧪 Testing Guidelines

### Test Categories

1. **Unit Tests** (`tests/unit/core_tests.rs`): Core diff logic
2. **Integration Tests** (`tests/integration/cli_tests.rs`): CLI behavior
3. **Benchmarks** (`diffx-core/benches/`): Performance validation

### Running Tests

```bash
# All tests (58 total)
cargo test --workspace

# Specific categories
cargo test core_tests::     # Unit tests
cargo test cli_tests::      # Integration tests

# With output
cargo test -- --nocapture

# Specific test
cargo test test_diff_json_objects
```

### Adding Tests

**Unit Test Example:**
```rust
#[test]
fn test_your_feature() {
    let v1 = json!({"key": "value1"});
    let v2 = json!({"key": "value2"});
    let result = diff(&v1, &v2, None, None, None);
    assert_eq!(result.len(), 1);
}
```

**Integration Test Example:**
```rust
#[test]
fn test_cli_feature() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("file1.json").arg("file2.json").arg("--flag");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected"));
    Ok(())
}
```

## 📝 Coding Standards

### Required (CI Enforced)
- **Formatting**: `cargo fmt --all` (zero warnings)
- **Linting**: `cargo clippy` (zero warnings)
- **Tests**: All tests must pass
- **Build**: Clean workspace build

### Conventions
- **Error Handling**: Use `anyhow::Result` for error propagation
- **Type Safety**: Leverage Rust's type system
- **Documentation**: `///` comments for public APIs
- **Naming**: Follow Rust naming conventions

### Commit Format
Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(core): add XML array comparison support
fix(cli): resolve config file parsing error
docs(readme): update installation instructions
test(integration): add CSV format test cases
```

## 🎨 Architecture Guidelines

### Core Principles

1. **Separation of Concerns**
   - `diffx-core`: Pure diff logic, no I/O
   - `diffx-cli`: CLI interface, file handling, output formatting

2. **Performance First**
   - Benchmark critical paths with `cargo bench`
   - Efficient data structures for large files
   - Memory-conscious algorithms

3. **Type Safety**
   - Strong types for all public APIs
   - Avoid `unwrap()` in production code
   - Comprehensive error handling

### Adding New Features

**New Data Format Support:**
1. Add enum variant to `Format`
2. Implement parser in `parse_value()`
3. Add CLI integration
4. Comprehensive test coverage

**New CLI Options:**
1. Add to `Args` struct with clap annotations
2. Integrate with main logic
3. Update help documentation
4. Add integration tests

## 🐛 Bug Reports

Include in your bug reports:

1. **Environment**
   - OS and version
   - Rust version (`rustc --version`)
   - diffx version (`cargo run -- --version`)

2. **Reproduction**
   - Minimal example
   - Input files (if safe to share)
   - Command line used
   - Expected vs actual output

3. **Context**
   - Error messages
   - Stack traces
   - Related issues

## 💡 Feature Requests

When requesting features:

1. **Use Case**: Describe the problem you're solving
2. **Proposal**: Specific implementation ideas
3. **Examples**: Concrete usage examples
4. **Alternatives**: Other approaches considered

## 📈 Performance Guidelines

When optimizing performance:

1. **Benchmark First**: Establish baseline with `cargo bench`
2. **Profile**: Identify actual bottlenecks
3. **Measure**: Verify improvements
4. **Document**: Explain optimization approach

## 🌍 Documentation

### Types
- **User Docs** (`docs/user-guide/`): How to use diffx
- **API Docs** (`docs/reference/`): Technical reference  
- **Integration** (`docs/guides/`): Real-world usage
- **Project** (`docs/project/`): Development info

### Guidelines
- Clear, concise writing
- Practical examples
- Cross-references to related sections
- Keep examples current with code

## 🚀 Release Process

Releases follow semantic versioning:

1. **CI/CD**: All checks must pass
2. **Testing**: Comprehensive test coverage
3. **Documentation**: Updated for new features
4. **Changelog**: Document breaking changes

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors are recognized in:
- Release notes for significant contributions
- GitHub contributors list
- `CHANGELOG.md` for major features

---

**Questions?** Open a GitHub Discussion or create an issue. We're here to help! 🎉
