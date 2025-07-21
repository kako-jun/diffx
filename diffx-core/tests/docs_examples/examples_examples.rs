use diffx_core::{DiffEngine, DiffConfig};

#[test]
fn examples_example_1() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(host|port|password|secret_.*)".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("config_dev_content", "config_prod_content", &config);
}

#[test]
fn examples_example_2() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.output_format = "unified".to_string();
    let _result = engine.diff_with_config("app_config_v1_content", "app_config_v2_content", &config);
}

#[test]
fn examples_example_3() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("features".to_string());
    config.show_unchanged = true;
    let _result = engine.diff_with_config("feature_flags_old_content", "feature_flags_new_content", &config);
}

#[test]
fn examples_example_4() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("config_base_content", "config_prod_content", &config);
}

#[test]
fn examples_example_5() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("resources".to_string());
    config.ignore_keys_regex = Some("^(last_updated|timeouts)".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("terraform_tfstate_content", "terraform_tfstate_backup_content", &config);
}

#[test]
fn examples_example_6() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.path_filter = Some("spec.containers".to_string());
    let _result = engine.diff_with_config("deployment_v1_content", "deployment_v2_content", &config);
}

#[test]
fn examples_example_7() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.ignore_keys_regex = Some("^(build|volumes)".to_string());
    let _result = engine.diff_with_config("docker_compose_content", "docker_compose_prod_content", &config);
}

#[test]
fn examples_example_8() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.array_id_key = Some("hostname".to_string());
    let _result = engine.diff_with_config("ansible_inventory_old_content", "ansible_inventory_new_content", &config);
}

#[test]
fn examples_example_9() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.ignore_keys = vec!["image.tag".to_string(), "replicas".to_string()];
    let _result = engine.diff_with_config("helm_values_staging_content", "helm_values_production_content", &config);
}

#[test]
fn examples_example_10() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.path_filter = Some("stages".to_string());
    let _result = engine.diff_with_config("ci_pipeline_old_content", "ci_pipeline_new_content", &config);
}

#[test]
fn examples_example_11() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("paths".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("api_v1_spec_content", "api_v2_spec_content", &config);
}

#[test]
fn examples_example_12() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(timestamp|request_id|server_time)".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("expected_user_response_content", "actual_response_content", &config);
}

#[test]
fn examples_example_13() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("item".to_string());
    config.array_id_key = Some("name".to_string());
    let _result = engine.diff_with_config("postman_collection_old_content", "postman_collection_new_content", &config);
}

#[test]
fn examples_example_14() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("data.__schema.types".to_string());
    config.array_id_key = Some("name".to_string());
    let _result = engine.diff_with_config("graphql_schema_v1_content", "graphql_schema_v2_content", &config);
}

#[test]
fn examples_example_15() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "xml".to_string();
    config.ignore_keys = vec!["MessageID".to_string(), "Timestamp".to_string()];
    let _result = engine.diff_with_config("soap_response_expected_content", "soap_response_actual_content", &config);
}

#[test]
fn examples_example_16() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("record_id".to_string());
    config.epsilon = Some(0.001);
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("input_data_sample_content", "output_data_sample_content", &config);
}

#[test]
fn examples_example_17() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "csv".to_string();
    config.ignore_keys = vec!["processing_time".to_string(), "job_id".to_string()];
    let _result = engine.diff_with_config("batch_job_input_content", "batch_job_output_content", &config);
}

#[test]
fn examples_example_18() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("event_id".to_string());
    config.path_filter = Some("events".to_string());
    let _result = engine.diff_with_config("raw_logs_content", "processed_logs_content", &config);
}

#[test]
fn examples_example_19() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.max_depth = Some(3);
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("data_warehouse_snapshot_old_content", "data_warehouse_snapshot_new_content", &config);
}

#[test]
fn examples_example_20() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.0001);
    config.show_types = true;
    let _result = engine.diff_with_config("ml_features_v1_content", "ml_features_v2_content", &config);
}

#[test]
fn examples_example_21() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "sql".to_string();
    config.output_format = "unified".to_string();
    let _result = engine.diff_with_config("db_schema_dump_old_content", "db_schema_dump_new_content", &config);
}

#[test]
fn examples_example_22() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("grants".to_string());
    config.array_id_key = Some("user".to_string());
    let _result = engine.diff_with_config("db_permissions_old_content", "db_permissions_new_content", &config);
}

#[test]
fn examples_example_23() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(cost|time|buffers)".to_string());
    let _result = engine.diff_with_config("query_plan_old_content", "query_plan_new_content", &config);
}

#[test]
fn examples_example_24() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "ini".to_string();
    config.ignore_keys = vec!["connection_string".to_string()];
    let _result = engine.diff_with_config("database_config_dev_content", "database_config_prod_content", &config);
}

#[test]
fn examples_example_25() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.path_filter = Some("groups.*.rules".to_string());
    config.array_id_key = Some("alert".to_string());
    let _result = engine.diff_with_config("prometheus_alerts_old_content", "prometheus_alerts_new_content", &config);
}

#[test]
fn examples_example_26() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("panels".to_string());
    config.array_id_key = Some("id".to_string());
    config.ignore_keys = vec!["version".to_string(), "uid".to_string()];
    let _result = engine.diff_with_config("grafana_dashboard_v1_content", "grafana_dashboard_v2_content", &config);
}

#[test]
fn examples_example_27() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("MetricAlarms".to_string());
    config.array_id_key = Some("AlarmName".to_string());
    let _result = engine.diff_with_config("cloudwatch_alarms_old_content", "cloudwatch_alarms_new_content", &config);
}

#[test]
fn examples_example_28() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("mappings.properties".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("elastic_mapping_old_content", "elastic_mapping_new_content", &config);
}

#[test]
fn examples_example_29() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("handlers".to_string());
    config.show_unchanged = true;
    let _result = engine.diff_with_config("logging_config_v1_content", "logging_config_v2_content", &config);
}

#[test]
fn examples_example_30() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("dependencies".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("package_json_content", "package_lock_json_content", &config);
}

#[test]
fn examples_example_31() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "xml".to_string();
    config.path_filter = Some("testsuites.testsuite".to_string());
    config.array_id_key = Some("name".to_string());
    let _result = engine.diff_with_config("test_results_old_content", "test_results_new_content", &config);
}

#[test]
fn examples_example_32() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.01);
    config.path_filter = Some("metrics".to_string());
    let _result = engine.diff_with_config("code_metrics_before_content", "code_metrics_after_content", &config);
}

#[test]
fn examples_example_33() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("rules".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("eslint_config_old_content", "eslint_config_new_content", &config);
}

#[test]
fn examples_example_34() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "text".to_string();
    config.output_format = "unified".to_string();
    let _result = engine.diff_with_config("gitignore_content", "gitignore_new_content", &config);
}

#[test]
fn examples_example_35() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("permissions".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("security_policy_v1_content", "security_policy_v2_content", &config);
}

#[test]
fn examples_example_36() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(serial_number|thumbprint)".to_string());
    config.show_types = true;
    let _result = engine.diff_with_config("ssl_cert_old_content", "ssl_cert_new_content", &config);
}

#[test]
fn examples_example_37() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("rule_id".to_string());
    config.path_filter = Some("inbound_rules".to_string());
    let _result = engine.diff_with_config("firewall_rules_old_content", "firewall_rules_new_content", &config);
}

#[test]
fn examples_example_38() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("findings".to_string());
    config.array_id_key = Some("finding_id".to_string());
    let _result = engine.diff_with_config("compliance_report_q1_content", "compliance_report_q2_content", &config);
}

#[test]
fn examples_example_39() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    config.path_filter = Some("events".to_string());
    config.show_unchanged = true;
    let _result = engine.diff_with_config("audit_log_config_old_content", "audit_log_config_new_content", &config);
}

#[test]
fn examples_example_40() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.05);
    config.path_filter = Some("metrics".to_string());
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("performance_baseline_content", "performance_current_content", &config);
}

#[test]
fn examples_example_41() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("redis.settings".to_string());
    config.ignore_keys = vec!["last_flush_time".to_string()];
    let _result = engine.diff_with_config("cache_config_old_content", "cache_config_new_content", &config);
}