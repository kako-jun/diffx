#!/bin/bash
set -e

# Same environment as GitHub Actions CI
export CARGO_TERM_COLOR=always

echo "🔄 Running complete CI simulation locally (matching GitHub Actions exactly)..."

echo "📝 Step 1: Check formatting"
cargo fmt --all --check

echo "🔍 Step 2: Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "🏗️ Step 3: Build"
cargo build --workspace --verbose

echo "🧪 Step 4: Run tests"
cargo test --workspace --verbose

echo "🚀 Step 5: Quick performance check"
# Light performance sanity check (just compilation and basic run)
cargo build --release --package diffx-core
echo "✅ Release build successful - performance optimizations applied"

echo "✅ All CI steps completed successfully!"
echo "🚀 Ready to push to remote repository"