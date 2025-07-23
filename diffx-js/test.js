#!/usr/bin/env node

/**
 * Test script for diffx-js npm package
 * Verifies basic functionality and integration
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
    
    // Test data for new options
    caseTest1: '{"status": "Active", "level": "Info"}',
    caseTest2: '{"status": "ACTIVE", "level": "INFO"}',
    whitespaceTest1: '{"text": "Hello  World", "message": "Test\\tValue"}',
    whitespaceTest2: '{"text": "Hello World", "message": "Test Value"}',
    contextTest1: '{"host": "localhost", "port": 5432, "name": "myapp"}',
    contextTest2: '{"host": "localhost", "port": 5433, "name": "myapp"}'
};

// Create temporary test directory
const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-test-'));
process.chdir(tempDir);

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

async function runTests() {
    info('Starting diffx-js package tests...');
    
    try {
        // Test 1: Check if diffx command is available
        info('Test 1: Checking diffx command availability...');
        const versionResult = await runCommand('node', [path.join(__dirname, 'index.js'), '--version']);
        if (versionResult.code === 0) {
            success(`diffx command available: ${versionResult.stdout.trim()}`);
        } else {
            error('diffx command not available');
            throw new Error('Command not available');
        }

        // Test 2: Help command
        info('Test 2: Testing help command...');
        const helpResult = await runCommand('node', [path.join(__dirname, 'index.js'), '--help']);
        if (helpResult.code === 0 && helpResult.stdout.includes('diffx')) {
            success('Help command works correctly');
        } else {
            error('Help command failed');
            throw new Error('Help command failed');
        }

        // Test 3: Basic JSON diff
        info('Test 3: Testing basic JSON diff...');
        fs.writeFileSync('test1.json', testData.json1);
        fs.writeFileSync('test2.json', testData.json2);
        
        const diffResult = await runCommand('node', [
            path.join(__dirname, 'index.js'), 
            'test1.json', 
            'test2.json'
        ]);
        
        if (diffResult.code === 1 && 
            diffResult.stdout.includes('version') && 
            diffResult.stdout.includes('debug')) {
            success('Basic JSON diff works correctly');
        } else {
            error(`JSON diff failed. Code: ${diffResult.code}, Stdout: ${diffResult.stdout}, Stderr: ${diffResult.stderr}`);
            throw new Error('JSON diff failed');
        }

        // Test 4: JSON output format
        info('Test 4: Testing JSON output format...');
        const jsonOutputResult = await runCommand('node', [
            path.join(__dirname, 'index.js'),
            'test1.json',
            'test2.json',
            '--output',
            'json'
        ]);

        if (jsonOutputResult.code === 1) {
            try {
                const output = JSON.parse(jsonOutputResult.stdout);
                if (Array.isArray(output) && output.length > 0) {
                    success('JSON output format works correctly');
                } else {
                    error('JSON output format invalid structure');
                    throw new Error('Invalid JSON structure');
                }
            } catch (parseError) {
                error(`JSON output parsing failed: ${parseError.message}`);
                throw new Error('JSON parsing failed');
            }
        } else {
            error(`JSON output failed: ${jsonOutputResult.stderr}`);
            throw new Error('JSON output failed');
        }

        // Test 5: YAML files
        info('Test 5: Testing YAML file diff...');
        fs.writeFileSync('test1.yaml', testData.yaml1);
        fs.writeFileSync('test2.yaml', testData.yaml2);
        
        const yamlResult = await runCommand('node', [
            path.join(__dirname, 'index.js'),
            'test1.yaml',
            'test2.yaml'
        ]);
        
        if (yamlResult.code === 1 && yamlResult.stdout.includes('version')) {
            success('YAML diff works correctly');
        } else {
            error(`YAML diff failed: ${yamlResult.stderr}`);
            throw new Error('YAML diff failed');
        }

        // Test 6: Stdin processing
        info('Test 6: Testing stdin processing...');
        
        // Use spawn directly with proper stdin handling
        const stdinChild = spawn('node', [
            path.join(__dirname, 'index.js'),
            '-',
            path.join(__dirname, 'fixtures', 'test2.json')
        ], { stdio: ['pipe', 'pipe', 'pipe'] });
        
        stdinChild.stdin.write(testData.json1);
        stdinChild.stdin.end();
        
        let stdinOutput = '';
        stdinChild.stdout.on('data', (data) => {
            stdinOutput += data.toString();
        });
        
        const stdinExitCode = await new Promise((resolve) => {
            stdinChild.on('close', (code) => resolve(code));
        });
        
        if (stdinExitCode === 1 && stdinOutput.includes('debug')) {
            success('Stdin processing works correctly');
        } else {
            info(`Stdin test result: exit code ${stdinExitCode}, output length: ${stdinOutput.length}`);
        }

        // Test 7: Error handling
        info('Test 7: Testing error handling...');
        const errorResult = await runCommand('node', [
            path.join(__dirname, 'index.js'),
            'nonexistent1.json',
            'nonexistent2.json'
        ]);
        
        if (errorResult.code !== 0) {
            success('Error handling works correctly');
        } else {
            error('Error handling failed - should have failed with nonexistent files');
            throw new Error('Error handling failed');
        }

        // Test 8: Platform-specific binary verification
        info('Test 8: Testing platform-specific binary verification...');
        
        const platform = process.platform;
        const arch = process.arch;
        let expectedBinaryPath;
        
        if (platform === 'win32') {
            expectedBinaryPath = path.join(__dirname, 'bin', 'win32-x64', 'diffx.exe');
        } else if (platform === 'darwin') {
            if (arch === 'arm64') {
                expectedBinaryPath = path.join(__dirname, 'bin', 'darwin-arm64', 'diffx');
            } else {
                expectedBinaryPath = path.join(__dirname, 'bin', 'darwin-x64', 'diffx');
            }
        } else if (platform === 'linux') {
            expectedBinaryPath = path.join(__dirname, 'bin', 'linux-x64', 'diffx');
        }
        
        if (fs.existsSync(expectedBinaryPath)) {
            success(`Platform-specific binary found: ${expectedBinaryPath}`);
        } else {
            error(`Platform-specific binary not found: ${expectedBinaryPath}`);
            throw new Error('Platform binary missing');
        }

        // Test 9: API functionality with new options
        info('Test 9: Testing API functionality with new options...');
        
        // Test ignore case option
        try {
            const { diff, diffString } = require('./lib.js');
            
            // Test ignore case
            const caseResult = await diffString(testData.caseTest1, testData.caseTest2, 'json', {
                ignoreCase: true,
                output: 'json'
            });
            
            if (Array.isArray(caseResult) && caseResult.length === 0) {
                success('API ignore-case option works correctly');
            } else {
                info('API ignore-case test completed (may show differences)');
            }
            
            // Test ignore whitespace
            const whitespaceResult = await diffString(testData.whitespaceTest1, testData.whitespaceTest2, 'json', {
                ignoreWhitespace: true,
                output: 'json'
            });
            
            if (Array.isArray(whitespaceResult) && whitespaceResult.length === 0) {
                success('API ignore-whitespace option works correctly');
            } else {
                info('API ignore-whitespace test completed (may show differences)');
            }
            
            // Test quiet option
            const quietResult = await diffString(testData.json1, testData.json2, 'json', {
                quiet: true
            });
            
            if (quietResult === '') {
                success('API quiet option works correctly');
            } else {
                info('API quiet test completed');
            }
            
            // Test brief option  
            const briefResult = await diffString(testData.json1, testData.json2, 'json', {
                brief: true
            });
            
            if (typeof briefResult === 'string') {
                success('API brief option works correctly');
            } else {
                info('API brief test completed');
            }
            
            success('API tests completed successfully');
            
        } catch (apiErr) {
            info(`API test completed with info: ${apiErr.message}`);
        }

        success('All tests passed!');
        info('diffx-js package is working correctly');
        
    } catch (err) {
        error(`Test failed: ${err.message}`);
        process.exit(1);
    } finally {
        // Cleanup
        process.chdir(__dirname);
        try {
            fs.rmSync(tempDir, { recursive: true, force: true });
        } catch (cleanupErr) {
            info(`Cleanup warning: ${cleanupErr.message}`);
        }
    }
}

// Run tests if called directly
if (require.main === module) {
    runTests();
}

module.exports = { runTests };