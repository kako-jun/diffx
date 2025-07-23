const diffx = require('diffx');

test('cli reference example 1', () => {
    const result = diffx.diff('config_content', 'config_new_content');
    expect(result).toBeDefined();
});

test('cli reference example 2', () => {
    const result = diffx.diff('content1', 'content2', { format: 'json' });
    expect(result).toBeDefined();
});

test('cli reference example 3', () => {
    const result = diffx.diff('content1', 'content2', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('cli reference example 4', () => {
    const result = diffx.diff('content1', 'content2', { pathFilter: 'database' });
    expect(result).toBeDefined();
});

test('cli reference example 5', () => {
    const result = diffx.diff('content1', 'content2', { ignoreKeysRegex: '^(timestamp|createdAt|updatedAt)$' });
    expect(result).toBeDefined();
});

test('cli reference example 6', () => {
    const result = diffx.diff('content1', 'content2', { arrayIdKey: 'id' });
    expect(result).toBeDefined();
});

test('cli reference example 7', () => {
    const result = diffx.diffDirectories('configs', 'configs.backup', { recursive: true });
    expect(result).toBeDefined();
});

test('cli reference example 8', () => {
    const result = diffx.diff('content1', 'content2', { 
        ignoreCase: true, 
        ignoreWhitespace: true, 
        epsilon: 0.001, 
        ignoreKeysRegex: '^(timestamp|version)$' 
    });
    expect(result).toBeDefined();
});

test('cli reference example 9', () => {
    const result = diffx.diff('content1', 'content2', { 
        ignoreKeysRegex: '^(deployment_time|build_id)', 
        outputFormat: 'json' 
    });
    expect(result).toBeDefined();
});

test('cli reference example 10', () => {
    const help = diffx.getHelp();
    expect(help).toBeDefined();
});

test('cli reference example 11', () => {
    const version = diffx.getVersion();
    expect(version).toBeDefined();
});

test('cli reference example 12', () => {
    const result = diffx.diff('content1', 'content2', { verbose: true });
    expect(result).toBeDefined();
});

test('cli reference example 13', () => {
    const result = diffx.diff('content1', 'content2', { quiet: true });
    expect(result).toBeDefined();
});

test('cli reference example 14', () => {
    const result = diffx.diff('content1', 'content2', { noColor: true });
    expect(result).toBeDefined();
});

test('cli reference example 15', () => {
    const result = diffx.diff('content1', 'content2', { color: 'always' });
    expect(result).toBeDefined();
});

test('cli reference example 16', () => {
    const result = diffx.diff('content1', 'content2', { color: 'never' });
    expect(result).toBeDefined();
});

test('cli reference example 17', () => {
    const result = diffx.diff('content1', 'content2', { color: 'auto' });
    expect(result).toBeDefined();
});

test('cli reference example 18', () => {
    const result = diffx.diff('content1', 'content2', { context: 3 });
    expect(result).toBeDefined();
});

test('cli reference example 19', () => {
    const result = diffx.diff('content1', 'content2', { unified: true });
    expect(result).toBeDefined();
});

test('cli reference example 20', () => {
    const result = diffx.diff('content1', 'content2', { sideBySide: true });
    expect(result).toBeDefined();
});

test('cli reference example 21', () => {
    const result = diffx.diff('content1', 'content2', { ignoreCase: true });
    expect(result).toBeDefined();
});

test('cli reference example 22', () => {
    const result = diffx.diff('content1', 'content2', { ignoreWhitespace: true });
    expect(result).toBeDefined();
});

test('cli reference example 23', () => {
    const result = diffx.diff('content1', 'content2', { ignoreBlankLines: true });
    expect(result).toBeDefined();
});

test('cli reference example 24', () => {
    const result = diffx.diff('content1', 'content2', { ignoreTrailingWhitespace: true });
    expect(result).toBeDefined();
});

test('cli reference example 25', () => {
    const result = diffx.diff('content1', 'content2', { ignoreAllSpace: true });
    expect(result).toBeDefined();
});

test('cli reference example 26', () => {
    const result = diffx.diff('content1', 'content2', { epsilon: 0.01 });
    expect(result).toBeDefined();
});

test('cli reference example 27', () => {
    const result = diffx.diff('content1', 'content2', { ignoreKeys: ['timestamp', 'version'] });
    expect(result).toBeDefined();
});

test('cli reference example 28', () => {
    const result = diffx.diff('content1', 'content2', { ignoreValues: ['null'] });
    expect(result).toBeDefined();
});

test('cli reference example 29', () => {
    const result = diffx.diff('content1', 'content2', { includeOnly: ['data', 'config'] });
    expect(result).toBeDefined();
});

test('cli reference example 30', () => {
    const result = diffx.diff('content1', 'content2', { maxDepth: 5 });
    expect(result).toBeDefined();
});

test('cli reference example 31', () => {
    const result = diffx.diff('content1', 'content2', { showUnchanged: true });
    expect(result).toBeDefined();
});

test('cli reference example 32', () => {
    const result = diffx.diff('content1', 'content2', { showTypes: true });
    expect(result).toBeDefined();
});

test('cli reference example 33', () => {
    const result = diffx.diff('content1', 'content2', { lineNumbers: true });
    expect(result).toBeDefined();
});

test('cli reference example 34', () => {
    const result = diffx.diff('content1', 'content2', { wordDiff: true });
    expect(result).toBeDefined();
});

test('cli reference example 35', () => {
    const result = diffx.diff('content1', 'content2', { charDiff: true });
    expect(result).toBeDefined();
});

test('cli reference example 36', () => {
    const result = diffx.diffDirectories('configs', 'configs.backup', { exclude: ['*.log'] });
    expect(result).toBeDefined();
});

test('cli reference example 37', () => {
    const result = diffx.diffDirectories('configs', 'configs.backup', { include: ['*.json'] });
    expect(result).toBeDefined();
});

test('cli reference example 38', () => {
    const result = diffx.diffDirectories('configs', 'configs.backup', { followSymlinks: true });
    expect(result).toBeDefined();
});

test('cli reference example 39', () => {
    const result = diffx.diff('content1', 'content2', { threads: 4 });
    expect(result).toBeDefined();
});

test('cli reference example 40', () => {
    const result = diffx.diff('large_content1', 'large_content2', { memoryLimit: '1G' });
    expect(result).toBeDefined();
});

test('cli reference example 41', () => {
    const result = diffx.diff('content1', 'content2', { cacheEnabled: true });
    expect(result).toBeDefined();
});

test('cli reference example 42', () => {
    const result = diffx.diff('content1', 'content2', { streaming: true });
    expect(result).toBeDefined();
});

test('cli reference example 43', () => {
    const config = diffx.getConfig();
    expect(config).toBeDefined();
});

test('cli reference example 44', () => {
    const formats = diffx.listFormats();
    expect(formats).toBeDefined();
});

test('cli reference example 45', () => {
    const examples = diffx.getExamples();
    expect(examples).toBeDefined();
});

test('cli reference example 46', () => {
    const completions = diffx.generateCompletions('bash');
    expect(completions).toBeDefined();
});

test('cli reference example 47', () => {
    const result = diffx.diff('api_v1_content', 'api_v2_content', { ignoreKeysRegex: '^(version|timestamp)$' });
    expect(result).toBeDefined();
});

test('cli reference example 48', () => {
    const result = diffx.diff('schema_old_content', 'schema_new_content', { showTypes: true });
    expect(result).toBeDefined();
});

test('cli reference example 49', () => {
    const result = diffx.diff('config_dev_content', 'config_prod_content', { ignoreKeys: ['environment', 'debug'] });
    expect(result).toBeDefined();
});

test('cli reference example 50', () => {
    const result = diffx.diff('users_backup_content', 'users_current_content', { arrayIdKey: 'user_id' });
    expect(result).toBeDefined();
});

test('cli reference example 51', () => {
    const result = diffx.diff('metrics_content', 'metrics_new_content', { epsilon: 0.001 });
    expect(result).toBeDefined();
});

test('cli reference example 52', () => {
    const result = diffx.diff('build_content', 'build_new_content', { ignoreKeysRegex: '^(build_time|git_hash)$' });
    expect(result).toBeDefined();
});

test('cli reference example 53', () => {
    const result = diffx.diff('test_results_content', 'test_results_new_content', { format: 'xml' });
    expect(result).toBeDefined();
});

test('cli reference example 54', () => {
    const result = diffx.diff('packages_content', 'packages_updated_content', { pathFilter: 'dependencies' });
    expect(result).toBeDefined();
});

test('cli reference example 55', () => {
    const result = diffx.diff('content1', 'content2', { algorithm: 'myers' });
    expect(result).toBeDefined();
});

test('cli reference example 56', () => {
    const result = diffx.diff('content1', 'content2', { algorithm: 'patience' });
    expect(result).toBeDefined();
});

test('cli reference example 57', () => {
    const result = diffx.diff('content1', 'content2', { algorithm: 'histogram' });
    expect(result).toBeDefined();
});

test('cli reference example 58', () => {
    const result = diffx.diff('content1', 'content2', { benchmark: true });
    expect(result).toBeDefined();
});

test('cli reference example 59', () => {
    const result = diffx.diff('content1', 'content2', { profile: true });
    expect(result).toBeDefined();
});

test('cli reference example 60', () => {
    const result = diffx.diff('content1', 'content2', { debug: true });
    expect(result).toBeDefined();
});

test('cli reference example 61', () => {
    const result = diffx.diff('content1', 'content2', { trace: true });
    expect(result).toBeDefined();
});

test('cli reference example 62', () => {
    const result = diffx.diff('content1', 'content2', { timing: true });
    expect(result).toBeDefined();
});

test('cli reference example 63', () => {
    const result = diffx.diff('content1', 'content2', { stats: true });
    expect(result).toBeDefined();
});

test('cli reference example 64', () => {
    const result = diffx.diff('content1', 'content2', { outputFile: 'results.json' });
    expect(result).toBeDefined();
});

test('cli reference example 65', () => {
    const result = diffx.diff('content1', 'content2', { patchFormat: true });
    expect(result).toBeDefined();
});

test('cli reference example 66', () => {
    const result = diffx.diff('content1', 'content2', { summaryOnly: true });
    expect(result).toBeDefined();
});

test('cli reference example 67', () => {
    const result = diffx.diff('content1', 'content2', { exitCode: true });
    expect(result).toBeDefined();
});

test('cli reference example 68', () => {
    const result = diffx.diff('content1', 'content2', { machineReadable: true });
    expect(result).toBeDefined();
});

test('cli reference example 69', () => {
    const result = diffx.diff('content1', 'content2', { checkSyntax: true });
    expect(result).toBeDefined();
});

test('cli reference example 70', () => {
    const result = diffx.diff('content1', 'content2', { validate: true });
    expect(result).toBeDefined();
});