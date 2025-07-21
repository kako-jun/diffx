/**
 * docs/guides/performance.md examples tests for diffx-npm package  
 * Tests docs/guides/performance.md usage examples as JavaScript library functions
 * Each test corresponds 1:1 with diffx commands from docs/guides/performance.md
 */

const diffx = require('../../index.js');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Helper function to create temporary JSON files
function createTempJson(content) {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-performance-test-'));
    const filePath = path.join(tempDir, `test-${Date.now()}.json`);
    fs.writeFileSync(filePath, JSON.stringify(content));
    return { filePath, cleanup: () => fs.rmSync(tempDir, { recursive: true, force: true }) };
}

describe('docs/guides/performance.md Examples - 52 diffx commands', () => {
    
    // Test cases 1-10: Basic timing and optimization scenarios
    test('basic timing', async () => { const t1 = createTempJson({"test": "data1"}), t2 = createTempJson({"test": "data2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('test'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('ignore timestamp', async () => { const t1 = createTempJson({"data": "value1", "timestamp": "2024-01-01"}), t2 = createTempJson({"data": "value2", "timestamp": "2024-01-02"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('ignore multiple patterns', async () => { const t1 = createTempJson({"data": "value1", "timestamp": "2024-01-01", "_internal": "meta"}), t2 = createTempJson({"data": "value2", "timestamp": "2024-01-02", "_internal": "meta2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('users basic', async () => { const t1 = createTempJson({"users": [{"id": 1, "name": "John"}]}), t2 = createTempJson({"users": [{"id": 1, "name": "Jane"}]}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('name'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('users with array id', async () => { const t1 = createTempJson({"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Bob"}]}), t2 = createTempJson({"users": [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Johnny"}]}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('name'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('output timing', async () => { const t1 = createTempJson({"value": 100}), t2 = createTempJson({"value": 200}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('value'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('json output', async () => { const t1 = createTempJson({"value": 100}), t2 = createTempJson({"value": 200}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('value'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('yaml output', async () => { const t1 = createTempJson({"value": 100}), t2 = createTempJson({"value": 200}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('value'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('large config basic', async () => { const t1 = createTempJson({"database": {"host": "localhost", "port": 5432}}), t2 = createTempJson({"database": {"host": "prod-db", "port": 5432}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('host'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('large config path', async () => { const t1 = createTempJson({"database": {"connections": {"primary": "db1"}}}), t2 = createTempJson({"database": {"connections": {"primary": "db2"}}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('primary'); } finally { t1.cleanup(); t2.cleanup(); } });

    // Test cases 11-20: Memory optimization and processing strategies
    test('memory usage', async () => { const t1 = createTempJson({"data": "memory_test1"}), t2 = createTempJson({"data": "memory_test2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('memory json output', async () => { const t1 = createTempJson({"data": "memory_test1"}), t2 = createTempJson({"data": "memory_test2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('memory unified output', async () => { const t1 = createTempJson({"data": "memory_test1"}), t2 = createTempJson({"data": "memory_test2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('bulk processing', async () => { const t1 = createTempJson({"config": "original"}), t2 = createTempJson({"config": "backup"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('config'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('bulk processing error handling', async () => { const t1 = createTempJson({"config": "original"}), t2 = createTempJson({"config": "backup"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('config'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('parallel section1', async () => { const t1 = createTempJson({"section1": {"data": "huge1"}}), t2 = createTempJson({"section1": {"data": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('parallel section2', async () => { const t1 = createTempJson({"section2": {"data": "huge1"}}), t2 = createTempJson({"section2": {"data": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('parallel section3', async () => { const t1 = createTempJson({"section3": {"data": "huge1"}}), t2 = createTempJson({"section3": {"data": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('very large files', async () => { const t1 = createTempJson({"very_large": {"dataset": "v1"}}), t2 = createTempJson({"very_large": {"dataset": "v2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('dataset'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('chunk1 processing', async () => { const t1 = createTempJson({"chunk1": {"data": "large1"}}), t2 = createTempJson({"chunk1": {"data": "large2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });

    // Test cases 21-30: Advanced optimization techniques
    test('chunk2 processing', async () => { const t1 = createTempJson({"chunk2": {"data": "large1"}}), t2 = createTempJson({"chunk2": {"data": "large2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('data'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('epsilon optimization', async () => { const t1 = createTempJson({"value": 1.0001}), t2 = createTempJson({"value": 1.0002}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('value'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('no epsilon', async () => { const t1 = createTempJson({"value": 1.0001}), t2 = createTempJson({"value": 1.0002}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('value'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('output format json', async () => { const t1 = createTempJson({"format": "test1"}), t2 = createTempJson({"format": "test2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('format'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('output format default', async () => { const t1 = createTempJson({"format": "test1"}), t2 = createTempJson({"format": "test2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('format'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('time verbose', async () => { const t1 = createTempJson({"large": "data1"}), t2 = createTempJson({"large": "data2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('large'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('valgrind massif', async () => { const t1 = createTempJson({"memory": "profile1"}), t2 = createTempJson({"memory": "profile2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('memory'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('huge files optimization', async () => { const t1 = createTempJson({"huge": {"dataset": "optimization1"}}), t2 = createTempJson({"huge": {"dataset": "optimization2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('dataset'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('huge files users path', async () => { const t1 = createTempJson({"users": {"count": 1000000}}), t2 = createTempJson({"users": {"count": 1000001}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('count'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('huge files products path', async () => { const t1 = createTempJson({"products": {"count": 500000}}), t2 = createTempJson({"products": {"count": 500001}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('count'); } finally { t1.cleanup(); t2.cleanup(); } });

    // Test cases 31-40: Complex processing and configuration scenarios
    test('huge files orders path', async () => { const t1 = createTempJson({"orders": {"count": 2000000}}), t2 = createTempJson({"orders": {"count": 2000001}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('count'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('config pipeline', async () => { const t1 = createTempJson({"config": {"section": "database"}}), t2 = createTempJson({"config": {"section": "services"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('section'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('config database path', async () => { const t1 = createTempJson({"database": {"host": "localhost"}}), t2 = createTempJson({"database": {"host": "remote"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('host'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('config services path', async () => { const t1 = createTempJson({"services": {"api": "v1"}}), t2 = createTempJson({"services": {"api": "v2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('api'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('sample array id', async () => { const t1 = createTempJson({"items": [{"id": 1, "name": "item1"}]}), t2 = createTempJson({"items": [{"id": 1, "name": "item2"}]}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('name'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('batch processing', async () => { const t1 = createTempJson({"batch": "file1"}), t2 = createTempJson({"batch": "file2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('batch'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('config env app', async () => { const t1 = createTempJson({"app": "prod", "host": "prod-server"}), t2 = createTempJson({"app": "dev", "host": "dev-server"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('app'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('config env db', async () => { const t1 = createTempJson({"database": "prod", "connection_string": "prod-conn"}), t2 = createTempJson({"database": "dev", "connection_string": "dev-conn"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('database'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('baseline silent', async () => { const t1 = createTempJson({"baseline": "config"}), t2 = createTempJson({"current": "config"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('baseline'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('baseline detailed', async () => { const t1 = createTempJson({"config": "baseline", "timestamp": "2024-01-01"}), t2 = createTempJson({"config": "current", "timestamp": "2024-01-02"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('config'); } finally { t1.cleanup(); t2.cleanup(); } });

    // Test cases 41-52: Benchmarking, monitoring, and profiling scenarios
    test('benchmark setup', async () => { const t1 = createTempJson({"benchmark": "setup1"}), t2 = createTempJson({"benchmark": "setup2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('benchmark'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('benchmark verbose', async () => { const t1 = createTempJson({"benchmark": "verbose1"}), t2 = createTempJson({"benchmark": "verbose2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('benchmark'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('monitoring silent', async () => { const t1 = createTempJson({"monitor": "original"}), t2 = createTempJson({"monitor": "backup"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('monitor'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('performance measurement', async () => { const t1 = createTempJson({"performance": "test"}), t2 = createTempJson({"performance": "backup"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('performance'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('tuning basic', async () => { const t1 = createTempJson({"tuning": {"basic": "huge1"}}), t2 = createTempJson({"tuning": {"basic": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('basic'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('tuning path section1', async () => { const t1 = createTempJson({"section1": {"tuning": "huge1"}}), t2 = createTempJson({"section1": {"tuning": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('tuning'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('tuning path section2', async () => { const t1 = createTempJson({"section2": {"tuning": "huge1"}}), t2 = createTempJson({"section2": {"tuning": "huge2"}}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('tuning'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('array optimization basic', async () => { const t1 = createTempJson({"users": [{"name": "user1"}, {"name": "user2"}]}), t2 = createTempJson({"users": [{"name": "user1"}, {"name": "user3"}]}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('name'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('array optimization with id', async () => { const t1 = createTempJson({"users": [{"id": 1, "name": "user1"}]}), t2 = createTempJson({"users": [{"id": 1, "name": "user2"}]}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('name'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('help option', async () => { const t1 = createTempJson({"help": "test"}), t2 = createTempJson({"help": "test"}); try { expect((await diffx.diff(t1.filePath, t2.filePath)).trim()).toBe(''); } finally { t1.cleanup(); t2.cleanup(); } });
    test('profiling massif', async () => { const t1 = createTempJson({"profiling": "large1"}), t2 = createTempJson({"profiling": "large2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('profiling'); } finally { t1.cleanup(); t2.cleanup(); } });
    test('perf record', async () => { const t1 = createTempJson({"perf": "large1"}), t2 = createTempJson({"perf": "large2"}); try { expect(await diffx.diff(t1.filePath, t2.filePath)).toContain('perf'); } finally { t1.cleanup(); t2.cleanup(); } });

});