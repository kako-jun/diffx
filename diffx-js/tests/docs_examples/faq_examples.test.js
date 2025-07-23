const diffx = require('diffx');

test('faq example 1', () => {
    const result = diffx.diff('stdin_content', 'other_data_content', { format: 'json' });
    expect(result).toBeDefined();
});

test('faq example 2', () => {
    const result = diffx.diff('file1_content', 'file2_content', { ignoreKeysRegex: '^id$|^timestamp$' });
    expect(result).toBeDefined();
});

test('faq example 3', () => {
    const result = diffx.diff('data1_content', 'data2_content', { epsilon: 0.00001 });
    expect(result).toBeDefined();
});

test('faq example 4', () => {
    const result = diffx.diff('users1_content', 'users2_content', { arrayIdKey: 'uuid' });
    expect(result).toBeDefined();
});

test('faq example 5', () => {
    const result = diffx.diff('file1_content', 'file2_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('faq example 6', () => {
    const result = diffx.diff('file1_yaml_content', 'file2_yaml_content', { format: 'yaml' });
    expect(result).toBeDefined();
});