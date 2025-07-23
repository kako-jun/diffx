const diffx = require('diffx');

test('examples example 1', () => {
    const result = diffx.diff('config_dev_content', 'config_prod_content', {
        ignoreKeysRegex: '^(host|port|password|secret_.*)',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 2', () => {
    const result = diffx.diff('app_config_v1_content', 'app_config_v2_content', {
        format: 'yaml',
        outputFormat: 'unified'
    });
    expect(result).toBeDefined();
});

test('examples example 3', () => {
    const result = diffx.diff('feature_flags_old_content', 'feature_flags_new_content', {
        pathFilter: 'features',
        showUnchanged: true
    });
    expect(result).toBeDefined();
});

test('examples example 4', () => {
    const result = diffx.diff('config_base_content', 'config_prod_content', {
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 5', () => {
    const result = diffx.diff('terraform_tfstate_content', 'terraform_tfstate_backup_content', {
        pathFilter: 'resources',
        ignoreKeysRegex: '^(last_updated|timeouts)',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 6', () => {
    const result = diffx.diff('deployment_v1_content', 'deployment_v2_content', {
        format: 'yaml',
        pathFilter: 'spec.containers'
    });
    expect(result).toBeDefined();
});

test('examples example 7', () => {
    const result = diffx.diff('docker_compose_content', 'docker_compose_prod_content', {
        format: 'yaml',
        ignoreKeysRegex: '^(build|volumes)'
    });
    expect(result).toBeDefined();
});

test('examples example 8', () => {
    const result = diffx.diff('ansible_inventory_old_content', 'ansible_inventory_new_content', {
        format: 'yaml',
        arrayIdKey: 'hostname'
    });
    expect(result).toBeDefined();
});

test('examples example 9', () => {
    const result = diffx.diff('helm_values_staging_content', 'helm_values_production_content', {
        format: 'yaml',
        ignoreKeys: ['image.tag', 'replicas']
    });
    expect(result).toBeDefined();
});

test('examples example 10', () => {
    const result = diffx.diff('ci_pipeline_old_content', 'ci_pipeline_new_content', {
        format: 'yaml',
        pathFilter: 'stages'
    });
    expect(result).toBeDefined();
});

test('examples example 11', () => {
    const result = diffx.diff('api_v1_spec_content', 'api_v2_spec_content', {
        pathFilter: 'paths',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 12', () => {
    const result = diffx.diff('expected_user_response_content', 'actual_response_content', {
        ignoreKeysRegex: '^(timestamp|request_id|server_time)',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 13', () => {
    const result = diffx.diff('postman_collection_old_content', 'postman_collection_new_content', {
        pathFilter: 'item',
        arrayIdKey: 'name'
    });
    expect(result).toBeDefined();
});

test('examples example 14', () => {
    const result = diffx.diff('graphql_schema_v1_content', 'graphql_schema_v2_content', {
        pathFilter: 'data.__schema.types',
        arrayIdKey: 'name'
    });
    expect(result).toBeDefined();
});

test('examples example 15', () => {
    const result = diffx.diff('soap_response_expected_content', 'soap_response_actual_content', {
        format: 'xml',
        ignoreKeys: ['MessageID', 'Timestamp']
    });
    expect(result).toBeDefined();
});

test('examples example 16', () => {
    const result = diffx.diff('input_data_sample_content', 'output_data_sample_content', {
        arrayIdKey: 'record_id',
        epsilon: 0.001,
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 17', () => {
    const result = diffx.diff('batch_job_input_content', 'batch_job_output_content', {
        format: 'csv',
        ignoreKeys: ['processing_time', 'job_id']
    });
    expect(result).toBeDefined();
});

test('examples example 18', () => {
    const result = diffx.diff('raw_logs_content', 'processed_logs_content', {
        arrayIdKey: 'event_id',
        pathFilter: 'events'
    });
    expect(result).toBeDefined();
});

test('examples example 19', () => {
    const result = diffx.diff('data_warehouse_snapshot_old_content', 'data_warehouse_snapshot_new_content', {
        maxDepth: 3,
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 20', () => {
    const result = diffx.diff('ml_features_v1_content', 'ml_features_v2_content', {
        epsilon: 0.0001,
        showTypes: true
    });
    expect(result).toBeDefined();
});

test('examples example 21', () => {
    const result = diffx.diff('db_schema_dump_old_content', 'db_schema_dump_new_content', {
        format: 'sql',
        outputFormat: 'unified'
    });
    expect(result).toBeDefined();
});

test('examples example 22', () => {
    const result = diffx.diff('db_permissions_old_content', 'db_permissions_new_content', {
        pathFilter: 'grants',
        arrayIdKey: 'user'
    });
    expect(result).toBeDefined();
});

test('examples example 23', () => {
    const result = diffx.diff('query_plan_old_content', 'query_plan_new_content', {
        ignoreKeysRegex: '^(cost|time|buffers)'
    });
    expect(result).toBeDefined();
});

test('examples example 24', () => {
    const result = diffx.diff('database_config_dev_content', 'database_config_prod_content', {
        format: 'ini',
        ignoreKeys: ['connection_string']
    });
    expect(result).toBeDefined();
});

test('examples example 25', () => {
    const result = diffx.diff('prometheus_alerts_old_content', 'prometheus_alerts_new_content', {
        format: 'yaml',
        pathFilter: 'groups.*.rules',
        arrayIdKey: 'alert'
    });
    expect(result).toBeDefined();
});

test('examples example 26', () => {
    const result = diffx.diff('grafana_dashboard_v1_content', 'grafana_dashboard_v2_content', {
        pathFilter: 'panels',
        arrayIdKey: 'id',
        ignoreKeys: ['version', 'uid']
    });
    expect(result).toBeDefined();
});

test('examples example 27', () => {
    const result = diffx.diff('cloudwatch_alarms_old_content', 'cloudwatch_alarms_new_content', {
        pathFilter: 'MetricAlarms',
        arrayIdKey: 'AlarmName'
    });
    expect(result).toBeDefined();
});

test('examples example 28', () => {
    const result = diffx.diff('elastic_mapping_old_content', 'elastic_mapping_new_content', {
        pathFilter: 'mappings.properties',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 29', () => {
    const result = diffx.diff('logging_config_v1_content', 'logging_config_v2_content', {
        pathFilter: 'handlers',
        showUnchanged: true
    });
    expect(result).toBeDefined();
});

test('examples example 30', () => {
    const result = diffx.diff('package_json_content', 'package_lock_json_content', {
        pathFilter: 'dependencies',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 31', () => {
    const result = diffx.diff('test_results_old_content', 'test_results_new_content', {
        format: 'xml',
        pathFilter: 'testsuites.testsuite',
        arrayIdKey: 'name'
    });
    expect(result).toBeDefined();
});

test('examples example 32', () => {
    const result = diffx.diff('code_metrics_before_content', 'code_metrics_after_content', {
        epsilon: 0.01,
        pathFilter: 'metrics'
    });
    expect(result).toBeDefined();
});

test('examples example 33', () => {
    const result = diffx.diff('eslint_config_old_content', 'eslint_config_new_content', {
        pathFilter: 'rules',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 34', () => {
    const result = diffx.diff('gitignore_content', 'gitignore_new_content', {
        format: 'text',
        outputFormat: 'unified'
    });
    expect(result).toBeDefined();
});

test('examples example 35', () => {
    const result = diffx.diff('security_policy_v1_content', 'security_policy_v2_content', {
        pathFilter: 'permissions',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 36', () => {
    const result = diffx.diff('ssl_cert_old_content', 'ssl_cert_new_content', {
        ignoreKeysRegex: '^(serial_number|thumbprint)',
        showTypes: true
    });
    expect(result).toBeDefined();
});

test('examples example 37', () => {
    const result = diffx.diff('firewall_rules_old_content', 'firewall_rules_new_content', {
        arrayIdKey: 'rule_id',
        pathFilter: 'inbound_rules'
    });
    expect(result).toBeDefined();
});

test('examples example 38', () => {
    const result = diffx.diff('compliance_report_q1_content', 'compliance_report_q2_content', {
        pathFilter: 'findings',
        arrayIdKey: 'finding_id'
    });
    expect(result).toBeDefined();
});

test('examples example 39', () => {
    const result = diffx.diff('audit_log_config_old_content', 'audit_log_config_new_content', {
        format: 'yaml',
        pathFilter: 'events',
        showUnchanged: true
    });
    expect(result).toBeDefined();
});

test('examples example 40', () => {
    const result = diffx.diff('performance_baseline_content', 'performance_current_content', {
        epsilon: 0.05,
        pathFilter: 'metrics',
        outputFormat: 'json'
    });
    expect(result).toBeDefined();
});

test('examples example 41', () => {
    const result = diffx.diff('cache_config_old_content', 'cache_config_new_content', {
        pathFilter: 'redis.settings',
        ignoreKeys: ['last_flush_time']
    });
    expect(result).toBeDefined();
});