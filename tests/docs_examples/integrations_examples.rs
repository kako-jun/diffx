#[allow(unused_imports)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

// Helper function to create temporary JSON files for testing
fn create_temp_json(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: diffx --version
#[test]
fn test_version_check() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");
    cmd.assert().success();
    Ok(())
}

/// Test case 2: diffx /tmp/base_file "$file" --ignore-keys-regex "^(timestamp|lastModified|createdAt|updatedAt|buildTime)$" --ignore-case --ignore-whitespace --output json
#[test]
fn test_config_validation_with_ignore_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let base_file = create_temp_json(r#"{"name": "app", "version": "1.0", "timestamp": "2024-01-01T00:00:00Z"}"#);
    let file = create_temp_json(r#"{"name": "APP", "version": "1.1", "timestamp": "2024-01-02T00:00:00Z"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(base_file.path()).arg(file.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|lastModified|createdAt|updatedAt|buildTime)$")
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: if diffx "tests/api_contracts/$endpoint.json" "actual_$endpoint.json" --ignore-keys-regex "^(timestamp|requestId|serverId|responseTime)$" --output json
#[test]
fn test_api_contract_validation() -> Result<(), Box<dyn std::error::Error>> {
    let contract = create_temp_json(r#"{"endpoint": "/users", "method": "GET", "timestamp": "2024-01-01"}"#);
    let actual = create_temp_json(r#"{"endpoint": "/users", "method": "POST", "timestamp": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(contract.path()).arg(actual.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|requestId|serverId|responseTime)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: diffx "config/production.yaml" "config/$env.yaml" --ignore-keys-regex "^(environment|host|port|replicas|resources\..*)" --output json
#[test]
fn test_environment_config_diff() -> Result<(), Box<dyn std::error::Error>> {
    let prod_config = create_temp_json(r#"{"app": "myapp", "environment": "production", "host": "prod.com", "port": 8080}"#);
    let env_config = create_temp_json(r#"{"app": "myapp", "environment": "staging", "host": "staging.com", "port": 8081}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(prod_config.path()).arg(env_config.path())
        .arg("--ignore-keys-regex").arg("^(environment|host|port|replicas|resources\\..*)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 5: diffx current_state.json planned_changes.json --path "planned_values.root_module.resources" --ignore-keys-regex "^(timeouts|creation_time|last_updated)" --output json
#[test]
fn test_terraform_diff() -> Result<(), Box<dyn std::error::Error>> {
    let current = create_temp_json(r#"{"planned_values": {"root_module": {"resources": [{"name": "server1", "type": "aws_instance"}]}}}"#);
    let planned = create_temp_json(r#"{"planned_values": {"root_module": {"resources": [{"name": "server2", "type": "aws_instance"}]}}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(current.path()).arg(planned.path())
        .arg("--ignore-keys-regex").arg("^(timeouts|creation_time|last_updated)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: if ! diffx baseline_config.json deployment_config.json --quiet
#[test]
fn test_quiet_baseline_check() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = create_temp_json(r#"{"version": "1.0"}"#);
    let deployment = create_temp_json(r#"{"version": "1.1"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(baseline.path()).arg(deployment.path()).arg("--quiet");
    cmd.assert().failure(); // Files differ, so should fail
    Ok(())
}

/// Test case 7: diffx configs/ updated_configs/ --recursive --brief
#[test]
fn test_recursive_brief_diff() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": "old"}"#);
    let file2 = create_temp_json(r#"{"config": "new"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path()).arg("--brief");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: diffx baseline_config.json deployment_config.json --ignore-case --ignore-whitespace --ignore-keys-regex "^(deploy_time|build_id|version)" --output json
#[test]
fn test_deployment_diff_with_ignores() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = create_temp_json(r#"{"APP": "myapp", "VERSION": "1.0", "deploy_time": "2024-01-01"}"#);
    let deployment = create_temp_json(r#"{"app":   "myapp", "version": "1.1", "deploy_time": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(baseline.path()).arg(deployment.path())
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("--ignore-keys-regex").arg("^(deploy_time|build_id|version)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: if ! diffx /etc/app/config.json expected_config.json --ignore-keys-regex "^(hostname|instance_id|last_.*)" --ignore-case --quiet
#[test]
fn test_config_drift_detection() -> Result<(), Box<dyn std::error::Error>> {
    let current = create_temp_json(r#"{"SERVICE": "api", "hostname": "server1", "instance_id": "i-123"}"#);
    let expected = create_temp_json(r#"{"service": "web", "hostname": "server2", "instance_id": "i-456"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(current.path()).arg(expected.path())
        .arg("--ignore-keys-regex").arg("^(hostname|instance_id|last_.*)")
        .arg("--ignore-case")
        .arg("--quiet");
    cmd.assert().failure(); // Services differ
    Ok(())
}

/// Test case 10: diffx /etc/app/config.json expected_config.json --ignore-keys-regex "^(hostname|instance_id|last_.*)" --ignore-case --context 2 --output unified
#[test]
fn test_config_drift_unified_output() -> Result<(), Box<dyn std::error::Error>> {
    let current = create_temp_json(r#"{"service": "API", "hostname": "server1"}"#);
    let expected = create_temp_json(r#"{"service": "api", "hostname": "server2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(current.path()).arg(expected.path())
        .arg("--ignore-keys-regex").arg("^(hostname|instance_id|last_.*)")
        .arg("--ignore-case")
        .arg("--context").arg("2")
        .arg("--output").arg("unified");
    cmd.assert().success();
    Ok(())
}

/// Test case 11: if ! diffx "$baseline" "$config" --quiet
#[test]
fn test_baseline_config_check() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = create_temp_json(r#"{"setting": "production"}"#);
    let config = create_temp_json(r#"{"setting": "development"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(baseline.path()).arg(config.path()).arg("--quiet");
    cmd.assert().failure();
    Ok(())
}

/// Test case 12: diffx "$baseline" "$file" --ignore-whitespace --context 1 --output unified
#[test]
fn test_baseline_file_unified() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = create_temp_json(r#"{"name": "app", "version": "1.0"}"#);
    let file = create_temp_json(r#"{"name":   "app", "version": "1.1"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(baseline.path()).arg(file.path())
        .arg("--ignore-whitespace")
        .arg("--context").arg("1")
        .arg("--output").arg("unified");
    cmd.assert().success();
    Ok(())
}

/// Test case 13: diffx --version (installation verification)
#[test]
fn test_installation_verification() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: diffx old_${file} ${file} --ignore-keys-regex "^(timestamp|version|buildNumber)$" --output json
#[test]
fn test_jenkins_file_diff() -> Result<(), Box<dyn std::error::Error>> {
    let old_file = create_temp_json(r#"{"build": "123", "timestamp": "2024-01-01", "version": "1.0"}"#);
    let new_file = create_temp_json(r#"{"build": "124", "timestamp": "2024-01-02", "version": "1.1"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(old_file.path()).arg(new_file.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|version|buildNumber)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 15: if diffx /tmp/head_version /tmp/staged_version --ignore-keys-regex "^(timestamp|lastModified)$" --output json
#[test]
fn test_git_version_diff() -> Result<(), Box<dyn std::error::Error>> {
    let head = create_temp_json(r#"{"commit": "abc123", "timestamp": "2024-01-01"}"#);
    let staged = create_temp_json(r#"{"commit": "def456", "timestamp": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(head.path()).arg(staged.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|lastModified)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 16: diffx /tmp/old_config /tmp/new_config --ignore-keys-regex "^(version|buildNumber|timestamp)$" --output json
#[test]
fn test_ansible_config_diff() -> Result<(), Box<dyn std::error::Error>> {
    let old_config = create_temp_json(r#"{"playbook": "deploy", "version": "1.0", "timestamp": "2024-01-01"}"#);
    let new_config = create_temp_json(r#"{"playbook": "update", "version": "1.1", "timestamp": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(old_config.path()).arg(new_config.path())
        .arg("--ignore-keys-regex").arg("^(version|buildNumber|timestamp)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 17: diffx {} {} --output json > /dev/null (git alias)
#[test]
fn test_git_alias_diff() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"git": "version1"}"#);
    let file2 = create_temp_json(r#"{"git": "version2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 18: RUN diffx config/default.json config/production.json --ignore-keys-regex "^(environment|host|port)$" --output json
#[test]
fn test_docker_config_diff() -> Result<(), Box<dyn std::error::Error>> {
    let default = create_temp_json(r#"{"app": "myapp", "environment": "dev", "host": "localhost", "port": 3000}"#);
    let production = create_temp_json(r#"{"app": "myapp", "environment": "prod", "host": "prod.com", "port": 8080}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(default.path()).arg(production.path())
        .arg("--ignore-keys-regex").arg("^(environment|host|port)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 19: diffx config/runtime.json config/expected.json --output json
#[test]
fn test_runtime_config_check() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = create_temp_json(r#"{"memory": "512MB", "cpu": "1"}"#);
    let expected = create_temp_json(r#"{"memory": "1GB", "cpu": "2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(runtime.path()).arg(expected.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 20: if ! diffx "$BASELINE_CONFIG" "$CURRENT_CONFIG" --ignore-keys-regex "^(timestamp|uptime|pid)$" --output json
#[test]
fn test_monitoring_config_drift() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = create_temp_json(r#"{"service": "monitor", "alert": true, "timestamp": "2024-01-01"}"#);
    let current = create_temp_json(r#"{"service": "monitor", "alert": false, "timestamp": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(baseline.path()).arg(current.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|uptime|pid)$")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}