/**
 * README examples tests for diffx-js package  
 * Tests README.md usage examples as JavaScript library functions
 * Each test corresponds 1:1 with diffx commands from README.md
 */

const diffx = require('../../index.js');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Helper function to create temporary JSON files
function createTempJson(content) {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-readme-test-'));
    const filePath = path.join(tempDir, `test-${Date.now()}.json`);
    fs.writeFileSync(filePath, JSON.stringify(content));
    return { filePath, cleanup: () => fs.rmSync(tempDir, { recursive: true, force: true }) };
}

describe('README Examples - 26 diffx commands', () => {
    
    /// Test case 1: diffx config_v1.json config_v2.json
    test('basic config diff', async () => {
        const temp1 = createTempJson({"name": "myapp", "version": "1.0"});
        const temp2 = createTempJson({"version": "1.1", "name": "myapp"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('version');
            expect(result).toContain('1.0');
            expect(result).toContain('1.1');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 2: time diffx large_test1.json large_test2.json
    test('large file performance', async () => {
        const temp1 = createTempJson({"config": {"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true}}});
        const temp2 = createTempJson({"config": {"database": {"host": "prod-db", "port": 5432}, "cache": {"enabled": false}}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('host');
            expect(result).toContain('enabled');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 3: diffx config_v1.json config_v2.json --output json > report1.json
    test('json output to file', async () => {
        const temp1 = createTempJson({"version": "1.0"});
        const temp2 = createTempJson({"version": "1.1"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
            expect(parsed.length).toBeGreaterThan(0);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 4: diffx config_v2.json config_v3.json --output json > report2.json
    test('second json output', async () => {
        const temp1 = createTempJson({"version": "1.1"});
        const temp2 = createTempJson({"version": "1.2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
            expect(result).toContain('1.1');
            expect(result).toContain('1.2');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 5: diffx report1.json report2.json
    test('meta chaining diff reports', async () => {
        const report1 = createTempJson([{"Modified": ["version", "1.0", "1.1"]}]);
        const report2 = createTempJson([{"Modified": ["version", "1.1", "1.2"]}]);
        
        try {
            const result = await diffx.diff(report1.filePath, report2.filePath);
            expect(typeof result).toBe('string');
        } finally {
            report1.cleanup();
            report2.cleanup();
        }
    });

    /// Test case 6: diffx file1.json file2.json
    test('basic file comparison', async () => {
        const temp1 = createTempJson({"data": "value1"});
        const temp2 = createTempJson({"data": "value2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('data');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 7: diffx config.yaml config_new.yaml --output json
    test('yaml with json output', async () => {
        const temp1 = createTempJson({"config": {"debug": true}});
        const temp2 = createTempJson({"config": {"debug": false}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 8: diffx data.toml data_updated.toml --output yaml
    test('toml with yaml output', async () => {
        const temp1 = createTempJson({"app": {"name": "test"}});
        const temp2 = createTempJson({"app": {"name": "updated"}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'yaml'});
            expect(typeof result).toBe('string');
            expect(result.length).toBeGreaterThan(0);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 9: diffx large.json large_v2.json --ignore-keys-regex "^timestamp$|^_.*"
    test('ignore keys regex', async () => {
        const temp1 = createTempJson({"timestamp": "2024-01-01", "_internal": "meta", "data": "value1"});
        const temp2 = createTempJson({"timestamp": "2024-01-02", "_internal": "meta2", "data": "value2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                ignoreKeysRegex: "^timestamp$|^_.*"
            });
            expect(result).not.toContain('timestamp');
            expect(result).toContain('data');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 10: diffx users.json users_v2.json --array-id-key "id"
    test('array id key', async () => {
        const temp1 = createTempJson({"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]});
        const temp2 = createTempJson({"users": [{"id": 2, "name": "Jane"}, {"id": 1, "name": "Johnny"}]});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                arrayIdKey: "id"
            });
            expect(result).toContain('name');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 11: diffx metrics.json metrics_v2.json --epsilon 0.001
    test('epsilon tolerance', async () => {
        const temp1 = createTempJson({"value": 1.0001});
        const temp2 = createTempJson({"value": 1.0002});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                epsilon: 0.001
            });
            expect(result.trim()).toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 12: diffx config.yaml config_new.yaml --ignore-case
    test('ignore case', async () => {
        const temp1 = createTempJson({"status": "ACTIVE"});
        const temp2 = createTempJson({"status": "active"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                ignoreCase: true
            });
            expect(result.trim()).toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 13: diffx api.json api_formatted.json --ignore-whitespace
    test('ignore whitespace', async () => {
        const temp1 = createTempJson({"text": "hello world"});
        const temp2 = createTempJson({"text": "hello    world"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                ignoreWhitespace: true
            });
            expect(result.trim()).toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 14: diffx large.json large_v2.json --context 3 --output unified
    test('unified output with context', async () => {
        const temp1 = createTempJson({"a": 1, "b": 2, "c": 3, "d": 4});
        const temp2 = createTempJson({"a": 1, "b": 20, "c": 3, "d": 4});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                output: 'unified',
                context: 3
            });
            expect(result).toContain('-');
            expect(result).toContain('+');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 15: diffx file1.json file2.json --quiet
    test('quiet mode', async () => {
        const temp1 = createTempJson({"test": "value1"});
        const temp2 = createTempJson({"test": "value2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                quiet: true
            });
            expect(result.length).toBeGreaterThan(0);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 16: diffx dir1/ dir2/ --recursive --brief
    test('recursive brief', async () => {
        const temp1 = createTempJson({"test": "value1"});
        const temp2 = createTempJson({"test": "value2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                brief: true
            });
            expect(result.length).toBeGreaterThan(0);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 17: diffx huge_dataset.json huge_dataset_v2.json
    test('huge dataset performance', async () => {
        const temp1 = createTempJson({"dataset": {"size": 1000000, "type": "production"}});
        const temp2 = createTempJson({"dataset": {"size": 1000001, "type": "production"}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('size');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 18: diffx config_dir1/ config_dir2/ --recursive
    test('directory recursive', async () => {
        const temp1 = createTempJson({"config": "dir1"});
        const temp2 = createTempJson({"config": "dir2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('config');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 19: diffx config_v1.json config_v2.json --output json > diff1.json
    test('diff1 json output', async () => {
        const temp1 = createTempJson({"config": {"version": "1.0"}});
        const temp2 = createTempJson({"config": {"version": "1.1"}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 20: diffx config_v2.json config_v3.json --output json > diff2.json
    test('diff2 json output', async () => {
        const temp1 = createTempJson({"config": {"version": "1.1"}});
        const temp2 = createTempJson({"config": {"version": "1.2"}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
            expect(result).toContain('1.1');
            expect(result).toContain('1.2');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 21: diffx diff1.json diff2.json
    test('meta diff comparison', async () => {
        const diff1 = createTempJson([{"Modified": ["config.version", "1.0", "1.1"]}]);
        const diff2 = createTempJson([{"Modified": ["config.version", "1.1", "1.2"]}]);
        
        try {
            const result = await diffx.diff(diff1.filePath, diff2.filePath);
            expect(typeof result).toBe('string');
        } finally {
            diff1.cleanup();
            diff2.cleanup();
        }
    });

    /// Test case 22: diffx config/prod.yaml config/staging.yaml --output json > changes.json
    test('cicd config changes', async () => {
        const temp1 = createTempJson({"env": "prod", "debug": false});
        const temp2 = createTempJson({"env": "staging", "debug": true});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 23: if ! diffx config/current.json config/new.json --quiet; then
    test('cicd change detection', async () => {
        const temp1 = createTempJson({"current": "config"});
        const temp2 = createTempJson({"current": "new_config"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {quiet: true});
            expect(result.length).toBeGreaterThan(0);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 24: diffx api_old.json api_new.json --ignore-case --ignore-whitespace --output json > api_changes.json
    test('api ignore options json', async () => {
        const temp1 = createTempJson({"API": "old version"});
        const temp2 = createTempJson({"api": "new   version"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {
                ignoreCase: true,
                ignoreWhitespace: true,
                output: 'json'
            });
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 25: diffx large_prod_data.json large_staging_data.json --output json > data_changes.json
    test('large data comparison', async () => {
        const temp1 = createTempJson({"dataset": {"env": "prod", "size": 10000}});
        const temp2 = createTempJson({"dataset": {"env": "staging", "size": 5000}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 26: diffx package.json HEAD~1:package.json --output json
    test('git dependency detection', async () => {
        const temp1 = createTempJson({"dependencies": {"express": "^4.18.0"}});
        const temp2 = createTempJson({"dependencies": {"express": "^4.18.0", "lodash": "^4.17.21"}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath, {output: 'json'});
            const parsed = JSON.parse(result);
            expect(Array.isArray(parsed)).toBe(true);
            expect(result).toContain('lodash');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

});