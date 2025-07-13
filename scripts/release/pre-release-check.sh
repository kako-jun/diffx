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
    echo -e "${BLUE}[CHECK]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Track errors
ERRORS=0

# Function to check git status
check_git_status() {
    print_info "Checking git status..."
    
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory has uncommitted changes"
        ((ERRORS++))
    else
        print_success "Working directory is clean"
    fi
    
    # Check for untracked files
    if [ -n "$(git ls-files --others --exclude-standard)" ]; then
        print_warning "Untracked files present (this might be okay)"
    fi
}

# Function to check branch
check_branch() {
    print_info "Checking current branch..."
    
    CURRENT_BRANCH=$(git branch --show-current)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        print_error "Not on main branch (current: $CURRENT_BRANCH)"
        ((ERRORS++))
    else
        print_success "On main branch"
    fi
    
    # Check if up to date with remote
    git fetch origin main --quiet
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})
    
    if [ "$LOCAL" != "$REMOTE" ]; then
        print_error "Branch is not up to date with remote"
        ((ERRORS++))
    else
        print_success "Branch is up to date with remote"
    fi
    
    # Check for unmerged branches
    print_info "Checking for unmerged branches..."
    UNMERGED_BRANCHES=$(git branch --no-merged main 2>/dev/null | grep -v "^\*" | wc -l)
    if [ "$UNMERGED_BRANCHES" -gt 0 ]; then
        print_warning "Found $UNMERGED_BRANCHES unmerged branches"
        git branch --no-merged main | grep -v "^\*" | sed 's/^/  /'
    else
        print_success "All branches are merged into main"
    fi
}

# Function to check version consistency
check_versions() {
    print_info "Checking version consistency..."
    
    # Get versions from different files (simplified to avoid hanging)
    CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    PYTHON_VERSION=$(grep '^version = ' diffx-python/pyproject.toml | head -1 | cut -d'"' -f2)
    NPM_VERSION=$(node -pe "require('./diffx-npm/package.json').version" 2>/dev/null || echo "unknown")
    
    print_info "Found versions:"
    echo "  - Cargo workspace: $CARGO_VERSION"  
    echo "  - diffx-python: $PYTHON_VERSION"
    echo "  - diffx-npm: $NPM_VERSION"
    
    # Check if all versions match (core/cli use workspace version)
    if [ "$CARGO_VERSION" = "$PYTHON_VERSION" ] && [ "$CARGO_VERSION" = "$NPM_VERSION" ]; then
        print_success "All versions are consistent: $CARGO_VERSION"
    else
        print_error "Version mismatch: Cargo=$CARGO_VERSION, Python=$PYTHON_VERSION, npm=$NPM_VERSION"
        ((ERRORS++))
    fi
}

# Function to check dependencies
check_dependencies() {
    print_info "Checking required tools..."
    
    # Check for required commands
    REQUIRED_COMMANDS=("cargo" "npm" "maturin" "git")
    
    for cmd in "${REQUIRED_COMMANDS[@]}"; do
        if command -v "$cmd" &> /dev/null; then
            print_success "$cmd is installed"
        else
            print_error "$cmd is not installed"
            ((ERRORS++))
        fi
    done
    
    # Check Python
    if command -v python3 &> /dev/null || command -v python &> /dev/null; then
        print_success "Python is installed"
    else
        print_error "Python is not installed"
        ((ERRORS++))
    fi
}

# Function to check GitHub issues and PRs
check_github_status() {
    print_info "Checking GitHub issues and PRs..."
    
    # Check if gh CLI is available
    if ! command -v gh &> /dev/null; then
        print_warning "GitHub CLI (gh) not found. Skipping GitHub checks."
        return
    fi
    
    # Check open issues
    OPEN_ISSUES=$(gh issue list --state open --json number 2>/dev/null | jq length 2>/dev/null || echo "unknown")
    if [ "$OPEN_ISSUES" != "unknown" ]; then
        if [ "$OPEN_ISSUES" -gt 0 ]; then
            print_warning "Found $OPEN_ISSUES open issues"
        else
            print_success "No open issues"
        fi
    fi
    
    # Check open PRs
    OPEN_PRS=$(gh pr list --state open --json number 2>/dev/null | jq length 2>/dev/null || echo "unknown")
    if [ "$OPEN_PRS" != "unknown" ]; then
        if [ "$OPEN_PRS" -gt 0 ]; then
            print_warning "Found $OPEN_PRS open PRs"
        else
            print_success "No open PRs"
        fi
    fi
}

# Function to check credentials
check_credentials() {
    print_info "Checking publishing credentials..."
    
    # Check crates.io token
    if [ -f "$HOME/.cargo/credentials.toml" ] || [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
        print_success "Cargo credentials found"
    else
        print_warning "Cargo credentials not found (might be using cargo login)"
    fi
    
    # Check npm login
    if npm whoami &> /dev/null; then
        NPM_USER=$(npm whoami)
        print_success "npm logged in as: $NPM_USER"
    else
        print_error "Not logged in to npm"
        ((ERRORS++))
    fi
    
    # Check PyPI token
    if [ -f "$HOME/.pypirc" ] || [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
        print_success "PyPI credentials found"
    else
        print_warning "PyPI credentials not found (might be configured elsewhere)"
    fi
}

# Function to check package contents
check_package_contents() {
    print_info "Checking package contents..."
    
    # Check for required files
    REQUIRED_FILES=(
        "README.md"
        "LICENSE"
        "CHANGELOG.md"
        "Cargo.toml"
        "diffx-core/Cargo.toml"
        "diffx-cli/Cargo.toml"
        "diffx-python/pyproject.toml"
        "diffx-npm/package.json"
    )
    
    for file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$file" ]; then
            print_success "$file exists"
        else
            print_error "$file is missing"
            ((ERRORS++))
        fi
    done
    
    # Check Python package structure
    if [ -d "diffx-python/src/diffx" ]; then
        print_success "Python package structure is correct"
    else
        print_error "Python package structure is incorrect"
        ((ERRORS++))
    fi
}

# Function to run basic tests
check_tests() {
    print_info "Running basic tests..."
    
    # Run Rust tests
    print_info "Running Rust tests..."
    if cargo test --quiet; then
        print_success "Rust tests passed"
    else
        print_error "Rust tests failed"
        ((ERRORS++))
    fi
    
    # Check if CI script exists and is executable
    if [ -x "./scripts/ci-local.sh" ]; then
        print_success "CI local script is executable"
    else
        print_error "CI local script is not executable"
        ((ERRORS++))
    fi
}

# Function to check for common issues
check_common_issues() {
    print_info "Checking for common issues..."
    
    # Check for TODO/FIXME in code
    TODO_COUNT=$(grep -r "TODO\|FIXME" --include="*.rs" --include="*.py" --include="*.js" . 2>/dev/null | wc -l || echo "0")
    if [ "$TODO_COUNT" -gt 0 ]; then
        print_warning "Found $TODO_COUNT TODO/FIXME comments in code"
    else
        print_success "No TODO/FIXME comments found"
    fi
    
    # Check for debug prints
    DEBUG_COUNT=$(grep -r "dbg!\|println!\|console\.log" --include="*.rs" --include="*.js" . 2>/dev/null | grep -v "test" | wc -l || echo "0")
    if [ "$DEBUG_COUNT" -gt 0 ]; then
        print_warning "Found $DEBUG_COUNT potential debug statements"
    else
        print_success "No debug statements found"
    fi
    
    # Check Cargo.lock is committed
    if [ -f "Cargo.lock" ] && git ls-files --error-unmatch Cargo.lock &> /dev/null; then
        print_success "Cargo.lock is tracked"
    else
        print_error "Cargo.lock is not tracked"
        ((ERRORS++))
    fi
}

# Main function
main() {
    echo "==================== Pre-Release Check ===================="
    echo ""
    
    check_git_status
    echo ""
    
    check_branch
    echo ""
    
    check_github_status
    echo ""
    
    check_versions
    echo ""
    
    check_dependencies
    echo ""
    
    check_credentials
    echo ""
    
    check_package_contents
    echo ""
    
    check_tests
    echo ""
    
    check_common_issues
    echo ""
    
    # Validate dynamic version handling
    print_info "Validating dynamic version handling..."
    if ./scripts/release/validate-dynamic-versions.sh; then
        print_success "Dynamic version validation passed"
    else
        print_error "Dynamic version validation failed"
        ((ERRORS++))
    fi
    echo ""
    
    echo "=========================================================="
    
    if [ $ERRORS -eq 0 ]; then
        print_success "All checks passed! Ready for release."
        exit 0
    else
        print_error "Found $ERRORS error(s). Please fix before releasing."
        exit 1
    fi
}

# Run main function
main "$@"