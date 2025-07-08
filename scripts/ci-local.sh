#!/bin/bash
set -e

# Same environment as GitHub Actions CI
export CARGO_TERM_COLOR=always

echo "🔄 Running complete CI simulation locally (matching GitHub Actions exactly)..."

echo "Step 1: Check formatting"
cargo fmt --all --check

echo "Step 2: Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "🏗️ Step 3: Build"
cargo build --workspace --verbose

echo "🧪 Step 4: Run tests"
cargo test --workspace --verbose

echo "Step 5: Quick performance check"
# Light performance sanity check (just compilation and basic run)
cargo build --release --package diffx-core
echo "Release build successful - performance optimizations applied"

echo "Step 6: Test core CLI functionality"
# Test basic JSON diff (must succeed)
echo '{"a": 1}' > /tmp/test1.json
echo '{"a": 2}' > /tmp/test2.json
cargo run --bin diffx -- /tmp/test1.json /tmp/test2.json > /dev/null

# Test YAML diff (must succeed)
echo 'name: old' > /tmp/test1.yaml
echo 'name: new' > /tmp/test2.yaml
cargo run --bin diffx -- /tmp/test1.yaml /tmp/test2.yaml > /dev/null

# Test stdin processing (must succeed)
echo '{"b": 1}' | cargo run --bin diffx -- - /tmp/test1.json > /dev/null

# Cleanup
rm -f /tmp/test1.json /tmp/test2.json /tmp/test1.yaml /tmp/test2.yaml

echo "All CI steps completed successfully!"
echo "Ready to push to remote repository"