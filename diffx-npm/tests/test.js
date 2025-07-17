#!/usr/bin/env node

/**
 * Unified test runner for diffx-npm package
 * Runs all test suites in organized fashion
 */

const fs = require('fs');
const path = require('path');

// Colors for output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m',
    magenta: '\x1b[35m',
    reset: '\x1b[0m',
    bold: '\x1b[1m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function success(message) {
    log(`✓ ${message}`, 'green');
}

function error(message) {
    log(`✗ ${message}`, 'red');
}

function info(message) {
    log(`ℹ ${message}`, 'blue');
}

function header(message) {
    log(`\n${colors.bold}${colors.cyan}=== ${message} ===${colors.reset}`);
}

function separator() {
    log(`${colors.yellow}${'─'.repeat(60)}${colors.reset}`);
}

// Import test modules
const { runCLITests } = require('./cli.test.js');
const { runBasicTests } = require('./basic.test.js');
const { runBinaryTests } = require('./binary.test.js');
const { runFormatTests } = require('./formats.test.js');
const { runErrorTests } = require('./errors.test.js');
const { runFeatureTests } = require('./features.test.js');

async function checkPrerequisites() {
    header('Prerequisites Check');
    
    // Check if package.json exists
    const packagePath = path.join(__dirname, '../package.json');
    if (!fs.existsSync(packagePath)) {
        error('package.json not found');
        return false;
    }
    
    // Check if main files exist
    const mainFiles = ['index.js', 'lib.js'];
    for (const file of mainFiles) {
        const filePath = path.join(__dirname, '..', file);
        if (!fs.existsSync(filePath)) {
            error(`${file} not found`);
            return false;
        }
    }
    
    success('All required files found');
    return true;
}

async function runTestSuite(name, testFunction, description) {
    header(`${name} Tests`);
    info(description);
    separator();
    
    const startTime = Date.now();
    let success = false;
    
    try {
        success = await testFunction();
        const duration = Date.now() - startTime;
        
        if (success) {
            log(`\n${colors.green}${colors.bold}✓ ${name} tests PASSED${colors.reset} (${duration}ms)`, 'green');
        } else {
            log(`\n${colors.red}${colors.bold}✗ ${name} tests FAILED${colors.reset} (${duration}ms)`, 'red');
        }
    } catch (err) {
        const duration = Date.now() - startTime;
        error(`${name} tests CRASHED: ${err.message} (${duration}ms)`);
        success = false;
    }
    
    separator();
    return success;
}

async function runAllTests() {
    const startTime = Date.now();
    
    log(`${colors.bold}${colors.magenta}diffx-npm Test Suite${colors.reset}`);
    log(`Running comprehensive tests for diffx npm package\n`);
    
    // Check prerequisites first
    const prereqsOk = await checkPrerequisites();
    if (!prereqsOk) {
        error('Prerequisites check failed. Cannot proceed with tests.');
        process.exit(1);
    }
    
    const testSuites = [
        {
            name: 'Binary',
            function: runBinaryTests,
            description: 'Verifies platform-specific binaries are present and functional'
        },
        {
            name: 'CLI',
            function: runCLITests,
            description: 'Tests command-line interface functionality and basic commands'
        },
        {
            name: 'Basic',
            function: runBasicTests,
            description: 'Tests core diff operations and basic functionality'
        },
        {
            name: 'Formats',
            function: runFormatTests,
            description: 'Tests support for various file formats (JSON, YAML, TOML, etc.)'
        },
        {
            name: 'Features',
            function: runFeatureTests,
            description: 'Tests advanced features and command-line options'
        },
        {
            name: 'Errors',
            function: runErrorTests,
            description: 'Tests error handling and edge cases'
        }
    ];
    
    const results = [];
    
    for (const suite of testSuites) {
        const success = await runTestSuite(suite.name, suite.function, suite.description);
        results.push({ name: suite.name, success });
    }
    
    // Summary
    const totalTime = Date.now() - startTime;
    const passed = results.filter(r => r.success).length;
    const failed = results.length - passed;
    
    header('Test Results Summary');
    
    for (const result of results) {
        if (result.success) {
            success(`${result.name} tests`);
        } else {
            error(`${result.name} tests`);
        }
    }
    
    separator();
    
    if (failed === 0) {
        log(`\n${colors.bold}${colors.green}🎉 ALL TESTS PASSED!${colors.reset}`, 'green');
        log(`${colors.green}✓ ${passed}/${results.length} test suites passed${colors.reset}`);
        log(`${colors.blue}Total time: ${totalTime}ms${colors.reset}\n`);
        return true;
    } else {
        log(`\n${colors.bold}${colors.red}❌ SOME TESTS FAILED${colors.reset}`, 'red');
        log(`${colors.red}✗ ${failed}/${results.length} test suites failed${colors.reset}`);
        log(`${colors.green}✓ ${passed}/${results.length} test suites passed${colors.reset}`);
        log(`${colors.blue}Total time: ${totalTime}ms${colors.reset}\n`);
        return false;
    }
}

// Run tests if called directly
if (require.main === module) {
    runAllTests().then(success => {
        process.exit(success ? 0 : 1);
    });
}

module.exports = { runAllTests };