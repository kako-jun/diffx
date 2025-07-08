#!/bin/bash

# diffx npm package publishing script
# Based on lawkit's publishing workflow

set -e  # Exit on any error

echo "Starting diffx npm package publishing..."

# Configuration
PACKAGE_DIR="diffx-npm"
REGISTRY="https://registry.npmjs.org"

# Check if we're in the correct directory
if [ ! -d "$PACKAGE_DIR" ]; then
    echo "ERROR: $PACKAGE_DIR directory not found. Run this script from the project root."
    exit 1
fi

cd "$PACKAGE_DIR"

echo "Working in $PACKAGE_DIR directory"

# Check npm authentication
echo "Checking npm authentication..."
if ! npm whoami >/dev/null 2>&1; then
    echo "ERROR: Not logged in to npm. Please run 'npm login' first."
    exit 1
fi

echo "npm authentication verified"

# Get current package version
CURRENT_VERSION=$(node -p "require('./package.json').version")
echo "Current package version: $CURRENT_VERSION"

# Clean and fresh install
echo "Cleaning node_modules and package-lock.json..."
rm -rf node_modules package-lock.json

echo "Installing dependencies..."
npm install

# Run tests if available
if npm run test --silent >/dev/null 2>&1; then
    echo "Running tests..."
    npm run test
else
    echo "WARNING: No test script found, skipping tests"
fi

# Dry run pack to check what will be included
echo "Running dry-run pack to verify package contents..."
npm pack --dry-run

# Verify package can be installed
echo "Verifying package installation..."
if npm run verify --silent >/dev/null 2>&1; then
    npm run verify
else
    echo "WARNING: No verify script found, skipping verification"
fi

# Final confirmation
echo ""
echo "Package Details:"
echo "   Name: $(node -p "require('./package.json').name")"
echo "   Version: $CURRENT_VERSION"
echo "   Registry: $REGISTRY"
echo ""

read -p "Ready to publish diffx-js@$CURRENT_VERSION to npm? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Publishing to npm..."
    npm publish --registry=$REGISTRY
    
    echo ""
    echo "Successfully published diffx-js@$CURRENT_VERSION!"
    echo "Verify at: https://www.npmjs.com/package/diffx-js"
    echo "Install with: npm install diffx-js"
else
    echo "Publishing cancelled"
    exit 1
fi

cd ..
echo "Returning to project root"
echo "npm publishing complete!"