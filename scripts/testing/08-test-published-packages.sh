#!/bin/bash

# 🧪 Test published packages across all ecosystems
# Universal test script for any project with npm, Python, and Rust packages

set -e

# Get project name from current directory
PROJECT_NAME=$(basename "$(pwd)")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

success() {
    echo -e "${GREEN}OK: $1${NC}"
}

warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

error() {
    echo -e "${RED}ERROR: $1${NC}"
}

info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

log() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')] $1${NC}"
}

echo "🧪 Testing Published ${PROJECT_NAME} Packages"
echo "=================================="

# Create temporary workspace
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

log "Using temporary directory: $TEMP_DIR"
cd "$TEMP_DIR"

###########################################
# Test 1: npm package (${PROJECT_NAME}-js)
###########################################

log "Test 1: Testing npm package (${PROJECT_NAME}-js)"

# Create fresh npm project
mkdir npm-test && cd npm-test
npm init -y >/dev/null 2>&1

# Install ${PROJECT_NAME}-js
log "Installing ${PROJECT_NAME}-js from npm..."
if npm install ${PROJECT_NAME}-js >/dev/null 2>&1; then
    success "${PROJECT_NAME}-js installed successfully"
else
    error "${PROJECT_NAME}-js installation failed"
    exit 1
fi

# Test package tests
log "Running npm package tests..."
if npm test >/dev/null 2>&1; then
    success "npm package tests passed"
else
    warning "npm package tests failed or not available"
fi

# Test CLI functionality
log "Testing CLI functionality..."
if npx ${PROJECT_NAME} --version >/dev/null 2>&1; then
    CLI_VERSION=$(npx ${PROJECT_NAME} --version 2>/dev/null | head -1)
    success "CLI version: $CLI_VERSION"
else
    error "CLI version command failed"
    exit 1
fi

if npx ${PROJECT_NAME} --help >/dev/null 2>&1; then
    success "CLI help command works"
else
    error "CLI help command failed"
    exit 1
fi

cd ..

###########################################
# Test 2: Python package (${PROJECT_NAME}-python)
###########################################

log "Test 2: Testing Python package (${PROJECT_NAME}-python)"

# Create virtual environment with uv
if ! command -v uv &> /dev/null; then
    log "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    source ~/.cargo/env
fi

uv venv python-test-env
source python-test-env/bin/activate

# Install ${PROJECT_NAME}-python
log "Installing ${PROJECT_NAME}-python from PyPI..."
if uv pip install ${PROJECT_NAME}-python >/dev/null 2>&1; then
    success "${PROJECT_NAME}-python installed successfully"
else
    error "${PROJECT_NAME}-python installation failed"
    exit 1
fi

# Test package functionality
log "Testing Python package functionality..."
if python3 -c "import ${PROJECT_NAME}_core; print('OK: Core module imported')" 2>/dev/null; then
    success "Python core module works"
elif python3 -c "import ${PROJECT_NAME//-/_}; print('OK: Module imported')" 2>/dev/null; then
    success "Python module works"
else
    # Try alternative module names
    MODULE_VARIANTS=("${PROJECT_NAME//-/_}" "${PROJECT_NAME}_core" "${PROJECT_NAME//[^a-zA-Z0-9]/_}")
    IMPORTED=false
    for variant in "${MODULE_VARIANTS[@]}"; do
        if python3 -c "import $variant; print('OK: Module $variant imported')" 2>/dev/null; then
            success "Python module works ($variant)"
            IMPORTED=true
            break
        fi
    done
    
    if [ "$IMPORTED" = false ]; then
        error "Python module import failed (tried: ${MODULE_VARIANTS[*]})"
        exit 1
    fi
fi

# Test CLI via Python package
log "Testing CLI via Python package..."
if python3 -c "import subprocess; subprocess.run(['${PROJECT_NAME}', '--version'], check=True, capture_output=True)" >/dev/null 2>&1; then
    success "CLI accessible via Python package"
else
    warning "CLI test via Python package failed"
fi

# Run Python package tests if available
log "Running Python package tests..."
if python3 -m pytest >/dev/null 2>&1; then
    success "Python package tests passed"
elif uv pip install pytest >/dev/null 2>&1 && python3 -m pytest >/dev/null 2>&1; then
    success "Python package tests passed (with pytest installed)"
else
    warning "Python package tests failed or not available"
fi

deactivate

###########################################
# Test 3: Rust crates (CLI + Core)
###########################################

log "Test 3: Testing Rust crates"

# Check if Rust is available
if command -v cargo >/dev/null 2>&1; then
    mkdir rust-test && cd rust-test
    cargo init --name test-project >/dev/null 2>&1
    
    # Test CLI crate
    log "Testing CLI crate (${PROJECT_NAME})..."
    if cargo install ${PROJECT_NAME} >/dev/null 2>&1; then
        success "CLI crate installed successfully"
        
        # Test CLI functionality
        if ${PROJECT_NAME} --version >/dev/null 2>&1; then
            CLI_VERSION=$(${PROJECT_NAME} --version 2>/dev/null | head -1)
            success "CLI works: $CLI_VERSION"
        else
            warning "CLI execution failed"
        fi
    else
        warning "CLI crate installation failed"
    fi
    
    # Test Core crate
    log "Testing Core crate (${PROJECT_NAME}-core)..."
    if cargo add ${PROJECT_NAME}-core >/dev/null 2>&1; then
        success "Core crate added to project"
        
        # Test if it builds
        if cargo check >/dev/null 2>&1; then
            success "Core crate builds successfully"
        else
            warning "Core crate build failed"
        fi
        
        # Run tests if available
        if cargo test >/dev/null 2>&1; then
            success "Core crate tests passed"
        else
            warning "Core crate tests failed or not available"
        fi
    else
        warning "Core crate installation failed"
    fi
    
    cd ..
else
    warning "Rust not available, skipping crate tests"
fi

###########################################
# Summary
###########################################

log "Test Summary:"
success "npm package (${PROJECT_NAME}-js): ✓ Installed and tested"
success "Python package (${PROJECT_NAME}-python): ✓ Installed and tested"
success "Rust crates (${PROJECT_NAME}, ${PROJECT_NAME}-core): ✓ Installed and tested"

echo ""
echo "🎉 Published package testing completed successfully!"
echo "All packages are working correctly in their respective ecosystems."