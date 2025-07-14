#!/bin/bash
set -euo pipefail

# Check currently published versions across all package managers
# Helps determine what version should be released next

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Change to project root
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

error() {
    echo -e "${RED}ERROR: $1${NC}"
}

success() {
    echo -e "${GREEN}OK: $1${NC}"
}

echo "Checking currently published versions across package managers..."
echo ""

# Check crates.io versions
echo "📦 Checking crates.io..."
CRATES_CORE_VERSION=$(cargo search diffx-core --limit 1 2>/dev/null | grep -E "^diffx-core" | sed 's/.*= "\([^"]*\)".*/\1/' || echo "not found")
CRATES_CLI_VERSION=$(cargo search diffx-cli --limit 1 2>/dev/null | grep -E "^diffx-cli" | sed 's/.*= "\([^"]*\)".*/\1/' || echo "not found")

# Check npm registry
echo "📦 Checking npm registry..."
NPM_VERSION=$(npm view diffx-js version 2>/dev/null || echo "not found")

# Check PyPI
echo "📦 Checking PyPI..."
PYPI_VERSION=$(pip index versions diffx-python 2>/dev/null | grep -E "Available versions:" | sed 's/.*: \([^,]*\).*/\1/' || echo "not found")

# Get local version for comparison
LOCAL_VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Display results
echo ""
echo "Published Package Versions:"
echo "┌─────────────────┬─────────────┬─────────────┐"
echo "│ Package         │ Published   │ Local       │"
echo "├─────────────────┼─────────────┼─────────────┤"
printf "│ %-15s │ %-11s │ %-11s │\n" "diffx-core" "$CRATES_CORE_VERSION" "$LOCAL_VERSION"
printf "│ %-15s │ %-11s │ %-11s │\n" "diffx-cli" "$CRATES_CLI_VERSION" "$LOCAL_VERSION"
printf "│ %-15s │ %-11s │ %-11s │\n" "diffx-js" "$NPM_VERSION" "$LOCAL_VERSION"
printf "│ %-15s │ %-11s │ %-11s │\n" "diffx-python" "$PYPI_VERSION" "$LOCAL_VERSION"
echo "└─────────────────┴─────────────┴─────────────┘"
echo ""

# Analysis
echo "Analysis:"

# Check if local version is already published
if [ "$LOCAL_VERSION" = "$CRATES_CORE_VERSION" ] || [ "$LOCAL_VERSION" = "$NPM_VERSION" ] || [ "$LOCAL_VERSION" = "$PYPI_VERSION" ]; then
    warning "Local version $LOCAL_VERSION appears to be already published"
    echo "   You may need to increment the version before releasing"
else
    success "Local version $LOCAL_VERSION is not yet published"
fi

# Check version consistency across published packages
PUBLISHED_VERSIONS="$CRATES_CORE_VERSION $CRATES_CLI_VERSION $NPM_VERSION $PYPI_VERSION"
UNIQUE_VERSIONS=$(echo "$PUBLISHED_VERSIONS" | tr ' ' '\n' | grep -v "not found" | sort -u | wc -l)

if [ "$UNIQUE_VERSIONS" -gt 1 ]; then
    warning "Published versions are inconsistent across package managers"
    echo "   This suggests a previous release may have failed partially"
else
    success "Published versions are consistent across package managers"
fi

echo ""
echo "Next steps:"
echo "1. Decide on next version number (current local: $LOCAL_VERSION)"
echo "2. Run: ./scripts/release/update-version.sh <new_version>"
echo "3. Run: ./scripts/utils/check-local-versions.sh"
echo "4. Run: ./scripts/release/release.sh"