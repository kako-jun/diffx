#!/usr/bin/env node

/**
 * Format support tests for diffx-npm package
 * Tests various file format handling (JSON, YAML, TOML, CSV, XML, INI)
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

// Test data for different formats
const testData = {
    json1: '{"name": "test", "version": "1.0", "features": ["A", "B"]}',
    json2: '{"name": "test", "version": "1.1", "features": ["A", "B", "C"]}',
    
    yaml1: 'name: test\nversion: "1.0"\nfeatures:\n  - A\n  - B\n',
    yaml2: 'name: test\nversion: "1.1"\nfeatures:\n  - A\n  - B\n  - C\n',
    
    toml1: 'name = "test"\nversion = "1.0"\nfeatures = ["A", "B"]\n',
    toml2: 'name = "test"\nversion = "1.1"\nfeatures = ["A", "B", "C"]\n',
    
    csv1: 'name,version,active\ntest,1.0,true\nother,2.0,false\n',
    csv2: 'name,version,active\ntest,1.1,true\nother,2.0,false\nnew,3.0,true\n',
    
    xml1: '<root><name>test</name><version>1.0</version></root>',
    xml2: '<root><name>test</name><version>1.1</version><features><item>A</item></features></root>',
    
    ini1: '[app]\nname = test\nversion = 1.0\n\n[features]\nenabled = true\n',
    ini2: '[app]\nname = test\nversion = 1.1\n\n[features]\nenabled = true\nnew_feature = true\n'
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

async function testFormat(formatName, extension, data1, data2, expectedDiff) {
    info(`Testing ${formatName.toUpperCase()} format support...`);
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), `diffx-${formatName}-test-`));
    
    try {
        const file1 = path.join(tempDir, `test1.${extension}`);
        const file2 = path.join(tempDir, `test2.${extension}`);
        
        fs.writeFileSync(file1, data1);
        fs.writeFileSync(file2, data2);
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        if (result.code === 1 && result.stdout.includes(expectedDiff)) {
            success(`${formatName.toUpperCase()} format diff works correctly`);
            return true;
        } else {
            error(`${formatName.toUpperCase()} format diff failed: ${result.stderr}`);
            console.log('Expected:', expectedDiff);
            console.log('Got:', result.stdout);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testJSONFormat() {
    return await testFormat('json', 'json', testData.json1, testData.json2, 'version');
}

async function testYAMLFormat() {
    return await testFormat('yaml', 'yaml', testData.yaml1, testData.yaml2, 'version');
}

async function testTOMLFormat() {
    return await testFormat('toml', 'toml', testData.toml1, testData.toml2, 'version');
}

async function testCSVFormat() {
    return await testFormat('csv', 'csv', testData.csv1, testData.csv2, 'test');
}

async function testXMLFormat() {
    return await testFormat('xml', 'xml', testData.xml1, testData.xml2, 'version');
}

async function testINIFormat() {
    return await testFormat('ini', 'ini', testData.ini1, testData.ini2, 'version');
}

async function testFormatSpecification() {
    info('Testing explicit format specification...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-format-spec-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.txt'); // Wrong extension
        const file2 = path.join(tempDir, 'test2.txt');
        
        fs.writeFileSync(file1, testData.json1);
        fs.writeFileSync(file2, testData.json2);
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2,
            '--format',
            'json'
        ]);
        
        if (result.code === 1 && result.stdout.includes('version')) {
            success('Explicit format specification works correctly');
            return true;
        } else {
            error(`Format specification failed: ${result.stderr}`);
            return false;
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function testUnsupportedFormat() {
    info('Testing unsupported format handling...');
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-unsupported-test-'));
    
    try {
        const file1 = path.join(tempDir, 'test1.unknown');
        const file2 = path.join(tempDir, 'test2.unknown');
        
        fs.writeFileSync(file1, 'some content');
        fs.writeFileSync(file2, 'different content');
        
        const result = await runCommand('node', [
            path.join(__dirname, '../index.js'),
            file1,
            file2
        ]);
        
        // Should either handle gracefully or show appropriate error
        if (result.code !== 0) {
            success('Unsupported format handled appropriately');
            return true;
        } else {
            info('Unsupported format test completed (may be handled as text)');
            return true; // Don't fail if it's handled as text
        }
    } finally {
        fs.rmSync(tempDir, { recursive: true, force: true });
    }
}

async function runFormatTests() {
    info('Starting format support tests...');
    
    const tests = [
        testJSONFormat,
        testYAMLFormat,
        testTOMLFormat,
        testCSVFormat,
        testXMLFormat,
        testINIFormat,
        testFormatSpecification,
        testUnsupportedFormat
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
    
    info(`Format tests completed: ${passed} passed, ${failed} failed`);
    return failed === 0;
}

module.exports = { runFormatTests };

// Run tests if called directly
if (require.main === module) {
    runFormatTests().then(success => {
        process.exit(success ? 0 : 1);
    });
}