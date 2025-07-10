# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation overhaul with hierarchical structure
- Performance benchmarks and optimization guide
- Integration guide with CI/CD platforms and development tools
- Tool comparison matrix with detailed feature analysis
- Real-world examples across 8 industry categories

### Changed
- Documentation structure reorganized into user-guide, reference, guides, and project sections
- README simplified and made more accessible with softer language
- Badge links updated to point to GitHub documentation instead of docs.rs

### Fixed
- Corrected unified format explanation to remove contradictory statements
- Updated format support information to reflect current implementation status

### Removed
- **Configuration file support** - `~/.config/diffx/config.toml` configuration file loading
- **Environment variable support** - `DIFFX_*` environment variable overrides
- Removed for consistency with sibling apps (diffai, lawkit) and adherence to UNIX philosophy

## [0.2.0] - 2025-01-15

### Added
- **XML format support** - Full support for XML file parsing and comparison
- **INI format support** - Complete INI/config file format support  
- **CSV format support** - CSV file comparison with array element tracking
- **Configuration file support** - TOML configuration files at `~/.config/diffx/config.toml`
- **Environment variable support** - All CLI options can be set via environment variables
- **Directory comparison** - Recursive directory comparison with `--recursive` flag
- **Path filtering** - `--path` option to focus comparisons on specific data sections
- **Floating-point tolerance** - `--epsilon` option for numeric comparison with tolerance
- **Array element tracking** - `--array-id-key` for intelligent array element identification
- **Regular expression filtering** - `--ignore-keys-regex` to exclude keys from comparison
- **Multiple output formats** - JSON, YAML, and unified diff output options
- **Standard input support** - Compare files with stdin using `-` as filename
- **Format auto-detection** - Automatic format detection from file extensions
- **Type change detection** - Explicit reporting of data type changes (e.g., string to number)
- **Comprehensive test suite** - 23 test cases covering all features and edge cases
- **Performance benchmarks** - Criterion-based benchmarks for performance monitoring
- **Cross-platform support** - Linux, macOS, and Windows compatibility
- **CI/CD integration** - GitHub Actions workflows for testing and releases

### Changed
- **Major version bump** to reflect significant feature additions
- **Rust edition updated** to 2021 for latest language features
- **CLI interface redesigned** with consistent argument naming and better help text
- **Error handling improved** with more descriptive error messages and exit codes
- **Memory usage optimized** for large file processing
- **Parser architecture refactored** for extensibility and performance

### Fixed
- **INI parsing edge cases** - Improved handling of malformed INI files
- **Array comparison logic** - Fixed issues with empty arrays and nested structures
- **Unicode handling** - Proper support for Unicode in all text formats
- **File extension detection** - More robust file type inference
- **Memory leaks** - Eliminated memory leaks in parser error paths

### Removed
- **Legacy command-line options** - Removed deprecated flags from 0.1.x series

## [0.1.2] - 2024-12-20

### Added
- **Python wrapper package** - `diffx-python` pip package for Python integration
- **Node.js wrapper package** - `diffx-npm` npm package for JavaScript integration
- **Japanese documentation** - Complete Japanese translations of README and core docs
- **Real benchmark data** - Actual performance measurements on AMD Ryzen 7 5800X

### Changed
- **Documentation improvements** - Better examples and clearer explanations
- **Performance optimizations** - 15-20% speed improvement for large files
- **Error message clarity** - More helpful error descriptions

### Fixed
- **TOML parsing edge cases** - Better handling of complex TOML structures
- **CLI argument validation** - Improved validation of command-line arguments
- **File path handling** - Fixed issues with relative path resolution

## [0.1.1] - 2024-12-10

### Added
- **TOML format support** - Complete TOML file parsing and comparison
- **Enhanced CLI output** - Improved visual formatting with colors and symbols
- **Extended test coverage** - Additional test cases for TOML and edge cases

### Changed
- **Output formatting** - More readable CLI output with better indentation
- **Performance improvements** - Optimized JSON and YAML parsing

### Fixed
- **YAML parsing issues** - Fixed handling of complex YAML structures
- **CLI exit codes** - Proper exit code handling for different scenarios
- **Memory usage** - Reduced memory footprint for large JSON files

## [0.1.0] - 2024-12-01

### Added
- **Initial release** of diffx semantic diff tool
- **JSON format support** - Complete JSON file parsing and comparison
- **YAML format support** - Full YAML file parsing and comparison
- **Core diff engine** - Semantic difference detection for structured data
- **CLI interface** - Command-line tool with basic options
- **Basic output formats** - CLI and JSON output modes
- **Rust crate** - `diffx-core` library for Rust applications
- **Cross-platform builds** - Support for Linux, macOS, and Windows
- **GitHub releases** - Automated binary releases
- **Basic documentation** - README with usage examples

### Technical Details
- Built with Rust 2021 edition
- Uses `serde_json` for JSON parsing
- Uses `serde_yaml` for YAML parsing
- Uses `clap` for CLI argument parsing
- Uses `anyhow` for error handling
- Comprehensive test suite with `cargo test`

## Version History Summary

| Version | Release Date | Key Features |
|---------|-------------|--------------|
| **0.2.0** | 2025-01-15 | XML/INI/CSV support, advanced filtering, configuration files |
| **0.1.2** | 2024-12-20 | Python/Node.js wrappers, Japanese docs, real benchmarks |
| **0.1.1** | 2024-12-10 | TOML support, enhanced CLI output, improved performance |
| **0.1.0** | 2024-12-01 | Initial release with JSON/YAML support |

## Migration Guide

### From 0.1.x to 0.2.0

**Breaking Changes:**
- None - 0.2.0 is fully backward compatible with 0.1.x

**New Features Available:**
```bash
# New format support
diffx config.xml config.new.xml
diffx database.ini database.new.ini
diffx data.csv data.new.csv

# New filtering options
diffx config.json config.new.json --path "database"
diffx config.json config.new.json --ignore-keys-regex "^timestamp$"
diffx users.json users.new.json --array-id-key "id"

# New output formats
diffx config.json config.new.json --output yaml
diffx config.json config.new.json --output unified

# Configuration file support
echo 'ignore_keys_regex = "^(timestamp|_.*)"' > ~/.config/diffx/config.toml
```

**Recommended Upgrades:**
1. **Add configuration files** for frequently used options
2. **Use array ID keys** for better array comparison
3. **Apply regex filtering** to ignore irrelevant fields
4. **Leverage path filtering** for large configuration files

### From 0.1.0 to 0.1.1

**New Features:**
```bash
# TOML support added
diffx config.toml config.new.toml

# Enhanced output formatting
diffx config.json config.new.json  # Now with colors and better formatting
```

## Contributors

- **kako-jun** - Project creator and primary maintainer
- **Community contributors** - See GitHub contributors page for complete list

## Acknowledgments

Special thanks to:
- **Rust community** for excellent parsing libraries
- **serde ecosystem** for serialization support
- **clap** for CLI argument parsing
- **criterion** for benchmarking framework
- **GitHub Actions** for CI/CD infrastructure

## Support and Feedback

- **Issues**: [GitHub Issues](https://github.com/kako-jun/diffx/issues)
- **Discussions**: [GitHub Discussions](https://github.com/kako-jun/diffx/discussions)
- **Documentation**: [GitHub Pages](https://kako-jun.github.io/diffx/)

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.