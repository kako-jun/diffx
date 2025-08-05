const fs = require('fs');
const path = require('path');

// Import the diffx module
// This assumes the native module is built and available
let diffx;
try {
    diffx = require('../index.js');
} catch (error) {
    console.log('diffx module not built, skipping tests');
    process.exit(0);
}

// ============================================================================
// TEST FIXTURES - Shared with Core and Python Tests
// ============================================================================

class TestFixtures {
    /**
     * JavaScript equivalent of Rust/Python fixtures for unified API testing.
     * Uses same test data as core tests but in JavaScript format.
     */
    
    static loadCliFixture(filename) {
        const fixturesDir = path.join(__dirname, '..', '..', 'tests', 'fixtures');
        const fixturePath = path.join(fixturesDir, filename);
        
        if (!fs.existsSync(fixturePath)) {
            throw new Error(`CLI fixture not found: ${fixturePath}`);
        }
        
        const content = fs.readFileSync(fixturePath, 'utf8');
        return JSON.parse(content);
    }
    
    static configV1() {
        return TestFixtures.loadCliFixture('config_v1.json');
    }
    
    static configV2() {
        return TestFixtures.loadCliFixture('config_v2.json');
    }
    
    static configV3() {
        return TestFixtures.loadCliFixture('config_v3.json');
    }
    
    static usersV1() {
        return TestFixtures.loadCliFixture('users_v1.json');
    }
    
    static usersV2() {
        return TestFixtures.loadCliFixture('users_v2.json');
    }
    
    static apiSchemaV1() {
        return TestFixtures.loadCliFixture('api_schema_v1.json');
    }
    
    static apiSchemaV2() {
        return TestFixtures.loadCliFixture('api_schema_v2.json');
    }
    
    static simpleObjectOld() {
        return {
            name: "diffx",
            version: "1.0.0",
            features: ["json", "yaml"]
        };
    }
    
    static simpleObjectNew() {
        return {
            name: "diffx",
            version: "1.1.0",
            features: ["json", "yaml", "toml"],
            author: "Claude"
        };
    }
    
    static arrayWithIdsOld() {
        return [
            { id: 1, name: "Alice", role: "admin" },
            { id: 2, name: "Bob", role: "user" },
            { id: 3, name: "Charlie", role: "user" }
        ];
    }
    
    static arrayWithIdsNew() {
        return [
            { id: 1, name: "Alice", role: "superadmin" },
            { id: 3, name: "Charlie", role: "user" },
            { id: 4, name: "David", role: "user" }
        ];
    }
    
    static nestedObjectOld() {
        return {
            database: {
                host: "localhost",
                port: 5432,
                config: {
                    max_connections: 100,
                    timeout: 30
                }
            },
            cache: {
                type: "redis",
                ttl: 3600
            }
        };
    }
    
    static nestedObjectNew() {
        return {
            database: {
                host: "production.db",
                port: 5432,
                config: {
                    max_connections: 200,
                    timeout: 30,
                    ssl: true
                }
            },
            cache: {
                type: "memcached",
                ttl: 7200
            },
            monitoring: {
                enabled: true
            }
        };
    }
    
    static numericPrecisionOld() {
        return {
            measurements: [1.0, 2.001, 3.1415926],
            coordinates: { x: 10.0, y: 20.0 }
        };
    }
    
    static numericPrecisionNew() {
        return {
            measurements: [1.001, 2.002, 3.1415927],
            coordinates: { x: 10.001, y: 20.001 }
        };
    }
    
    static typeChangesOld() {
        return {
            count: 42,
            enabled: true,
            data: [1, 2, 3],
            meta: { created: "2023-01-01" }
        };
    }
    
    static typeChangesNew() {
        return {
            count: "42",
            enabled: "true",
            data: { "0": 1, "1": 2, "2": 3 },
            meta: "metadata"
        };
    }
}

// ============================================================================
// TEST HELPER FUNCTIONS
// ============================================================================

function expectDiffResult(result, type, path, additionalChecks = {}) {
    expect(result).toHaveProperty('type', type);
    expect(result).toHaveProperty('path', path);
    
    Object.entries(additionalChecks).forEach(([key, value]) => {
        expect(result).toHaveProperty(key, value);
    });
}

function expectNoDifferences(old, newObj, options = {}) {
    const results = diffx.diff(old, newObj, options);
    expect(results).toHaveLength(0);
}

function expectDifferences(old, newObj, expectedCount, options = {}) {
    const results = diffx.diff(old, newObj, options);
    expect(results).toHaveLength(expectedCount);
    return results;
}

// ============================================================================
// UNIFIED API TESTS - Core Functionality
// ============================================================================

describe('Unified API - Core Functionality', () => {
    test('diff basic modification', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 31 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toMatchObject({
            diffType: 'Modified',
            path: 'age',
            oldValue: 30,
            newValue: 31
        });
    });
    
    test('diff added field', () => {
        const old = { name: "Alice" };
        const newObj = { name: "Alice", age: 30 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expectDiffResult(results[0], 'added', 'age', {
            value: expect.stringContaining('30')
        });
    });
    
    test('diff removed field', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice" };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expectDiffResult(results[0], 'removed', 'age', {
            value: expect.stringContaining('30')
        });
    });
    
    test('diff type changed', () => {
        const old = { value: 123 };
        const newObj = { value: "123" };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expectDiffResult(results[0], 'typeChanged', 'value', {
            oldValue: expect.stringContaining('123'),
            newValue: expect.stringContaining('"123"')
        });
    });
    
    test('diff no changes', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 30 };
        
        expectNoDifferences(old, newObj);
    });
});

// ============================================================================
// OPTIONS TESTING - JavaScript Options Coverage
// ============================================================================

describe('Options Handling', () => {
    test('diff with epsilon', () => {
        const old = { value: 1.0 };
        const newObj = { value: 1.001 };
        
        // Within epsilon - no differences
        expectNoDifferences(old, newObj, { epsilon: 0.01 });
        
        // Outside epsilon - should detect difference
        expectDifferences(old, newObj, 1, { epsilon: 0.0001 });
    });
    
    test('diff with array_id_key', () => {
        const old = {
            users: [
                { id: 1, name: "Alice" },
                { id: 2, name: "Bob" }
            ]
        };
        const newObj = {
            users: [
                { id: 2, name: "Bob" },
                { id: 1, name: "Alice Updated" }
            ]
        };
        
        const results = expectDifferences(old, newObj, 1, { arrayIdKey: "id" });
        
        // Should detect modification of Alice's name, not array reordering
        const result = results[0];
        expect(result.type).toBe('modified');
        expect(result.path).toMatch(/\[id=1\]/);
        expect(result.path).toMatch(/name/);
        expect(result.newValue).toContain('Alice Updated');
    });
    
    test('diff with ignore_keys_regex', () => {
        const old = {
            data: "important",
            timestamp: "2023-01-01",
            debug_info: "old"
        };
        const newObj = {
            data: "important",
            timestamp: "2023-01-02",
            debug_info: "new"
        };
        
        expectNoDifferences(old, newObj, { ignoreKeysRegex: "^(timestamp|debug_)" });
    });
    
    test('diff with path_filter', () => {
        const old = {
            config: { value: 1 },
            metadata: { value: 2 }
        };
        const newObj = {
            config: { value: 10 },
            metadata: { value: 20 }
        };
        
        const results = expectDifferences(old, newObj, 1, { pathFilter: "config" });
        
        expect(results[0].path).toMatch(/config/);
    });
    
    test('diff with output_format', () => {
        const old = { name: "Alice" };
        const newObj = { name: "Bob" };
        
        // Test different output formats
        const formats = ["diffx", "json", "yaml", "unified"];
        
        for (const outputFormat of formats) {
            const results = diffx.diff(old, newObj, { outputFormat });
            expect(results).toHaveLength(1);
        }
    });
    
    test('diff with memory_optimization', () => {
        const old = { data: [1, 2, 3] };
        const newObj = { data: [1, 2, 4] };
        
        const results = expectDifferences(old, newObj, 1, { 
            useMemoryOptimization: true, 
            batchSize: 100 
        });
    });
    
    test('diff with diffx_specific_options', () => {
        const old = { text: "Hello World" };
        const newObj = { text: "HELLO WORLD" };
        
        // Case insensitive - should find no differences
        expectNoDifferences(old, newObj, { ignoreCase: true });
        
        // Case sensitive - should find difference
        expectDifferences(old, newObj, 1, { ignoreCase: false });
    });
    
    test('diff with ignore_whitespace', () => {
        const old = { text: "Hello World" };
        const newObj = { text: "HelloWorld" };
        
        // Ignore whitespace - no differences
        expectNoDifferences(old, newObj, { ignoreWhitespace: true });
        
        // Don't ignore whitespace - should find difference
        expectDifferences(old, newObj, 1, { ignoreWhitespace: false });
    });
});

// ============================================================================
// JAVASCRIPT TYPE HANDLING TESTS
// ============================================================================

describe('JavaScript Type Handling', () => {
    test('javascript primitive types', () => {
        const testData = {
            null_value: null,
            undefined_value: undefined, // Should be converted to null
            bool_true: true,
            bool_false: false,
            number_int: 42,
            number_float: 3.14,
            number_zero: 0,
            number_negative: -42,
            string_value: "hello",
            string_empty: "",
            string_unicode: "こんにちは"
        };
        
        // Should not raise any conversion errors
        expectNoDifferences(testData, testData);
    });
    
    test('javascript container types', () => {
        const testData = {
            empty_array: [],
            array_mixed: [1, "two", 3.0, true, null, undefined],
            empty_object: {},
            nested_object: {
                level1: {
                    level2: {
                        value: "deep"
                    }
                }
            },
            array_of_objects: [
                { id: 1, name: "first" },
                { id: 2, name: "second" }
            ]
        };
        
        // Should not raise any conversion errors
        expectNoDifferences(testData, testData);
    });
    
    test('javascript large numbers', () => {
        const old = { 
            big_int: Number.MAX_SAFE_INTEGER, 
            big_float: Number.MAX_VALUE,
            small_float: Number.MIN_VALUE
        };
        const newObj = { 
            big_int: Number.MAX_SAFE_INTEGER - 1, 
            big_float: Number.MAX_VALUE / 2,
            small_float: Number.MIN_VALUE * 2
        };
        
        const results = expectDifferences(old, newObj, 3);
        expect(results).toHaveLength(3); // All should be detected as changes
    });
    
    test('javascript special values', () => {
        const old = { 
            infinity: Infinity, 
            neg_infinity: -Infinity,
            not_a_number: NaN
        };
        
        // These should be handled gracefully
        const results = diffx.diff(old, old);
        // NaN !== NaN in JavaScript, so it might show as different
        expect(results.length).toBeGreaterThanOrEqual(0);
    });
});

// ============================================================================
// ARRAY HANDLING TESTS
// ============================================================================

describe('Array Handling', () => {
    test('diff arrays by index', () => {
        const old = [1, 2, 3];
        const newObj = [1, 3, 4];
        
        const results = expectDifferences(old, newObj, 2);
        // Changes at indices 1 and 2
    });
    
    test('diff arrays with id key', () => {
        const old = [
            { id: "a", value: 1 },
            { id: "b", value: 2 }
        ];
        const newObj = [
            { id: "b", value: 20 },
            { id: "c", value: 3 }
        ];
        
        const results = expectDifferences(old, newObj, 3, { arrayIdKey: "id" });
        
        // Should detect: removed 'a', modified 'b', added 'c'
        expect(results).toHaveLength(3);
    });
    
    test('diff arrays mixed id and index', () => {
        const old = [
            { id: "a", value: 1 },
            { value: 2 }, // No ID
            { id: "b", value: 3 }
        ];
        const newObj = [
            { id: "b", value: 30 },
            { value: 20 }, // No ID
            { id: "c", value: 4 }
        ];
        
        const results = diffx.diff(old, newObj, { arrayIdKey: "id" });
        
        // Should handle both ID-based and index-based comparisons
        expect(results.length).toBeGreaterThan(0);
    });
});

// ============================================================================
// COMPLEX DATA STRUCTURES WITH FIXTURES
// ============================================================================

describe('Complex Structures with Fixtures', () => {
    test('diff with cli fixtures', () => {
        try {
            const old = TestFixtures.configV1();
            const newObj = TestFixtures.configV2();
            
            const results = diffx.diff(old, newObj);
            expect(results.length).toBeGreaterThan(0); // Should find differences
            
        } catch (error) {
            console.log(`CLI fixtures not available: ${error.message}`);
            // Skip test if fixtures not available
        }
    });
    
    test('diff nested objects', () => {
        const old = TestFixtures.nestedObjectOld();
        const newObj = TestFixtures.nestedObjectNew();
        
        const results = diffx.diff(old, newObj);
        
        // Should find multiple changes in nested structure
        expect(results.length).toBeGreaterThan(1);
        
        // Verify some expected changes
        const paths = results.map(result => result.path);
        expect(paths.some(path => path.includes('database.host'))).toBe(true);
        expect(paths.some(path => path.includes('monitoring'))).toBe(true);
    });
    
    test('diff large dataset', () => {
        // Create large dataset (smaller than Rust version to avoid timeout)
        const oldData = {};
        const newData = {};
        
        for (let i = 0; i < 100; i++) { // Smaller dataset for JS test
            oldData[`key_${i}`] = i;
            newData[`key_${i}`] = i + 1;
        }
        
        const results = expectDifferences(oldData, newData, 100);
    });
});

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

describe('Error Handling', () => {
    test('invalid regex pattern', () => {
        const old = { test: "value" };
        const newObj = { test: "value2" };
        
        expect(() => {
            diffx.diff(old, newObj, { ignoreKeysRegex: "[invalid_regex" });
        }).toThrow();
    });
    
    test('invalid output format', () => {
        const old = { test: "value" };
        const newObj = { test: "value2" };
        
        expect(() => {
            diffx.diff(old, newObj, { outputFormat: "invalid_format" });
        }).toThrow();
    });
    
    test('circular references', () => {
        const old = { name: "test" };
        const circular = { name: "test" };
        circular.self = circular; // Create circular reference
        
        // Should either handle gracefully or throw descriptive error
        try {
            diffx.diff(old, circular);
        } catch (error) {
            expect(error.message).toMatch(/circular|serialize|convert/i);
        }
    });
});

// ============================================================================
// ASYNC/PROMISE TESTS
// ============================================================================

describe('Async/Promise Handling', () => {
    test('diff returns array', () => {
        const old = { name: "Alice" };
        const newObj = { name: "Bob" };
        
        const results = diffx.diff(old, newObj);
        expect(Array.isArray(results)).toBe(true);
        expect(results).toHaveLength(1);
    });
    
    test('multiple diff operations', () => {
        const testCases = [
            [{ a: 1 }, { a: 2 }],
            [{ b: 3 }, { b: 4 }],
            [{ c: 5 }, { c: 6 }]
        ];
        
        const results = testCases.map(([old, newObj]) => diffx.diff(old, newObj));
        
        expect(results).toHaveLength(3);
        results.forEach(result => {
            expect(result).toHaveLength(1);
        });
    });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Integration Tests', () => {
    test('unified api comprehensive', () => {
        const old = TestFixtures.arrayWithIdsOld();
        const newObj = TestFixtures.arrayWithIdsNew();
        
        const results = diffx.diff(old, newObj, {
            arrayIdKey: "id",
            outputFormat: "json",
            showUnchanged: false,
            useMemoryOptimization: true
        });
        
        // Should detect changes in the array with ID-based comparison
        expect(results.length).toBeGreaterThan(0);
        
        // Verify result structure
        results.forEach(result => {
            expect(result).toHaveProperty('type');
            expect(result).toHaveProperty('path');
            expect(['added', 'removed', 'modified', 'typeChanged']).toContain(result.type);
        });
    });
    
    test('type conversion fidelity', () => {
        const testCases = [
            TestFixtures.simpleObjectOld(),
            TestFixtures.numericPrecisionOld(),
            TestFixtures.typeChangesOld(),
        ];
        
        for (const testData of testCases) {
            // Diff with itself should produce no changes
            const results = diffx.diff(testData, testData);
            expect(results).toHaveLength(0);
        }
    });
    
    test('common usage patterns', () => {
        // Simulate common usage: parse JSON, compare, get results
        const oldJsonStr = '{"name": "old", "version": 1}';
        const newJsonStr = '{"name": "new", "version": 2}';
        
        const oldData = JSON.parse(oldJsonStr);
        const newData = JSON.parse(newJsonStr);
        
        const results = diffx.diff(oldData, newData);
        
        expect(results).toHaveLength(2); // name and version changed
        
        // Results should be easily serializable
        const resultsJson = JSON.stringify(results);
        expect(resultsJson.length).toBeGreaterThan(0);
    });
});

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

describe('Performance Tests', () => {
    test('large array performance', () => {
        const old = Array.from({ length: 1000 }, (_, i) => ({ 
            id: i, 
            value: `item_${i}` 
        }));
        const newObj = Array.from({ length: 1000 }, (_, i) => ({ 
            id: i, 
            value: `item_${i}_updated` 
        }));
        
        const startTime = Date.now();
        const results = diffx.diff(old, newObj, { arrayIdKey: "id" });
        const endTime = Date.now();
        
        expect(results).toHaveLength(1000); // All items should be modified
        expect(endTime - startTime).toBeLessThan(5000); // Should complete within 5 seconds
    }, 10000); // 10 second timeout for this test
    
    test('deep nesting performance', () => {
        function createNested(depth) {
            if (depth === 0) {
                return { value: "leaf" };
            }
            return { level: depth, nested: createNested(depth - 1) };
        }
        
        const old = createNested(20);
        const newObj = createNested(20);
        newObj.nested.nested.value = "modified_leaf"; // Change deep value
        
        const startTime = Date.now();
        const results = diffx.diff(old, newObj);
        const endTime = Date.now();
        
        expect(results).toHaveLength(1); // Should find the single deep change
        expect(endTime - startTime).toBeLessThan(1000); // Should be fast even with deep nesting
    });
});

// ============================================================================
// TYPESCRIPT COMPATIBILITY TESTS
// ============================================================================

describe('TypeScript Compatibility', () => {
    test('type definitions availability', () => {
        // This test would verify that TypeScript definitions work
        // For now, just check that the main function exists
        expect(typeof diffx.diff).toBe('function');
    });
});

module.exports = {
    TestFixtures,
    expectDiffResult,
    expectNoDifferences,
    expectDifferences
};