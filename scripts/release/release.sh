#!/usr/bin/env bash
set -euo pipefail

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
NC='\033[0m' # No Color

# Configuration
MAIN_BRANCH="main"
CURRENT_BRANCH=$(git branch --show-current)
VERSION_FILE="Cargo.toml"

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

# Function to check if we're on the main branch
check_branch() {
    if [ "$CURRENT_BRANCH" != "$MAIN_BRANCH" ]; then
        print_error "You must be on the $MAIN_BRANCH branch to release"
        exit 1
    fi
}

# Function to check if working directory is clean
check_git_status() {
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        exit 1
    fi
}

# Function to run pre-release checks
run_pre_release_checks() {
    print_info "Running pre-release checks..."
    
    # Run CI local checks
    if [ -f "$PROJECT_ROOT/scripts/testing/ci-local.sh" ]; then
        print_info "Running CI local checks..."
        "$PROJECT_ROOT/scripts/testing/ci-local.sh"
    else
        print_error "ci-local.sh not found"
        exit 1
    fi
    
    # Check package tests
    print_info "Running package tests..."
    if [ -f "$PROJECT_ROOT/scripts/testing/test-published-packages.sh" ]; then
        "$PROJECT_ROOT/scripts/testing/test-published-packages.sh"
    fi
    
    print_success "All pre-release checks passed!"
}

# Function to get current version
get_current_version() {
    grep -E '^version = ".*"' "$VERSION_FILE" | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to validate version format
validate_version() {
    local version=$1
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
        print_error "Invalid version format: $version"
        print_error "Expected format: X.Y.Z or X.Y.Z-tag"
        exit 1
    fi
}

# Function to update version in a file
update_version_in_file() {
    local file=$1
    local old_version=$2
    local new_version=$3
    
    if [ -f "$file" ]; then
        print_info "Updating version in $file"
        sed -i "s/version = \"$old_version\"/version = \"$new_version\"/g" "$file"
    fi
}

# Function to update all version references
update_all_versions() {
    local old_version=$1
    local new_version=$2
    
    print_info "Updating version from $old_version to $new_version"
    
    # Update Rust crates
    update_version_in_file "Cargo.toml" "$old_version" "$new_version"
    update_version_in_file "diffx-core/Cargo.toml" "$old_version" "$new_version"
    update_version_in_file "diffx-cli/Cargo.toml" "$old_version" "$new_version"
    
    # Update Python package
    update_version_in_file "diffx-python/pyproject.toml" "$old_version" "$new_version"
    update_version_in_file "diffx-python/Cargo.toml" "$old_version" "$new_version"
    
    # Update npm package
    if [ -f "$PROJECT_ROOT/diffx-npm/package.json" ]; then
        print_info "Updating version in diffx-npm/package.json"
        cd "$PROJECT_ROOT/diffx-npm"
        npm version "$new_version" --no-git-tag-version
        cd "$PROJECT_ROOT"
    fi
    
    # Update Cargo.lock
    print_info "Updating Cargo.lock"
    cargo update --workspace
}

# Function to create git tag and push
create_and_push_tag() {
    local version=$1
    local tag="v$version"
    
    print_info "Creating git tag $tag"
    
    # Commit version updates
    git add -A
    git commit -m "chore: release version $version"
    
    # Create tag
    git tag -a "$tag" -m "Release $tag"
    
    # Push changes and tag
    print_info "Pushing changes and tag to remote"
    git push origin "$MAIN_BRANCH"
    git push origin "$tag"
    
    print_success "Tag $tag created and pushed!"
}

# Function to wait for GitHub Actions
wait_for_ci() {
    local tag=$1
    print_info "Waiting for GitHub Actions to complete..."
    print_info "Check progress at: https://github.com/kako-jun/diffx/actions"
    print_info "Release will be created at: https://github.com/kako-jun/diffx/releases/tag/$tag"
    echo ""
    print_warning "Please wait for all CI checks to pass before proceeding with manual publishing"
}

# Function to publish to crates.io
publish_crates() {
    print_info "Publishing to crates.io..."
    
    # Publish core first
    print_info "Publishing diffx-core..."
    cd "$PROJECT_ROOT/diffx-core"
    cargo publish
    cd "$PROJECT_ROOT"
    
    # Wait a bit for crates.io to process
    print_info "Waiting 30 seconds for crates.io to process diffx-core..."
    sleep 30
    
    # Publish CLI
    print_info "Publishing diffx-cli..."
    cd "$PROJECT_ROOT/diffx-cli"
    cargo publish
    cd "$PROJECT_ROOT"
    
    print_success "Crates published successfully!"
}

# Function to publish Python package
publish_python() {
    print_info "Building and publishing Python package..."
    
    cd "$PROJECT_ROOT/diffx-python"
    
    # Clean previous builds
    rm -rf dist/ target/wheels/
    
    # Build with maturin
    print_info "Building Python wheels with maturin..."
    maturin build --release
    
    # Upload to PyPI
    print_info "Uploading to PyPI..."
    maturin publish
    
    cd "$PROJECT_ROOT"
    
    print_success "Python package published successfully!"
}

# Function to publish npm package
publish_npm() {
    print_info "Publishing npm package..."
    
    cd "$PROJECT_ROOT/diffx-npm"
    
    # Ensure we're logged in
    if ! npm whoami &> /dev/null; then
        print_error "Not logged in to npm. Please run 'npm login' first."
        exit 1
    fi
    
    # Publish
    npm publish
    
    cd "$PROJECT_ROOT"
    
    print_success "npm package published successfully!"
}

# Main release flow
main() {
    print_info "Starting release process for diffx"
    
    # Pre-flight checks
    check_branch
    check_git_status
    
    # Get current version
    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: $CURRENT_VERSION"
    
    # Ask for new version
    read -p "Enter new version (current: $CURRENT_VERSION): " NEW_VERSION
    validate_version "$NEW_VERSION"
    
    # Confirmation
    echo ""
    print_warning "This will:"
    echo "  1. Run all pre-release checks"
    echo "  2. Update version to $NEW_VERSION in all packages"
    echo "  3. Create and push tag v$NEW_VERSION"
    echo "  4. Trigger GitHub Actions for release"
    echo "  5. Publish to crates.io, PyPI, and npm"
    echo ""
    read -p "Continue? (y/N) " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Release cancelled"
        exit 0
    fi
    
    # Run comprehensive pre-release checks (includes environment and version validation)
    run_pre_release_checks
    
    # Update versions
    update_all_versions "$CURRENT_VERSION" "$NEW_VERSION"
    
    # Create and push tag
    create_and_push_tag "$NEW_VERSION"
    
    # Wait for CI and monitor release
    wait_for_ci "v$NEW_VERSION"
    
    # Ask if ready to monitor the release
    echo ""
    print_warning "GitHub Actions should now be running. Would you like to start monitoring?"
    read -p "Start release monitoring? (Y/n) " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        print_info "Starting release monitoring..."
        if "$PROJECT_ROOT/scripts/release/monitor-release.sh" "v$NEW_VERSION"; then
            print_success "🎉 Release $NEW_VERSION completed successfully across all platforms!"
            
            # Automatic release notes enhancement check
            print_info "Performing automatic release quality checks..."
            
            RELEASE_BODY=$(gh release view "v$NEW_VERSION" --json body --jq '.body' 2>/dev/null || echo "")
            BODY_LENGTH=${#RELEASE_BODY}
            
            if [ "$BODY_LENGTH" -lt 200 ] || [[ "$RELEASE_BODY" == *"**Full Changelog**"* ]] && [[ ! "$RELEASE_BODY" == *"Key Highlights"* ]]; then
                print_warning "⚠️  ATTENTION: Release notes need enhancement!"
                echo ""
                print_info "According to release guide, detailed release notes should be added."
                print_info "This release currently has minimal content and should be enhanced."
                echo ""
                print_info "Next steps:"
                echo "  1. Identify the last substantial release (skip 'garbage' releases)"
                echo "  2. Collect changes from that version to v$NEW_VERSION"
                echo "  3. Create comprehensive release notes with:"
                echo "     - Key highlights and user-facing changes"
                echo "     - Technical improvements"
                echo "     - Package availability information"
                echo "     - Migration notes if applicable"
                echo ""
                print_warning "Current release notes are insufficient for user consumption."
            else
                print_success "✓ Release notes appear comprehensive"
            fi
        else
            print_error "❌ Release monitoring detected failures. Please check the issues above."
            exit 1
        fi
    else
        print_warning "Monitoring skipped. You can manually monitor with:"
        echo "  ./scripts/release/monitor-release.sh v$NEW_VERSION"
        echo ""
        print_warning "Manual verification steps:"
        echo "  - GitHub Actions: gh run list"
        echo "  - GitHub Release: gh release view v$NEW_VERSION"
        echo "  - crates.io: cargo search diffx-core && cargo search diffx-cli"
        echo "  - PyPI: pip index versions diffx-python"
        echo "  - npm: npm view diffx-js version"
    fi
}

# Run main function
main "$@"