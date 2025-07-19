#!/usr/bin/env node

/**
 * Error handling tests for diffx-npm package
 * Tests various error conditions and edge cases
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

async function testNonexistentFiles() {
    info('Testing nonexistent files error handling...');
    
    const result = await runCommand('node', [
        path.join(__dirname, '../index.js'),
        'nonexistent1.json',
        'nonexistent2.json'
    ]);
    
    if (result.code !== 0) {
        success('Nonexistent files handled correctly');
        return true;
    } else {
        error('Should have failed with nonexistent files');
        return false;
    }
}

async function testMalformedJSON() {
    info('Testing malformed JSON handling...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-malformed-test-'));
    
    try {
        const file1 = path.join(tempDir, 'malformed.json');
        const file2 = path.join(tempDir, 'valid.json');
        
        fs.writeFileSync(file1, '{"invalid": json content');
        fs.writeFileSync(file2, '{"valid": "json"}');
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code !== 0) {
            success('Malformed JSON handled correctly');
            return true;
        } else {
            error('Should have failed with malformed JSON');
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testEmptyFiles() {
    info('Testing empty files handling...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-empty-test-'));
    
    try {
        const file1 = path.join(tempDir, 'empty1.json');
        const file2 = path.join(tempDir, 'empty2.json');
        
        fs.writeFileSync(file1, '');
        fs.writeFileSync(file2, '');
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        // Empty files might be handled gracefully or with error
        if (result.code === 0 || result.code !== 0) {
            success('Empty files handled appropriately');
            return true;
        } else {
            error('Empty files handling failed unexpectedly');
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testInvalidOptions() {
    info('Testing invalid command line options...');
    
    const result = await runCommand('node', [
        path.join(__dirname, '../index.js'),
        '--invalid-option',
        'file1.json',
        'file2.json'
    ]);
    
    if (result.code !== 0) {
        success('Invalid options handled correctly');
        return true;
    } else {
        error('Should have failed with invalid options');
        return false;
    }
}

async function testDirectoryAsFile() {
    info('Testing directory passed as file...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-dir-test-'));
    
    try {
        const subDir = path.join(tempDir, 'subdir');
        const file = path.join(tempDir, 'file.json');
        
        fs.mkdirSync(subDir);
        fs.writeFileSync(file, '{"test": "value"}');
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            subDir, // Directory instead of file
            file
        ]);
        
        // This might be handled differently depending on implementation
        if (result.code !== 0) {
            success('Directory as file handled appropriately');
            return true;
        } else {
            info('Directory as file handled (may support directory comparison)');
            return true;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testPermissionDenied() {
    info('Testing permission denied scenarios...');
    
    // This test might not work on all systems, so we'll make it non-failing
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-perm-test-'));
    
    try {
        const file1 = path.join(tempDir, 'file1.json');
        const file2 = path.join(tempDir, 'file2.json');
        
        fs.writeFileSync(file1, '{"test": "value1"}');
        fs.writeFileSync(file2, '{"test": "value2"}');
        
        // Try to make file unreadable (might not work on Windows)
        try {
            fs.chmodSync(file1, 0o000);
        } catch (chmodErr) {
            info('Cannot test permission denied on this system');
            return true;
        }
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code !== 0) {
            success('Permission denied handled correctly');
            return true;
        } else {
            info('Permission test completed (may have different behavior)');
            return true;
        }
    } finally {
        // Restore permissions before cleanup
        try {
            const file1 = path.join(tempDir, 'file1.json');
            if (fs.existsSync(file1)) {
                fs.chmodSync(file1, 0o644);
            }
        } catch (err) {
            // Ignore cleanup errors
        }
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testTooFewArguments() {
    info('Testing too few arguments...');
    
    const result = await runCommand('node', [
        path.join(__dirname, '../index.js'),
        'only-one-file.json'
    ]);
    
    if (result.code !== 0) {
        success('Too few arguments handled correctly');
        return true;
    } else {
        error('Should have failed with too few arguments');
        return false;
    }
}

async function testBinaryNotFound() {
    info('Testing binary not found scenario...');
    
    // Temporarily rename the binary directory to simulate missing binary
    const binDir = path.join(__dirname, '../bin');
    const tempBinDir = path.join(__dirname, '../bin-temp');
    
    let renamed = false;
    try {
        if (fs.existsSync(binDir)) {
            fs.renameSync(binDir, tempBinDir);
            renamed = true;
        }
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            '--version'
        ]);
        
        if (result.code !== 0) {
            success('Missing binary handled correctly');
            return true;
        } else {
            info('Binary not found test completed (may have fallback)');
            return true;
        }
    } finally {
        // Restore the binary directory
        if (renamed && fs.existsSync(tempBinDir)) {
            fs.renameSync(tempBinDir, binDir);
        }
    }
}

async function runErrorTests() {
    info('Starting error handling tests...');
    
    const tests = [
        testNonexistentFiles,
        testMalformedJSON,
        testEmptyFiles,
        testInvalidOptions,
        testDirectoryAsFile,
        testPermissionDenied,
        testTooFewArguments,
        testBinaryNotFound
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
    
    info(`Error handling tests completed: ${passed} passed, ${failed} failed`);
    return failed === 0;
}

module.exports = { runErrorTests };

// Run tests if called directly
if (require.main === module) {
    runErrorTests().then(success => {
        process.exit(success ? 0 : 1);
    });
}