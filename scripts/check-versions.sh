#!/bin/bash

# 🔍 Version consistency checker for diffx packages
# Ensures all packages have consistent versions

set -e

echo "🔍 Checking version consistency across diffx packages..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Extract versions
CORE_VERSION=$(grep '^version = ' diffx-core/Cargo.toml | head -1 | cut -d'"' -f2)
CLI_VERSION=$(grep '^version = ' diffx-cli/Cargo.toml | head -1 | cut -d'"' -f2)

if [ -f "diffx-npm/package.json" ]; then
    NPM_VERSION=$(node -p "require('./diffx-npm/package.json').version" 2>/dev/null || echo "unknown")
else
    NPM_VERSION="not found"
fi

if [ -f "diffx-python/pyproject.toml" ]; then
    PYTHON_VERSION=$(python -c "import tomli; print(tomli.load(open('diffx-python/pyproject.toml', 'rb'))['project']['version'])" 2>/dev/null || echo "unknown")
else
    PYTHON_VERSION="not found"
fi

# Display versions
echo ""
echo "📊 Current Package Versions:"
echo "┌─────────────────┬─────────────┐"
echo "│ Package         │ Version     │"
echo "├─────────────────┼─────────────┤"
printf "│ %-15s │ %-11s │\n" "diffx-core" "$CORE_VERSION"
printf "│ %-15s │ %-11s │\n" "diffx-cli" "$CLI_VERSION"
printf "│ %-15s │ %-11s │\n" "diffx-js" "$NPM_VERSION"
printf "│ %-15s │ %-11s │\n" "diffx-python" "$PYTHON_VERSION"
echo "└─────────────────┴─────────────┘"
echo ""

# Check consistency
ISSUES_FOUND=0

# Core vs CLI version check
if [ "$CORE_VERSION" != "$CLI_VERSION" ]; then
    error "Rust package versions don't match:"
    echo "   diffx-core: $CORE_VERSION"
    echo "   diffx-cli:  $CLI_VERSION"
    ISSUES_FOUND=$((ISSUES_FOUND + 1))
else
    success "Rust package versions are consistent"
fi

# Check if wrapper packages exist and have valid versions
if [ "$NPM_VERSION" = "not found" ]; then
    warning "npm package not found (diffx-npm/package.json missing)"
elif [ "$NPM_VERSION" = "unknown" ]; then
    error "npm package version could not be determined"
    ISSUES_FOUND=$((ISSUES_FOUND + 1))
else
    success "npm package version found: $NPM_VERSION"
fi

if [ "$PYTHON_VERSION" = "not found" ]; then
    warning "Python package not found (diffx-python/pyproject.toml missing)"
elif [ "$PYTHON_VERSION" = "unknown" ]; then
    error "Python package version could not be determined"
    ISSUES_FOUND=$((ISSUES_FOUND + 1))
else
    success "Python package version found: $PYTHON_VERSION"
fi

# Check if core dependency versions in CLI match
CLI_CORE_DEP=$(grep 'diffx-core.*version' diffx-cli/Cargo.toml | grep -o '"[^"]*"' | tr -d '"' || echo "not found")
if [ "$CLI_CORE_DEP" != "not found" ] && [ "$CLI_CORE_DEP" != "$CORE_VERSION" ]; then
    error "CLI references wrong core version:"
    echo "   Core version: $CORE_VERSION"
    echo "   CLI dependency: $CLI_CORE_DEP"
    ISSUES_FOUND=$((ISSUES_FOUND + 1))
fi

# Summary
echo ""
if [ $ISSUES_FOUND -eq 0 ]; then
    success "All version checks passed! ✨"
    echo ""
    info "Ready for publishing:"
    echo "   ./scripts/publish-all.sh"
else
    error "Found $ISSUES_FOUND version consistency issue(s)"
    echo ""
    info "Fix issues before publishing:"
    echo "   1. Update version numbers in respective files"
    echo "   2. Ensure core dependency versions match in CLI"
    echo "   3. Re-run this script to verify"
    exit 1
fi