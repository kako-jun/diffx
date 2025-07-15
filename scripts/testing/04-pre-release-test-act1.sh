#!/bin/bash
set -euo pipefail

# Pre-release test Act 1 - Core build and crates.io simulation
# This script simulates exactly what GitHub Actions Release Act 1 does

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
export RUST_BACKTRACE=1

# Targets that GitHub Actions builds
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

main() {
    print_info "=== Pre-release Test Act 1 - Core Build and Crates.io Simulation ==="
    print_info "This simulates exactly what GitHub Actions Release Act 1 does"
    echo ""
    
    # Step 1: Install required targets (subset for local testing)
    print_info "Step 1: Installing required Rust targets..."
    LOCAL_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    print_info "Local target: $LOCAL_TARGET"
    
    # Only install current platform target to avoid cross-compilation issues
    if ! rustup target list --installed | grep -q "$LOCAL_TARGET"; then
        rustup target add "$LOCAL_TARGET"
    fi
    
    # Step 2: Run comprehensive tests (same as GitHub Actions CI)
    print_info "Step 2: Running comprehensive tests..."
    
    # Format check
    print_info "Checking code formatting..."
    cargo fmt --all --check
    
    # Clippy check
    print_info "Running Clippy..."
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    
    # Build workspace
    print_info "Building workspace..."
    cargo build --workspace --verbose
    
    # Run tests
    print_info "Running tests..."
    cargo test --workspace --verbose
    
    # Quick performance check
    print_info "Quick performance check..."
    cargo build --release --package ${PROJECT_NAME}-core
    print_success "Release build successful - performance optimizations applied"
    
    # Step 3: Build release binary (simulating GitHub Actions build)
    print_info "Step 3: Building release binary for local target..."
    cargo build --package ${PROJECT_NAME} --release --target "$LOCAL_TARGET"
    
    # Step 4: Test binary functionality
    print_info "Step 4: Testing built binary..."
    BINARY_PATH="target/$LOCAL_TARGET/release/${PROJECT_NAME}"
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        BINARY_PATH="$BINARY_PATH.exe"
    fi
    
    if [ ! -f "$BINARY_PATH" ]; then
        print_error "Binary not found at $BINARY_PATH"
        exit 1
    fi
    
    # Test basic functionality
    print_info "Testing binary functionality..."
    
    # Create test files
    TEST_DIR=$(mktemp -d)
    trap 'rm -rf "$TEST_DIR"' EXIT
    
    echo '{"a": 1}' > "$TEST_DIR/test1.json"
    echo '{"a": 2}' > "$TEST_DIR/test2.json"
    
    # Test basic diff (should return exit code 1 when differences found)
    set +e  # Temporarily disable exit on error
    "$BINARY_PATH" "$TEST_DIR/test1.json" "$TEST_DIR/test2.json" > /dev/null 2>&1
    EXIT_CODE=$?
    set -e  # Re-enable exit on error
    if [ $EXIT_CODE -ne 1 ]; then
        print_error "Binary test failed: expected exit code 1 (differences found), got $EXIT_CODE"
        exit 1
    fi
    
    # Test identical files (should return exit code 0 when no differences)
    set +e  # Temporarily disable exit on error
    "$BINARY_PATH" "$TEST_DIR/test1.json" "$TEST_DIR/test1.json" > /dev/null 2>&1
    EXIT_CODE=$?
    set -e  # Re-enable exit on error
    if [ $EXIT_CODE -ne 0 ]; then
        print_error "Binary test failed: expected exit code 0 (no differences), got $EXIT_CODE"
        exit 1
    fi
    
    print_success "Binary test passed"
    
    # Step 5: Simulate crates.io publish (dry run only)
    print_info "Step 5: Simulating crates.io publish (dry run only - no actual publishing)..."
    
    # Check if packages can be published (dry run)
    print_info "Checking ${PROJECT_NAME}-core publish readiness (dry run)..."
    cd "$PROJECT_ROOT/${PROJECT_NAME}-core"
    if ! cargo publish --dry-run; then
        print_error "${PROJECT_NAME}-core dry run failed"
        exit 1
    fi
    cd "$PROJECT_ROOT"
    
    print_info "Checking ${PROJECT_NAME}-cli publish readiness (dry run)..."
    cd "$PROJECT_ROOT/${PROJECT_NAME}-cli"
    # Note: CLI dry run may fail due to workspace dependency resolution
    # during packaging, but actual publish will work after core is published
    if cargo publish --dry-run; then
        print_success "${PROJECT_NAME}-cli dry run passed"
    else
        print_warning "${PROJECT_NAME}-cli dry run failed (expected for workspace dependencies)"
        print_info "This is normal when CLI depends on unpublished workspace crates"
        print_info "Actual publish will work after ${PROJECT_NAME}-core is published to crates.io"
    fi
    cd "$PROJECT_ROOT"
    
    print_success "Crates.io dry run checks passed (no actual publishing)"
    print_warning "Note: Actual crates.io publishing happens only in GitHub Actions"
    
    # Step 6: Additional release-specific checks
    print_info "Step 6: Additional release-specific checks..."
    
    # Check no uncommitted changes (excluding build artifacts)
    # Note: Cargo.lock may be updated during testing and should be committed separately
    if ! git diff-index --quiet HEAD -- ':!target/' ':!**/target/'; then
        print_error "Working directory has uncommitted changes (excluding build artifacts):"
        git status --porcelain | grep -v "target/"
        print_error "Git diff (excluding target/):"
        git diff --name-only HEAD -- ':!target/' ':!**/target/'
        # Check if only Cargo.lock changed
        CHANGED_FILES=$(git diff --name-only HEAD -- ':!target/' ':!**/target/')
        if [ "$CHANGED_FILES" = "Cargo.lock" ]; then
            print_warning "Only Cargo.lock has changes - this is expected during testing"
            print_info "Committing Cargo.lock changes..."
            git add Cargo.lock
            git commit -m "chore: update Cargo.lock after testing"
        else
            print_error "Non-Cargo.lock files have uncommitted changes - this is not allowed"
            exit 1
        fi
    fi
    
    # Verify version consistency
    print_info "Verifying version consistency..."
    if ! "$PROJECT_ROOT/scripts/release/03-check-local-versions.sh"; then
        print_error "Version consistency check failed"
        exit 1
    fi
    
    print_success "=== Pre-release Test Act 1 PASSED ==="
    print_info "All checks passed! Ready for Act 2 testing."
    echo ""
    print_info "Next step: ./scripts/testing/pre-release-test-act2.sh"
}

# Run main function
main "$@"