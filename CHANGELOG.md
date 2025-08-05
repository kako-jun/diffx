# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.21] - 2025-08-05

### Added
- **Automated manylinux wheel building**: Docker-based Python wheel compilation for maximum Linux compatibility
- **Complete JavaScript/npm package**: Full NAPI bindings with platform-specific native binaries
- **Unified API implementation**: Consistent interface across Rust, Python, and JavaScript packages
- **Automatic directory detection**: Removes need for manual --recursive flag specification
- **Comprehensive stdin support**: Enhanced pipeline-friendly operations for all input methods
- **Multilingual documentation**: Complete Japanese and Chinese translations added
- **Enhanced CLI options**: --no-color support and improved error messaging

### Changed
- **Release infrastructure overhaul**: Atomic publishing across all package registries with rollback capability
- **Testing infrastructure**: 1,000+ automated tests with multi-platform CI/CD validation
- **Package ecosystem**: Standardized naming and API consistency across all language bindings
- **Performance optimizations**: Core algorithm improvements and memory usage reduction

### Fixed
- **manylinux compatibility**: Resolved GLIBC version conflicts in Python wheel distribution
- **GitHub Actions reliability**: Enhanced CI/CD pipeline with proper artifact management
- **Cross-platform binary distribution**: Fixed platform-specific binary paths and dependencies
- **Test suite stability**: Comprehensive test restructuring with 100% success rate
- **Documentation consistency**: Unified examples and API references across all languages

### Breaking Changes
- **Removed --unified format option**: Simplified output format for better consistency
- **Removed --recursive flag**: Directory detection is now automatic
- **Python import changes**: Module now uses `diffx_python` namespace for better isolation

### Infrastructure
- **Enhanced release automation**: Complete CI/CD pipeline with maturin-action integration
- **Cross-registry publishing**: Simultaneous deployment to crates.io, PyPI, and npmjs.com
- **Comprehensive quality gates**: Multi-stage testing before any public release
- **Package installation verification**: Real-world testing from all package registries

## [0.5.6] - 2025-07-16

### Added
- **Linux ARM64 platform support**: Extended platform matrix to include ARM64 Linux servers
- **Unified platform support**: All packages (Rust/npm/PyPI) now support the same 5 platforms consistently

### Fixed
- **npm binary download path**: Corrected script path reference in GitHub Actions workflow
- **Project detection in shared scripts**: Fixed common.sh usage across all release scripts
- **Platform inconsistency**: Unified platform support matrix across all packaging systems

### Changed
- **Shared CI/CD system completion**: Full migration to kako-jun/.github repository with workflow_call
- **Benchmark workflow simplification**: Removed complex performance regression detection, kept simple weekly runs
- **CI/CD optimization**: Eliminated duplicate tests between Act1 and Act2, improved reliability

### Infrastructure
- **3-project unification**: diffx, lawkit, and diffai now share the same CI/CD infrastructure
- **Cross-platform reliability**: All 5 platforms tested and verified across all package managers
- **Documentation updates**: Updated all project documentation to reflect new shared system

## [0.5.4] - 2025-07-15

### Fixed
- **GitHub Actions reliability**: Fixed git status checks failing due to build artifacts in CI environment
- **Act1 test script robustness**: Improved binary test exit code handling and Cargo.lock automatic processing
- **Release script paths**: Corrected version consistency check script references
- **npm package warnings**: Corrected package.json configuration warnings
- **Workspace dependency resolution**: Enhanced CLI dry run handling for workspace dependencies

### Changed
- **Release infrastructure**: Enhanced release and testing scripts with universal project compatibility
- **CI/CD integration**: Integrated npm/pip tests into daily CI with unified test execution
- **Script organization**: Reorganized release scripts with numbered sequence for clarity
- **Error handling**: Improved cleanup and rollback functionality for failed releases
- **Build artifact management**: Enhanced .gitignore to exclude all target directories from version control

### Removed
- **Obsolete installation methods**: Removed Scoop and Chocolatey installation sections (no support planned)
- **Legacy scripts**: Cleaned up obsolete npm postinstall and download scripts
- **Debug output**: Removed temporary debugging logs from release scripts

## [0.5.1] - 2025-07-12

### Added
- **🆕 Self-contained Python wheel distribution**: Python package now uses maturin to embed diffx binary directly in wheel
- Comprehensive documentation overhaul with hierarchical structure
- Performance benchmarks and optimization guide
- Integration guide with CI/CD platforms and development tools
- Tool comparison matrix with detailed feature analysis
- Real-world examples across 8 industry categories

### Changed
- **BREAKING CHANGE**: Python package distribution method changed from external binary download to embedded binary in wheel
- Python package now follows ruff-style distribution with maturin build system
- GitHub Actions updated to build platform-specific Python wheels (Linux/Windows/macOS)
- Documentation structure reorganized into user-guide, reference, guides, and project sections
- README simplified and made more accessible with softer language
- Badge links updated to point to GitHub documentation instead of docs.rs
- Python installation now requires no external downloads or network dependencies

### Fixed
- Python package binary path detection for maturin wheel structure
- Exit code handling in Python wrapper (exit code 1 = differences found, not error)
- Corrected unified format explanation to remove contradictory statements
- Updated format support information to reflect current implementation status
- Exit code implementation following diff conventions (0=no diff, 1=diff found, 2=error)
- Unified output path filtering bug fixed

### Removed
- **Configuration file support** - `~/.config/diffx/config.toml` configuration file loading
- **Environment variable support** - `DIFFX_*` environment variable overrides
- Python package external binary download mechanism (replaced with embedded binary)
- Removed for consistency with sibling apps (diffai, lawkit) and adherence to UNIX philosophy

## [0.3.2] - 2025-01-15

### Added
- **Package releases** - npm (diffx-js) and pip (diffx-python) wrapper packages
- **Comprehensive test automation** - 91 integration tests with 100% pass rate
- **Documentation verification** - 1:1 mapping between documented examples and test cases
- **Package testing infrastructure** - Automated verification for published packages

### Fixed
- **Exit codes** - Proper exit codes (0=no diff, 1=diff found, 2=error)
- **Python package options** - Added missing optimization options (now automatic)
- **Test coverage** - All documented command examples now have corresponding test cases

## [0.2.0] - 2025-01-15

### Added
- **XML format support** - Full support for XML file parsing and comparison
- **INI format support** - Complete INI/config file format support  
- **CSV format support** - CSV file comparison with array element tracking
- **Directory comparison** - Recursive directory comparison with `--recursive` flag
- **Path filtering** - `--path` option to focus comparisons on specific data sections
- **Floating-point tolerance** - `--epsilon` option for numeric comparison with tolerance
- **Array element tracking** - `--array-id-key` for intelligent array element identification
- **Regular expression filtering** - `--ignore-keys-regex` to exclude keys from comparison
- **Multiple output formats** - JSON, YAML, and diffx output options
- **Standard input support** - Compare files with stdin using `-` as filename
- **Format auto-detection** - Automatic format detection from file extensions
- **Type change detection** - Explicit reporting of data type changes (e.g., string to number)
- **Comprehensive test suite** - 73+ test cases covering all features and edge cases
- **Performance benchmarks** - Criterion-based benchmarks for performance monitoring
- **Cross-platform support** - Linux, macOS, and Windows compatibility

### Changed
- **Output format** - Improved CLI output with color-coded differences
- **Error handling** - Enhanced error messages with proper context
- **Performance** - Optimized diff algorithms for large files
- **Documentation** - Complete documentation overhaul with user guides

### Fixed
- **Unicode handling** - Proper support for non-ASCII characters
- **Array comparison** - Improved semantic array element matching
- **Memory usage** - Optimized memory consumption for large files
- **Edge cases** - Fixed various edge cases in format parsing

## [0.1.0] - 2024-12-XX

### Added
- **Core functionality** - Initial implementation of structured diff extraction
- **Basic formats** - JSON, YAML, and TOML support
- **CLI interface** - Basic command-line interface
- **Output formats** - CLI and JSON output formats

<!-- generated by git-cliff -->