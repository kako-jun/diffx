#!/usr/bin/env bash
set -euo pipefail

# Quick release readiness check
# Performs essential checks without full CI run

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

ERRORS=0

echo "🚀 Quick Release Readiness Check"
echo "================================"
echo ""

# 1. Git status check
print_info "Checking git status..."
if git diff-index --quiet HEAD --; then
    print_success "No uncommitted changes"
else
    print_error "Uncommitted changes detected"
    ((ERRORS++))
fi

CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" = "main" ]; then
    print_success "On main branch"
else
    print_error "Not on main branch (current: $CURRENT_BRANCH)"
    ((ERRORS++))
fi

# 2. Version consistency
print_info "Checking version consistency..."
CARGO_VERSION=$(grep -E '^version = ".*"' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
PYTHON_VERSION=$(grep -E '^version = ".*"' diffx-python/pyproject.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
NPM_VERSION=$(node -p "require('./diffx-npm/package.json').version" 2>/dev/null || echo "unknown")

if [ "$CARGO_VERSION" = "$PYTHON_VERSION" ] && [ "$CARGO_VERSION" = "$NPM_VERSION" ]; then
    print_success "All versions consistent: $CARGO_VERSION"
else
    print_error "Version mismatch: Cargo=$CARGO_VERSION, Python=$PYTHON_VERSION, npm=$NPM_VERSION"
    ((ERRORS++))
fi

# 3. Authentication quick check
print_info "Checking authentication..."

if npm whoami &> /dev/null; then
    print_success "npm authenticated"
else
    print_error "npm not authenticated"
    ((ERRORS++))
fi

if [ -f "$HOME/.cargo/credentials.toml" ] || [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
    print_success "Cargo credentials found"
else
    print_warning "Cargo credentials not found"
fi

if [ -f "$HOME/.pypirc" ] || [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
    print_success "PyPI credentials found"
else
    print_warning "PyPI credentials not found"
fi

# 4. Dynamic version validation
print_info "Running dynamic version validation..."
if ./scripts/release/validate-dynamic-versions.sh > /dev/null 2>&1; then
    print_success "Dynamic version validation passed"
else
    print_error "Dynamic version validation failed"
    ((ERRORS++))
fi

echo ""
echo "================================"

if [ $ERRORS -eq 0 ]; then
    print_success "✅ Quick check passed! Ready to proceed with full release process."
    echo ""
    print_info "Next steps:"
    echo "  1. Run full pre-release check: ./scripts/release/pre-release-check.sh"
    echo "  2. Run CI local: ./scripts/testing/ci-local.sh"
    echo "  3. Execute release: ./scripts/release/release.sh"
else
    print_error "❌ Found $ERRORS critical issue(s). Fix before proceeding."
    echo ""
    print_info "Run full check for details: ./scripts/release/pre-release-check.sh"
    exit 1
fi