#!/usr/bin/env node

/**
 * CLI tests for diffx-npm package
 * Tests command-line interface functionality
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

async function testVersionCommand() {
    info('Testing version command...');
    const result = await runCommand('node', [path.join(__dirname, '../index.js'), '--version']);
    if (result.code === 0 && result.stdout.includes('diffx')) {
        success('Version command works correctly');
        return true;
    } else {
        error('Version command failed');
        return false;
    }
}

async function testHelpCommand() {
    info('Testing help command...');
    const result = await runCommand('node', [path.join(__dirname, '../index.js'), '--help']);
    if (result.code === 0 && result.stdout.includes('diffx') && result.stdout.includes('USAGE')) {
        success('Help command works correctly');
        return true;
    } else {
        error('Help command failed');
        return false;
    }
}

async function testBasicDiff() {
    info('Testing basic diff functionality...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-cli-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.json');
        const file2 = path.join(tempDir, 'test2.json');
        
        fs.writeFileSync(file1, '{"name": "test", "version": "1.0"}');
        fs.writeFileSync(file2, '{"name": "test", "version": "1.1"}');
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code === 1 && result.stdout.includes('version')) {
            success('Basic diff works correctly');
            return true;
        } else {
            error(`Basic diff failed: ${result.stderr}`);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testOutputFormats() {
    info('Testing output formats...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-format-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.json');
        const file2 = path.join(tempDir, 'test2.json');
        
        fs.writeFileSync(file1, '{"name": "test", "version": "1.0"}');
        fs.writeFileSync(file2, '{"name": "test", "version": "1.1"}');
        
        // Test JSON output
        const jsonResult = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2,
            '--output',
            'json'
        ]);
        
        if (jsonResult.code === 1) {
            try {
                const output = JSON.parse(jsonResult.stdout);
                if (Array.isArray(output) && output.length > 0) {
                    success('JSON output format works correctly');
                    return true;
                }
            } catch (parseError) {
                error(`JSON output parsing failed: ${parseError.message}`);
                return false;
            }
        }
        
        error('Output format test failed');
        return false;
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function runCLITests() {
    info('Starting CLI tests...');
    
    const tests = [
        testVersionCommand,
        testHelpCommand,
        testBasicDiff,
        testOutputFormats
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
    
    info(`CLI tests completed: ${passed} passed, ${failed} failed`);
    return failed === 0;
}

module.exports = { runCLITests };

// Run tests if called directly
if (require.main === module) {
    runCLITests().then(success => {
        process.exit(success ? 0 : 1);
    });
}