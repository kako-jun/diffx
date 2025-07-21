use assert_cmd::Command;

#[test]
fn getting_started_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_v1.json", "config_v2.json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["docker-compose.yml", "docker-compose.new.yml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["Cargo.toml", "Cargo.toml.backup"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["settings.xml", "settings.new.xml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["database.ini", "database.prod.ini"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users.csv", "users_updated.csv"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["app.json", "app.new.json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.yaml", "config.yml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["settings.toml", "backup.toml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_10() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "json", "file1", "file2"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_11() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "json", "file1.txt", "--format", "yaml", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_12() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "-"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_13() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["-", "container2_inspect.json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_14() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["-", "config_v2.json", "--format", "json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_15() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_dir1/", "config_dir2/", "--recursive"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_16() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["configs/", "configs_backup/", "--recursive", "--format", "json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_17() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["app.json", "app.new.json", "--ignore-keys-regex", "^(timestamp|_.*|createdAt)$"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_18() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["package.json", "package.new.json", "--ignore-keys-regex", "version|buildNumber"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_19() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users.json", "users_updated.json", "--array-id-key", "id"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_20() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["inventory.json", "inventory.new.json", "--array-id-key", "sku"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_21() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["records.json", "records.new.json", "--array-id-key", "pk"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_22() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["metrics.json", "metrics.new.json", "--epsilon", "0.001"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_23() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["measurements.json", "measurements.new.json", "--epsilon", "0.01"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_24() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--path", "database"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_25() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--path", "servers[0]"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_26() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["app.json", "app.new.json", "--path", "microservices.auth.database.connection"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_27() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_28() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_29() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--output", "yaml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_30() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--output", "unified"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_31() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["application.properties", "application.prod.properties"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_32() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.prod.json", "--ignore-keys-regex", "^(host|password|apiKey)"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_33() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["expected_output.json", "actual_output.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_34() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["schema_v1.sql", "schema_v2.sql", "--format", "sql"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_35() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users_export.csv", "users_import.csv", "--array-id-key", "email"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_36() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["package.json", "package-lock.json", "--path", "dependencies"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_37() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--", "terraform.tfstate", "terraform.tfstate.backup"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_38() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["openapi_v1.yaml", "openapi_v2.yaml", "--path", "paths"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_39() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["benchmark_baseline.json", "benchmark_current.json", "--epsilon", "0.05"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_40() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_dev.json", "config_prod.json", "--output", "json", "--ignore-keys-regex", "^(debug|test_)"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_41() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["api_response_expected.json", "api_response_actual.json", "--ignore-keys-regex", "^(timestamp|requestId)"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_42() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["k8s_deployment.yaml", "k8s_deployment_new.yaml", "--path", "spec.template.spec.containers"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_43() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["prometheus_rules.yml", "prometheus_rules_new.yml", "--array-id-key", "alert"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_44() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["eslint_config.json", "eslint_config_new.json", "--path", "rules"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_45() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ml_model_params_v1.json", "ml_model_params_v2.json", "--epsilon", "0.0001"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_46() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["db_migrations/", "db_migrations_new/", "--recursive", "--format", "sql"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_47() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["graphql_schema.json", "graphql_schema_new.json", "--path", "data.__schema.types", "--array-id-key", "name"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_48() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["nginx.conf", "nginx_new.conf", "--format", "conf"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_49() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ansible_inventory.ini", "ansible_inventory_new.ini", "--format", "ini"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_50() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["helm_values.yaml", "helm_values_prod.yaml", "--ignore-keys", "image.tag,replicas"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_51() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["security_policy.json", "security_policy_new.json", "--path", "rules", "--array-id-key", "ruleId"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_52() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["test_results.xml", "test_results_new.xml", "--format", "xml", "--path", "testsuites.testsuite"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_53() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["monitoring_config.toml", "monitoring_config_new.toml", "--format", "toml"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_54() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["feature_flags.json", "feature_flags_new.json", "--path", "flags", "--show-unchanged"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_55() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["load_test_results.json", "load_test_results_new.json", "--epsilon", "0.1", "--path", "metrics"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_56() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["translation_en.json", "translation_en_new.json", "--output", "json", "--show-types"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_57() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ci_pipeline.yml", "ci_pipeline_new.yml", "--path", "jobs", "--array-id-key", "name"])
        .assert()
        .success();
}

#[test]
fn getting_started_example_58() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["audit_log.json", "audit_log_new.json", "--ignore-keys-regex", "^(timestamp|eventId)", "--array-id-key", "userId"])
        .assert()
        .success();
}