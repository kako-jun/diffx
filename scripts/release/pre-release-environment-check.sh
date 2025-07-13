#!/usr/bin/env bash
set -euo pipefail

# Pre-release environment and dependency check
# Ensures all tools and authentication are properly configured

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

WARNINGS=0
ERRORS=0

print_info "Checking release environment and dependencies..."
echo ""

# Function to check tool versions
check_tool_versions() {
    print_info "Checking tool versions..."
    
    # Rust toolchain
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        print_success "Rust: $RUST_VERSION"
    else
        print_error "Rust toolchain not found"
        ((ERRORS++))
    fi
    
    if command -v cargo &> /dev/null; then
        CARGO_VERSION=$(cargo --version)
        print_success "Cargo: $CARGO_VERSION"
    else
        print_error "Cargo not found"
        ((ERRORS++))
    fi
    
    # Node.js and npm
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        print_success "Node.js: $NODE_VERSION"
    else
        print_error "Node.js not found"
        ((ERRORS++))
    fi
    
    if command -v npm &> /dev/null; then
        NPM_VERSION=$(npm --version)
        print_success "npm: $NPM_VERSION"
    else
        print_error "npm not found"
        ((ERRORS++))
    fi
    
    # Python and maturin
    if command -v python3 &> /dev/null; then
        PYTHON_VERSION=$(python3 --version)
        print_success "Python: $PYTHON_VERSION"
    elif command -v python &> /dev/null; then
        PYTHON_VERSION=$(python --version)
        print_success "Python: $PYTHON_VERSION"
    else
        print_error "Python not found"
        ((ERRORS++))
    fi
    
    if command -v maturin &> /dev/null; then
        MATURIN_VERSION=$(maturin --version)
        print_success "Maturin: $MATURIN_VERSION"
    else
        print_error "Maturin not found (required for Python package)"
        ((ERRORS++))
    fi
    
    # Git and GitHub CLI
    if command -v git &> /dev/null; then
        GIT_VERSION=$(git --version)
        print_success "Git: $GIT_VERSION"
    else
        print_error "Git not found"
        ((ERRORS++))
    fi
    
    if command -v gh &> /dev/null; then
        GH_VERSION=$(gh --version | head -1)
        print_success "GitHub CLI: $GH_VERSION"
    else
        print_error "GitHub CLI not found"
        ((ERRORS++))
    fi
}

# Function to check authentication
check_authentication() {
    print_info "Checking authentication..."
    
    # Cargo/crates.io
    if [ -f "$HOME/.cargo/credentials.toml" ] || [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
        print_success "Cargo credentials configured"
    else
        print_warning "Cargo credentials not found"
        print_info "Run: cargo login"
        ((WARNINGS++))
    fi
    
    # npm
    if npm whoami &> /dev/null; then
        NPM_USER=$(npm whoami)
        print_success "npm authenticated as: $NPM_USER"
    else
        print_error "npm not authenticated"
        print_info "Run: npm login"
        ((ERRORS++))
    fi
    
    # PyPI/maturin
    if [ -f "$HOME/.pypirc" ] || [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
        print_success "PyPI credentials configured"
    else
        print_warning "PyPI credentials not found"
        print_info "Set MATURIN_PYPI_TOKEN or configure ~/.pypirc"
        ((WARNINGS++))
    fi
    
    # GitHub
    if gh auth status &> /dev/null; then
        print_success "GitHub CLI authenticated"
    else
        print_error "GitHub CLI not authenticated"
        print_info "Run: gh auth login"
        ((ERRORS++))
    fi
}

# Function to check network and external services
check_external_services() {
    print_info "Checking external service connectivity..."
    
    # crates.io
    if curl -s --max-time 10 https://crates.io &> /dev/null; then
        print_success "crates.io accessible"
    else
        print_warning "crates.io connection issues"
        ((WARNINGS++))
    fi
    
    # PyPI
    if curl -s --max-time 10 https://pypi.org &> /dev/null; then
        print_success "PyPI accessible"
    else
        print_warning "PyPI connection issues"
        ((WARNINGS++))
    fi
    
    # npm registry
    if curl -s --max-time 10 https://registry.npmjs.org &> /dev/null; then
        print_success "npm registry accessible"
    else
        print_warning "npm registry connection issues"
        ((WARNINGS++))
    fi
    
    # GitHub
    if curl -s --max-time 10 https://api.github.com &> /dev/null; then
        print_success "GitHub API accessible"
    else
        print_warning "GitHub API connection issues"
        ((WARNINGS++))
    fi
}

# Function to check git repository state
check_git_state() {
    print_info "Checking git repository state..."
    
    # Check if on main branch
    CURRENT_BRANCH=$(git branch --show-current)
    if [ "$CURRENT_BRANCH" = "main" ]; then
        print_success "On main branch"
    else
        print_error "Not on main branch (current: $CURRENT_BRANCH)"
        ((ERRORS++))
    fi
    
    # Check for uncommitted changes
    if git diff-index --quiet HEAD --; then
        print_success "No uncommitted changes"
    else
        print_error "Uncommitted changes detected"
        ((ERRORS++))
    fi
    
    # Check remote status
    git fetch origin main --quiet
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})
    
    if [ "$LOCAL" = "$REMOTE" ]; then
        print_success "Up to date with remote"
    else
        print_error "Not up to date with remote"
        ((ERRORS++))
    fi
    
    # Check for recent successful CI runs
    if command -v gh &> /dev/null; then
        RECENT_RUNS=$(gh run list --limit 3 --json status,conclusion | jq -r '.[] | select(.status == "completed") | .conclusion' | head -1)
        if [ "$RECENT_RUNS" = "success" ]; then
            print_success "Recent CI run successful"
        else
            print_warning "Recent CI run not successful or not found"
            ((WARNINGS++))
        fi
    fi
}

# Function to check security
check_security() {
    print_info "Checking security and vulnerabilities..."
    
    # Cargo audit
    if command -v cargo &> /dev/null && command -v cargo-audit &> /dev/null; then
        if cargo audit --quiet; then
            print_success "No Rust vulnerabilities found"
        else
            print_warning "Rust vulnerabilities detected"
            ((WARNINGS++))
        fi
    else
        print_info "cargo-audit not installed (optional)"
    fi
    
    # npm audit
    if command -v npm &> /dev/null; then
        if npm audit --audit-level=high &> /dev/null; then
            print_success "No critical npm vulnerabilities"
        else
            print_warning "npm vulnerabilities detected"
            ((WARNINGS++))
        fi
    fi
    
    # Check for secrets in environment
    if [ -n "${CARGO_REGISTRY_TOKEN:-}" ]; then
        print_info "CARGO_REGISTRY_TOKEN is set"
    fi
    
    if [ -n "${MATURIN_PYPI_TOKEN:-}" ]; then
        print_info "MATURIN_PYPI_TOKEN is set"
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
            ((WARNINGS++))
        fi
    else
        print_warning "Consider releasing on weekdays for better support availability"
        ((WARNINGS++))
    fi
}

# Run all checks
check_tool_versions
echo ""
check_authentication
echo ""
check_external_services
echo ""
check_git_state
echo ""
check_security
echo ""
check_timing

echo ""
echo "======================================="

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    print_success "✅ All environment checks passed! Ready for release."
elif [ $ERRORS -eq 0 ]; then
    print_warning "⚠️  Environment ready with $WARNINGS warning(s). Consider addressing warnings."
else
    print_error "❌ Found $ERRORS error(s) and $WARNINGS warning(s). Fix errors before releasing."
    echo ""
    print_info "Common fixes:"
    echo "  - Install missing tools: rustup, node, python, maturin, gh"
    echo "  - Authenticate services: cargo login, npm login, gh auth login"
    echo "  - Update git repository: git fetch, git merge"
    exit 1
fi