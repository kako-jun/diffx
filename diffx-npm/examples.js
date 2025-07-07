#!/usr/bin/env node

/**
 * Examples demonstrating diffx-js usage
 * Shows various use cases and integration patterns
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
    cyan: '\x1b[36m',
    magenta: '\x1b[35m',
    reset: '\x1b[0m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function header(message) {
    log(`\n🔥 ${message}`, 'cyan');
    log('='.repeat(message.length + 4), 'cyan');
}

function example(title, description) {
    log(`\n📝 ${title}`, 'yellow');
    log(`   ${description}`, 'blue');
}

function code(command) {
    log(`   $ ${command}`, 'green');
}

function output(text) {
    log(`   ${text}`, 'magenta');
}

async function runDiffx(args) {
    return new Promise((resolve, reject) => {
        const child = spawn('node', [path.join(__dirname, 'index.js'), ...args], {
            stdio: ['pipe', 'pipe', 'pipe']
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

async function runExamples() {
    header('diffx-js Usage Examples');
    
    // Create temporary directory for examples
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-examples-'));
    const oldCwd = process.cwd();
    process.chdir(tempDir);

    try {
        // Example 1: Basic JSON comparison
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

        fs.writeFileSync('config_v1.json', JSON.stringify(config1, null, 2));
        fs.writeFileSync('config_v2.json', JSON.stringify(config2, null, 2));

        example(
            'Application Configuration Migration',
            'Compare two versions of app configuration to see what changed'
        );
        code('diffx config_v1.json config_v2.json');
        
        const result1 = await runDiffx(['config_v1.json', 'config_v2.json']);
        output(result1.stdout);

        // Example 2: YAML CI/CD pipeline changes
        header('2. CI/CD Pipeline Configuration Changes');

        const pipeline1 = `name: CI
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
      - run: npm test`;

        const pipeline2 = `name: CI
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
          node-version: \${{ matrix.node-version }}
      - run: npm ci
      - run: npm test`;

        fs.writeFileSync('ci_old.yml', pipeline1);
        fs.writeFileSync('ci_new.yml', pipeline2);

        example(
            'GitHub Actions Workflow Evolution',
            'See how CI pipeline evolved to support multiple Node.js versions'
        );
        code('diffx ci_old.yml ci_new.yml');
        
        const result2 = await runDiffx(['ci_old.yml', 'ci_new.yml']);
        output(result2.stdout);

        // Example 3: JSON output for automation
        header('3. Machine-Readable Output for Automation');

        example(
            'JSON Output for CI/CD Integration',
            'Generate structured output for automated processing'
        );
        code('diffx config_v1.json config_v2.json --output json');
        
        const result3 = await runDiffx(['config_v1.json', 'config_v2.json', '--output', 'json']);
        try {
            const jsonOutput = JSON.parse(result3.stdout);
            output(JSON.stringify(jsonOutput, null, 2));
        } catch (e) {
            output(result3.stdout);
        }

        // Example 4: API Schema Evolution
        header('4. API Schema Version Comparison');

        const apiV1 = {
            openapi: "3.0.0",
            info: {
                title: "User API",
                version: "1.0.0"
            },
            paths: {
                "/users": {
                    get: {
                        responses: {
                            "200": {
                                content: {
                                    "application/json": {
                                        schema: {
                                            type: "array",
                                            items: {
                                                properties: {
                                                    id: { type: "integer" },
                                                    name: { type: "string" },
                                                    email: { type: "string" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        const apiV2 = {
            openapi: "3.0.0",
            info: {
                title: "User API",
                version: "2.0.0"
            },
            paths: {
                "/users": {
                    get: {
                        responses: {
                            "200": {
                                content: {
                                    "application/json": {
                                        schema: {
                                            type: "array",
                                            items: {
                                                properties: {
                                                    id: { type: "integer" },
                                                    name: { type: "string" },
                                                    email: { type: "string" },
                                                    created_at: { type: "string", format: "date-time" },
                                                    is_active: { type: "boolean" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/users/{id}": {
                    get: {
                        parameters: [
                            {
                                name: "id",
                                in: "path",
                                required: true,
                                schema: { type: "integer" }
                            }
                        ]
                    }
                }
            }
        };

        fs.writeFileSync('api_v1.json', JSON.stringify(apiV1, null, 2));
        fs.writeFileSync('api_v2.json', JSON.stringify(apiV2, null, 2));

        example(
            'OpenAPI Schema Breaking Changes Detection',
            'Identify API changes that may break client compatibility'
        );
        code('diffx api_v1.json api_v2.json');
        
        const result4 = await runDiffx(['api_v1.json', 'api_v2.json']);
        output(result4.stdout);

        // Example 5: Environment configuration
        header('5. Environment Configuration Drift Detection');

        const prodConfig = {
            database: {
                host: "prod-db.company.com",
                port: 5432,
                pool_size: 20,
                ssl: true,
                timeout: 30000
            },
            cache: {
                redis_url: "redis://prod-cache.company.com:6379",
                ttl: 3600
            },
            api: {
                rate_limit: 1000,
                cors_origins: ["https://app.company.com"],
                debug: false
            }
        };

        const stagingConfig = {
            database: {
                host: "staging-db.company.com", 
                port: 5432,
                pool_size: 10,
                ssl: true,
                timeout: 30000
            },
            cache: {
                redis_url: "redis://staging-cache.company.com:6379",
                ttl: 1800
            },
            api: {
                rate_limit: 100,
                cors_origins: ["https://staging.company.com", "http://localhost:3000"],
                debug: true
            }
        };

        fs.writeFileSync('prod.json', JSON.stringify(prodConfig, null, 2));
        fs.writeFileSync('staging.json', JSON.stringify(stagingConfig, null, 2));

        example(
            'Production vs Staging Configuration Audit',
            'Verify configuration differences between environments'
        );
        code('diffx prod.json staging.json --output yaml');
        
        const result5 = await runDiffx(['prod.json', 'staging.json', '--output', 'yaml']);
        output(result5.stdout);

        // Example 6: Package.json dependency changes
        header('6. Package Dependencies Change Tracking');

        const pkg1 = {
            name: "my-project",
            version: "1.0.0",
            dependencies: {
                "express": "^4.18.0",
                "lodash": "^4.17.21",
                "axios": "^0.27.0"
            },
            devDependencies: {
                "jest": "^28.0.0",
                "eslint": "^8.0.0"
            }
        };

        const pkg2 = {
            name: "my-project",
            version: "1.1.0", 
            dependencies: {
                "express": "^4.19.0",
                "lodash": "^4.17.21",
                "axios": "^1.0.0",
                "helmet": "^6.0.0"
            },
            devDependencies: {
                "jest": "^29.0.0",
                "eslint": "^8.0.0",
                "prettier": "^2.8.0"
            }
        };

        fs.writeFileSync('package_old.json', JSON.stringify(pkg1, null, 2));
        fs.writeFileSync('package_new.json', JSON.stringify(pkg2, null, 2));

        example(
            'Dependency Update Audit',
            'Track package dependency changes for security and compatibility'
        );
        code('diffx package_old.json package_new.json');
        
        const result6 = await runDiffx(['package_old.json', 'package_new.json']);
        output(result6.stdout);

        // Example 7: Integration with Node.js scripts
        header('7. Integration with Node.js Applications');

        example(
            'Programmatic Usage in Node.js',
            'Use diffx within your Node.js applications for automated config validation'
        );

        log('\n📄 Example Node.js Integration:', 'yellow');
        const nodeExample = `
const { spawn } = require('child_process');

async function checkConfigChanges(oldConfig, newConfig) {
    return new Promise((resolve, reject) => {
        const diffx = spawn('npx', ['diffx', oldConfig, newConfig, '--output', 'json']);
        
        let output = '';
        diffx.stdout.on('data', (data) => {
            output += data.toString();
        });
        
        diffx.on('close', (code) => {
            if (code === 0) {
                try {
                    const changes = JSON.parse(output);
                    resolve(changes);
                } catch (e) {
                    reject(e);
                }
            } else {
                reject(new Error(\`diffx failed with code \${code}\`));
            }
        });
    });
}

// Usage
checkConfigChanges('config_v1.json', 'config_v2.json')
    .then(changes => {
        console.log(\`Found \${changes.length} changes:\`);
        changes.forEach(change => {
            console.log(\`- \${change.path}: \${change.change_type}\`);
        });
    })
    .catch(console.error);`;

        output(nodeExample);

        log('\n🎯 Use Cases:', 'cyan');
        log('   • Configuration drift detection in DevOps pipelines', 'blue');
        log('   • API schema validation in CI/CD', 'blue');
        log('   • Environment parity checking', 'blue');
        log('   • Dependency audit automation', 'blue');
        log('   • Infrastructure as Code validation', 'blue');
        log('   • Database schema migration verification', 'blue');

        log('\n✨ Tips for Better Results:', 'cyan');
        log('   • Use --output json for programmatic processing', 'blue');
        log('   • Combine with jq for advanced JSON manipulation', 'blue');
        log('   • Set up automated alerts for critical changes', 'blue');
        log('   • Version control your configuration files', 'blue');
        log('   • Use in pre-commit hooks for validation', 'blue');

        log('\n🔗 More Information:', 'green');
        log('   • Documentation: https://github.com/kako-jun/diffx/tree/main/docs', 'blue');
        log('   • Issues: https://github.com/kako-jun/diffx/issues', 'blue');
        log('   • npm package: https://www.npmjs.com/package/diffx-js', 'blue');

    } catch (error) {
        log(`\n❌ Error running examples: ${error.message}`, 'red');
    } finally {
        // Cleanup
        process.chdir(oldCwd);
        try {
            fs.rmSync(tempDir, { recursive: true, force: true });
        } catch (cleanupErr) {
            log(`Cleanup warning: ${cleanupErr.message}`, 'yellow');
        }
    }
}

// Run examples if called directly
if (require.main === module) {
    runExamples();
}

module.exports = { runExamples };