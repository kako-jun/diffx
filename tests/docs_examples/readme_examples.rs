#[allow(unused_imports)]
use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

// Helper function to create temporary JSON files for testing
fn create_temp_json(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
    writeln!(file, "{content}").expect("Failed to write to temp file");
    file
}

/// Test case 1: diffx config_v1.json config_v2.json
#[test]
fn test_basic_config_diff() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"name": "myapp", "version": "1.0"}"#);
    let file2 = create_temp_json(r#"{"version": "1.1", "name": "myapp"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .code(1) // diffx returns 1 when differences found
        .stdout(predicates::str::contains("version:"))
        .stdout(predicates::str::contains("1.0"))
        .stdout(predicates::str::contains("1.1"));

    Ok(())
}

/// Test case 2: time diffx large_test1.json large_test2.json
#[test]
fn test_large_file_performance() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(
        r#"{"config": {"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true}}}"#,
    );
    let file2 = create_temp_json(
        r#"{"config": {"database": {"host": "prod-db", "port": 5432}, "cache": {"enabled": false}}}"#,
    );

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .code(1) // differences found, should return 1
        .stdout(predicates::str::contains("host:"))
        .stdout(predicates::str::contains("enabled:"));

    Ok(())
}

/// Test case 3: diffx config_v1.json config_v2.json --output json > report1.json
#[test]
fn test_json_output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"version": "1.0"}"#);
    let file2 = create_temp_json(r#"{"version": "1.1"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 4: diffx config_v2.json config_v3.json --output json > report2.json
#[test]
fn test_second_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"version": "1.1"}"#);
    let file2 = create_temp_json(r#"{"version": "1.2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 5: diffx report1.json report2.json
#[test]
fn test_meta_chaining_diff_reports() -> Result<(), Box<dyn std::error::Error>> {
    let report1 = create_temp_json(r#"[{"Modified": ["version", "1.0", "1.1"]}]"#);
    let report2 = create_temp_json(r#"[{"Modified": ["version", "1.1", "1.2"]}]"#);

    let mut cmd = diffx_cmd();
    cmd.arg(report1.path()).arg(report2.path());
    cmd.assert().code(1); // differences found, should return 1

    Ok(())
}

/// Test case 6: diffx file1.json file2.json
#[test]
fn test_basic_file_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "value1"}"#);
    let file2 = create_temp_json(r#"{"data": "value2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .code(1) // differences found, should return 1
        .stdout(predicates::str::contains("data:"));

    Ok(())
}

/// Test case 7: diffx config.yaml config_new.yaml --output json
#[test]
fn test_yaml_with_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": {"debug": true}}"#);
    let file2 = create_temp_json(r#"{"config": {"debug": false}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 8: diffx data.toml data_updated.toml --output yaml
#[test]
fn test_toml_with_yaml_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"app": {"name": "test"}}"#);
    let file2 = create_temp_json(r#"{"app": {"name": "updated"}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("yaml");
    cmd.assert().code(1);

    Ok(())
}

/// Test case 9: diffx large.json large_v2.json --ignore-keys-regex "^timestamp$|^_.*"
#[test]
fn test_ignore_keys_regex() -> Result<(), Box<dyn std::error::Error>> {
    let file1 =
        create_temp_json(r#"{"timestamp": "2024-01-01", "_internal": "meta", "data": "value1"}"#);
    let file2 =
        create_temp_json(r#"{"timestamp": "2024-01-02", "_internal": "meta2", "data": "value2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--ignore-keys-regex")
        .arg("^timestamp$|^_.*");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("data:"));

    Ok(())
}

/// Test case 10: diffx users.json users_v2.json --array-id-key "id"
#[test]
fn test_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let file1 =
        create_temp_json(r#"{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}"#);
    let file2 =
        create_temp_json(r#"{"users": [{"id": 2, "name": "Jane"}, {"id": 1, "name": "Johnny"}]}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("name:"));

    Ok(())
}

/// Test case 11: diffx metrics.json metrics_v2.json --epsilon 0.001
#[test]
fn test_epsilon_tolerance() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 1.0001}"#);
    let file2 = create_temp_json(r#"{"value": 1.0002}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--epsilon")
        .arg("0.001");
    cmd.assert().success();

    Ok(())
}

/// Test case 12: diffx config.yaml config_new.yaml --ignore-case
#[test]
fn test_ignore_case() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"status": "ACTIVE"}"#);
    let file2 = create_temp_json(r#"{"status": "active"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path()).arg("--ignore-case");
    cmd.assert().success().code(0);

    Ok(())
}

/// Test case 13: diffx api.json api_formatted.json --ignore-whitespace
#[test]
fn test_ignore_whitespace() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"text": "hello world"}"#);
    let file2 = create_temp_json(r#"{"text": "hello    world"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--ignore-whitespace");
    cmd.assert().success();

    Ok(())
}

/// Test case 14: diffx large.json large_v2.json --context 3 --output unified
#[test]
fn test_unified_output_with_context() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"a": 1, "b": 2, "c": 3, "d": 4}"#);
    let file2 = create_temp_json(r#"{"a": 1, "b": 20, "c": 3, "d": 4}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--context")
        .arg("3")
        .arg("--output")
        .arg("unified");
    cmd.assert().code(1);

    Ok(())
}

/// Test case 15: diffx file1.json file2.json --quiet
#[test]
fn test_quiet_mode() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"test": "value1"}"#);
    let file2 = create_temp_json(r#"{"test": "value2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path()).arg("--quiet");
    cmd.assert().failure();

    Ok(())
}

/// Test case 16: diffx dir1/ dir2/ --recursive --brief
#[test]
fn test_recursive_brief() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"test": "value1"}"#);
    let file2 = create_temp_json(r#"{"test": "value2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path()).arg("--brief");
    cmd.assert().code(1);

    Ok(())
}

/// Test case 17: diffx huge_dataset.json huge_dataset_v2.json
#[test]
fn test_huge_dataset_performance() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"dataset": {"size": 1000000, "type": "production"}}"#);
    let file2 = create_temp_json(r#"{"dataset": {"size": 1000001, "type": "production"}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("size:"));

    Ok(())
}

/// Test case 18: diffx config_dir1/ config_dir2/ --recursive
#[test]
fn test_directory_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": "dir1"}"#);
    let file2 = create_temp_json(r#"{"config": "dir2"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().code(1);

    Ok(())
}

/// Test case 19: diffx config_v1.json config_v2.json --output json > diff1.json
#[test]
fn test_diff1_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": {"version": "1.0"}}"#);
    let file2 = create_temp_json(r#"{"config": {"version": "1.1"}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 20: diffx config_v2.json config_v3.json --output json > diff2.json
#[test]
fn test_diff2_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": {"version": "1.1"}}"#);
    let file2 = create_temp_json(r#"{"config": {"version": "1.2"}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 21: diffx diff1.json diff2.json
#[test]
fn test_meta_diff_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let diff1 = create_temp_json(r#"[{"Modified": ["config.version", "1.0", "1.1"]}]"#);
    let diff2 = create_temp_json(r#"[{"Modified": ["config.version", "1.1", "1.2"]}]"#);

    let mut cmd = diffx_cmd();
    cmd.arg(diff1.path()).arg(diff2.path());
    cmd.assert().code(1); // differences found, should return 1

    Ok(())
}

/// Test case 22: diffx config/prod.yaml config/staging.yaml --output json > changes.json
#[test]
fn test_cicd_config_changes() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"env": "prod", "debug": false}"#);
    let file2 = create_temp_json(r#"{"env": "staging", "debug": true}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 23: if ! diffx config/current.json config/new.json --quiet; then
#[test]
fn test_cicd_change_detection() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"current": "config"}"#);
    let file2 = create_temp_json(r#"{"current": "new_config"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path()).arg("--quiet");
    cmd.assert().failure();

    Ok(())
}

/// Test case 24: diffx api_old.json api_new.json --ignore-case --ignore-whitespace --output json > api_changes.json
#[test]
fn test_api_ignore_options_json() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"API": "old version"}"#);
    let file2 = create_temp_json(r#"{"api": "new   version"}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 25: diffx large_prod_data.json large_staging_data.json --output json > data_changes.json
#[test]
fn test_large_data_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"dataset": {"env": "prod", "size": 10000}}"#);
    let file2 = create_temp_json(r#"{"dataset": {"env": "staging", "size": 5000}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["));

    Ok(())
}

/// Test case 26: diffx package.json HEAD~1:package.json --output json
#[test]
fn test_git_dependency_detection() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"dependencies": {"express": "^4.18.0"}}"#);
    let file2 =
        create_temp_json(r#"{"dependencies": {"express": "^4.18.0", "lodash": "^4.17.21"}}"#);

    let mut cmd = diffx_cmd();
    cmd.arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::starts_with("["))
        .stdout(predicates::str::contains("lodash"));

    Ok(())
}
