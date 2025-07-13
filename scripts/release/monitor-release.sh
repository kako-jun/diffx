#!/usr/bin/env bash
set -euo pipefail

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
    print_error "Usage: $0 <version>"
    print_error "Example: $0 v0.6.0"
    exit 1
fi

VERSION=$1
TAG_VERSION=$VERSION
if [[ ! $VERSION =~ ^v ]]; then
    TAG_VERSION="v$VERSION"
fi

print_info "Monitoring release for version: $TAG_VERSION"

# Function to check GitHub Actions
check_github_actions() {
    print_info "Checking GitHub Actions status..."
    
    if ! command -v gh &> /dev/null; then
        print_error "GitHub CLI (gh) is required for monitoring"
        return 1
    fi
    
    # Get recent workflow runs
    local runs=$(gh run list --limit 20 --json status,conclusion,workflowName,createdAt,headSha | jq -r '.[] | select(.createdAt > (now - 3600)) | "\(.status)|\(.conclusion)|\(.workflowName)"')
    
    local act1_status=""
    local act2_status=""
    local pending_count=0
    local failed_count=0
    
    while IFS='|' read -r status conclusion workflow; do
        if [[ "$workflow" == *"Act1"* ]]; then
            act1_status="$status|$conclusion"
        elif [[ "$workflow" == *"Act2"* ]]; then
            act2_status="$status|$conclusion"
        fi
        
        if [[ "$status" == "in_progress" || "$status" == "queued" ]]; then
            ((pending_count++))
        elif [[ "$conclusion" == "failure" ]]; then
            ((failed_count++))
        fi
    done <<< "$runs"
    
    # Report status
    if [[ "$act1_status" == *"completed|success"* ]]; then
        print_success "Act1 workflow completed successfully"
    elif [[ "$act1_status" == *"completed|failure"* ]]; then
        print_error "Act1 workflow failed"
        return 1
    elif [[ "$act1_status" == *"in_progress"* ]]; then
        print_warning "Act1 workflow is still running"
    else
        print_warning "Act1 workflow status unknown"
    fi
    
    if [[ "$act2_status" == *"completed|success"* ]]; then
        print_success "Act2 workflow completed successfully"
    elif [[ "$act2_status" == *"completed|failure"* ]]; then
        print_error "Act2 workflow failed"
        return 1
    elif [[ "$act2_status" == *"in_progress"* ]]; then
        print_warning "Act2 workflow is still running"
    else
        print_warning "Act2 workflow status unknown"
    fi
    
    if [ $pending_count -gt 0 ]; then
        print_warning "$pending_count workflows are still running"
        return 2  # Still pending
    fi
    
    if [ $failed_count -gt 0 ]; then
        print_error "$failed_count workflows failed"
        return 1
    fi
    
    print_success "All workflows completed successfully"
    return 0
}

# Function to check GitHub release
check_github_release() {
    print_info "Checking GitHub release..."
    
    if gh release view "$TAG_VERSION" &> /dev/null; then
        print_success "GitHub release $TAG_VERSION exists"
        
        # Check assets
        local assets=$(gh release view "$TAG_VERSION" --json assets -q '.assets[].name')
        local expected_assets=("diffx-linux-x86_64" "diffx-windows-x86_64.exe" "diffx-macos-x86_64" "diffx-macos-arm64")
        
        for asset in "${expected_assets[@]}"; do
            if echo "$assets" | grep -q "$asset"; then
                print_success "Asset found: $asset"
            else
                print_error "Missing asset: $asset"
                return 1
            fi
        done
        
        return 0
    else
        print_error "GitHub release $TAG_VERSION not found"
        return 1
    fi
}

# Function to check crates.io (lightweight check)
check_crates_io() {
    print_info "Verifying crates.io packages are published..."
    
    local version_clean=${VERSION#v}  # Remove 'v' prefix
    
    # Simple check - just verify the version exists without detailed validation
    # The pre-release checks already validated cargo is working
    if curl -s "https://crates.io/api/v1/crates/diffx-core" | grep -q "\"newest_version\":\"$version_clean\""; then
        print_success "diffx-core $version_clean published on crates.io"
    else
        print_error "diffx-core $version_clean not yet published on crates.io"
        return 1
    fi
    
    if curl -s "https://crates.io/api/v1/crates/diffx-cli" | grep -q "\"newest_version\":\"$version_clean\""; then
        print_success "diffx-cli $version_clean published on crates.io"
    else
        print_error "diffx-cli $version_clean not yet published on crates.io"
        return 1
    fi
    
    return 0
}

# Function to check PyPI (lightweight check)
check_pypi() {
    print_info "Verifying PyPI package is published..."
    
    local version_clean=${VERSION#v}  # Remove 'v' prefix
    
    # Use PyPI API instead of pip command to avoid dependency on local pip setup
    if curl -s "https://pypi.org/pypi/diffx-python/json" | grep -q "\"version\":\"$version_clean\""; then
        print_success "diffx-python $version_clean published on PyPI"
        return 0
    else
        print_error "diffx-python $version_clean not yet published on PyPI"
        return 1
    fi
}

# Function to check npm (lightweight check)
check_npm() {
    print_info "Verifying npm package is published..."
    
    local version_clean=${VERSION#v}  # Remove 'v' prefix
    
    # Use npm registry API instead of npm command to avoid dependency on local npm setup
    if curl -s "https://registry.npmjs.org/diffx-js" | grep -q "\"$version_clean\""; then
        print_success "diffx-js $version_clean published on npm"
        return 0
    else
        print_error "diffx-js $version_clean not yet published on npm"
        return 1
    fi
}

# Function to wait for workflows to complete
wait_for_workflows() {
    print_info "Waiting for GitHub Actions to complete..."
    local max_wait=1800  # 30 minutes
    local wait_interval=30  # 30 seconds
    local elapsed=0
    
    while [ $elapsed -lt $max_wait ]; do
        if check_github_actions; then
            print_success "All workflows completed!"
            return 0
        elif [ $? -eq 2 ]; then
            # Still pending
            print_info "Workflows still running... waiting ${wait_interval}s (elapsed: ${elapsed}s)"
            sleep $wait_interval
            elapsed=$((elapsed + wait_interval))
        else
            # Failed
            print_error "Workflows failed!"
            return 1
        fi
    done
    
    print_error "Timeout waiting for workflows to complete"
    return 1
}

# Main monitoring function
main() {
    echo "==================== Release Monitoring ===================="
    echo "Monitoring release: $TAG_VERSION"
    echo ""
    
    # Step 1: Wait for GitHub Actions
    if ! wait_for_workflows; then
        print_error "GitHub Actions monitoring failed"
        exit 1
    fi
    
    echo ""
    print_info "GitHub Actions completed. Checking release artifacts..."
    echo ""
    
    # Step 2: Check GitHub release
    local github_ok=false
    for i in {1..5}; do
        if check_github_release; then
            github_ok=true
            break
        else
            print_warning "GitHub release check failed, retrying in 30s... (attempt $i/5)"
            sleep 30
        fi
    done
    
    if ! $github_ok; then
        print_error "GitHub release verification failed"
        exit 1
    fi
    
    echo ""
    
    # Step 3: Check package registries (with retries)
    print_info "Checking package registries..."
    echo ""
    
    # Wait a bit for packages to propagate
    print_info "Waiting 60s for package registries to update..."
    sleep 60
    
    local crates_ok=false
    local pypi_ok=false
    local npm_ok=false
    
    # Check crates.io with retries
    for i in {1..10}; do
        if check_crates_io; then
            crates_ok=true
            break
        else
            print_warning "crates.io check failed, retrying in 30s... (attempt $i/10)"
            sleep 30
        fi
    done
    
    # Check PyPI with retries
    for i in {1..10}; do
        if check_pypi; then
            pypi_ok=true
            break
        else
            print_warning "PyPI check failed, retrying in 30s... (attempt $i/10)"
            sleep 30
        fi
    done
    
    # Check npm with retries
    for i in {1..10}; do
        if check_npm; then
            npm_ok=true
            break
        else
            print_warning "npm check failed, retrying in 30s... (attempt $i/10)"
            sleep 30
        fi
    done
    
    echo ""
    echo "==================== Release Summary ===================="
    
    if $github_ok; then
        print_success "✓ GitHub release and assets"
    else
        print_error "✗ GitHub release and assets"
    fi
    
    if $crates_ok; then
        print_success "✓ crates.io packages (diffx-core, diffx-cli)"
    else
        print_error "✗ crates.io packages"
    fi
    
    if $pypi_ok; then
        print_success "✓ PyPI package (diffx-python)"
    else
        print_error "✗ PyPI package"
    fi
    
    if $npm_ok; then
        print_success "✓ npm package (diffx-js)"
    else
        print_error "✗ npm package"
    fi
    
    echo ""
    
    if $github_ok && $crates_ok && $pypi_ok && $npm_ok; then
        print_success "🎉 Release $TAG_VERSION completed successfully across all platforms!"
        echo ""
        print_info "Release URLs:"
        echo "  - GitHub: https://github.com/kako-jun/diffx/releases/tag/$TAG_VERSION"
        echo "  - crates.io: https://crates.io/crates/diffx-core"
        echo "  - PyPI: https://pypi.org/project/diffx-python/"
        echo "  - npm: https://www.npmjs.com/package/diffx-js"
        exit 0
    else
        print_error "❌ Release $TAG_VERSION had failures. Please check the issues above."
        echo ""
        print_warning "If this release is completely failed and you want to clean it up:"
        echo "  ./scripts/cleanup-failed-release.sh $TAG_VERSION"
        echo ""
        print_info "This will remove:"
        echo "  - GitHub release page and assets"
        echo "  - Git tags (local and remote)"
        echo "  - Optionally yank packages from registries"
        exit 1
    fi
}

# Run main function
main "$@"