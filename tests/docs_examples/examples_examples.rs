#[allow(unused_imports)]
use assert_cmd::prelude::*;
use assert_cmd::Command;

#[test]
fn examples_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config/dev.json", "config/prod.json", "--ignore-keys-regex", "^(host|port|password|secret_.*)", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["app_config_v1.yaml", "app_config_v2.yaml", "--format", "yaml", "--output", "unified"])
        .assert()
        .success();
}

#[test]
fn examples_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["feature_flags_old.json", "feature_flags_new.json", "--path", "features", "--show-unchanged"])
        .assert()
        .success();
}

#[test]
fn examples_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_base.json", "config_prod.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["terraform.tfstate", "terraform.tfstate.backup", "--path", "resources", "--ignore-keys-regex", "^(last_updated|timeouts)", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["kubernetes/deployment_v1.yaml", "kubernetes/deployment_v2.yaml", "--format", "yaml", "--path", "spec.containers"])
        .assert()
        .success();
}

#[test]
fn examples_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["docker-compose.yml", "docker-compose.prod.yml", "--format", "yaml", "--ignore-keys-regex", "^(build|volumes)"])
        .assert()
        .success();
}

#[test]
fn examples_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ansible_inventory_old.yml", "ansible_inventory_new.yml", "--format", "yaml", "--array-id-key", "hostname"])
        .assert()
        .success();
}

#[test]
fn examples_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["helm_values_staging.yaml", "helm_values_production.yaml", "--format", "yaml", "--ignore-keys", "image.tag,replicas"])
        .assert()
        .success();
}

#[test]
fn examples_example_10() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ci_pipeline_old.yml", "ci_pipeline_new.yml", "--format", "yaml", "--path", "stages"])
        .assert()
        .success();
}

#[test]
fn examples_example_11() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["api_v1_spec.json", "api_v2_spec.json", "--path", "paths", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_12() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["expected_user_response.json", "actual_response.json", "--ignore-keys-regex", "^(timestamp|request_id|server_time)", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_13() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["postman_collection_old.json", "postman_collection_new.json", "--path", "item", "--array-id-key", "name"])
        .assert()
        .success();
}

#[test]
fn examples_example_14() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["graphql_schema_v1.json", "graphql_schema_v2.json", "--path", "data.__schema.types", "--array-id-key", "name"])
        .assert()
        .success();
}

#[test]
fn examples_example_15() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["soap_response_expected.xml", "soap_response_actual.xml", "--format", "xml", "--ignore-keys", "MessageID,Timestamp"])
        .assert()
        .success();
}

#[test]
fn examples_example_16() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["input_data_sample.json", "output_data_sample.json", "--array-id-key", "record_id", "--epsilon", "0.001", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_17() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["batch_job_input.csv", "batch_job_output.csv", "--format", "csv", "--ignore-keys", "processing_time,job_id"])
        .assert()
        .success();
}

#[test]
fn examples_example_18() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["raw_logs.json", "processed_logs.json", "--array-id-key", "event_id", "--path", "events"])
        .assert()
        .success();
}

#[test]
fn examples_example_19() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["data_warehouse_snapshot_old.json", "data_warehouse_snapshot_new.json", "--max-depth", "3", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_20() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ml_features_v1.json", "ml_features_v2.json", "--epsilon", "0.0001", "--show-types"])
        .assert()
        .success();
}

#[test]
fn examples_example_21() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["db_schema_dump_old.sql", "db_schema_dump_new.sql", "--format", "sql", "--output", "unified"])
        .assert()
        .success();
}

#[test]
fn examples_example_22() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["db_permissions_old.json", "db_permissions_new.json", "--path", "grants", "--array-id-key", "user"])
        .assert()
        .success();
}

#[test]
fn examples_example_23() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["query_plan_old.json", "query_plan_new.json", "--ignore-keys-regex", "^(cost|time|buffers)"])
        .assert()
        .success();
}

#[test]
fn examples_example_24() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["database_config_dev.ini", "database_config_prod.ini", "--format", "ini", "--ignore-keys", "connection_string"])
        .assert()
        .success();
}

#[test]
fn examples_example_25() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["prometheus_alerts_old.yml", "prometheus_alerts_new.yml", "--format", "yaml", "--path", "groups.*.rules", "--array-id-key", "alert"])
        .assert()
        .success();
}

#[test]
fn examples_example_26() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["grafana_dashboard_v1.json", "grafana_dashboard_v2.json", "--path", "panels", "--array-id-key", "id", "--ignore-keys", "version,uid"])
        .assert()
        .success();
}

#[test]
fn examples_example_27() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["cloudwatch_alarms_old.json", "cloudwatch_alarms_new.json", "--path", "MetricAlarms", "--array-id-key", "AlarmName"])
        .assert()
        .success();
}

#[test]
fn examples_example_28() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["elastic_mapping_old.json", "elastic_mapping_new.json", "--path", "mappings.properties", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_29() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["logging_config_v1.json", "logging_config_v2.json", "--path", "handlers", "--show-unchanged"])
        .assert()
        .success();
}

#[test]
fn examples_example_30() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["package.json", "package-lock.json", "--path", "dependencies", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_31() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["test_results_old.xml", "test_results_new.xml", "--format", "xml", "--path", "testsuites.testsuite", "--array-id-key", "name"])
        .assert()
        .success();
}

#[test]
fn examples_example_32() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["code_metrics_before.json", "code_metrics_after.json", "--epsilon", "0.01", "--path", "metrics"])
        .assert()
        .success();
}

#[test]
fn examples_example_33() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["eslint_config_old.json", "eslint_config_new.json", "--path", "rules", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_34() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&[".gitignore", ".gitignore.new", "--format", "text", "--output", "unified"])
        .assert()
        .success();
}

#[test]
fn examples_example_35() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["security_policy_v1.json", "security_policy_v2.json", "--path", "permissions", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_36() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["ssl_cert_old.json", "ssl_cert_new.json", "--ignore-keys-regex", "^(serial_number|thumbprint)", "--show-types"])
        .assert()
        .success();
}

#[test]
fn examples_example_37() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["firewall_rules_old.json", "firewall_rules_new.json", "--array-id-key", "rule_id", "--path", "inbound_rules"])
        .assert()
        .success();
}

#[test]
fn examples_example_38() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["compliance_report_q1.json", "compliance_report_q2.json", "--path", "findings", "--array-id-key", "finding_id"])
        .assert()
        .success();
}

#[test]
fn examples_example_39() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["audit_log_config_old.yaml", "audit_log_config_new.yaml", "--format", "yaml", "--path", "events", "--show-unchanged"])
        .assert()
        .success();
}

#[test]
fn examples_example_40() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["performance_baseline.json", "performance_current.json", "--epsilon", "0.05", "--path", "metrics", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn examples_example_41() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["cache_config_old.json", "cache_config_new.json", "--path", "redis.settings", "--ignore-keys", "last_flush_time"])
        .assert()
        .success();
}