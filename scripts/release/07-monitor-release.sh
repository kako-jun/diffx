#!/usr/bin/env bash
set -uo pipefail

# Release monitoring script - correctly handles Act1 -> Act2 workflow
# Fixes issues with false positive errors and workflow dependency understanding

# Get project name from current directory
PROJECT_NAME=$(basename "$(pwd)")

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

# Check if version is provided
if [ $# -eq 0 ]; then
    print_error "Usage: $0 <version>"
    print_error "Example: $0 v0.5.4"
    exit 1
fi

VERSION=$1
if [[ ! $VERSION =~ ^v ]]; then
    VERSION="v$VERSION"
fi

print_info "Monitoring release for version: $VERSION"

# Check GitHub CLI availability
if ! command -v gh &> /dev/null; then
    print_error "GitHub CLI (gh) is required for monitoring"
    exit 1
fi

# Function to get workflow runs for this specific tag
get_workflow_runs() {
    local workflow_name="$1"
    local tag="$2"
    
    # Get the most recent run for this tag - handle both push and workflow_run events
    gh run list --workflow="$workflow_name" --json status,conclusion,event,headBranch,createdAt,url --limit 50 | \
    jq -r --arg tag "$tag" '
        map(select(
            (.event == "push" and (.headBranch == $tag or .headBranch == "refs/tags/" + $tag)) or
            (.event == "workflow_run") or
            (.event == "workflow_dispatch")
        )) | 
        sort_by(.createdAt) | 
        reverse | 
        .[0] // empty
    '
}

# Function to check Act1 workflow
check_act1() {
    local tag="$1"
    print_info "Checking Act1 workflow for $tag..."
    
    local run_info=$(get_workflow_runs "release-act1.yml" "$tag")
    
    if [ -z "$run_info" ] || [ "$run_info" = "null" ]; then
        print_warning "No Act1 run found for $tag yet (may still be queuing)"
        return 1
    fi
    
    local status=$(echo "$run_info" | jq -r '.status // "unknown"')
    local conclusion=$(echo "$run_info" | jq -r '.conclusion // "null"')
    local url=$(echo "$run_info" | jq -r '.url // ""')
    local event=$(echo "$run_info" | jq -r '.event // ""')
    
    print_info "Act1 status: $status, conclusion: $conclusion, event: $event"
    if [ -n "$url" ]; then
        print_info "Act1 URL: $url"
    fi
    
    case "$status" in
        "completed")
            case "$conclusion" in
                "success")
                    print_success "Act1 completed successfully"
                    return 0
                    ;;
                "failure")
                    print_error "Act1 failed with conclusion: $conclusion"
                    return 2
                    ;;
                "cancelled")
                    print_error "Act1 was cancelled"
                    return 2
                    ;;
                *)
                    print_error "Act1 completed with unknown conclusion: $conclusion"
                    return 2
                    ;;
            esac
            ;;
        "in_progress"|"queued")
            print_info "Act1 is still running..."
            return 1
            ;;
        *)
            print_warning "Act1 has unknown status: $status"
            return 1
            ;;
    esac
}

# Function to check Act2 workflow (only after Act1 succeeds)
check_act2() {
    local tag="$1"
    print_info "Checking Act2 workflow for $tag..."
    
    local run_info=$(get_workflow_runs "release-act2.yml" "$tag")
    
    if [ -z "$run_info" ]; then
        print_info "No Act2 run found yet (Act2 starts after Act1 completes)"
        return 1
    fi
    
    local status=$(echo "$run_info" | jq -r '.status // "unknown"')
    local conclusion=$(echo "$run_info" | jq -r '.conclusion // "null"')
    local url=$(echo "$run_info" | jq -r '.url // ""')
    local run_id=$(echo "$run_info" | jq -r '.databaseId // ""')
    
    print_info "Act2 status: $status, conclusion: $conclusion"
    if [ -n "$url" ]; then
        print_info "Act2 URL: $url"
    fi
    
    # Check for job failures even if workflow is still running
    if [ "$status" = "in_progress" ] && [ -n "$run_id" ]; then
        print_info "Checking individual job statuses..."
        local failed_jobs=$(gh run view "$run_id" --json jobs --jq '.jobs[] | select(.conclusion == "failure") | .name' 2>/dev/null || echo "")
        if [ -n "$failed_jobs" ]; then
            print_error "Act2 has failed jobs: $failed_jobs"
            return 2
        fi
    fi
    
    case "$status" in
        "completed")
            case "$conclusion" in
                "success")
                    print_success "Act2 completed successfully"
                    return 0
                    ;;
                "failure")
                    print_error "Act2 failed with conclusion: $conclusion"
                    return 2
                    ;;
                "cancelled")
                    print_error "Act2 was cancelled"
                    return 2
                    ;;
                *)
                    print_error "Act2 completed with unknown conclusion: $conclusion"
                    return 2
                    ;;
            esac
            ;;
        "in_progress"|"queued")
            print_info "Act2 is still running..."
            return 1
            ;;
        *)
            print_warning "Act2 has unknown status: $status"
            return 1
            ;;
    esac
}

# Function to check package availability
check_packages() {
    local version="${VERSION#v}"  # Remove 'v' prefix
    
    print_info "Checking package availability for version $version..."
    
    # Check crates.io
    if curl -s "https://crates.io/api/v1/crates/${PROJECT_NAME}-core" | grep -q "\"newest_version\":\"$version\""; then
        print_success "✓ crates.io: ${PROJECT_NAME}-core $version"
    else
        print_warning "✗ crates.io: ${PROJECT_NAME}-core $version not yet available"
        return 1
    fi
    
    # Check PyPI
    if curl -s "https://pypi.org/pypi/${PROJECT_NAME}-python/json" | grep -q "\"version\":\"$version\""; then
        print_success "✓ PyPI: ${PROJECT_NAME}-python $version"
    else
        print_warning "✗ PyPI: ${PROJECT_NAME}-python $version not yet available"
        return 1
    fi
    
    # Check npm
    if curl -s "https://registry.npmjs.org/${PROJECT_NAME}-js" | grep -q "\"$version\""; then
        print_success "✓ npm: ${PROJECT_NAME}-js $version"
    else
        print_warning "✗ npm: ${PROJECT_NAME}-js $version not yet available"
        return 1
    fi
    
    return 0
}

# Main monitoring function
main() {
    echo "==================== Release Monitoring ===================="
    echo "Target version: $VERSION"
    echo "Current time: $(date)"
    echo ""
    
    # Phase 1: Wait for Act1 to complete
    print_info "Phase 1: Waiting for Act1 to complete..."
    local max_wait=1200  # 20 minutes
    local wait_interval=30
    local elapsed=0
    
    while [ $elapsed -lt $max_wait ]; do
        set +e
        check_act1 "$VERSION"
        local act1_code=$?
        set -e
        
        if [ $act1_code -eq 0 ]; then
            print_success "Act1 completed successfully!"
            break
        elif [ $act1_code -eq 2 ]; then
            print_error "Act1 failed. Release cannot continue."
            exit 1
        else
            print_info "Act1 still running... waiting ${wait_interval}s (elapsed: ${elapsed}s)"
            sleep $wait_interval
            elapsed=$((elapsed + wait_interval))
        fi
    done
    
    if [ $elapsed -ge $max_wait ]; then
        print_error "Timeout waiting for Act1 to complete"
        exit 1
    fi
    
    echo ""
    
    # Phase 2: Wait for Act2 to start and complete
    print_info "Phase 2: Waiting for Act2 to start and complete..."
    print_info "Note: Act2 starts automatically after Act1 succeeds"
    
    # Give Act2 some time to start
    sleep 60
    
    elapsed=0
    while [ $elapsed -lt $max_wait ]; do
        set +e
        check_act2 "$VERSION"
        local act2_code=$?
        set -e
        
        if [ $act2_code -eq 0 ]; then
            print_success "Act2 completed successfully!"
            break
        elif [ $act2_code -eq 2 ]; then
            print_error "Act2 failed. Release is incomplete."
            exit 1
        else
            print_info "Act2 still running or not started... waiting ${wait_interval}s (elapsed: ${elapsed}s)"
            sleep $wait_interval
            elapsed=$((elapsed + wait_interval))
        fi
    done
    
    if [ $elapsed -ge $max_wait ]; then
        print_error "Timeout waiting for Act2 to complete"
        exit 1
    fi
    
    echo ""
    
    # Phase 3: Verify package availability
    print_info "Phase 3: Verifying package availability..."
    
    # Wait for packages to propagate
    sleep 120
    
    local package_attempts=0
    local max_package_attempts=10
    
    while [ $package_attempts -lt $max_package_attempts ]; do
        if check_packages; then
            break
        else
            print_info "Packages not yet available, waiting 60s... (attempt $((package_attempts + 1))/$max_package_attempts)"
            sleep 60
            package_attempts=$((package_attempts + 1))
        fi
    done
    
    if [ $package_attempts -ge $max_package_attempts ]; then
        print_warning "Some packages may not be available yet, but workflows completed"
    fi
    
    echo ""
    echo "==================== Release Complete ===================="
    print_success "🎉 Release $VERSION completed successfully!"
    echo ""
    print_info "Release information:"
    echo "  - GitHub: $(git remote get-url origin | sed 's/\.git$//')/releases/tag/$VERSION"
    echo "  - crates.io: https://crates.io/crates/${PROJECT_NAME}-core"
    echo "  - PyPI: https://pypi.org/project/${PROJECT_NAME}-python/"
    echo "  - npm: https://www.npmjs.com/package/${PROJECT_NAME}-js"
    
    return 0
}

# Run main function
main "$@"