const diffx = require('diffx');

test('api reference example 1', () => {
    const result = diffx.diff('content1', 'content2');
    expect(result).toBeDefined();
});

test('api reference example 2', () => {
    const result = diffx.parseIni('key=value');
    expect(result).toBeDefined();
});

test('api reference example 3', () => {
    const result = diffx.parseXml('<root></root>');
    expect(result).toBeDefined();
});

test('api reference example 4', () => {
    const result = diffx.parseCsv('col1,col2\nval1,val2');
    expect(result).toBeDefined();
});

test('api reference example 5', () => {
    const result = diffx.valueTypeName('test');
    expect(result).toBeDefined();
});

test('api reference example 6', () => {
    const result = diffx.diffWithEpsilon('1.0', '1.001', 0.001);
    expect(result).toBeDefined();
});

test('api reference example 7', () => {
    const result = diffx.diffWithRegexFilter('content1', 'content2', 'pattern');
    expect(result).toBeDefined();
});

test('api reference example 8', () => {
    const result = diffx.diffWithArrayId('content1', 'content2', 'id');
    expect(result).toBeDefined();
});

test('api reference example 9', () => {
    const result = diffx.processPipeline('content1', 'content2');
    expect(result).toBeDefined();
});

test('api reference example 10', () => {
    const result = diffx.customDiffProcessor('content1', 'content2');
    expect(result).toBeDefined();
});

test('api reference example 11', async () => {
    const result = await diffx.asyncDiff('content1', 'content2');
    expect(result).toBeDefined();
});

test('api reference example 12', () => {
    expect(() => {
        diffx.parseIni('invalid content');
    }).toThrow();
});

test('api reference example 13', () => {
    const result = diffx.robustDiff('content1', 'content2');
    expect(result).toBeDefined();
});

test('api reference example 14', () => {
    const result = diffx.largeDataDiff('large_content1', 'large_content2');
    expect(result).toBeDefined();
});

test('api reference example 15', () => {
    const result = diffx.diff('test1', 'test2');
    expect(result).toBeDefined();
});

test('api reference example 16', () => {
    const result = diffx.diffWithEpsilon('1.0', '1.0001', 0.001);
    expect(result).toBeDefined();
});

test('api reference example 17', () => {
    const diffResult = diffx.DiffResult.Added('new_value');
    expect(diffResult).toBeDefined();
});

test('api reference example 18', () => {
    const diffResult = diffx.DiffResult.Modified('old', 'new');
    expect(diffResult).toBeDefined();
});

test('api reference example 19', () => {
    const diffResult = diffx.DiffResult.TypeChanged('String', 'Number');
    expect(diffResult).toBeDefined();
});

test('api reference example 20', () => {
    const result = diffx.parseIni('[section]\nkey=value');
    expect(result).toBeDefined();
});

test('api reference example 21', () => {
    const result = diffx.parseXml('<root><child>value</child></root>');
    expect(result).toBeDefined();
});

test('api reference example 22', () => {
    const result = diffx.parseCsv('header1,header2\nvalue1,value2');
    expect(result).toBeDefined();
});

test('api reference example 23', () => {
    const typeName = diffx.valueTypeName('example');
    expect(typeName).toBe('string');
});