#!/usr/bin/env bash
set -euo pipefail

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
NC='\033[0m' # No Color

# Function to print colored output
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

# Check if version is provided
if [ $# -eq 0 ]; then
    print_error "Usage: $0 <failed-version> [--force]"
    print_error "Example: $0 v0.6.0"
    print_error "Use --force to skip confirmation prompts"
    exit 1
fi

FAILED_VERSION=$1
FORCE_MODE=false
if [ "$#" -eq 2 ] && [ "$2" = "--force" ]; then
    FORCE_MODE=true
fi
TAG_VERSION=$FAILED_VERSION
if [[ ! $FAILED_VERSION =~ ^v ]]; then
    TAG_VERSION="v$FAILED_VERSION"
fi

VERSION_CLEAN=${TAG_VERSION#v}  # Remove 'v' prefix

print_warning "This will clean up the failed release: $TAG_VERSION"
echo ""
print_warning "This action will:"
echo "  1. Delete the GitHub release page"
echo "  2. Delete the local git tag"
echo "  3. Delete the remote git tag"
echo "  4. Optionally yank crates from crates.io"
echo "  5. Show instructions for npm/PyPI cleanup"
echo ""

if [ "$FORCE_MODE" = false ]; then
    read -p "Continue with cleanup? (y/N) " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Cleanup cancelled"
        exit 0
    fi
else
    print_info "Force mode enabled, proceeding with cleanup..."
fi

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    print_error "GitHub CLI (gh) is required for cleanup"
    exit 1
fi

print_info "Starting cleanup for failed release: $TAG_VERSION"
echo ""

# Step 1: Delete GitHub release
print_info "Deleting GitHub release..."
if gh release view "$TAG_VERSION" &> /dev/null; then
    if gh release delete "$TAG_VERSION" --yes; then
        print_success "GitHub release $TAG_VERSION deleted"
    else
        print_error "Failed to delete GitHub release $TAG_VERSION"
    fi
else
    print_warning "GitHub release $TAG_VERSION not found (already deleted?)"
fi

# Step 2: Delete local tag
print_info "Deleting local git tag..."
if git tag --list | grep -q "^$TAG_VERSION$"; then
    if git tag -d "$TAG_VERSION"; then
        print_success "Local tag $TAG_VERSION deleted"
    else
        print_error "Failed to delete local tag $TAG_VERSION"
    fi
else
    print_warning "Local tag $TAG_VERSION not found (already deleted?)"
fi

# Step 3: Delete remote tag
print_info "Deleting remote git tag..."
if git ls-remote --tags origin | grep -q "refs/tags/$TAG_VERSION$"; then
    if git push --delete origin "$TAG_VERSION"; then
        print_success "Remote tag $TAG_VERSION deleted"
    else
        print_error "Failed to delete remote tag $TAG_VERSION"
    fi
else
    print_warning "Remote tag $TAG_VERSION not found (already deleted?)"
fi

# Step 4: Check and optionally yank crates
echo ""
print_info "Checking crates.io packages..."

check_and_yank_crate() {
    local crate_name=$1
    
    if cargo search "$crate_name" | grep -q "$crate_name.*= \"$VERSION_CLEAN\""; then
        print_warning "Found $crate_name version $VERSION_CLEAN on crates.io"
        
        if [ "$FORCE_MODE" = true ]; then
            print_info "Force mode: yanking $crate_name@$VERSION_CLEAN from crates.io"
            if cargo yank --vers "$VERSION_CLEAN" "$crate_name"; then
                print_success "$crate_name@$VERSION_CLEAN yanked from crates.io"
            else
                print_error "Failed to yank $crate_name@$VERSION_CLEAN"
            fi
        else
            read -p "Yank $crate_name@$VERSION_CLEAN from crates.io? (y/N) " -n 1 -r
            echo ""
            
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                if cargo yank --vers "$VERSION_CLEAN" "$crate_name"; then
                    print_success "$crate_name@$VERSION_CLEAN yanked from crates.io"
                else
                    print_error "Failed to yank $crate_name@$VERSION_CLEAN"
                fi
            else
                print_info "Skipping yank for $crate_name@$VERSION_CLEAN"
            fi
        fi
    else
        print_success "$crate_name@$VERSION_CLEAN not found on crates.io (good)"
    fi
}

check_and_yank_crate "${PROJECT_NAME}-core"
check_and_yank_crate "${PROJECT_NAME}"

# Step 5: Check npm package
echo ""
print_info "Checking npm package..."
if command -v npm &> /dev/null; then
    local npm_versions=$(npm view ${PROJECT_NAME}-js versions --json 2>/dev/null | jq -r '.[]' 2>/dev/null || echo "")
    if echo "$npm_versions" | grep -q "^$VERSION_CLEAN$"; then
        print_warning "Found ${PROJECT_NAME}-js@$VERSION_CLEAN on npm"
        print_info "To unpublish from npm (only possible within 24 hours):"
        echo "  npm unpublish ${PROJECT_NAME}-js@$VERSION_CLEAN"
        echo ""
        if [ "$FORCE_MODE" = true ]; then
            print_info "Force mode: attempting to unpublish ${PROJECT_NAME}-js@$VERSION_CLEAN"
            if npm unpublish "${PROJECT_NAME}-js@$VERSION_CLEAN"; then
                print_success "${PROJECT_NAME}-js@$VERSION_CLEAN unpublished from npm"
            else
                print_error "Failed to unpublish ${PROJECT_NAME}-js@$VERSION_CLEAN (may be too late or insufficient permissions)"
            fi
        else
            read -p "Attempt to unpublish ${PROJECT_NAME}-js@$VERSION_CLEAN now? (y/N) " -n 1 -r
            echo ""
            
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                if npm unpublish "${PROJECT_NAME}-js@$VERSION_CLEAN"; then
                    print_success "${PROJECT_NAME}-js@$VERSION_CLEAN unpublished from npm"
                else
                    print_error "Failed to unpublish ${PROJECT_NAME}-js@$VERSION_CLEAN (may be too late or insufficient permissions)"
                fi
            fi
        fi
    else
        print_success "${PROJECT_NAME}-js@$VERSION_CLEAN not found on npm (good)"
    fi
else
    print_warning "npm not available, skipping npm check"
fi

# Step 6: Check PyPI package
echo ""
print_info "Checking PyPI package..."
if command -v pip &> /dev/null; then
    local pypi_versions=$(pip index versions ${PROJECT_NAME}-python 2>/dev/null | grep "Available versions:" | sed 's/Available versions: //' || echo "")
    if echo "$pypi_versions" | grep -q "$VERSION_CLEAN"; then
        print_warning "Found ${PROJECT_NAME}-python@$VERSION_CLEAN on PyPI"
        print_warning "PyPI does not allow deletion of packages automatically."
        print_info "If you need to remove ${PROJECT_NAME}-python@$VERSION_CLEAN from PyPI:"
        echo "  1. Contact PyPI support: https://pypi.org/help/#file-bug-reports"
        echo "  2. Explain the situation (failed release)"
        echo "  3. Request removal of version $VERSION_CLEAN"
    else
        print_success "${PROJECT_NAME}-python@$VERSION_CLEAN not found on PyPI (good)"
    fi
else
    print_warning "pip not available, skipping PyPI check"
fi

# Step 7: Summary
echo ""
echo "==================== Cleanup Summary ===================="
print_success "✓ GitHub release $TAG_VERSION cleaned up"
print_success "✓ Git tags cleaned up"

print_info "Manual follow-up if needed:"
echo "  - Check crates.io: https://crates.io/crates/${PROJECT_NAME}-core"
echo "  - Check npm: https://www.npmjs.com/package/${PROJECT_NAME}-js"
echo "  - Check PyPI: https://pypi.org/project/${PROJECT_NAME}-python/"
echo ""
print_info "You can now safely create a new release with the same or different version."

echo ""
print_success "Cleanup completed for failed release $TAG_VERSION"