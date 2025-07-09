#!/usr/bin/env node
/**
 * Test all diffx options combinations for npm package
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Test data
const testData1 = {
    name: "Alice",
    age: 30,
    settings: {
        theme: "dark",
        notifications: true,
        limits: {
            max_files: 100,
            timeout: 30.5
        }
    },
    users: [
        { id: 1, name: "User1", active: true },
        { id: 2, name: "User2", active: false }
    ]
};

const testData2 = {
    name: "Alice",
    age: 31,
    settings: {
        theme: "light",
        notifications: true,
        limits: {
            max_files: 150,
            timeout: 30.501
        }
    },
    users: [
        { id: 1, name: "User1", active: true },
        { id: 3, name: "User3", active: true }
    ]
};

function runDiffx(args) {
    return new Promise((resolve, reject) => {
        const binaryPath = path.join(__dirname, 'diffx-npm', 'bin', 'diffx');
        const child = spawn(binaryPath, args, { stdio: 'pipe' });
        
        let stdout = '';
        let stderr = '';
        
        child.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        child.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        child.on('close', (code) => {
            if (code === 0) {
                resolve({ stdout, stderr });
            } else {
                reject({ code, stdout, stderr });
            }
        });
        
        child.on('error', (err) => {
            reject({ error: err.message });
        });
    });
}

async function testAllOptionCombinations() {
    console.log("🧪 Testing all diffx option combinations...");
    
    // Create temporary files
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-test-'));
    const file1 = path.join(tmpDir, 'test1.json');
    const file2 = path.join(tmpDir, 'test2.json');
    
    fs.writeFileSync(file1, JSON.stringify(testData1, null, 2));
    fs.writeFileSync(file2, JSON.stringify(testData2, null, 2));
    
    const testCases = [
        // Basic options
        { name: "Basic diff", args: [file1, file2] },
        { name: "JSON format", args: [file1, file2, "--format", "json"] },
        { name: "YAML format", args: [file1, file2, "--format", "yaml"] },
        // TOML format test skipped for JSON files - would need .toml files
        
        // Output formats
        { name: "CLI output", args: [file1, file2, "--output", "cli"] },
        { name: "JSON output", args: [file1, file2, "--output", "json"] },
        { name: "YAML output", args: [file1, file2, "--output", "yaml"] },
        { name: "Unified output", args: [file1, file2, "--output", "unified"] },
        
        // Path filtering
        { name: "Path filter", args: [file1, file2, "--path", "settings.theme"] },
        { name: "Nested path", args: [file1, file2, "--path", "settings.limits.max_files"] },
        
        // Regex ignore
        { name: "Ignore id keys", args: [file1, file2, "--ignore-keys-regex", "^id$"] },
        { name: "Ignore name keys", args: [file1, file2, "--ignore-keys-regex", "name"] },
        
        // Epsilon tolerance
        { name: "Epsilon 0.1", args: [file1, file2, "--epsilon", "0.1"] },
        { name: "Epsilon 0.001", args: [file1, file2, "--epsilon", "0.001"] },
        
        // Array ID key
        { name: "Array ID key", args: [file1, file2, "--array-id-key", "id"] },
        
        // Performance options
        { name: "Optimize", args: [file1, file2, "--optimize"] },
        { name: "Batch size", args: [file1, file2, "--batch-size", "500"] },
        { name: "Optimize + Batch", args: [file1, file2, "--optimize", "--batch-size", "500"] },
        
        // Complex combinations
        { name: "Multi-option 1", args: [file1, file2, "--format", "json", "--output", "json", "--path", "settings", "--epsilon", "0.01"] },
        { name: "Multi-option 2", args: [file1, file2, "--output", "yaml", "--ignore-keys-regex", "^id$", "--array-id-key", "id", "--optimize"] },
        { name: "Multi-option 3", args: [file1, file2, "--format", "json", "--output", "json", "--path", "users", "--array-id-key", "id", "--batch-size", "100"] },
        { name: "All options", args: [file1, file2, "--format", "json", "--output", "json", "--path", "settings", "--ignore-keys-regex", "timeout", "--epsilon", "0.01", "--array-id-key", "id", "--optimize", "--batch-size", "1000"] }
    ];
    
    let successCount = 0;
    let totalCount = testCases.length;
    
    for (const testCase of testCases) {
        try {
            const result = await runDiffx(testCase.args);
            console.log(`✓ ${testCase.name}: OK`);
            successCount++;
        } catch (error) {
            console.log(`✗ ${testCase.name}: FAILED - ${error.stderr || error.error || error.code}`);
        }
    }
    
    // Add TOML format test with proper files
    try {
        const result = await runDiffx(["tests/fixtures/file1.toml", "tests/fixtures/file2.toml", "--format", "toml"]);
        console.log("✓ TOML format: OK");
        successCount++;
    } catch (error) {
        console.log(`✗ TOML format: FAILED - ${error.stderr || error.error || error.code}`);
    }
    totalCount++;
    
    // Clean up
    fs.rmSync(tmpDir, { recursive: true, force: true });
    
    console.log(`\n📊 Results: ${successCount}/${totalCount} tests passed`);
    return successCount === totalCount;
}

async function testDirectoryComparison() {
    console.log("\n🧪 Testing directory comparison...");
    
    const testCases = [
        { name: "Directory recursive", args: ["tests/fixtures/dir1", "tests/fixtures/dir2", "--recursive"] },
        { name: "Directory with format", args: ["tests/fixtures/dir1", "tests/fixtures/dir2", "--recursive", "--format", "json"] },
        { name: "Directory with output", args: ["tests/fixtures/dir1", "tests/fixtures/dir2", "--recursive", "--output", "json"] },
        { name: "Directory with path", args: ["tests/fixtures/dir1", "tests/fixtures/dir2", "--recursive", "--path", "name"] },
        { name: "Directory with all options", args: ["tests/fixtures/dir1", "tests/fixtures/dir2", "--recursive", "--format", "json", "--output", "json", "--optimize", "--batch-size", "100"] }
    ];
    
    let successCount = 0;
    let totalCount = testCases.length;
    
    for (const testCase of testCases) {
        try {
            const result = await runDiffx(testCase.args);
            console.log(`✓ ${testCase.name}: OK`);
            successCount++;
        } catch (error) {
            console.log(`✗ ${testCase.name}: FAILED - ${error.stderr || error.error || error.code}`);
        }
    }
    
    console.log(`\n📊 Directory comparison results: ${successCount}/${totalCount} tests passed`);
    return successCount === totalCount;
}

async function testEdgeCases() {
    console.log("\n🧪 Testing edge cases...");
    
    const testCases = [
        { name: "Non-existent file", args: ["nonexistent1.json", "nonexistent2.json"] },
        { name: "Invalid regex", args: ["tests/fixtures/config_v1.json", "tests/fixtures/config_v2.json", "--ignore-keys-regex", "[invalid"] },
        { name: "Invalid epsilon", args: ["tests/fixtures/config_v1.json", "tests/fixtures/config_v2.json", "--epsilon", "invalid"] },
        { name: "Invalid batch size", args: ["tests/fixtures/config_v1.json", "tests/fixtures/config_v2.json", "--batch-size", "invalid"] },
    ];
    
    let successCount = 0;
    let totalCount = testCases.length;
    
    for (const testCase of testCases) {
        try {
            const result = await runDiffx(testCase.args);
            console.log(`✗ ${testCase.name}: Should have failed but didn't`);
        } catch (error) {
            console.log(`✓ ${testCase.name}: OK (properly caught error)`);
            successCount++;
        }
    }
    
    console.log(`\n📊 Edge case results: ${successCount}/${totalCount} tests passed`);
    return successCount === totalCount;
}

async function main() {
    console.log("🔬 diffx npm Package - Comprehensive Option Testing");
    console.log("=".repeat(60));
    
    let allPassed = true;
    
    // Test all option combinations
    allPassed &= await testAllOptionCombinations();
    
    // Test directory comparison
    allPassed &= await testDirectoryComparison();
    
    // Test edge cases
    allPassed &= await testEdgeCases();
    
    console.log("\n" + "=".repeat(60));
    if (allPassed) {
        console.log("🎉 All tests passed! npm package handles all option combinations correctly.");
        process.exit(0);
    } else {
        console.log("❌ Some tests failed. Please check the output above.");
        process.exit(1);
    }
}

main().catch(console.error);