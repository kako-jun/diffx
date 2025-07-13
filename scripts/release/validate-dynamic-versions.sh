#!/usr/bin/env bash
set -euo pipefail

# Validate that no hardcoded version checks exist in the codebase
# This prevents release failures due to hardcoded version assertions

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

print_info "Validating dynamic version handling in diffx codebase..."
echo ""

# Function to check for hardcoded version patterns
check_hardcoded_versions() {
    local file_pattern=$1
    local description=$2
    
    print_info "Checking $description..."
    
    # Look for hardcoded version patterns like "0.4.2", "v0.5.1", etc.
    # Exclude venv, .venv, node_modules, target, and other dependency directories
    # Also exclude comments, examples, and usage strings
    local hardcoded_versions=$(grep -r -n "v\?0\.[0-9]\+\.[0-9]\+" --include="$file_pattern" . 2>/dev/null | \
        grep -v "/venv/" | \
        grep -v "/.venv/" | \
        grep -v "/node_modules/" | \
        grep -v "/target/" | \
        grep -v "/test_env/" | \
        grep -v "/site-packages/" | \
        grep -v "\.git/" | \
        grep -v "test.*version.*1\.[0-9]" | \
        grep -v "example" | \
        grep -v "fixture" | \
        grep -v "Example:" | \
        grep -v "# Look for" | \
        grep -v "#.*version.*patterns" || true)
    
    if [ -n "$hardcoded_versions" ]; then
        print_error "Found hardcoded version numbers in $description:"
        echo "$hardcoded_versions"
        ((ERRORS++))
    else
        print_success "No hardcoded versions found in $description"
    fi
}

# Function to check specific Python version handling
check_python_version() {
    print_info "Checking Python package version handling..."
    
    if [ -f "diffx-python/src/diffx/__init__.py" ]; then
        if grep -q "__version__ = \"[0-9]" diffx-python/src/diffx/__init__.py; then
            print_error "Found hardcoded __version__ in diffx-python/__init__.py"
            grep -n "__version__ = \"[0-9]" diffx-python/src/diffx/__init__.py
            ((ERRORS++))
        else
            print_success "Python package uses dynamic version loading"
        fi
    else
        print_warning "Python package __init__.py not found"
    fi
}

# Function to check for version assertions in tests
check_test_assertions() {
    print_info "Checking for hardcoded version assertions in tests..."
    
    # Look for version assertions that might be hardcoded
    local version_assertions=$(grep -r -n "assert.*version.*[0-9]" --include="*.py" --include="*.js" --include="*.sh" . 2>/dev/null | \
        grep -v "/venv/" | \
        grep -v "/.venv/" | \
        grep -v "/node_modules/" | \
        grep -v "/target/" | \
        grep -v "/test_env/" | \
        grep -v "/site-packages/" | \
        grep -v "\.git/" | \
        grep -v "1\.[0-9]" | \
        grep -v "example" | \
        grep -v "validate-dynamic-versions.sh" || true)
    
    if [ -n "$version_assertions" ]; then
        print_error "Found potentially hardcoded version assertions:"
        echo "$version_assertions"
        ((ERRORS++))
    else
        print_success "No hardcoded version assertions found"
    fi
}

# Function to validate version consistency scripts
check_version_scripts() {
    print_info "Validating version consistency scripts use dynamic extraction..."
    
    # Check that version scripts use dynamic extraction
    if [ -f "scripts/utils/check-versions.sh" ]; then
        if grep -q "grep.*version.*Cargo.toml" scripts/utils/check-versions.sh; then
            print_success "check-versions.sh uses dynamic extraction"
        else
            print_error "check-versions.sh might not use dynamic extraction"
            ((ERRORS++))
        fi
    else
        print_warning "check-versions.sh not found"
    fi
}

# Function to check for good patterns
check_good_patterns() {
    print_info "Checking for proper dynamic version patterns..."
    
    local good_patterns=0
    
    # Check for dynamic version extraction patterns
    if grep -q "cargo search" scripts/*/* 2>/dev/null; then
        print_success "Found cargo search usage (good)"
        ((good_patterns++))
    fi
    
    if grep -q "npm view.*version" scripts/*/* 2>/dev/null; then
        print_success "Found npm view version usage (good)"
        ((good_patterns++))
    fi
    
    if grep -q "pip.*index.*versions\|pip.*show" scripts/*/* 2>/dev/null; then
        print_success "Found pip version checking usage (good)"
        ((good_patterns++))
    fi
    
    if grep -q "importlib.metadata.*version\|pkg_resources.*version" diffx-python/src/diffx/__init__.py 2>/dev/null; then
        print_success "Found dynamic Python version loading (good)"
        ((good_patterns++))
    fi
    
    if [ $good_patterns -eq 0 ]; then
        print_warning "No dynamic version patterns found - this might indicate issues"
    fi
}

# Run all checks
check_hardcoded_versions "*.py" "Python files"
check_hardcoded_versions "*.js" "JavaScript files"
check_hardcoded_versions "*.sh" "Shell scripts"
check_hardcoded_versions "*.rs" "Rust files"

echo ""
check_python_version
echo ""
check_test_assertions
echo ""
check_version_scripts
echo ""
check_good_patterns

echo ""
echo "======================================="

if [ $ERRORS -eq 0 ]; then
    print_success "✅ All version handling validation passed!"
    echo ""
    print_info "The codebase correctly uses dynamic version extraction and"
    print_info "should not have hardcoded version issues during releases."
else
    print_error "❌ Found $ERRORS issue(s) with version handling"
    echo ""
    print_info "Fix these issues before releasing to prevent version-related failures:"
    echo "  1. Replace hardcoded versions with dynamic extraction"
    echo "  2. Use check-versions.sh for consistency validation"
    echo "  3. Test with different version numbers before releasing"
    exit 1
fi