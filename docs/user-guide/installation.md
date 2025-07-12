# Installation Guide

This guide covers various ways to install `diffx` on different platforms.

## Quick Install

### Install from Crates.io (Recommended)

The easiest way to install `diffx` is using Cargo:

```bash
cargo install diffx
```

This will download, compile, and install the latest version of `diffx` from [crates.io](https://crates.io/crates/diffx).

## Platform-Specific Installation

### Linux

#### Ubuntu/Debian

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install diffx
cargo install diffx
```

#### Arch Linux

```bash
# Install Rust
sudo pacman -S rust

# Install diffx
cargo install diffx
```

#### Alpine Linux

```bash
# Install Rust
apk add rust cargo

# Install diffx
cargo install diffx
```

### macOS

#### Using Homebrew (Future)

```bash
# Coming soon
brew install diffx
```

#### Using Cargo

```bash
# Install Rust via Homebrew
brew install rust

# Or install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install diffx
cargo install diffx
```

### Windows

#### Using Cargo

```powershell
# Install Rust from https://rustup.rs/
# Then install diffx
cargo install diffx
```

#### Using Scoop (Future)

```powershell
# Coming soon
scoop install diffx
```

#### Using Chocolatey (Future)

```powershell
# Coming soon
choco install diffx
```

## Building from Source

### Prerequisites

- Rust 1.70.0 or later
- Git

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/kako-jun/diffx.git
cd diffx

# Build and install
cargo install --path diffx-cli

# Or just build for development
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --package diffx-core
cargo test --package diffx-cli

# Run integration tests
cargo test --test integration
```

## Docker Installation

### Using Pre-built Images (Future)

```bash
# Coming soon
docker pull ghcr.io/kako-jun/diffx:latest
docker run --rm -v $(pwd):/workspace ghcr.io/kako-jun/diffx file1.json file2.json
```

### Building Your Own Image

```dockerfile
FROM rust:1.70-alpine AS builder

WORKDIR /app
COPY . .
RUN cargo install --path diffx-cli

FROM alpine:latest
RUN apk add --no-cache libc6-compat
COPY --from=builder /usr/local/cargo/bin/diffx /usr/local/bin/diffx
ENTRYPOINT ["/usr/local/bin/diffx"]
```

```bash
# Build the image
docker build -t diffx .

# Use the image
docker run --rm -v $(pwd):/workspace diffx /workspace/file1.json /workspace/file2.json
```

## Package Managers

### Node.js Ecosystem

```bash
# Install Node.js wrapper
npm install diffx-js

# Use in your Node.js projects
const diffx = require('diffx-js');
const result = diffx.diff('file1.json', 'file2.json');
```

### Python Ecosystem

```bash
# 🆕 Self-contained wheel with embedded binary (v0.5.1+)
pip install diffx-python

# Use in your Python projects
import diffx
result = diffx.diff('file1.json', 'file2.json')
print(result)

# Verify installation
import diffx
print("diffx available:", diffx.is_diffx_available())
print("Version:", diffx.__version__)
```

**Key Benefits of Python Package (v0.5.1+):**
- **🚀 Zero setup**: No external downloads or binary management
- **📦 Self-contained**: Everything needed is in the wheel
- **⚡ Fast installation**: No network dependencies after `pip install`
- **🔒 Secure**: No runtime downloads from external sources
- **🌐 Offline-ready**: Works in air-gapped environments

The Python package uses [maturin](https://github.com/PyO3/maturin) to embed the native `diffx` binary directly in the Python wheel, similar to tools like `ruff`.

## Verification

After installation, verify that `diffx` is working correctly:

```bash
# Check version
diffx --version

# Run a simple test
echo '{"a": 1}' > test1.json
echo '{"a": 2}' > test2.json
diffx test1.json test2.json

# Expected output:
# ~ a: 1 -> 2

# Clean up
rm test1.json test2.json
```

## Updating

### Update via Cargo

```bash
cargo install diffx --force
```

### Check for Updates

```bash
# Check current version
diffx --version

# Check latest version on crates.io
cargo search diffx
```

## Troubleshooting

### Common Issues

#### Rust Not Found

If you get an error that `cargo` or `rust` is not found:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Permission Denied

If you get permission errors during installation:

```bash
# On Linux/macOS, make sure ~/.cargo/bin is in your PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Compilation Errors

If compilation fails:

```bash
# Update Rust to the latest version
rustup update

# Clear cargo cache and try again
cargo clean
cargo install diffx
```

#### Out of Memory During Compilation

For systems with limited memory:

```bash
# Use fewer parallel jobs
cargo install diffx --jobs 1
```

### Getting Help

If you encounter issues:

1. Check the [FAQ](faq.md)
2. Search existing [GitHub issues](https://github.com/kako-jun/diffx/issues)
3. Create a new issue with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce

## Uninstallation

To remove `diffx`:

```bash
# Uninstall diffx
cargo uninstall diffx

# Remove any configuration files (optional)
rm -rf ~/.config/diffx
```

## System Requirements

### Minimum Requirements

- **RAM**: 256 MB available memory
- **Disk**: 50 MB free space for binary
- **CPU**: Any modern x86_64 or ARM64 processor

### Recommended Requirements

- **RAM**: 1 GB or more for large files
- **Disk**: 500 MB for source build
- **CPU**: Multi-core processor for parallel processing

### Supported Platforms

- **Linux**: x86_64, ARM64
- **macOS**: x86_64, ARM64 (Apple Silicon)
- **Windows**: x86_64
- **FreeBSD**: x86_64 (community supported)

## Performance Considerations

### Large Files

For very large files (>100MB), consider:

```bash
# Use streaming mode (if available)
diffx --stream large1.json large2.json

# Increase memory limit
diffx --help
diffx large1.json large2.json
```

### Multiple Files

For batch processing:

```bash
# Use parallel processing
find . -name "*.json" -print0 | xargs -0 -P $(nproc) -I {} diffx {} {}.backup
```