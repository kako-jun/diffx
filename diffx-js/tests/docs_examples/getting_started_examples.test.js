const diffx = require('diffx');

test('getting started example 1', () => {
    const result = diffx.diff('config_v1_content', 'config_v2_content');
    expect(result).toBeDefined();
});

test('getting started example 2', () => {
    const result = diffx.diff('docker_compose_content', 'docker_compose_new_content');
    expect(result).toBeDefined();
});

test('getting started example 3', () => {
    const result = diffx.diff('cargo_toml_content', 'cargo_toml_backup_content');
    expect(result).toBeDefined();
});

test('getting started example 4', () => {
    const result = diffx.diff('settings_xml_content', 'settings_new_xml_content');
    expect(result).toBeDefined();
});

test('getting started example 5', () => {
    const result = diffx.diff('database_ini_content', 'database_prod_ini_content');
    expect(result).toBeDefined();
});

test('getting started example 6', () => {
    const result = diffx.diff('users_csv_content', 'users_updated_csv_content');
    expect(result).toBeDefined();
});

test('getting started example 7', () => {
    const result = diffx.diff('app_json_content', 'app_new_json_content');
    expect(result).toBeDefined();
});

test('getting started example 8', () => {
    const result = diffx.diff('config_yaml_content', 'config_yml_content');
    expect(result).toBeDefined();
});

test('getting started example 9', () => {
    const result = diffx.diff('settings_toml_content', 'backup_toml_content');
    expect(result).toBeDefined();
});

test('getting started example 10', () => {
    const result = diffx.diff('file1_content', 'file2_content', { format: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 11', () => {
    const result = diffx.diff('file1_txt_content', 'file2_txt_content', { format: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 12', () => {
    const result = diffx.diff('config_json_content', 'stdin_content');
    expect(result).toBeDefined();
});

test('getting started example 13', () => {
    const result = diffx.diff('stdin_content', 'container2_inspect_content');
    expect(result).toBeDefined();
});

test('getting started example 14', () => {
    const result = diffx.diff('stdin_content', 'config_v2_content', { format: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 15', () => {
    const result = diffx.diffDirectories('config_dir1', 'config_dir2', { recursive: true });
    expect(result).toBeDefined();
});

test('getting started example 16', () => {
    const result = diffx.diffDirectories('configs', 'configs_backup', { recursive: true, format: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 17', () => {
    const result = diffx.diff('app_content', 'app_new_content', { ignoreKeysRegex: '^(timestamp|_.*|createdAt)$' });
    expect(result).toBeDefined();
});

test('getting started example 18', () => {
    const result = diffx.diff('package_content', 'package_new_content', { ignoreKeysRegex: 'version|buildNumber' });
    expect(result).toBeDefined();
});

test('getting started example 19', () => {
    const result = diffx.diff('users_content', 'users_updated_content', { arrayIdKey: 'id' });
    expect(result).toBeDefined();
});

test('getting started example 20', () => {
    const result = diffx.diff('inventory_content', 'inventory_new_content', { arrayIdKey: 'sku' });
    expect(result).toBeDefined();
});

test('getting started example 21', () => {
    const result = diffx.diff('records_content', 'records_new_content', { arrayIdKey: 'pk' });
    expect(result).toBeDefined();
});

test('getting started example 22', () => {
    const result = diffx.diff('metrics_content', 'metrics_new_content', { epsilon: 0.001 });
    expect(result).toBeDefined();
});

test('getting started example 23', () => {
    const result = diffx.diff('measurements_content', 'measurements_new_content', { epsilon: 0.01 });
    expect(result).toBeDefined();
});

test('getting started example 24', () => {
    const result = diffx.diff('config_content', 'config_new_content', { pathFilter: 'database' });
    expect(result).toBeDefined();
});

test('getting started example 25', () => {
    const result = diffx.diff('config_content', 'config_new_content', { pathFilter: 'servers[0]' });
    expect(result).toBeDefined();
});

test('getting started example 26', () => {
    const result = diffx.diff('app_content', 'app_new_content', { pathFilter: 'microservices.auth.database.connection' });
    expect(result).toBeDefined();
});

test('getting started example 27', () => {
    const result = diffx.diff('config_content', 'config_new_content');
    expect(result).toBeDefined();
});

test('getting started example 28', () => {
    const result = diffx.diff('config_content', 'config_new_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 29', () => {
    const result = diffx.diff('config_content', 'config_new_content', { outputFormat: 'yaml' });
    expect(result).toBeDefined();
});

test('getting started example 30', () => {
    const result = diffx.diff('config_content', 'config_new_content', { outputFormat: 'unified' });
    expect(result).toBeDefined();
});

test('getting started example 31', () => {
    const result = diffx.diff('application_properties_content', 'application_prod_properties_content');
    expect(result).toBeDefined();
});

test('getting started example 32', () => {
    const result = diffx.diff('config_content', 'config_prod_content', { ignoreKeysRegex: '^(host|password|apiKey)' });
    expect(result).toBeDefined();
});

test('getting started example 33', () => {
    const result = diffx.diff('expected_output_content', 'actual_output_content', { outputFormat: 'json' });
    expect(result).toBeDefined();
});

test('getting started example 34', () => {
    const result = diffx.diff('schema_v1_content', 'schema_v2_content', { format: 'sql' });
    expect(result).toBeDefined();
});

test('getting started example 35', () => {
    const result = diffx.diff('users_export_content', 'users_import_content', { arrayIdKey: 'email' });
    expect(result).toBeDefined();
});

test('getting started example 36', () => {
    const result = diffx.diff('package_content', 'package_lock_content', { pathFilter: 'dependencies' });
    expect(result).toBeDefined();
});

test('getting started example 37', () => {
    const result = diffx.diff('terraform_tfstate_content', 'terraform_tfstate_backup_content');
    expect(result).toBeDefined();
});

test('getting started example 38', () => {
    const result = diffx.diff('openapi_v1_content', 'openapi_v2_content', { pathFilter: 'paths' });
    expect(result).toBeDefined();
});

test('getting started example 39', () => {
    const result = diffx.diff('benchmark_baseline_content', 'benchmark_current_content', { epsilon: 0.05 });
    expect(result).toBeDefined();
});

test('getting started example 40', () => {
    const result = diffx.diff('config_dev_content', 'config_prod_content', { 
        outputFormat: 'json', 
        ignoreKeysRegex: '^(debug|test_)' 
    });
    expect(result).toBeDefined();
});

test('getting started example 41', () => {
    const result = diffx.diff('api_response_expected_content', 'api_response_actual_content', { 
        ignoreKeysRegex: '^(timestamp|requestId)' 
    });
    expect(result).toBeDefined();
});

test('getting started example 42', () => {
    const result = diffx.diff('k8s_deployment_content', 'k8s_deployment_new_content', { 
        pathFilter: 'spec.template.spec.containers' 
    });
    expect(result).toBeDefined();
});

test('getting started example 43', () => {
    const result = diffx.diff('prometheus_rules_content', 'prometheus_rules_new_content', { arrayIdKey: 'alert' });
    expect(result).toBeDefined();
});

test('getting started example 44', () => {
    const result = diffx.diff('eslint_config_content', 'eslint_config_new_content', { pathFilter: 'rules' });
    expect(result).toBeDefined();
});

test('getting started example 45', () => {
    const result = diffx.diff('ml_model_params_v1_content', 'ml_model_params_v2_content', { epsilon: 0.0001 });
    expect(result).toBeDefined();
});

test('getting started example 46', () => {
    const result = diffx.diffDirectories('db_migrations', 'db_migrations_new', { recursive: true, format: 'sql' });
    expect(result).toBeDefined();
});

test('getting started example 47', () => {
    const result = diffx.diff('graphql_schema_content', 'graphql_schema_new_content', { 
        pathFilter: 'data.__schema.types', 
        arrayIdKey: 'name' 
    });
    expect(result).toBeDefined();
});

test('getting started example 48', () => {
    const result = diffx.diff('nginx_conf_content', 'nginx_new_conf_content', { format: 'conf' });
    expect(result).toBeDefined();
});

test('getting started example 49', () => {
    const result = diffx.diff('ansible_inventory_content', 'ansible_inventory_new_content', { format: 'ini' });
    expect(result).toBeDefined();
});

test('getting started example 50', () => {
    const result = diffx.diff('helm_values_content', 'helm_values_prod_content', { 
        ignoreKeys: ['image.tag', 'replicas'] 
    });
    expect(result).toBeDefined();
});

test('getting started example 51', () => {
    const result = diffx.diff('security_policy_content', 'security_policy_new_content', { 
        pathFilter: 'rules', 
        arrayIdKey: 'ruleId' 
    });
    expect(result).toBeDefined();
});

test('getting started example 52', () => {
    const result = diffx.diff('test_results_content', 'test_results_new_content', { 
        format: 'xml', 
        pathFilter: 'testsuites.testsuite' 
    });
    expect(result).toBeDefined();
});

test('getting started example 53', () => {
    const result = diffx.diff('monitoring_config_content', 'monitoring_config_new_content', { format: 'toml' });
    expect(result).toBeDefined();
});

test('getting started example 54', () => {
    const result = diffx.diff('feature_flags_content', 'feature_flags_new_content', { 
        pathFilter: 'flags', 
        showUnchanged: true 
    });
    expect(result).toBeDefined();
});

test('getting started example 55', () => {
    const result = diffx.diff('load_test_results_content', 'load_test_results_new_content', { 
        epsilon: 0.1, 
        pathFilter: 'metrics' 
    });
    expect(result).toBeDefined();
});

test('getting started example 56', () => {
    const result = diffx.diff('translation_en_content', 'translation_en_new_content', { 
        outputFormat: 'json', 
        showTypes: true 
    });
    expect(result).toBeDefined();
});

test('getting started example 57', () => {
    const result = diffx.diff('ci_pipeline_content', 'ci_pipeline_new_content', { 
        pathFilter: 'jobs', 
        arrayIdKey: 'name' 
    });
    expect(result).toBeDefined();
});

test('getting started example 58', () => {
    const result = diffx.diff('audit_log_content', 'audit_log_new_content', { 
        ignoreKeysRegex: '^(timestamp|eventId)', 
        arrayIdKey: 'userId' 
    });
    expect(result).toBeDefined();
});