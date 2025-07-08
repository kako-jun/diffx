#!/bin/bash

# diffx Python package publishing script
# Based on lawkit's publishing workflow patterns

set -e  # Exit on any error

echo "Starting diffx Python package publishing..."

# Configuration
PACKAGE_DIR="diffx-python"
TEST_PYPI="https://test.pypi.org/legacy/"
PYPI="https://upload.pypi.org/legacy/"

# Check if we're in the correct directory
if [ ! -d "$PACKAGE_DIR" ]; then
    echo "ERROR: $PACKAGE_DIR directory not found. Run this script from the project root."
    exit 1
fi

cd "$PACKAGE_DIR"

echo "Working in $PACKAGE_DIR directory"

# Check if twine is installed
if ! command -v twine &> /dev/null; then
    echo "ERROR: twine is not installed. Please install with 'pip install twine'"
    exit 1
fi

# Check PyPI authentication
echo "Checking PyPI authentication..."
if [ ! -f ~/.pypirc ] && [ -z "$TWINE_USERNAME" ]; then
    echo "ERROR: PyPI credentials not found. Please configure ~/.pypirc or set TWINE_USERNAME/TWINE_PASSWORD"
    exit 1
fi

echo "PyPI authentication configured"

# Get current package version
CURRENT_VERSION=$(python -c "import tomli; print(tomli.load(open('pyproject.toml', 'rb'))['project']['version'])" 2>/dev/null || echo "unknown")
if [ "$CURRENT_VERSION" = "unknown" ]; then
    CURRENT_VERSION=$(python -c "import configparser; p=configparser.ConfigParser(); p.read('setup.cfg'); print(p['metadata']['version'])" 2>/dev/null || echo "0.0.0")
fi

echo "Current package version: $CURRENT_VERSION"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf dist/ build/ *.egg-info/

# Install build dependencies
echo "Installing build dependencies..."
python -m pip install --upgrade build twine

# Run tests if available
if [ -f "tests/test_integration.py" ] || [ -f "test_integration.py" ]; then
    echo "Running tests..."
    python -m pytest tests/ || python -m pytest . || echo "WARNING: Tests failed or not configured properly"
else
    echo "WARNING: No test files found, skipping tests"
fi

# Build package
echo "Building package..."
python -m build

# Check package
echo "Checking package integrity..."
twine check dist/*

# Verify package contents
echo "Package contents:"
ls -la dist/

# Test installation locally
echo "Testing local installation..."
pip install dist/*.whl --force-reinstall --quiet
if command -v diffx-python &> /dev/null; then
    diffx-python --help || echo "WARNING: Package installed but CLI not working properly"
else
    echo "WARNING: CLI command not available after installation"
fi

# Final confirmation
echo ""
echo "Package Details:"
echo "   Name: diffx-python"
echo "   Version: $CURRENT_VERSION"
echo "   Files: $(ls dist/)"
echo ""

# Ask about test PyPI first
read -p "Upload to test PyPI first? (Y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    echo "Uploading to test PyPI..."
    twine upload --repository-url $TEST_PYPI dist/*
    echo "Uploaded to test PyPI: https://test.pypi.org/project/diffx-python/"
    echo "Test with: pip install -i https://test.pypi.org/simple/ diffx-python"
    echo ""
fi

read -p "Ready to publish diffx-python@$CURRENT_VERSION to PyPI? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Publishing to PyPI..."
    twine upload dist/*
    
    echo ""
    echo "Successfully published diffx-python@$CURRENT_VERSION!"
    echo "Verify at: https://pypi.org/project/diffx-python/"
    echo "Install with: pip install diffx-python"
else
    echo "Publishing cancelled"
    exit 1
fi

cd ..
echo "Returning to project root"
echo "PyPI publishing complete!"