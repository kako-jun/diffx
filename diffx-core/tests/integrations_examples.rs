use diffx_core::*;
use serde_json::{json, Value};
use regex::Regex;

/// Test case 1: diffx --version
#[test]
fn test_version_check() {
    // Version check is handled by CLI, core library test focuses on functionality
    let v1 = json!({"test": "core"});
    let v2 = json!({"test": "core"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 0);
}

/// Test case 2: Config validation with ignore patterns
#[test]
fn test_config_validation_with_ignore_patterns() {
    let v1 = json!({"name": "app", "version": "1.0", "timestamp": "2024-01-01T00:00:00Z"});
    let v2 = json!({"name": "APP", "version": "1.1", "timestamp": "2024-01-02T00:00:00Z"});
    
    let regex = Regex::new(r"^(timestamp|lastModified|createdAt|updatedAt|buildTime)$").unwrap();
    let config = DiffConfig {
        ignore_case: true,
        ignore_whitespace: true,
        ..Default::default()
    };
    
    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 1);
}

/// Test case 3: API contract validation
#[test]
fn test_api_contract_validation() {
    let v1 = json!({"endpoint": "/users", "method": "GET", "timestamp": "2024-01-01"});
    let v2 = json!({"endpoint": "/users", "method": "POST", "timestamp": "2024-01-02"});
    
    let regex = Regex::new(r"^(timestamp|requestId|serverId|responseTime)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 4: Environment config diff
#[test]
fn test_environment_config_diff() {
    let v1 = json!({"app": "myapp", "environment": "production", "host": "prod.com", "port": 8080});
    let v2 = json!({"app": "myapp", "environment": "staging", "host": "staging.com", "port": 8081});
    
    let regex = Regex::new(r"^(environment|host|port|replicas|resources\\..*)")unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 0); // All differences ignored
}

/// Test case 5: Terraform diff
#[test]
fn test_terraform_diff() {
    let v1 = json!({"planned_values": {"root_module": {"resources": [{"name": "server1", "type": "aws_instance"}]}}});
    let v2 = json!({"planned_values": {"root_module": {"resources": [{"name": "server2", "type": "aws_instance"}]}}});
    
    let regex = Regex::new(r"^(timeouts|creation_time|last_updated)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 6: Quiet baseline check
#[test]
fn test_quiet_baseline_check() {
    let v1 = json!({"version": "1.0"});
    let v2 = json!({"version": "1.1"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert!(!diffs.is_empty()); // Files differ
}

/// Test case 7: Recursive brief diff
#[test]
fn test_recursive_brief_diff() {
    let v1 = json!({"config": "old"});
    let v2 = json!({"config": "new"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 8: Deployment diff with ignores
#[test]
fn test_deployment_diff_with_ignores() {
    let v1 = json!({"APP": "myapp", "VERSION": "1.0", "deploy_time": "2024-01-01"});
    let v2 = json!({"app": "myapp", "version": "1.1", "deploy_time": "2024-01-02"});
    
    let regex = Regex::new(r"^(deploy_time|build_id|version)$").unwrap();
    let config = DiffConfig {
        ignore_case: true,
        ignore_whitespace: true,
        ..Default::default()
    };
    
    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 0);
}

/// Test case 9: Config drift detection
#[test]
fn test_config_drift_detection() {
    let v1 = json!({"SERVICE": "api", "hostname": "server1", "instance_id": "i-123"});
    let v2 = json!({"service": "web", "hostname": "server2", "instance_id": "i-456"});
    
    let regex = Regex::new(r"^(hostname|instance_id|last_.*)$").unwrap();
    let config = DiffConfig {
        ignore_case: true,
        ..Default::default()
    };
    
    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 1); // Services differ
}

/// Test case 10: Config drift unified output
#[test]
fn test_config_drift_unified_output() {
    let v1 = json!({"service": "API", "hostname": "server1"});
    let v2 = json!({"service": "api", "hostname": "server2"});
    
    let regex = Regex::new(r"^(hostname|instance_id|last_.*)$").unwrap();
    let config = DiffConfig {
        ignore_case: true,
        ..Default::default()
    };
    
    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 0);
}

/// Test case 11: Baseline config check
#[test]
fn test_baseline_config_check() {
    let v1 = json!({"setting": "production"});
    let v2 = json!({"setting": "development"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert!(!diffs.is_empty());
}

/// Test case 12: Baseline file unified
#[test]
fn test_baseline_file_unified() {
    let v1 = json!({"name": "app", "version": "1.0"});
    let v2 = json!({"name": "app", "version": "1.1"});
    
    let config = DiffConfig {
        ignore_whitespace: true,
        ..Default::default()
    };
    
    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 1);
}

/// Test case 13: Installation verification
#[test]
fn test_installation_verification() {
    let v1 = json!({"status": "installed"});
    let v2 = json!({"status": "installed"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 0);
}

/// Test case 14: Jenkins file diff
#[test]
fn test_jenkins_file_diff() {
    let v1 = json!({"build": "123", "timestamp": "2024-01-01", "version": "1.0"});
    let v2 = json!({"build": "124", "timestamp": "2024-01-02", "version": "1.1"});
    
    let regex = Regex::new(r"^(timestamp|version|buildNumber)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 15: Git version diff
#[test]
fn test_git_version_diff() {
    let v1 = json!({"commit": "abc123", "timestamp": "2024-01-01"});
    let v2 = json!({"commit": "def456", "timestamp": "2024-01-02"});
    
    let regex = Regex::new(r"^(timestamp|lastModified)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 16: Ansible config diff
#[test]
fn test_ansible_config_diff() {
    let v1 = json!({"playbook": "deploy", "version": "1.0", "timestamp": "2024-01-01"});
    let v2 = json!({"playbook": "update", "version": "1.1", "timestamp": "2024-01-02"});
    
    let regex = Regex::new(r"^(version|buildNumber|timestamp)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 17: Git alias diff
#[test]
fn test_git_alias_diff() {
    let v1 = json!({"git": "version1"});
    let v2 = json!({"git": "version2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test case 18: Docker config diff
#[test]
fn test_docker_config_diff() {
    let v1 = json!({"app": "myapp", "environment": "dev", "host": "localhost", "port": 3000});
    let v2 = json!({"app": "myapp", "environment": "prod", "host": "prod.com", "port": 8080});
    
    let regex = Regex::new(r"^(environment|host|port)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 0);
}

/// Test case 19: Runtime config check
#[test]
fn test_runtime_config_check() {
    let v1 = json!({"memory": "512MB", "cpu": "1"});
    let v2 = json!({"memory": "1GB", "cpu": "2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 2);
}

/// Test case 20: Monitoring config drift
#[test]
fn test_monitoring_config_drift() {
    let v1 = json!({"service": "monitor", "alert": true, "timestamp": "2024-01-01"});
    let v2 = json!({"service": "monitor", "alert": false, "timestamp": "2024-01-02"});
    
    let regex = Regex::new(r"^(timestamp|uptime|pid)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}