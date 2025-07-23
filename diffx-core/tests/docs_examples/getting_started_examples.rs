use diffx_core::{DiffEngine, DiffConfig};

#[test]
fn getting_started_example_1() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_v1_content", "config_v2_content");
}

#[test]
fn getting_started_example_2() {
    let engine = DiffEngine::new();
    let _result = engine.diff("docker_compose_content", "docker_compose_new_content");
}

#[test]
fn getting_started_example_3() {
    let engine = DiffEngine::new();
    let _result = engine.diff("cargo_toml_content", "cargo_toml_backup_content");
}

#[test]
fn getting_started_example_4() {
    let engine = DiffEngine::new();
    let _result = engine.diff("settings_xml_content", "settings_new_xml_content");
}

#[test]
fn getting_started_example_5() {
    let engine = DiffEngine::new();
    let _result = engine.diff("database_ini_content", "database_prod_ini_content");
}

#[test]
fn getting_started_example_6() {
    let engine = DiffEngine::new();
    let _result = engine.diff("users_csv_content", "users_updated_csv_content");
}

#[test]
fn getting_started_example_7() {
    let engine = DiffEngine::new();
    let _result = engine.diff("app_json_content", "app_new_json_content");
}

#[test]
fn getting_started_example_8() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_yaml_content", "config_yml_content");
}

#[test]
fn getting_started_example_9() {
    let engine = DiffEngine::new();
    let _result = engine.diff("settings_toml_content", "backup_toml_content");
}

#[test]
fn getting_started_example_10() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "json".to_string();
    let _result = engine.diff_with_config("file1_content", "file2_content", &config);
}

#[test]
fn getting_started_example_11() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "json".to_string();
    let _result = engine.diff_with_config("file1_txt_content", "file2_txt_content", &config);
}

#[test]
fn getting_started_example_12() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_json_content", "stdin_content");
}

#[test]
fn getting_started_example_13() {
    let engine = DiffEngine::new();
    let _result = engine.diff("stdin_content", "container2_inspect_content");
}

#[test]
fn getting_started_example_14() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "json".to_string();
    let _result = engine.diff_with_config("stdin_content", "config_v2_content", &config);
}

#[test]
fn getting_started_example_15() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.recursive = true;
    let _result = engine.diff_directories("config_dir1", "config_dir2", &config);
}

#[test]
fn getting_started_example_16() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.recursive = true;
    config.format = "json".to_string();
    let _result = engine.diff_directories("configs", "configs_backup", &config);
}

#[test]
fn getting_started_example_17() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(timestamp|_.*|createdAt)$".to_string());
    let _result = engine.diff_with_config("app_content", "app_new_content", &config);
}

#[test]
fn getting_started_example_18() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("version|buildNumber".to_string());
    let _result = engine.diff_with_config("package_content", "package_new_content", &config);
}

#[test]
fn getting_started_example_19() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("id".to_string());
    let _result = engine.diff_with_config("users_content", "users_updated_content", &config);
}

#[test]
fn getting_started_example_20() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("sku".to_string());
    let _result = engine.diff_with_config("inventory_content", "inventory_new_content", &config);
}

#[test]
fn getting_started_example_21() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("pk".to_string());
    let _result = engine.diff_with_config("records_content", "records_new_content", &config);
}

#[test]
fn getting_started_example_22() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.001);
    let _result = engine.diff_with_config("metrics_content", "metrics_new_content", &config);
}

#[test]
fn getting_started_example_23() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.01);
    let _result = engine.diff_with_config("measurements_content", "measurements_new_content", &config);
}

#[test]
fn getting_started_example_24() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("database".to_string());
    let _result = engine.diff_with_config("config_content", "config_new_content", &config);
}

#[test]
fn getting_started_example_25() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("servers[0]".to_string());
    let _result = engine.diff_with_config("config_content", "config_new_content", &config);
}

#[test]
fn getting_started_example_26() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("microservices.auth.database.connection".to_string());
    let _result = engine.diff_with_config("app_content", "app_new_content", &config);
}

#[test]
fn getting_started_example_27() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_content", "config_new_content");
}

#[test]
fn getting_started_example_28() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("config_content", "config_new_content", &config);
}

#[test]
fn getting_started_example_29() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "yaml".to_string();
    let _result = engine.diff_with_config("config_content", "config_new_content", &config);
}

#[test]
fn getting_started_example_30() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "unified".to_string();
    let _result = engine.diff_with_config("config_content", "config_new_content", &config);
}

#[test]
fn getting_started_example_31() {
    let engine = DiffEngine::new();
    let _result = engine.diff("application_properties_content", "application_prod_properties_content");
}

#[test]
fn getting_started_example_32() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(host|password|apiKey)".to_string());
    let _result = engine.diff_with_config("config_content", "config_prod_content", &config);
}

#[test]
fn getting_started_example_33() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("expected_output_content", "actual_output_content", &config);
}

#[test]
fn getting_started_example_34() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "sql".to_string();
    let _result = engine.diff_with_config("schema_v1_content", "schema_v2_content", &config);
}

#[test]
fn getting_started_example_35() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("email".to_string());
    let _result = engine.diff_with_config("users_export_content", "users_import_content", &config);
}

#[test]
fn getting_started_example_36() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("dependencies".to_string());
    let _result = engine.diff_with_config("package_content", "package_lock_content", &config);
}

#[test]
fn getting_started_example_37() {
    let engine = DiffEngine::new();
    let _result = engine.diff("terraform_tfstate_content", "terraform_tfstate_backup_content");
}

#[test]
fn getting_started_example_38() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("paths".to_string());
    let _result = engine.diff_with_config("openapi_v1_content", "openapi_v2_content", &config);
}

#[test]
fn getting_started_example_39() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.05);
    let _result = engine.diff_with_config("benchmark_baseline_content", "benchmark_current_content", &config);
}

#[test]
fn getting_started_example_40() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    config.ignore_keys_regex = Some("^(debug|test_)".to_string());
    let _result = engine.diff_with_config("config_dev_content", "config_prod_content", &config);
}

#[test]
fn getting_started_example_41() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(timestamp|requestId)".to_string());
    let _result = engine.diff_with_config("api_response_expected_content", "api_response_actual_content", &config);
}

#[test]
fn getting_started_example_42() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("spec.template.spec.containers".to_string());
    let _result = engine.diff_with_config("k8s_deployment_content", "k8s_deployment_new_content", &config);
}

#[test]
fn getting_started_example_43() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("alert".to_string());
    let _result = engine.diff_with_config("prometheus_rules_content", "prometheus_rules_new_content", &config);
}

#[test]
fn getting_started_example_44() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("rules".to_string());
    let _result = engine.diff_with_config("eslint_config_content", "eslint_config_new_content", &config);
}

#[test]
fn getting_started_example_45() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.0001);
    let _result = engine.diff_with_config("ml_model_params_v1_content", "ml_model_params_v2_content", &config);
}

#[test]
fn getting_started_example_46() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.recursive = true;
    config.format = "sql".to_string();
    let _result = engine.diff_directories("db_migrations", "db_migrations_new", &config);
}

#[test]
fn getting_started_example_47() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("data.__schema.types".to_string());
    config.array_id_key = Some("name".to_string());
    let _result = engine.diff_with_config("graphql_schema_content", "graphql_schema_new_content", &config);
}

#[test]
fn getting_started_example_48() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "conf".to_string();
    let _result = engine.diff_with_config("nginx_conf_content", "nginx_new_conf_content", &config);
}

#[test]
fn getting_started_example_49() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "ini".to_string();
    let _result = engine.diff_with_config("ansible_inventory_content", "ansible_inventory_new_content", &config);
}

#[test]
fn getting_started_example_50() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys = vec!["image.tag".to_string(), "replicas".to_string()];
    let _result = engine.diff_with_config("helm_values_content", "helm_values_prod_content", &config);
}

#[test]
fn getting_started_example_51() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("rules".to_string());
    config.array_id_key = Some("ruleId".to_string());
    let _result = engine.diff_with_config("security_policy_content", "security_policy_new_content", &config);
}

#[test]
fn getting_started_example_52() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "xml".to_string();
    config.path_filter = Some("testsuites.testsuite".to_string());
    let _result = engine.diff_with_config("test_results_content", "test_results_new_content", &config);
}

#[test]
fn getting_started_example_53() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "toml".to_string();
    let _result = engine.diff_with_config("monitoring_config_content", "monitoring_config_new_content", &config);
}

#[test]
fn getting_started_example_54() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("flags".to_string());
    config.show_unchanged = true;
    let _result = engine.diff_with_config("feature_flags_content", "feature_flags_new_content", &config);
}

#[test]
fn getting_started_example_55() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.1);
    config.path_filter = Some("metrics".to_string());
    let _result = engine.diff_with_config("load_test_results_content", "load_test_results_new_content", &config);
}

#[test]
fn getting_started_example_56() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    config.show_types = true;
    let _result = engine.diff_with_config("translation_en_content", "translation_en_new_content", &config);
}

#[test]
fn getting_started_example_57() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("jobs".to_string());
    config.array_id_key = Some("name".to_string());
    let _result = engine.diff_with_config("ci_pipeline_content", "ci_pipeline_new_content", &config);
}

#[test]
fn getting_started_example_58() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^(timestamp|eventId)".to_string());
    config.array_id_key = Some("userId".to_string());
    let _result = engine.diff_with_config("audit_log_content", "audit_log_new_content", &config);
}