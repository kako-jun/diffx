#!/usr/bin/env node

/**
 * Binary verification tests for diffx-npm package
 * Tests platform-specific binary availability and functionality
 */

const fs = require('fs');
const path = require('path');

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

function testBinaryExists() {
    info('Testing platform-specific binary existence...');
    
    const platform = process.platform;
    const arch = process.arch;
    let expectedBinaryPath;
    
    if (platform === 'win32') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'win32-x64', 'diffx.exe');
    } else if (platform === 'darwin') {
        if (arch === 'arm64') {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-arm64', 'diffx');
        } else {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-x64', 'diffx');
        }
    } else if (platform === 'linux') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'linux-x64', 'diffx');
    } else {
        error(`Unsupported platform: ${platform}`);
        return false;
    }
    
    if (fs.existsSync(expectedBinaryPath)) {
        success(`Platform-specific binary found: ${expectedBinaryPath}`);
        return true;
    } else {
        error(`Platform-specific binary not found: ${expectedBinaryPath}`);
        return false;
    }
}

function testBinaryPermissions() {
    info('Testing binary permissions...');
    
    const platform = process.platform;
    const arch = process.arch;
    let expectedBinaryPath;
    
    if (platform === 'win32') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'win32-x64', 'diffx.exe');
    } else if (platform === 'darwin') {
        if (arch === 'arm64') {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-arm64', 'diffx');
        } else {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-x64', 'diffx');
        }
    } else if (platform === 'linux') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'linux-x64', 'diffx');
    } else {
        info(`Skipping permissions test for unsupported platform: ${platform}`);
        return true;
    }
    
    if (!fs.existsSync(expectedBinaryPath)) {
        error(`Binary not found for permissions test: ${expectedBinaryPath}`);
        return false;
    }
    
    try {
        const stats = fs.statSync(expectedBinaryPath);
        
        // On Unix-like systems, check if binary is executable
        if (platform !== 'win32') {
            const mode = stats.mode;
            const isExecutable = !!(mode & parseInt('111', 8)); // Check if any execute bit is set
            
            if (isExecutable) {
                success('Binary has correct executable permissions');
                return true;
            } else {
                error('Binary is not executable');
                return false;
            }
        } else {
            // On Windows, just check if file exists and is not empty
            if (stats.size > 0) {
                success('Binary file is present and non-empty');
                return true;
            } else {
                error('Binary file is empty');
                return false;
            }
        }
    } catch (err) {
        error(`Error checking binary permissions: ${err.message}`);
        return false;
    }
}

function testBinarySize() {
    info('Testing binary size reasonableness...');
    
    const platform = process.platform;
    const arch = process.arch;
    let expectedBinaryPath;
    
    if (platform === 'win32') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'win32-x64', 'diffx.exe');
    } else if (platform === 'darwin') {
        if (arch === 'arm64') {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-arm64', 'diffx');
        } else {
            expectedBinaryPath = path.join(__dirname, '../bin', 'darwin-x64', 'diffx');
        }
    } else if (platform === 'linux') {
        expectedBinaryPath = path.join(__dirname, '../bin', 'linux-x64', 'diffx');
    } else {
        info(`Skipping size test for unsupported platform: ${platform}`);
        return true;
    }
    
    if (!fs.existsSync(expectedBinaryPath)) {
        error(`Binary not found for size test: ${expectedBinaryPath}`);
        return false;
    }
    
    try {
        const stats = fs.statSync(expectedBinaryPath);
        const sizeInMB = stats.size / (1024 * 1024);
        
        // Reasonable size check: should be between 1MB and 100MB
        if (sizeInMB >= 1 && sizeInMB <= 100) {
            success(`Binary size is reasonable: ${sizeInMB.toFixed(2)} MB`);
            return true;
        } else {
            error(`Binary size seems unreasonable: ${sizeInMB.toFixed(2)} MB`);
            return false;
        }
    } catch (err) {
        error(`Error checking binary size: ${err.message}`);
        return false;
    }
}

function testAllPlatformBinariesExist() {
    info('Testing that all platform binaries are included...');
    
    const expectedBinaries = [
        'bin/win32-x64/diffx.exe',
        'bin/darwin-x64/diffx',
        'bin/darwin-arm64/diffx',
        'bin/linux-x64/diffx'
    ];
    
    let allPresent = true;
    const missingBinaries = [];
    
    for (const binaryPath of expectedBinaries) {
        const fullPath = path.join(__dirname, '..', binaryPath);
        if (!fs.existsSync(fullPath)) {
            allPresent = false;
            missingBinaries.push(binaryPath);
        }
    }
    
    if (allPresent) {
        success('All platform binaries are present');
        return true;
    } else {
        error(`Missing binaries: ${missingBinaries.join(', ')}`);
        return false;
    }
}

async function runBinaryTests() {
    info('Starting binary verification tests...');
    
    const tests = [
        testBinaryExists,
        testBinaryPermissions,
        testBinarySize,
        testAllPlatformBinariesExist
    ];
    
    let passed = 0;
    let failed = 0;
    
    for (const test of tests) {
        try {
            const result = test();
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
    
    info(`Binary tests completed: ${passed} passed, ${failed} failed`);
    return failed === 0;
}

module.exports = { runBinaryTests };

// Run tests if called directly
if (require.main === module) {
    runBinaryTests().then(success => {
        process.exit(success ? 0 : 1);
    });
}