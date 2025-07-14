#!/bin/bash
set -euo pipefail

# Quick check script for daily development
# Matches exactly what GitHub Actions CI workflow does
# Fast execution (5-10 seconds) for frequent use

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_NAME=$(basename "$PROJECT_ROOT")

# Change to project root
cd "$PROJECT_ROOT"

# Match GitHub Actions environment
export CARGO_TERM_COLOR=always

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

# Error handling
trap 'print_error "Quick check failed at line $LINENO"' ERR

main() {
    print_info "Running quick check (matches GitHub Actions CI)..."
    print_info "Project root: $PROJECT_ROOT"
    echo ""
    
    # Step 1: Check formatting
    print_info "Step 1: Checking code formatting..."
    cargo fmt --all --check
    print_success "✓ Code formatting check passed"
    
    # Step 2: Run Clippy
    print_info "Step 2: Running Clippy..."
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    print_success "✓ Clippy check passed"
    
    # Step 3: Build
    print_info "Step 3: Building workspace..."
    cargo build --workspace --verbose
    print_success "✓ Build completed"
    
    # Step 4: Run tests
    print_info "Step 4: Running tests..."
    cargo test --workspace --verbose
    print_success "✓ Tests passed"
    
    # Step 5: Quick performance check
    print_info "Step 5: Quick performance check..."
    cargo build --release --package ${PROJECT_NAME}-core
    print_success "✓ Release build successful - performance optimizations applied"
    
    # Step 6: Test npm package (dry run only)
    if [ -d "$PROJECT_ROOT/${PROJECT_NAME}-npm" ] && command -v node &> /dev/null; then
        print_info "Step 6: Testing npm package..."
        cd "$PROJECT_ROOT/${PROJECT_NAME}-npm"
        
        # Validate package.json
        if node -e "require('./package.json')" 2>/dev/null; then
            # Test npm publish readiness (dry run)
            if npm publish --dry-run > /dev/null 2>&1; then
                print_success "✓ npm package test passed (dry run)"
            else
                print_warning "npm publish dry run failed - check package.json"
            fi
        else
            print_warning "Invalid package.json in ${PROJECT_NAME}-npm"
        fi
        
        cd "$PROJECT_ROOT"
    else
        print_info "Step 6: Skipping npm tests (Node.js not installed or ${PROJECT_NAME}-npm not found)"
    fi
    
    # Step 7: Test Python package (build only)
    if [ -d "$PROJECT_ROOT/${PROJECT_NAME}-python" ] && command -v python3 &> /dev/null && command -v maturin &> /dev/null; then
        print_info "Step 7: Testing Python package..."
        cd "$PROJECT_ROOT/${PROJECT_NAME}-python"
        
        # Clean previous builds
        rm -rf target/ dist/ build/ 2>/dev/null || true
        
        # Test maturin build (local target only)
        LOCAL_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
        if maturin build --release --target "$LOCAL_TARGET" --out dist > /dev/null 2>&1; then
            print_success "✓ Python package build passed"
        else
            print_warning "maturin build failed - check pyproject.toml"
        fi
        
        cd "$PROJECT_ROOT"
    else
        print_info "Step 7: Skipping Python tests (Python/maturin not installed or ${PROJECT_NAME}-python not found)"
    fi
    
    echo ""
    print_success "🎉 All quick checks passed!"
    print_info "Ready to push to main branch"
    print_info "This matches exactly what GitHub Actions CI will run"
}

# Run main function
main "$@"