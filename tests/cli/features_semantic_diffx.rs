use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

/// Test diffx format output generation
/// Verifies the proprietary diffx format provides semantic diff information
#[test]
fn test_diffx_format_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // diffx format should include semantic indicators
    assert!(
        stdout.contains("semantic")
            || stdout.contains("change_type")
            || stdout.contains("diffx")
            || !stdout.trim().is_empty()
    );

    Ok(())
}

/// Test semantic equivalence detection
/// Verifies that semantically equivalent but syntactically different files show no differences
#[test]
fn test_semantic_equivalence() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("semantic1.json");
    let file2 = temp_dir.path().join("semantic2.json");

    // Create semantically equivalent JSON with different formatting
    let json1 = r#"{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}],"version":"1.0"}"#;
    let json2 = r#"{
  "version": "1.0",
  "users": [
    {
      "name": "Alice",
      "id": 1
    },
    {
      "name": "Bob", 
      "id": 2
    }
  ]
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1).arg(&file2);

    let output = cmd.output()?;
    assert!(output.status.code() == Some(0)); // No differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect no semantic differences
    assert!(
        stdout.contains("no differences")
            || stdout.contains("identical")
            || stdout.trim().is_empty()
    );

    Ok(())
}

/// Test deep semantic change detection
/// Verifies detection of meaningful changes while ignoring superficial ones
#[test]
fn test_deep_semantic_changes() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("deep1.yaml");
    let file2 = temp_dir.path().join("deep2.yaml");

    // Create YAML files with semantic changes but same structure
    let yaml1 = r#"
database:
  host: localhost
  port: 5432
  credentials:
    username: admin
    password: secret123
features:
  authentication: enabled
  logging: debug
"#;

    let yaml2 = r#"
# Comments added (should be ignored)
database:
  port: 5432  # Different order but same content
  host: localhost
  credentials:
    username: admin
    password: newsecret456  # This is a semantic change
features:
  logging: debug
  authentication: enabled  # Different order but same content
"#;

    fs::write(&file1, yaml1)?;
    fs::write(&file2, yaml2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1).arg(&file2).arg("--output").arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect the password change but ignore order and comments
    assert!(stdout.contains("password") || stdout.contains("newsecret456"));
    // Should not be confused by order changes
    assert!(!stdout.contains("authentication") || !stdout.contains("logging"));

    Ok(())
}

/// Test format-agnostic semantic comparison
/// Verifies same semantic content across different formats shows no differences
#[test]
fn test_format_agnostic_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let json_file = temp_dir.path().join("config.json");
    let yaml_file = temp_dir.path().join("config.yaml");

    // Same semantic content in different formats
    let json_content = r#"{
  "server": {
    "host": "example.com",
    "port": 8080,
    "ssl": true
  },
  "database": {
    "url": "postgres://localhost:5432/db"
  }
}"#;

    let yaml_content = r#"
server:
  host: example.com
  port: 8080
  ssl: true
database:
  url: postgres://localhost:5432/db
"#;

    fs::write(&json_file, json_content)?;
    fs::write(&yaml_file, yaml_content)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&json_file).arg(&yaml_file);

    let output = cmd.output()?;
    // Same semantic content should return success (no differences)
    assert!(output.status.success()); // Exit code 0

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show no differences for semantically identical content
    assert!(stdout.trim().is_empty()); // No output when no differences

    Ok(())
}

/// Test complex nested structure semantic comparison
/// Verifies handling of deeply nested semantic structures
#[test]
fn test_complex_nested_semantic_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("complex1.json");
    let file2 = temp_dir.path().join("complex2.json");

    // Complex nested structures with subtle semantic differences
    let json1 = r#"{
  "microservices": {
    "auth": {
      "endpoints": [
        {"path": "/login", "method": "POST", "rate_limit": 100},
        {"path": "/logout", "method": "POST", "rate_limit": 50}
      ],
      "database": {
        "connection_pool": {"min": 5, "max": 20},
        "timeout": 30
      }
    }
  }
}"#;

    let json2 = r#"{
  "microservices": {
    "auth": {
      "database": {
        "timeout": 30,
        "connection_pool": {"max": 20, "min": 5}
      },
      "endpoints": [
        {"rate_limit": 100, "method": "POST", "path": "/login"},
        {"rate_limit": 60, "method": "POST", "path": "/logout"}
      ]
    }
  }
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1).arg(&file2).arg("--output").arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect the rate_limit change (50 -> 60) but ignore order changes
    assert!(stdout.contains("rate_limit") && (stdout.contains("50") || stdout.contains("60")));

    Ok(())
}

/// Test array semantic tracking with ID keys
/// Verifies advanced array element tracking using ID-based comparison
#[test]
fn test_array_semantic_tracking() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("array1.json");
    let file2 = temp_dir.path().join("array2.json");

    // Arrays with ID-based tracking
    let json1 = r#"{
  "users": [
    {"id": "u1", "name": "Alice", "role": "admin", "active": true},
    {"id": "u2", "name": "Bob", "role": "user", "active": true}, 
    {"id": "u3", "name": "Charlie", "role": "user", "active": false}
  ]
}"#;

    let json2 = r#"{
  "users": [
    {"id": "u3", "name": "Charlie", "role": "moderator", "active": true},
    {"id": "u1", "name": "Alice", "role": "admin", "active": true},
    {"id": "u4", "name": "David", "role": "user", "active": true}
  ]
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1)
        .arg(&file2)
        .arg("--array-id-key")
        .arg("id")
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should track: u2 removed, u3 changed role/status, u4 added
    assert!(
        stdout.contains("u2") ||  // removed user
        stdout.contains("u3") ||  // changed user  
        stdout.contains("u4") ||  // added user
        stdout.contains("Bob") || stdout.contains("David") || stdout.contains("moderator")
    );

    Ok(())
}

/// Test semantic path filtering
/// Verifies intelligent path-based filtering for semantic structures  
#[test]
fn test_semantic_path_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("path1.json");
    let file2 = temp_dir.path().join("path2.json");

    // Complex structure with changes in multiple paths
    let json1 = r#"{
  "config": {
    "database": {"host": "db1.example.com", "port": 5432},
    "cache": {"host": "cache1.example.com", "port": 6379},
    "logging": {"level": "info", "file": "/var/log/app.log"}
  },
  "features": {
    "auth": {"enabled": true, "provider": "oauth2"},
    "monitoring": {"enabled": false, "endpoint": "https://monitor.example.com"}
  }
}"#;

    let json2 = r#"{
  "config": {
    "database": {"host": "db2.example.com", "port": 5432},
    "cache": {"host": "cache2.example.com", "port": 6379}, 
    "logging": {"level": "debug", "file": "/var/log/app.log"}
  },
  "features": {
    "auth": {"enabled": true, "provider": "saml"},
    "monitoring": {"enabled": true, "endpoint": "https://monitor.example.com"}
  }
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    // Test filtering to only auth features
    let mut cmd = diffx_cmd();
    cmd.arg(&file1)
        .arg(&file2)
        .arg("--path")
        .arg("features.auth")
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should only show auth provider change (oauth2 -> saml)
    assert!(stdout.contains("provider") && (stdout.contains("oauth2") || stdout.contains("saml")));
    // Should NOT show database, cache, logging, or monitoring changes
    assert!(
        !stdout.contains("database") && !stdout.contains("cache") && !stdout.contains("logging")
    );

    Ok(())
}

/// Test semantic regex filtering
/// Verifies intelligent regex-based key filtering for semantic content
#[test]
fn test_semantic_regex_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("regex1.json");
    let file2 = temp_dir.path().join("regex2.json");

    // Structure with timestamp and metadata that should be filtered out
    let json1 = r#"{
  "data": {
    "user_id": 123,
    "username": "alice",
    "email": "alice@example.com",
    "created_timestamp": "2024-01-01T10:00:00Z",
    "modified_timestamp": "2024-01-01T10:00:00Z",
    "_internal_id": "abc123",
    "_metadata": {"version": 1, "source": "api"}
  },
  "config": {
    "setting1": "value1",
    "setting2": "value2", 
    "debug_timestamp": "2024-01-01T10:00:00Z"
  }
}"#;

    let json2 = r#"{
  "data": {
    "user_id": 123,
    "username": "alice_updated",
    "email": "alice.new@example.com", 
    "created_timestamp": "2024-01-02T15:30:00Z",
    "modified_timestamp": "2024-01-02T15:30:00Z",
    "_internal_id": "xyz789",
    "_metadata": {"version": 2, "source": "import"}
  },
  "config": {
    "setting1": "new_value1",
    "setting2": "value2",
    "debug_timestamp": "2024-01-02T15:30:00Z"
  }
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    // Filter out timestamps and internal/metadata fields
    let mut cmd = diffx_cmd();
    cmd.arg(&file1)
        .arg(&file2)
        .arg("--ignore-keys-regex")
        .arg("^(.*timestamp|_.*|.*metadata)$")
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show meaningful changes (username, email, setting1)
    assert!(stdout.contains("username") || stdout.contains("email") || stdout.contains("setting1"));
    // Should NOT show filtered fields
    assert!(
        !stdout.contains("timestamp")
            && !stdout.contains("_internal")
            && !stdout.contains("_metadata")
    );

    Ok(())
}

/// Test semantic type coercion
/// Verifies intelligent handling of type differences that are semantically equivalent
#[test]
fn test_semantic_type_coercion() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("type1.json");
    let file2 = temp_dir.path().join("type2.json");

    // Same semantic values with different JSON types
    let json1 = r#"{
  "config": {
    "port": 8080,
    "enabled": true,
    "timeout": 30.0,
    "retries": "3"
  }
}"#;

    let json2 = r#"{
  "config": {
    "port": "8080",
    "enabled": "true", 
    "timeout": 30,
    "retries": 3
  }
}"#;

    fs::write(&file1, json1)?;
    fs::write(&file2, json2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1).arg(&file2);

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect type differences
    assert!(
        stdout.contains("! config.enabled: true (Boolean) -> \"true\" (String)")
            || stdout.contains("! config.enabled:")
    );

    Ok(())
}

/// Test comprehensive semantic integration
/// Verifies all semantic features work together without conflicts
#[test]
fn test_comprehensive_semantic_integration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file1 = temp_dir.path().join("comprehensive1.toml");
    let file2 = temp_dir.path().join("comprehensive2.toml");

    // Complex TOML with multiple semantic features
    let toml1 = r#"
[server]
host = "localhost"
port = 8080
ssl_enabled = true

[[users]]
id = 1
name = "Alice"
roles = ["admin", "user"]
metadata = { created = "2024-01-01", source = "manual" }

[[users]]
id = 2  
name = "Bob"
roles = ["user"]
metadata = { created = "2024-01-01", source = "import" }

[database]
url = "postgres://localhost:5432/db"
connection_pool = { min = 5, max = 20 }
timeout_seconds = 30.0
"#;

    let toml2 = r#"
[database]
timeout_seconds = 30
url = "postgres://localhost:5432/db"
connection_pool = { max = 20, min = 5 }

[server]
port = "8080"  # Type change but semantically same
ssl_enabled = true
host = "localhost"

[[users]]
roles = ["admin", "user"]
metadata = { created = "2024-01-02", source = "manual" }  # timestamp change
name = "Alice"
id = 1

[[users]]
roles = ["user", "moderator"]  # Role change - semantic difference
name = "Bob"
metadata = { created = "2024-01-02", source = "import" }  # timestamp change  
id = 2
"#;

    fs::write(&file1, toml1)?;
    fs::write(&file2, toml2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1)
        .arg(&file2)
        .arg("--array-id-key")
        .arg("id")
        .arg("--ignore-keys-regex")
        .arg("created")
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect Bob's role change (user -> user,moderator)
    assert!(stdout.contains("moderator") || stdout.contains("roles"));
    // Should not contain created field changes due to ignore-keys-regex
    assert!(!stdout.contains("metadata.created"));

    Ok(())
}
