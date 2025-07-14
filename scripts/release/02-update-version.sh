#!/bin/bash
set -euo pipefail

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_NAME=$(basename "$PROJECT_ROOT")

# Change to project root
cd "$PROJECT_ROOT"

# Function to validate version format
validate_version() {
    local version=$1
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
        echo "ERROR: Invalid version format: $version"
        echo "Expected format: X.Y.Z or X.Y.Z-tag"
        exit 1
    fi
}

# Function to get current version from main Cargo.toml
get_current_version() {
    grep -E '^version = ".*"' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to update version in a file
update_version_in_file() {
    local file=$1
    local old_version=$2
    local new_version=$3
    
    if [ -f "$file" ]; then
        echo "Updating version in $file"
        sed -i "s/version = \"$old_version\"/version = \"$new_version\"/g" "$file"
    else
        echo "WARNING: File $file not found"
    fi
}

# Function to update npm package version
update_npm_version() {
    local new_version=$1
    local package_json="$PROJECT_ROOT/${PROJECT_NAME}-npm/package.json"
    
    if [ -f "$package_json" ]; then
        echo "Updating version in ${PROJECT_NAME}-npm/package.json"
        cd "$PROJECT_ROOT/${PROJECT_NAME}-npm"
        npm version "$new_version" --no-git-tag-version
        cd "$PROJECT_ROOT"
    else
        echo "WARNING: ${PROJECT_NAME}-npm/package.json not found"
    fi
}

# Main function
main() {
    if [ $# -ne 1 ]; then
        echo "Usage: $0 <new_version>"
        echo "Example: $0 0.5.4"
        exit 1
    fi
    
    local new_version="$1"
    validate_version "$new_version"
    
    # Get current version
    local current_version=$(get_current_version)
    echo "Current version: $current_version"
    echo "New version: $new_version"
    
    # Update all product component versions
    echo "Updating product component versions..."
    
    # Update Rust crates
    update_version_in_file "$PROJECT_ROOT/Cargo.toml" "$current_version" "$new_version"
    update_version_in_file "$PROJECT_ROOT/${PROJECT_NAME}-core/Cargo.toml" "$current_version" "$new_version"
    update_version_in_file "$PROJECT_ROOT/${PROJECT_NAME}-cli/Cargo.toml" "$current_version" "$new_version"
    
    # Update Python package
    update_version_in_file "$PROJECT_ROOT/${PROJECT_NAME}-python/pyproject.toml" "$current_version" "$new_version"
    update_version_in_file "$PROJECT_ROOT/${PROJECT_NAME}-python/Cargo.toml" "$current_version" "$new_version"
    
    # Update npm package
    update_npm_version "$new_version"
    
    # Update Cargo.lock
    echo "Updating Cargo.lock"
    cargo update --workspace
    
    echo "Version update completed successfully!"
    echo ""
    echo "REMINDER: Please manually update the following files:"
    echo "  - CLAUDE.md (latest release information)"
    echo "  - CHANGELOG.md (release notes)"
    echo "  - docs/examples/README.md (if version references exist)"
}

# Run main function
main "$@"