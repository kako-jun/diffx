#!/usr/bin/env tsx

/**
 * diffx-js TypeScript Examples - UNIFIED API DESIGN
 * 
 * Demonstrates native NAPI-RS API usage for semantic diffing
 * Users parse files themselves and call the unified diff() function
 */

import { diff, DiffOptions, DiffResult } from './index';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

// Colors for console output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m',
    magenta: '\x1b[35m',
    reset: '\x1b[0m'
} as const;

function log(message: string, color: keyof typeof colors = 'reset'): void {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function header(message: string): void {
    log(`\n${message}`, 'cyan');
    log('='.repeat(message.length), 'cyan');
}

function example(title: string, description: string): void {
    log(`\n${title}`, 'yellow');
    log(`   ${description}`, 'blue');
}

async function runExamples(): Promise<void> {
    header('diffx-js Native API Examples');
    
    // Create temporary directory
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-examples-'));
    const oldCwd = process.cwd();
    process.chdir(tempDir);

    try {
        // Example 1: Basic JSON Configuration Comparison
        header('1. Basic JSON Configuration Comparison');
        
        const config1 = {
            app: {
                name: "my-app",
                version: "1.0.0",
                database: {
                    host: "localhost",
                    port: 5432,
                    ssl: false
                }
            },
            features: ["auth", "logging"]
        };

        const config2 = {
            app: {
                name: "my-app", 
                version: "1.1.0",
                database: {
                    host: "prod-db.example.com",
                    port: 5432,
                    ssl: true
                }
            },
            features: ["auth", "logging", "metrics"]
        };

        example(
            'Application Configuration Migration',
            'Compare two versions of app configuration using native API'
        );
        
        const results1 = diff(config1, config2);
        log('API Results:', 'green');
        console.log(JSON.stringify(results1, null, 2));

        // Example 2: YAML Configuration with Options
        header('2. Advanced Options Usage');

        const oldYaml = `
name: CI
on:
  push:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm test
`;

        const newYaml = `
name: CI
on:
  push:
    branches: [main, develop]
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16, 18, 20]
    steps:
      - uses: actions/checkout@v4
      - run: npm ci
      - run: npm test
`;

        // Parse YAML using js-yaml (users would do this)
        const yaml = await import('js-yaml');
        const oldData = yaml.load(oldYaml) as any;
        const newData = yaml.load(newYaml) as any;

        const options: DiffOptions = {
            outputFormat: 'json',
            showTypes: true,
            ignoreKeysRegex: '^(timestamp|updated_at)',
            diffxOptions: {
                ignoreWhitespace: true,
                contextLines: 2
            }
        };

        example(
            'YAML Comparison with Advanced Options',
            'Parse YAML yourself and use diffx options for customized output'
        );

        const results2 = diff(oldData, newData, options);
        log('Filtered Results:', 'green');
        console.log(JSON.stringify(results2, null, 2));

        // Example 3: Array Comparison with ID Key
        header('3. Smart Array Comparison');

        const oldUsers = {
            users: [
                { id: 1, name: "Alice", role: "admin" },
                { id: 2, name: "Bob", role: "user" },
                { id: 3, name: "Charlie", role: "user" }
            ]
        };

        const newUsers = {
            users: [
                { id: 1, name: "Alice", role: "admin" },
                { id: 2, name: "Bob", role: "moderator" },
                { id: 4, name: "David", role: "user" }
            ]
        };

        const arrayOptions: DiffOptions = {
            arrayIdKey: 'id',
            showUnchanged: false
        };

        example(
            'User Management Changes with Array ID Matching',
            'Track user changes by ID rather than array position'
        );

        const results3 = diff(oldUsers, newUsers, arrayOptions);
        log('User Changes:', 'green');
        results3.forEach((result: DiffResult) => {
            console.log(`${result.type}: ${result.path}`);
            if (result.oldValue) console.log(`  Old: ${JSON.stringify(result.oldValue)}`);
            if (result.newValue) console.log(`  New: ${JSON.stringify(result.newValue)}`);
        });

        // Example 4: Error Handling
        header('4. Error Handling');

        example(
            'Handling Invalid Data Gracefully',
            'Demonstrate proper error handling for malformed data'
        );

        try {
            // Simulate circular reference (not serializable)
            const circularObj: any = { name: "test" };
            circularObj.self = circularObj;
            
            diff({ valid: "data" }, circularObj);
        } catch (error) {
            log(`Caught expected error: ${error}`, 'red');
        }

        // Example 5: Performance with Large Data
        header('5. Large Data Performance');

        const largeData1 = {
            items: Array.from({ length: 1000 }, (_, i) => ({
                id: i,
                value: Math.random(),
                category: `cat_${i % 10}`
            }))
        };

        const largeData2 = {
            items: Array.from({ length: 1000 }, (_, i) => ({
                id: i,
                value: Math.random(),
                category: `cat_${i % 10}`,
                newField: i % 100 === 0 ? "special" : undefined
            })).filter(item => item.newField !== undefined || Math.random() > 0.1)
        };

        const perfOptions: DiffOptions = {
            useMemoryOptimization: true,
            batchSize: 100,
            arrayIdKey: 'id'
        };

        example(
            'Large Dataset Comparison with Memory Optimization',
            'Handle large datasets efficiently with batching'
        );

        const startTime = Date.now();
        const results5 = diff(largeData1, largeData2, perfOptions);
        const endTime = Date.now();

        log(`Processed ${results5.length} differences in ${endTime - startTime}ms`, 'green');

        // Example 6: TypeScript Integration Patterns
        header('6. TypeScript Integration Patterns');

        interface ConfigSchema {
            database: {
                host: string;
                port: number;
                ssl: boolean;
            };
            features: string[];
        }

        const typedConfig1: ConfigSchema = {
            database: { host: "localhost", port: 5432, ssl: false },
            features: ["auth"]
        };

        const typedConfig2: ConfigSchema = {
            database: { host: "remote", port: 5433, ssl: true },
            features: ["auth", "logging"]
        };

        example(
            'Type-Safe Configuration Comparison',
            'Use TypeScript interfaces for better development experience'
        );

        const results6 = diff(typedConfig1, typedConfig2);
        
        // Type-safe result processing
        results6.forEach((result: DiffResult) => {
            switch (result.type) {
                case 'added':
                    log(`➕ Added: ${result.path} = ${JSON.stringify(result.newValue)}`, 'green');
                    break;
                case 'removed':
                    log(`➖ Removed: ${result.path} = ${JSON.stringify(result.oldValue)}`, 'red');
                    break;
                case 'modified':
                    log(`🔄 Modified: ${result.path}`, 'yellow');
                    log(`   Old: ${JSON.stringify(result.oldValue)}`, 'red');
                    log(`   New: ${JSON.stringify(result.newValue)}`, 'green');
                    break;
                case 'typeChanged':
                    log(`🔀 Type Changed: ${result.path} (${result.oldType} → ${result.newType})`, 'magenta');
                    break;
            }
        });

        // Summary
        header('Summary');
        log('✅ All examples completed successfully!', 'green');
        log('\nKey Benefits of Native API:', 'cyan');
        log('  • No external CLI dependency', 'blue');
        log('  • Better error handling', 'blue');
        log('  • Type safety with TypeScript', 'blue');
        log('  • Memory efficient for large data', 'blue');
        log('  • Customizable output formats', 'blue');
        log('  • Integration-friendly', 'blue');

        log('\nNext Steps:', 'cyan');
        log('  • See TypeScript definitions for full API', 'blue');
        log('  • Check documentation for advanced options', 'blue');
        log('  • Integrate into your CI/CD pipeline', 'blue');

    } catch (error) {
        log(`\nError running examples: ${error}`, 'red');
        console.error(error);
    } finally {
        // Cleanup
        process.chdir(oldCwd);
        try {
            fs.rmSync(tempDir, { recursive: true, force: true });
        } catch (cleanupErr) {
            log(`Cleanup warning: ${cleanupErr}`, 'yellow');
        }
    }
}

// Run examples if called directly
if (require.main === module) {
    runExamples().catch(console.error);
}

export { runExamples };