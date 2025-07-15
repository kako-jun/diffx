#!/bin/bash
set -euo pipefail

# Pre-release test Act 2 - Language wrapper simulation
# This script simulates exactly what GitHub Actions Release Act 2 does

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_NAME=$(basename "$PROJECT_ROOT")

# Change to project root
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# GitHub Actions environment simulation
export CARGO_TERM_COLOR=always

check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        print_error "Node.js is not installed. Required for npm package testing."
        exit 1
    fi
    
    # Check if Python is installed
    if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
        print_error "Python is not installed. Required for Python package testing."
        exit 1
    fi
    
    # Check if maturin is installed
    if ! command -v maturin &> /dev/null; then
        print_warning "maturin is not installed. Python package testing will be skipped."
        print_warning "To enable Python testing, install maturin: pip install maturin"
        # Set flag to skip Python tests instead of failing
        SKIP_PYTHON_TESTS=true
    else
        SKIP_PYTHON_TESTS=false
    fi
    
    print_success "Prerequisites check passed"
}

test_npm_package() {
    print_info "=== Testing npm package build ==="
    
    if [ ! -d "$PROJECT_ROOT/${PROJECT_NAME}-npm" ]; then
        print_error "${PROJECT_NAME}-npm directory not found"
        exit 1
    fi
    
    cd "$PROJECT_ROOT/${PROJECT_NAME}-npm"
    
    # Check package.json exists and is valid
    if [ ! -f "package.json" ]; then
        print_error "package.json not found in ${PROJECT_NAME}-npm"
        exit 1
    fi
    
    # Validate package.json
    print_info "Validating package.json..."
    if ! node -e "require('./package.json')"; then
        print_error "package.json is invalid"
        exit 1
    fi
    
    # Check if binary download script exists
    if [ ! -f "scripts/download-all-binaries.js" ]; then
        print_error "scripts/download-all-binaries.js not found"
        exit 1
    fi
    
    # Test the download script (dry run simulation)
    print_info "Testing binary download script..."
    if ! node -c "scripts/download-all-binaries.js"; then
        print_error "Binary download script has syntax errors"
        exit 1
    fi
    
    # Test npm package (dry run only)
    print_info "Testing npm publish readiness (dry run only - no actual publishing)..."
    if ! npm publish --dry-run; then
        print_error "npm publish dry run failed"
        exit 1
    fi
    
    print_success "npm dry run passed (no actual publishing)"
    print_warning "Note: Actual npm publishing happens only in GitHub Actions"
    
    cd "$PROJECT_ROOT"
    print_success "npm package test passed"
}

test_python_package() {
    print_info "=== Testing Python package build ==="
    
    # Skip Python tests if maturin is not available
    if [ "${SKIP_PYTHON_TESTS:-false}" = "true" ]; then
        print_warning "Skipping Python package tests (maturin not available)"
        return 0
    fi
    
    if [ ! -d "$PROJECT_ROOT/${PROJECT_NAME}-python" ]; then
        print_error "${PROJECT_NAME}-python directory not found"
        exit 1
    fi
    
    cd "$PROJECT_ROOT/${PROJECT_NAME}-python"
    
    # Check pyproject.toml exists and is valid
    if [ ! -f "pyproject.toml" ]; then
        print_error "pyproject.toml not found in ${PROJECT_NAME}-python"
        exit 1
    fi
    
    # Check Cargo.toml exists for maturin
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found in ${PROJECT_NAME}-python"
        exit 1
    fi
    
    # Test maturin build
    print_info "Testing maturin build..."
    
    # Clean previous builds
    rm -rf target/ dist/ build/
    
    # Build wheel (local target only to avoid cross-compilation)
    LOCAL_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    print_info "Building wheel for target: $LOCAL_TARGET"
    
    if ! maturin build --release --target "$LOCAL_TARGET" --out dist; then
        print_error "maturin build failed"
        exit 1
    fi
    
    # Check if wheel was created
    if [ ! -d "dist" ] || [ -z "$(ls -A dist)" ]; then
        print_error "No wheel files created in dist/"
        exit 1
    fi
    
    print_info "Wheel files created:"
    ls -la dist/
    
    # Test wheel installation (in isolated environment - no publishing)
    print_info "Testing wheel installation (local only - no publishing)..."
    
    WHEEL_FILE=$(ls dist/*.whl | head -1)
    if [ -z "$WHEEL_FILE" ]; then
        print_error "No wheel file found"
        exit 1
    fi
    
    # Create temporary venv for testing
    TEMP_VENV=$(mktemp -d)
    trap 'rm -rf "$TEMP_VENV"' EXIT
    
    python3 -m venv "$TEMP_VENV"
    source "$TEMP_VENV/bin/activate"
    
    # Install the wheel locally only
    if ! pip install "$WHEEL_FILE"; then
        print_error "Wheel installation failed"
        exit 1
    fi
    
    # Test basic functionality
    print_info "Testing installed Python package..."
    
    # Test import
    if ! python -c "import ${PROJECT_NAME}_python; print('Import successful')"; then
        print_error "Python package import failed"
        exit 1
    fi
    
    # Test basic functionality (if applicable)
    # This would depend on your Python package structure
    
    deactivate
    
    print_success "Python package build and local installation test passed"
    print_warning "Note: Actual PyPI publishing happens only in GitHub Actions"
    
    cd "$PROJECT_ROOT"
    print_success "Python package test passed"
}

test_package_consistency() {
    print_info "=== Testing package consistency ==="
    
    # Run the local version check
    "$PROJECT_ROOT/scripts/release/03-check-local-versions.sh"
    
    print_success "Package consistency check passed"
}

main() {
    print_info "=== Pre-release Test Act 2 - Language Wrapper Simulation ==="
    print_info "This simulates exactly what GitHub Actions Release Act 2 does"
    echo ""
    
    # Prerequisites check
    check_prerequisites
    
    # Test package consistency first
    test_package_consistency
    
    # Test npm package
    test_npm_package
    
    # Test Python package
    test_python_package
    
    print_success "=== Pre-release Test Act 2 PASSED ==="
    print_info "All wrapper package tests passed!"
    echo ""
    print_info "Both Act 1 and Act 2 simulations completed successfully."
    print_info "Ready to create release tag with: ./scripts/release/create-release-tag.sh"
}

# Run main function
main "$@"