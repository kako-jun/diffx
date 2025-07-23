const diffx = require('diffx');

test('performance benchmarks example 1', () => {
    const result = diffx.benchmarkDiff('content1', 'content2');
    expect(result).toBeDefined();
});

test('performance benchmarks example 2', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('performance benchmarks example 3', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { iterations: 100 });
    expect(result).toBeDefined();
});

test('performance benchmarks example 4', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { memoryProfile: true });
    expect(result).toBeDefined();
});

test('performance benchmarks example 5', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { cpuProfile: true });
    expect(result).toBeDefined();
});

test('performance benchmarks example 6', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { warmup: 10 });
    expect(result).toBeDefined();
});

test('performance benchmarks example 7', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { noColor: true });
    expect(result).toBeDefined();
});

test('performance benchmarks example 8', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { algorithm: 'myers' });
    expect(result).toBeDefined();
});

test('performance benchmarks example 9', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { threads: 4 });
    expect(result).toBeDefined();
});

test('performance benchmarks example 10', () => {
    const result = diffx.benchmarkDiff('content1', 'content2', { saveResults: 'benchmark_results.json' });
    expect(result).toBeDefined();
});