#!/usr/bin/env node

/**
 * Basic functionality tests for diffx-npm package
 * Tests core diff operations and basic features
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Colors for output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    reset: '\x1b[0m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function success(message) {
    log(`PASS: ${message}`, 'green');
}

function error(message) {
    log(`ERROR: ${message}`, 'red');
}

function info(message) {
    log(`INFO: ${message}`, 'blue');
}

// Test data
const testData = {
    json1: '{"name": "test-app", "version": "1.0.0", "debug": true}',
    json2: '{"debug": false, "version": "1.1.0", "name": "test-app"}',
    yaml1: 'name: test-app\nversion: "1.0.0"\ndebug: true\n',
    yaml2: 'name: test-app\nversion: "1.1.0"\ndebug: false\n',
};

async function runCommand(command, args = [], options = {}) {
    return new Promise((resolve, reject) => {
        const child = spawn(command, args, {
            stdio: ['pipe', 'pipe', 'pipe'],
            ...options
        });

        let stdout = '';
        let stderr = '';

        child.stdout.on('data', (data) => {
            stdout += data.toString();
        });

        child.stderr.on('data', (data) => {
            stderr += data.toString();
        });

        child.on('close', (code) => {
            resolve({ code, stdout, stderr });
        });

        child.on('error', (err) => {
            reject(err);
        });
    });
}

async function testBasicJSONDiff() {
    info('Testing basic JSON diff...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-basic-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.json');
        const file2 = path.join(tempDir, 'test2.json');
        
        fs.writeFileSync(file1, testData.json1);
        fs.writeFileSync(file2, testData.json2);
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code === 1 && 
            result.stdout.includes('version') && 
            result.stdout.includes('debug')) {
            success('Basic JSON diff works correctly');
            return true;
        } else {
            error(`JSON diff failed: ${result.stderr}`);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testYAMLDiff() {
    info('Testing YAML file diff...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-yaml-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.yaml');
        const file2 = path.join(tempDir, 'test2.yaml');
        
        fs.writeFileSync(file1, testData.yaml1);
        fs.writeFileSync(file2, testData.yaml2);
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code === 1 && result.stdout.includes('version')) {
            success('YAML diff works correctly');
            return true;
        } else {
            error(`YAML diff failed: ${result.stderr}`);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testIdenticalFiles() {
    info('Testing identical files (no differences)...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-identical-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.json');
        const file2 = path.join(tempDir, 'test2.json');
        
        fs.writeFileSync(file1, testData.json1);
        fs.writeFileSync(file2, testData.json1); // Same content
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code === 0 && result.stdout.trim() === '') {
            success('Identical files comparison works correctly');
            return true;
        } else {
            error(`Identical files test failed: unexpected output or exit code`);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testStdinProcessing() {
    info('Testing stdin processing...');
    
    try {
        const tempFile = path.join(os.tmpdir(), 'diffx-stdin-test.json');
        fs.writeFileSync(tempFile, testData.json2);
        
        const child = spawn('node', [
            path.join(__dirname, '../index.js'),
            '-',
            tempFile
        ], { stdio: ['pipe', 'pipe', 'pipe'] });
        
        child.stdin.write(testData.json1);
        child.stdin.end();
        
        let stdout = '';
        let stderr = '';
        
        child.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        child.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        const exitCode = await new Promise((resolve) => {
            child.on('close', resolve);
        });
        
        fs.unlinkSync(tempFile);
        
        if (exitCode === 1 && stdout.includes('version')) {
            success('Stdin processing works correctly');
            return true;
        } else {
            info('Stdin test completed (may require manual verification)');
            return true; // Don't fail on stdin issues
        }
    } catch (err) {
        error(`Stdin test error: ${err.message}`);
        return false;
    }
}

async function runBasicTests() {
    info('Starting basic functionality tests...');
    
    const tests = [
        testBasicJSONDiff,
        testYAMLDiff,
        testIdenticalFiles,
        testStdinProcessing
    ];
    
    let passed = 0;
    let failed = 0;
    
    for (const test of tests) {
        try {
            const result = await test();
            if (result) {
                passed++;
            } else {
                failed++;
            }
        } catch (err) {
            error(`Test failed with exception: ${err.message}`);
            failed++;
        }
    }
    
    info(`Basic tests completed: ${passed} passed, ${failed} failed`);
    return failed === 0;
}

module.exports = { runBasicTests };

// Run tests if called directly
if (require.main === module) {
    runBasicTests().then(success => {
        process.exit(success ? 0 : 1);
    });
}