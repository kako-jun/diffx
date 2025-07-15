#!/usr/bin/env bash
set -euo pipefail

# Comprehensive pre-release check
# Validates environment, tools, authentication, and git state before release

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

# Track errors and warnings
ERRORS=0
WARNINGS=0

# Function to check git status
check_git_status() {
    print_info "Checking git status..."
    
    if ! git diff-index --quiet HEAD --; then
        print_error "Working directory has uncommitted changes"
        ERRORS=$((ERRORS + 1))
    else
        print_success "Working directory is clean"
    fi
    
    # Check for untracked files
    if [ -n "$(git ls-files --others --exclude-standard)" ]; then
        print_warning "Untracked files present (this might be okay)"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to check branch
check_branch() {
    print_info "Checking current branch..."
    
    CURRENT_BRANCH=$(git branch --show-current)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        print_error "Not on main branch (current: $CURRENT_BRANCH)"
        ERRORS=$((ERRORS + 1))
    else
        print_success "On main branch"
    fi
    
    # Check if up to date with remote
    git fetch origin main --quiet
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})
    
    if [ "$LOCAL" != "$REMOTE" ]; then
        print_error "Branch is not up to date with remote"
        ERRORS=$((ERRORS + 1))
    else
        print_success "Branch is up to date with remote"
    fi
    
    # Check for unmerged branches
    print_info "Checking for unmerged branches..."
    UNMERGED_BRANCHES=$(git branch --no-merged main 2>/dev/null | grep -v "^\*" | wc -l)
    if [ "$UNMERGED_BRANCHES" -gt 0 ]; then
        print_warning "Found $UNMERGED_BRANCHES unmerged branches"
        git branch --no-merged main | grep -v "^\*" | sed 's/^/  /'
        WARNINGS=$((WARNINGS + 1))
    else
        print_success "All branches are merged into main"
    fi
}

# Function to check GitHub status
check_github_status() {
    print_info "Checking GitHub status..."
    
    # Check if gh CLI is available
    if ! command -v gh &> /dev/null; then
        print_warning "GitHub CLI (gh) not found. Skipping GitHub checks."
        WARNINGS=$((WARNINGS + 1))
        return
    fi
    
    # Check GitHub authentication
    if gh auth status &> /dev/null; then
        print_success "GitHub CLI authenticated"
    else
        print_error "GitHub CLI not authenticated"
        print_info "Run: gh auth login"
        ERRORS=$((ERRORS + 1))
        return
    fi
    
    # Check open issues
    OPEN_ISSUES=$(gh issue list --state open --json number 2>/dev/null | jq length 2>/dev/null || echo "unknown")
    if [ "$OPEN_ISSUES" != "unknown" ]; then
        if [ "$OPEN_ISSUES" -gt 0 ]; then
            print_warning "Found $OPEN_ISSUES open issues"
            WARNINGS=$((WARNINGS + 1))
        else
            print_success "No open issues"
        fi
    fi
    
    # Check open PRs
    OPEN_PRS=$(gh pr list --state open --json number 2>/dev/null | jq length 2>/dev/null || echo "unknown")
    if [ "$OPEN_PRS" != "unknown" ]; then
        if [ "$OPEN_PRS" -gt 0 ]; then
            print_warning "Found $OPEN_PRS open PRs"
            WARNINGS=$((WARNINGS + 1))
        else
            print_success "No open PRs"
        fi
    fi
    
    # Check recent CI runs
    RECENT_RUNS=$(gh run list --limit 3 --json status,conclusion | jq -r '.[] | select(.status == "completed") | .conclusion' | head -1)
    if [ "$RECENT_RUNS" = "success" ]; then
        print_success "Recent CI run successful"
    else
        print_warning "Recent CI run not successful or not found"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to check tool versions
check_tool_versions() {
    print_info "Checking required tools..."
    
    # Rust toolchain
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        print_success "Rust: $RUST_VERSION"
    else
        print_error "Rust toolchain not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    if command -v cargo &> /dev/null; then
        CARGO_VERSION=$(cargo --version)
        print_success "Cargo: $CARGO_VERSION"
    else
        print_error "Cargo not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    # Node.js and npm
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        print_success "Node.js: $NODE_VERSION"
    else
        print_error "Node.js not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    if command -v npm &> /dev/null; then
        NPM_VERSION=$(npm --version)
        print_success "npm: $NPM_VERSION"
    else
        print_error "npm not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    # Python
    if command -v python3 &> /dev/null; then
        PYTHON_VERSION=$(python3 --version)
        print_success "Python: $PYTHON_VERSION"
    elif command -v python &> /dev/null; then
        PYTHON_VERSION=$(python --version)
        print_success "Python: $PYTHON_VERSION"
    else
        print_error "Python not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    # uv (Python package manager)
    if command -v uv &> /dev/null; then
        UV_VERSION=$(uv --version)
        print_success "uv: $UV_VERSION"
    else
        print_error "uv not found (required for Python development)"
        ERRORS=$((ERRORS + 1))
    fi
    
    # Git
    if command -v git &> /dev/null; then
        GIT_VERSION=$(git --version)
        print_success "Git: $GIT_VERSION"
    else
        print_error "Git not found"
        ERRORS=$((ERRORS + 1))
    fi
    
    # Check Python virtual environment and maturin
    if [ -d ".venv" ]; then
        print_success "Python virtual environment exists"
        
        # Check if maturin is available
        if command -v maturin &> /dev/null; then
            MATURIN_VERSION=$(maturin --version)
            print_success "Maturin: $MATURIN_VERSION"
        elif [ -f ".venv/bin/maturin" ]; then
            print_success "Maturin available in virtual environment"
        else
            print_info "Installing maturin automatically..."
            if source .venv/bin/activate && uv pip install maturin; then
                print_success "Maturin installed successfully"
            else
                print_error "Failed to install maturin"
                ERRORS=$((ERRORS + 1))
            fi
        fi
    else
        print_info "Creating Python virtual environment and installing maturin..."
        if uv venv && source .venv/bin/activate && uv pip install maturin; then
            print_success "Virtual environment and maturin installed successfully"
        else
            print_error "Failed to create virtual environment or install maturin"
            ERRORS=$((ERRORS + 1))
        fi
    fi
}

# Function to check authentication
check_authentication() {
    print_info "Checking publishing credentials..."
    
    # Cargo/crates.io
    if [ -f "$HOME/.cargo/credentials.toml" ] || [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
        print_success "Cargo credentials configured"
    else
        print_warning "Cargo credentials not found"
        print_info "Run: cargo login"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    # npm
    if npm whoami &> /dev/null; then
        NPM_USER=$(npm whoami)
        print_success "npm authenticated as: $NPM_USER"
    else
        print_error "npm not authenticated"
        print_info "Run: npm login"
        ERRORS=$((ERRORS + 1))
    fi
    
    # PyPI/maturin
    if [ -f "$HOME/.pypirc" ] || [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
        print_success "PyPI credentials configured"
    else
        # Try to verify with pip config
        if command -v pip &> /dev/null && pip config get global.index-url &> /dev/null; then
            print_success "PyPI credentials may be configured via pip"
        else
            print_warning "PyPI credentials not found"
            print_info "Set MATURIN_PYPI_TOKEN or configure ~/.pypirc"
            WARNINGS=$((WARNINGS + 1))
        fi
    fi
}

# Function to check external service connectivity
check_external_services() {
    print_info "Checking external service connectivity..."
    
    # crates.io
    if curl -s --max-time 10 https://crates.io &> /dev/null; then
        print_success "crates.io accessible"
    else
        print_warning "crates.io connection issues"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    # PyPI
    if curl -s --max-time 10 https://pypi.org &> /dev/null; then
        print_success "PyPI accessible"
    else
        print_warning "PyPI connection issues"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    # npm registry
    if curl -s --max-time 10 https://registry.npmjs.org &> /dev/null; then
        print_success "npm registry accessible"
    else
        print_warning "npm registry connection issues"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    # GitHub API
    if curl -s --max-time 10 https://api.github.com &> /dev/null; then
        print_success "GitHub API accessible"
    else
        print_warning "GitHub API connection issues"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to check security
check_security() {
    print_info "Checking security..."
    
    # Check for sensitive environment variables
    if [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
        print_info "CARGO_REGISTRY_TOKEN is set"
    fi
    
    if [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
        print_info "MATURIN_PYPI_TOKEN is set"
    fi
    
    # cargo-audit (optional)
    if command -v cargo &> /dev/null && command -v cargo-audit &> /dev/null; then
        print_info "Running cargo audit..."
        if cargo audit --quiet; then
            print_success "No Rust vulnerabilities found"
        else
            print_warning "Rust vulnerabilities detected"
            WARNINGS=$((WARNINGS + 1))
        fi
    else
        print_info "cargo-audit not installed (optional)"
    fi
    
    # npm audit (only high severity)
    if command -v npm &> /dev/null && [ -f "$PROJECT_ROOT/${PROJECT_NAME}-npm/package-lock.json" ]; then
        print_info "Running npm audit..."
        cd "$PROJECT_ROOT/${PROJECT_NAME}-npm"
        if npm audit --audit-level=high &> /dev/null; then
            print_success "No critical npm vulnerabilities"
        else
            print_warning "npm vulnerabilities detected"
            WARNINGS=$((WARNINGS + 1))
        fi
        cd "$PROJECT_ROOT"
    fi
}

# Function to check timing
check_timing() {
    print_info "Checking release timing..."
    
    CURRENT_HOUR=$(date +%H)
    CURRENT_DAY=$(date +%u)  # 1=Monday, 7=Sunday
    
    # Recommend business hours on weekdays
    if [ "$CURRENT_DAY" -ge 1 ] && [ "$CURRENT_DAY" -le 5 ]; then
        if [ "$CURRENT_HOUR" -ge 10 ] && [ "$CURRENT_HOUR" -le 15 ]; then
            print_success "Good timing: Business hours on weekday"
        else
            print_warning "Consider releasing during business hours (10:00-15:00)"
            WARNINGS=$((WARNINGS + 1))
        fi
    else
        print_warning "Consider releasing on weekdays for better support availability"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to check package structure
check_package_structure() {
    print_info "Checking package structure..."
    
    # Check for required files
    REQUIRED_FILES=(
        "README.md"
        "LICENSE"
        "CHANGELOG.md"
        "Cargo.toml"
        "${PROJECT_NAME}-core/Cargo.toml"
        "${PROJECT_NAME}-cli/Cargo.toml"
        "${PROJECT_NAME}-python/pyproject.toml"
        "${PROJECT_NAME}-npm/package.json"
    )
    
    for file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$PROJECT_ROOT/$file" ]; then
            print_success "$file exists"
        else
            print_error "$file is missing"
            ERRORS=$((ERRORS + 1))
        fi
    done
    
    # Check Python package structure
    if [ -d "$PROJECT_ROOT/${PROJECT_NAME}-python/src/${PROJECT_NAME}" ]; then
        print_success "Python package structure is correct"
    else
        print_error "Python package structure is incorrect"
        ERRORS=$((ERRORS + 1))
    fi
    
    # Check npm package structure
    if [ -f "$PROJECT_ROOT/${PROJECT_NAME}-npm/index.js" ]; then
        print_success "npm package structure is correct"
    else
        print_error "npm package structure is incorrect"
        ERRORS=$((ERRORS + 1))
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
    
    check_tool_versions
    echo ""
    
    check_authentication
    echo ""
    
    check_external_services
    echo ""
    
    check_security
    echo ""
    
    check_timing
    echo ""
    
    check_package_structure
    echo ""
    
    echo "=========================================================="
    
    if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
        print_success "✅ All checks passed! Ready for release."
        exit 0
    elif [ $ERRORS -eq 0 ]; then
        print_warning "⚠️  Environment ready with $WARNINGS warning(s). Consider addressing warnings."
        exit 0
    else
        print_error "❌ Found $ERRORS error(s) and $WARNINGS warning(s). Fix errors before releasing."
        echo ""
        print_info "Common fixes:"
        echo "  - Commit changes: git add . && git commit -m 'message'"
        echo "  - Update branch: git pull origin main"
        echo "  - Authenticate: cargo login, npm login, gh auth login"
        echo "  - Install tools: rustup, node, python, uv, maturin"
        exit 1
    fi
}

# Run main function
main "$@"