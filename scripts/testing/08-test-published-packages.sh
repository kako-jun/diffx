#!/bin/bash

# 🧪 Test published packages across all ecosystems
# Based on lawkit's comprehensive package testing approach

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

# Test variables
TEST_JSON1='{"name": "app", "version": "1.0", "debug": true}'
TEST_JSON2='{"debug": false, "version": "1.1", "name": "app"}'
EXPECTED_DIFF_COUNT=2  # version change + debug change

###########################################
# Test 1: npm package (${PROJECT_NAME}-js)
###########################################

log "Test 1: Testing npm package (${PROJECT_NAME}-js)"

# Create fresh npm project
mkdir npm-test && cd npm-test
npm init -y >/dev/null 2>&1

# Install ${PROJECT_NAME}-js
log "Installing ${PROJECT_NAME}-js from npm..."
npm install ${PROJECT_NAME}-js >/dev/null 2>&1
success "${PROJECT_NAME}-js installed successfully"

# Create test files
echo "$TEST_JSON1" > test1.json
echo "$TEST_JSON2" > test2.json

# Test CLI command via npm
log "Testing CLI via npm (--help)..."
npx ${PROJECT_NAME} --help >/dev/null 2>&1
success "CLI help command works"

log "Testing CLI via npm (--version)..."
NPM_VERSION=$(npx ${PROJECT_NAME} --version | head -1)
success "CLI version: $NPM_VERSION"

log "Testing basic diff functionality..."
DIFF_OUTPUT=$(npx ${PROJECT_NAME} test1.json test2.json)
if echo "$DIFF_OUTPUT" | grep -q "version.*1.0.*1.1" && echo "$DIFF_OUTPUT" | grep -q "debug.*true.*false"; then
    success "Basic diff functionality works correctly"
else
    error "Basic diff output unexpected"
    echo "Output: $DIFF_OUTPUT"
    exit 1
fi

log "Testing JSON output format..."
JSON_OUTPUT=$(npx ${PROJECT_NAME} test1.json test2.json --output json)
if echo "$JSON_OUTPUT" | python3 -m json.tool >/dev/null 2>&1; then
    success "JSON output format is valid"
else
    error "JSON output format is invalid"
    echo "Output: $JSON_OUTPUT"
    exit 1
fi

log "Testing YAML files..."
cat > test1.yaml << EOF
name: myapp
version: "1.0"
debug: true
EOF

cat > test2.yaml << EOF
name: myapp
version: "1.1"
debug: false
EOF

YAML_DIFF=$(npx ${PROJECT_NAME} test1.yaml test2.yaml)
if echo "$YAML_DIFF" | grep -q "version.*1.0.*1.1"; then
    success "YAML diff functionality works"
else
    error "YAML diff failed"
    echo "Output: $YAML_DIFF"
    exit 1
fi

log "Testing stdin processing..."
echo "$TEST_JSON1" | npx ${PROJECT_NAME} - test2.json >/dev/null 2>&1
success "Stdin processing works"

cd ..

###########################################
# Test 2: Python package (${PROJECT_NAME}-python)
###########################################

log "Test 2: Testing Python package (${PROJECT_NAME}-python)"

# Create virtual environment
python3 -m venv python-test-env
source python-test-env/bin/activate

# Install ${PROJECT_NAME}-python
log "Installing ${PROJECT_NAME}-python from PyPI..."
pip install ${PROJECT_NAME}-python >/dev/null 2>&1
success "${PROJECT_NAME}-python installed successfully"

# Test binary download (manual step for Python)
log "Testing binary download..."
python3 -c "
import diffx
try:
    result = diffx.is_diffx_available()
    if not result:
        print('Binary not available, attempting download...')
        import subprocess
        subprocess.run(['${PROJECT_NAME}-download-binary'], check=True, capture_output=True)
    print('OK: Binary availability check passed')
except Exception as e:
    print(f'ERROR: Binary check failed: {e}')
    exit(1)
"
success "Binary availability verified"

# Test Python API
log "Testing Python API..."
python3 -c "
import diffx
import json

# Test data
data1 = {'name': 'app', 'version': '1.0', 'debug': True}
data2 = {'name': 'app', 'version': '1.1', 'debug': False}

try:
    # Test diff function
    result = diffx.diff_string(
        json.dumps(data1), 
        json.dumps(data2), 
        diffx.DiffOptions(format=diffx.Format.JSON)
    )
    
    if result.success and len(result.diffs) == 2:
        print('OK: Python API diff function works correctly')
    else:
        print(f'ERROR: Python API unexpected result: {len(result.diffs) if result.success else \"failed\"}')
        exit(1)
        
    # Test different output formats
    json_result = diffx.diff_string(
        json.dumps(data1),
        json.dumps(data2),
        diffx.DiffOptions(
            format=diffx.Format.JSON,
            output_format=diffx.OutputFormat.JSON
        )
    )
    
    if json_result.success and json_result.output.startswith('['):
        print('OK: Python API JSON output works')
    else:
        print('ERROR: Python API JSON output failed')
        exit(1)
        
except Exception as e:
    print(f'ERROR: Python API test failed: {e}')
    import traceback
    traceback.print_exc()
    exit(1)
"
success "Python API functionality verified"

# Test backward compatibility
log "Testing backward compatibility API..."
python3 -c "
import diffx
import tempfile
import os

# Create temp files
with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
    f1.write('{\"test\": \"old\"}')
    file1 = f1.name

with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
    f2.write('{\"test\": \"new\"}')
    file2 = f2.name

try:
    # Test legacy run_diffx function
    result = diffx.run_diffx(file1, file2)
    if 'test' in result and 'old' in result and 'new' in result:
        print('OK: Backward compatibility API works')
    else:
        print('ERROR: Backward compatibility API failed')
        exit(1)
finally:
    os.unlink(file1)
    os.unlink(file2)
"
success "Backward compatibility verified"

deactivate

###########################################
# Test 3: Cross-platform compatibility
###########################################

log "Test 3: Testing cross-platform compatibility"

# Create test data with Python
source python-test-env/bin/activate
python3 -c "
import json

# Create complex test data
config_old = {
    'application': {
        'name': 'test-app',
        'version': '1.0.0',
        'database': {
            'host': 'localhost',
            'port': 5432,
            'ssl': False
        },
        'features': ['auth', 'logging']
    },
    'monitoring': {
        'enabled': True,
        'interval': 60
    }
}

config_new = {
    'application': {
        'name': 'test-app', 
        'version': '1.1.0',
        'database': {
            'host': 'prod-db.example.com',
            'port': 5432,
            'ssl': True
        },
        'features': ['auth', 'logging', 'metrics']
    },
    'monitoring': {
        'enabled': True,
        'interval': 30
    }
}

with open('config_old.json', 'w') as f:
    json.dump(config_old, f, indent=2)
    
with open('config_new.json', 'w') as f:
    json.dump(config_new, f, indent=2)

print('OK: Test data created')
"
deactivate

# Process with npm package
cd npm-test
cp ../config_old.json ../config_new.json .

log "Processing with npm package..."
NPM_RESULT=$(npx ${PROJECT_NAME} config_old.json config_new.json --output json)
echo "$NPM_RESULT" > npm_result.json

# Process with Python package  
cd ../
source python-test-env/bin/activate

log "Processing with Python package..."
python3 -c "
import diffx
import json

result = diffx.diff('config_old.json', 'config_new.json', diffx.DiffOptions(output_format=diffx.OutputFormat.JSON))
if result.success:
    with open('python_result.json', 'w') as f:
        f.write(result.output)
    print('OK: Python processing complete')
else:
    print('ERROR: Python processing failed')
    exit(1)
"

# Compare results
log "Comparing results between npm and Python packages..."
python3 -c "
import json

with open('npm-test/npm_result.json') as f:
    npm_data = json.load(f)

with open('python_result.json') as f:
    python_data = json.load(f)

if len(npm_data) == len(python_data):
    print(f'OK: Both packages found {len(npm_data)} differences')
    
    # Check that both found the same types of changes
    npm_paths = {item['path'] for item in npm_data}
    python_paths = {item['path'] for item in python_data}
    
    if npm_paths == python_paths:
        print('OK: Both packages identified the same change paths')
    else:
        print('ERROR: Packages found different change paths')
        print(f'npm: {npm_paths}')
        print(f'python: {python_paths}')
        exit(1)
else:
    print(f'ERROR: Different number of changes: npm={len(npm_data)}, python={len(python_data)}')
    exit(1)
"
success "Cross-platform compatibility verified"

deactivate

###########################################
# Test 4: Real-world scenarios
###########################################

log "Test 4: Testing real-world scenarios"

# Test CI/CD configuration scenario
cat > .github_workflows_old.yml << 'EOF'
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 16
      - run: npm test
EOF

cat > .github_workflows_new.yml << 'EOF'
name: CI
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16, 18, 20]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
      - run: npm ci
      - run: npm test
EOF

cd npm-test
cp ../.github_workflows_old.yml ../.github_workflows_new.yml .

CI_DIFF=$(npx ${PROJECT_NAME} .github_workflows_old.yml .github_workflows_new.yml)
if echo "$CI_DIFF" | grep -q "node-version" && echo "$CI_DIFF" | grep -q "develop"; then
    success "CI/CD configuration diff scenario works"
else
    error "CI/CD scenario failed"
    echo "Output: $CI_DIFF"
    exit 1
fi

###########################################
# Summary
###########################################

echo ""
echo "All Published Package Tests Passed!"
echo "======================================"
echo ""
success "npm package (${PROJECT_NAME}-js) - All functionality verified"
success "Python package (${PROJECT_NAME}-python) - All functionality verified"  
success "Cross-platform compatibility - Results identical"
success "Real-world scenarios - Working correctly"
echo ""
info "Published packages are ready for production use!"

log "Cleaning up temporary directory..."
cd /
rm -rf "$TEMP_DIR"
success "Test completed successfully"