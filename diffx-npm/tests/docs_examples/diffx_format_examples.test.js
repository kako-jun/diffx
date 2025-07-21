const diffx = require('diffx');

test('diffx format example 1', () => {
    const result = diffx.diff('infrastructure_content', 'infrastructure_new_content');
    expect(result).toBeDefined();
});

test('diffx format example 2', () => {
    const result = diffx.diff('api_v1_content', 'api_v2_content', { pathFilter: 'paths' });
    expect(result).toBeDefined();
});

test('diffx format example 3', () => {
    const result = diffx.diff('expected_output_content', 'actual_output_content', { arrayIdKey: 'id' });
    expect(result).toBeDefined();
});

test('diffx format example 4', () => {
    const result = diffx.diff('config_content', 'config_new_content');
    expect(result).toBeDefined();
});