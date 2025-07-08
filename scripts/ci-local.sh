#!/bin/bash
set -euo pipefail

# Exactly match GitHub Actions CI environment
export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1

# Stricter error handling to match CI
trap 'echo "Error occurred on line $LINENO. Exit code: $?" >&2' ERR

echo "Running complete CI simulation locally (matching GitHub Actions exactly)..."

echo "Step 1: Check formatting"
cargo fmt --all --check

echo "Step 2: Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "Step 3: Build"
cargo build --workspace --verbose

echo "Step 4: Run tests"
cargo test --workspace --verbose

echo "Step 5: Quick performance check"
# Light performance sanity check (just compilation and basic run)
cargo build --release --package diffx-core
echo "Release build successful - performance optimizations applied"

echo "Step 6: Test core CLI functionality"

# Create temp directory for test files (like CI would)
TEST_DIR=$(mktemp -d)
trap 'rm -rf "$TEST_DIR"' EXIT

# Test basic JSON diff (must succeed)
echo '{"a": 1}' > "$TEST_DIR/test1.json"
echo '{"a": 2}' > "$TEST_DIR/test2.json"
if ! cargo run --bin diffx -- "$TEST_DIR/test1.json" "$TEST_DIR/test2.json" > /dev/null 2>&1; then
    echo "ERROR: Basic JSON diff test failed" >&2
    exit 1
fi

# Test YAML diff (must succeed)
echo 'name: old' > "$TEST_DIR/test1.yaml"
echo 'name: new' > "$TEST_DIR/test2.yaml"
if ! cargo run --bin diffx -- "$TEST_DIR/test1.yaml" "$TEST_DIR/test2.yaml" > /dev/null 2>&1; then
    echo "ERROR: YAML diff test failed" >&2
    exit 1
fi

# Test stdin processing (must succeed)
if ! echo '{"b": 1}' | cargo run --bin diffx -- - "$TEST_DIR/test1.json" > /dev/null 2>&1; then
    echo "ERROR: Stdin processing test failed" >&2
    exit 1
fi

# Additional tests to ensure exact CI parity
echo "Step 7: Additional strict checks"

# Ensure no warnings in release mode
if ! cargo build --release --workspace 2>&1 | grep -v "Finished" | grep -v "Compiling" | grep -v "Building" | grep -q .; then
    echo "Release build completed without warnings"
else
    echo "ERROR: Release build produced warnings" >&2
    exit 1
fi

# Check for any TODO or FIXME comments (optional but good practice)
if grep -r "TODO\|FIXME" --include="*.rs" . | grep -v "target/"; then
    echo "WARNING: Found TODO/FIXME comments in code"
fi

# Verify Cargo.lock is committed and up to date
if ! git diff --quiet Cargo.lock; then
    echo "ERROR: Cargo.lock has uncommitted changes" >&2
    exit 1
fi

# Check for large files that shouldn't be committed
if find . -type f -size +1M -not -path "./target/*" -not -path "./.git/*" | grep -q .; then
    echo "WARNING: Found files larger than 1MB"
    find . -type f -size +1M -not -path "./target/*" -not -path "./.git/*" -exec ls -lh {} \;
fi

echo "All CI steps completed successfully!"
echo "Ready to push to remote repository"