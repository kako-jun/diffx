const diffx = require('diffx');

test('comparison example 1', () => {
    const result = diffx.diff('config_v1_content', 'config_v2_content');
    expect(result).toBeDefined();
});

test('comparison example 2', () => {
    const result = diffx.diff('file1_content', 'file2_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('comparison example 3', () => {
    const result = diffx.diff('file1_yaml_content', 'file2_yaml_content');
    expect(result).toBeDefined();
});

test('comparison example 4', () => {
    const result = diffx.diff('data1_csv_content', 'data2_csv_content', { arrayIdKey: 'id' });
    expect(result).toBeDefined();
});

test('comparison example 5', () => {
    const result = diffx.diff('file1_json_content', 'file2_json_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('comparison example 6', () => {
    const result = diffx.diff('stdin_content', 'config_content');
    expect(result).toBeDefined();
});

test('comparison example 7', () => {
    const result = diffx.diff('config1_content', 'config2_content', { outputFormat: 'unified' });
    expect(result).toBeDefined();
});

test('comparison example 8', () => {
    const result = diffx.diff('config1_content', 'config2_content');
    expect(result).toBeDefined();
});

test('comparison example 9', () => {
    const result = diffx.diff('file1_json_content', 'file2_json_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});