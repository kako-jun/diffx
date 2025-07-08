#!/bin/bash

# diffx unified publishing script for all packages
# Publishes Rust crates, npm, and PyPI packages in sequence

set -e  # Exit on any error

echo "Starting unified diffx publishing workflow..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper function for colored output
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

success() {
    echo -e "${GREEN}OK: $1${NC}"
}

warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

error() {
    echo -e "${RED}ERROR: $1${NC}"
}

# Check prerequisites
log "Checking prerequisites..."

# Check if all package directories exist
REQUIRED_DIRS=("diffx-npm" "diffx-python")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        error "Directory $dir not found. Please run from project root."
        exit 1
    fi
done

# Check if we have the necessary tools
REQUIRED_TOOLS=("cargo" "npm" "python" "twine")
for tool in "${REQUIRED_TOOLS[@]}"; do
    if ! command -v "$tool" &> /dev/null; then
        error "Required tool '$tool' not found. Please install it."
        exit 1
    fi
done

success "All prerequisites satisfied"

# Get version information
log "Gathering version information..."

CORE_VERSION=$(grep '^version = ' diffx-core/Cargo.toml | head -1 | cut -d'"' -f2)
CLI_VERSION=$(grep '^version = ' diffx-cli/Cargo.toml | head -1 | cut -d'"' -f2)
NPM_VERSION=$(node -p "require('./diffx-npm/package.json').version")
PYTHON_VERSION=$(python -c "import tomli; print(tomli.load(open('diffx-python/pyproject.toml', 'rb'))['project']['version'])" 2>/dev/null || echo "unknown")

echo ""
echo "Current Versions:"
echo "   diffx-core:   $CORE_VERSION"
echo "   diffx-cli:    $CLI_VERSION"
echo "   diffx-js:     $NPM_VERSION"
echo "   diffx-python: $PYTHON_VERSION"
echo ""

# Version consistency check
if [ "$CORE_VERSION" != "$CLI_VERSION" ]; then
    warning "Core and CLI versions don't match ($CORE_VERSION vs $CLI_VERSION)"
fi

# Confirm publishing plan
log "Publishing plan:"
echo "   1. 🦀 Rust crates (diffx-core, diffx)"
echo "   2. npm package (diffx-js)"
echo "   3. 🐍 PyPI package (diffx-python)"
echo ""

read -p "Continue with unified publishing? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    error "Publishing cancelled by user"
    exit 1
fi

# Step 1: Publish Rust crates
log "Step 1: Publishing Rust crates..."
echo ""

# Check cargo authentication
if ! cargo owner --list diffx-core >/dev/null 2>&1; then
    warning "Not logged in to crates.io. Please run 'cargo login' first."
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        error "Cargo authentication required"
        exit 1
    fi
fi

# Build and test first
log "Building and testing Rust packages..."
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Publish core first, then CLI
log "Publishing diffx-core..."
cargo publish -p diffx-core

log "Waiting 30 seconds for crates.io to update..."
sleep 30

log "Publishing diffx..."
cargo publish -p diffx

success "Rust crates published successfully"

# Step 2: Publish npm package
log "Step 2: Publishing npm package..."
echo ""

./scripts/publish-npm.sh

success "npm package published successfully"

# Step 3: Publish PyPI package
log "Step 3: Publishing PyPI package..."
echo ""

./scripts/publish-pypi.sh

success "PyPI package published successfully"

# Final summary
echo ""
echo "Unified publishing complete!"
echo ""
echo "Published packages:"
echo "   🦀 diffx-core@$CORE_VERSION    → https://crates.io/crates/diffx-core"
echo "   🦀 diffx@$CLI_VERSION         → https://crates.io/crates/diffx"
echo "   diffx-js@$NPM_VERSION      → https://www.npmjs.com/package/diffx-js"
echo "   🐍 diffx-python@$PYTHON_VERSION → https://pypi.org/project/diffx-python/"
echo ""

log "Installation commands:"
echo "   cargo install diffx"
echo "   npm install diffx-js"
echo "   pip install diffx-python"
echo ""

success "All packages are now available!"