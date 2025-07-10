use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_git_integration_simulation() {
    let mut current_file = NamedTempFile::new().unwrap();
    let mut old_file = NamedTempFile::new().unwrap();

    // Simulate git show HEAD~1:config.json
    writeln!(
        old_file,
        r#"{{
        "version": "1.0.0",
        "database": {{"host": "localhost", "port": 5432}},
        "features": {{"new_ui": false}}
    }}"#
    )
    .unwrap();

    writeln!(
        current_file,
        r#"{{
        "version": "1.0.1",
        "database": {{"host": "localhost", "port": 5432}},
        "features": {{"new_ui": true}}
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(old_file.path())
        .arg(current_file.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg("unified")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("version"));
    assert!(stdout.contains("new_ui"));
}

#[test]
fn test_cicd_pipeline_simulation() {
    let mut expected_file = NamedTempFile::new().unwrap();
    let mut actual_file = NamedTempFile::new().unwrap();

    writeln!(
        expected_file,
        r#"{{
        "service": "api",
        "version": "1.0.0",
        "environment": "production",
        "deployment_time": "2024-01-01T00:00:00Z",
        "build_id": "12345"
    }}"#
    )
    .unwrap();

    writeln!(
        actual_file,
        r#"{{
        "service": "api",
        "version": "1.0.1",
        "environment": "production",
        "deployment_time": "2024-01-02T00:00:00Z",
        "build_id": "12346"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(expected_file.path())
        .arg(actual_file.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^(deployment_time|build_id)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("version"));
    assert!(!stdout.contains("deployment_time"));
    assert!(!stdout.contains("build_id"));
}

#[test]
fn test_monitoring_script_simulation() {
    let mut baseline_file = NamedTempFile::new().unwrap();
    let mut current_file = NamedTempFile::new().unwrap();

    // Simulate configuration baseline
    writeln!(
        baseline_file,
        r#"{{
        "database": {{"max_connections": 100}},
        "cache": {{"size": "1GB"}},
        "logging": {{"level": "INFO"}},
        "uptime": 1000,
        "timestamp": "2024-01-01T00:00:00Z"
    }}"#
    )
    .unwrap();

    // Simulate current configuration
    writeln!(
        current_file,
        r#"{{
        "database": {{"max_connections": 200}},
        "cache": {{"size": "1GB"}},
        "logging": {{"level": "INFO"}},
        "uptime": 2000,
        "timestamp": "2024-01-02T00:00:00Z"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(baseline_file.path())
        .arg(current_file.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^(uptime|timestamp)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("max_connections"));
    assert!(!stdout.contains("uptime"));
    assert!(!stdout.contains("timestamp"));
}

#[test]
fn test_kubernetes_manifest_comparison() {
    let temp_dir = TempDir::new().unwrap();
    let manifest1 = temp_dir.path().join("deployment.yaml");
    let manifest2 = temp_dir.path().join("deployment.new.yaml");

    fs::write(
        &manifest1,
        r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  namespace: default
  creationTimestamp: "2024-01-01T00:00:00Z"
  resourceVersion: "12345"
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
  template:
    spec:
      containers:
      - name: myapp
        image: myapp:1.0.0
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
"#,
    )
    .unwrap();

    fs::write(
        &manifest2,
        r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  namespace: default
  creationTimestamp: "2024-01-02T00:00:00Z"
  resourceVersion: "12346"
spec:
  replicas: 5
  selector:
    matchLabels:
      app: myapp
  template:
    spec:
      containers:
      - name: myapp
        image: myapp:1.0.1
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
"#,
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(&manifest1)
        .arg(&manifest2)
        .arg("--ignore-keys-regex")
        .arg("^(creationTimestamp|resourceVersion)$")
        .arg("--format")
        .arg("yaml")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("replicas"));
    assert!(stdout.contains("image"));
    assert!(stdout.contains("memory"));
    assert!(!stdout.contains("creationTimestamp"));
    assert!(!stdout.contains("resourceVersion"));
}

#[test]
fn test_api_contract_testing() {
    let mut expected_response = NamedTempFile::new().unwrap();
    let mut actual_response = NamedTempFile::new().unwrap();

    writeln!(
        expected_response,
        r#"{{
        "users": [
            {{"id": 1, "name": "Alice", "email": "alice@example.com"}},
            {{"id": 2, "name": "Bob", "email": "bob@example.com"}}
        ],
        "pagination": {{"page": 1, "total": 2}},
        "timestamp": "2024-01-01T00:00:00Z",
        "request_id": "abc123"
    }}"#
    )
    .unwrap();

    writeln!(
        actual_response,
        r#"{{
        "users": [
            {{"id": 1, "name": "Alice", "email": "alice@example.com"}},
            {{"id": 2, "name": "Bob", "email": "bob@example.com"}}
        ],
        "pagination": {{"page": 1, "total": 2}},
        "timestamp": "2024-01-02T00:00:00Z",
        "request_id": "def456"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(expected_response.path())
        .arg(actual_response.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|request_id)$")
        .arg("--array-id-key")
        .arg("id")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0)); // No differences after filtering
    assert!(output.stdout.is_empty());
}

#[test]
fn test_data_pipeline_validation() {
    let mut input_data = NamedTempFile::new().unwrap();
    let mut output_data = NamedTempFile::new().unwrap();

    writeln!(
        input_data,
        r#"{{
        "records": [
            {{"id": 1, "value": 100.0, "processed": false}},
            {{"id": 2, "value": 200.0, "processed": false}}
        ],
        "batch_id": "batch_001",
        "timestamp": "2024-01-01T00:00:00Z"
    }}"#
    )
    .unwrap();

    writeln!(
        output_data,
        r#"{{
        "records": [
            {{"id": 1, "value": 105.0, "processed": true}},
            {{"id": 2, "value": 210.0, "processed": true}}
        ],
        "batch_id": "batch_002",
        "timestamp": "2024-01-01T01:00:00Z"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(input_data.path())
        .arg(output_data.path())
        .arg("--format")
        .arg("json")
        .arg("--array-id-key")
        .arg("id")
        .arg("--ignore-keys-regex")
        .arg("^(batch_id|timestamp)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("value"));
    assert!(stdout.contains("processed"));
    assert!(!stdout.contains("batch_id"));
    assert!(!stdout.contains("timestamp"));
}

#[test]
fn test_configuration_drift_detection() {
    let mut production_config = NamedTempFile::new().unwrap();
    let mut staging_config = NamedTempFile::new().unwrap();

    writeln!(
        production_config,
        r#"{{
        "database": {{
            "host": "prod-db.example.com",
            "port": 5432,
            "max_connections": 100,
            "ssl": true
        }},
        "cache": {{
            "redis_url": "redis://prod-cache.example.com:6379",
            "ttl": 3600
        }},
        "logging": {{
            "level": "INFO",
            "output": "file"
        }},
        "environment": "production"
    }}"#
    )
    .unwrap();

    writeln!(
        staging_config,
        r#"{{
        "database": {{
            "host": "staging-db.example.com",
            "port": 5432,
            "max_connections": 50,
            "ssl": true
        }},
        "cache": {{
            "redis_url": "redis://staging-cache.example.com:6379",
            "ttl": 1800
        }},
        "logging": {{
            "level": "DEBUG",
            "output": "console"
        }},
        "environment": "staging"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(production_config.path())
        .arg(staging_config.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^(host|.*_url|environment)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("max_connections"));
    assert!(stdout.contains("ttl"));
    assert!(stdout.contains("level"));
    assert!(stdout.contains("output"));
    assert!(!stdout.contains("host"));
    assert!(!stdout.contains("redis_url"));
    assert!(!stdout.contains("environment"));
}

#[test]
fn test_terraform_state_comparison() {
    let mut current_state = NamedTempFile::new().unwrap();
    let mut planned_state = NamedTempFile::new().unwrap();

    writeln!(
        current_state,
        r#"{{
        "resources": [
            {{
                "type": "aws_instance",
                "name": "web_server",
                "attributes": {{
                    "instance_type": "t2.micro",
                    "ami": "ami-12345678",
                    "last_updated": "2024-01-01T00:00:00Z"
                }}
            }}
        ],
        "terraform_version": "1.0.0",
        "serial": 1
    }}"#
    )
    .unwrap();

    writeln!(
        planned_state,
        r#"{{
        "resources": [
            {{
                "type": "aws_instance",
                "name": "web_server",
                "attributes": {{
                    "instance_type": "t2.small",
                    "ami": "ami-12345678",
                    "last_updated": "2024-01-02T00:00:00Z"
                }}
            }}
        ],
        "terraform_version": "1.0.0",
        "serial": 2
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(current_state.path())
        .arg(planned_state.path())
        .arg("--format")
        .arg("json")
        .arg("--path")
        .arg("resources")
        .arg("--ignore-keys-regex")
        .arg("^(last_updated|serial)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("instance_type"));
    assert!(!stdout.contains("last_updated"));
    assert!(!stdout.contains("serial"));
}

#[test]
fn test_security_policy_comparison() {
    let mut baseline_policy = NamedTempFile::new().unwrap();
    let mut current_policy = NamedTempFile::new().unwrap();

    writeln!(
        baseline_policy,
        r#"{{
        "permissions": [
            {{"resource": "users", "actions": ["read", "write"]}},
            {{"resource": "admin", "actions": ["read"]}}
        ],
        "roles": [
            {{"name": "user", "permissions": ["users:read"]}},
            {{"name": "admin", "permissions": ["users:read", "users:write", "admin:read"]}}
        ],
        "policy_version": "1.0",
        "last_audit": "2024-01-01T00:00:00Z"
    }}"#
    )
    .unwrap();

    writeln!(current_policy, r#"{{
        "permissions": [
            {{"resource": "users", "actions": ["read", "write"]}},
            {{"resource": "admin", "actions": ["read", "write"]}}
        ],
        "roles": [
            {{"name": "user", "permissions": ["users:read"]}},
            {{"name": "admin", "permissions": ["users:read", "users:write", "admin:read", "admin:write"]}}
        ],
        "policy_version": "1.1",
        "last_audit": "2024-01-02T00:00:00Z"
    }}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(baseline_policy.path())
        .arg(current_policy.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^(policy_version|last_audit)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("permissions"));
    assert!(stdout.contains("admin"));
    assert!(!stdout.contains("policy_version"));
    assert!(!stdout.contains("last_audit"));
}

#[test]
fn test_database_schema_comparison() {
    let mut schema_v1 = NamedTempFile::new().unwrap();
    let mut schema_v2 = NamedTempFile::new().unwrap();

    writeln!(
        schema_v1,
        r#"{{
        "tables": [
            {{
                "name": "users",
                "columns": [
                    {{"name": "id", "type": "integer", "primary_key": true}},
                    {{"name": "name", "type": "varchar(255)", "nullable": false}},
                    {{"name": "email", "type": "varchar(255)", "nullable": false}}
                ]
            }},
            {{
                "name": "orders",
                "columns": [
                    {{"name": "id", "type": "integer", "primary_key": true}},
                    {{"name": "user_id", "type": "integer", "foreign_key": "users.id"}}
                ]
            }}
        ],
        "version": "1.0",
        "migration_timestamp": "2024-01-01T00:00:00Z"
    }}"#
    )
    .unwrap();

    writeln!(
        schema_v2,
        r#"{{
        "tables": [
            {{
                "name": "users",
                "columns": [
                    {{"name": "id", "type": "integer", "primary_key": true}},
                    {{"name": "name", "type": "varchar(255)", "nullable": false}},
                    {{"name": "email", "type": "varchar(255)", "nullable": false}},
                    {{"name": "created_at", "type": "timestamp", "nullable": false}}
                ]
            }},
            {{
                "name": "orders",
                "columns": [
                    {{"name": "id", "type": "integer", "primary_key": true}},
                    {{"name": "user_id", "type": "integer", "foreign_key": "users.id"}},
                    {{"name": "total", "type": "decimal(10,2)", "nullable": false}}
                ]
            }}
        ],
        "version": "2.0",
        "migration_timestamp": "2024-01-02T00:00:00Z"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(schema_v1.path())
        .arg(schema_v2.path())
        .arg("--format")
        .arg("json")
        .arg("--array-id-key")
        .arg("name")
        .arg("--ignore-keys-regex")
        .arg("^(version|migration_timestamp)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("columns"));
    assert!(stdout.contains("created_at"));
    assert!(stdout.contains("total"));
    assert!(!stdout.contains("version"));
    assert!(!stdout.contains("migration_timestamp"));
}

#[test]
fn test_multi_environment_comparison() {
    let temp_dir = TempDir::new().unwrap();
    let prod_config = temp_dir.path().join("prod.json");
    let staging_config = temp_dir.path().join("staging.json");

    fs::write(
        &prod_config,
        r#"{
        "app": {
            "name": "myapp",
            "version": "1.0.0",
            "environment": "production"
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "name": "myapp_prod",
            "ssl": true
        },
        "cache": {
            "enabled": true,
            "ttl": 3600
        },
        "features": {
            "experimental": false,
            "debug": false
        }
    }"#,
    )
    .unwrap();

    fs::write(
        &staging_config,
        r#"{
        "app": {
            "name": "myapp",
            "version": "1.0.0-staging",
            "environment": "staging"
        },
        "database": {
            "host": "staging-db.example.com",
            "port": 5432,
            "name": "myapp_staging",
            "ssl": true
        },
        "cache": {
            "enabled": true,
            "ttl": 1800
        },
        "features": {
            "experimental": true,
            "debug": true
        }
    }"#,
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(&prod_config)
        .arg(&staging_config)
        .arg("--ignore-keys-regex")
        .arg("^(environment|host|name|version)$")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ttl"));
    assert!(stdout.contains("experimental"));
    assert!(stdout.contains("debug"));
    assert!(!stdout.contains("environment"));
    assert!(!stdout.contains("host"));
    assert!(!stdout.contains("name"));
}
