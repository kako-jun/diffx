use std::fs;
use std::process::Command;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_config_file_custom_path() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("custom-config.toml");
    
    // Create custom config file
    fs::write(&config_path, r#"
output = "json"
ignore_keys_regex = "^timestamp$"
epsilon = 0.001
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30, "timestamp": "2024-01-01", "value": 1.0001}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31, "timestamp": "2024-01-02", "value": 1.0002}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("[")); // JSON format
    assert!(stdout.contains("age"));
    assert!(!stdout.contains("timestamp")); // Should be ignored
    assert!(!stdout.contains("value")); // Should be within epsilon
}

#[test]
fn test_config_file_with_cli_override() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("override-config.toml");
    
    // Create config file with JSON output
    fs::write(&config_path, r#"
output = "json"
ignore_keys_regex = "^timestamp$"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30, "timestamp": "2024-01-01"}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31, "timestamp": "2024-01-02"}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg("yaml") // Override config file setting
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.starts_with("[")); // Not JSON format
    assert!(stdout.contains("age"));
    assert!(!stdout.contains("timestamp")); // Should still be ignored from config
}

#[test]
fn test_config_file_all_options() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("full-config.toml");
    
    // Create config file with all options
    fs::write(&config_path, r#"
output = "yaml"
format = "Json"
ignore_keys_regex = "^(timestamp|_.*)"
epsilon = 0.01
array_id_key = "id"
use_memory_optimization = true
batch_size = 500
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{
        "users": [{{"id": 1, "name": "Alice", "timestamp": "2024-01-01", "_internal": "data1"}}],
        "config": {{"timeout": 30.01}}
    }}"#).unwrap();
    writeln!(file2, r#"{{
        "users": [{{"id": 1, "name": "Alice", "timestamp": "2024-01-02", "_internal": "data2"}}],
        "config": {{"timeout": 30.02}}
    }}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0)); // No differences after filtering and epsilon
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty()); // No output for identical results
}

#[test]
fn test_config_file_directory_processing() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("dir-config.toml");
    
    // Create config file
    fs::write(&config_path, r#"
output = "json"
ignore_keys_regex = "^(timestamp|build_.*)"
"#).unwrap();
    
    // Create test directories
    let dir1 = temp_dir.path().join("dir1");
    let dir2 = temp_dir.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();
    
    // Create test files
    fs::write(dir1.join("config.json"), r#"{"name": "service1", "timestamp": "2024-01-01"}"#).unwrap();
    fs::write(dir2.join("config.json"), r#"{"name": "service2", "timestamp": "2024-01-02"}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(&dir1)
        .arg(&dir2)
        .arg("--recursive")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name"));
    assert!(!stdout.contains("timestamp")); // Should be ignored
}

#[test]
fn test_config_file_invalid_format() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid-config.toml");
    
    // Create invalid config file
    fs::write(&config_path, r#"
invalid_toml_syntax = [
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    // Should fall back to default behavior when config is invalid
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
}

#[test]
fn test_config_file_nonexistent_path() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent-config.toml");
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    // Should work with default behavior when config file doesn't exist
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
}

#[test]
fn test_config_file_environment_precedence() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("precedence-config.toml");
    
    // Create config file
    fs::write(&config_path, r#"
output = "json"
ignore_keys_regex = "^timestamp$"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30, "timestamp": "2024-01-01", "debug": "info"}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31, "timestamp": "2024-01-02", "debug": "warn"}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .env("DIFFX_IGNORE_KEYS_REGEX", "^(timestamp|debug)$") // Environment overrides config
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
    assert!(!stdout.contains("timestamp"));
    assert!(!stdout.contains("debug")); // Should be ignored by env var
}

#[test]
fn test_config_file_cli_precedence() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("cli-precedence-config.toml");
    
    // Create config file
    fs::write(&config_path, r#"
output = "json"
ignore_keys_regex = "^timestamp$"
epsilon = 0.1
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{"name": "Alice", "age": 30, "timestamp": "2024-01-01", "value": 1.0}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31, "timestamp": "2024-01-02", "value": 1.05}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--epsilon")
        .arg("0.01") // CLI overrides config
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("value")); // Should not be within 0.01 epsilon
}

#[test]
fn test_config_file_performance_options() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("performance-config.toml");
    
    // Create config file with performance options
    fs::write(&config_path, r#"
use_memory_optimization = true
batch_size = 100
array_id_key = "id"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    let data1 = serde_json::json!({
        "items": (0..50).map(|i| serde_json::json!({
            "id": i,
            "name": format!("Item{}", i),
            "active": i % 2 == 0
        })).collect::<Vec<_>>()
    });
    
    let data2 = serde_json::json!({
        "items": (0..50).map(|i| serde_json::json!({
            "id": i,
            "name": format!("Item{}", i),
            "active": i % 3 == 0
        })).collect::<Vec<_>>()
    });
    
    writeln!(file1, "{}", serde_json::to_string(&data1).unwrap()).unwrap();
    writeln!(file2, "{}", serde_json::to_string(&data2).unwrap()).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("active"));
}

#[test]
fn test_config_file_format_specification() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("format-config.toml");
    
    // Create config file with format specification
    fs::write(&config_path, r#"
format = "Json"
output = "yaml"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    // Write JSON content to files without .json extension
    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.starts_with("[")); // Should be YAML format
    assert!(stdout.contains("age"));
}

#[test]
fn test_config_file_complex_regex() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("regex-config.toml");
    
    // Create config file with complex regex
    fs::write(&config_path, r#"
ignore_keys_regex = "^(timestamp|_.*|temp_.*|.*_cache)$"
output = "json"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{
        "name": "Alice",
        "age": 30,
        "timestamp": "2024-01-01",
        "_internal": "data1",
        "temp_file": "temp1.txt",
        "result_cache": "cached1"
    }}"#).unwrap();
    writeln!(file2, r#"{{
        "name": "Alice",
        "age": 31,
        "timestamp": "2024-01-02",
        "_internal": "data2",
        "temp_file": "temp2.txt",
        "result_cache": "cached2"
    }}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
    assert!(!stdout.contains("timestamp"));
    assert!(!stdout.contains("_internal"));
    assert!(!stdout.contains("temp_file"));
    assert!(!stdout.contains("result_cache"));
}

#[test]
fn test_config_file_path_filtering() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("path-config.toml");
    
    // Create config file with path filtering
    fs::write(&config_path, r#"
# Note: path filtering is typically a CLI-only option
# This test verifies config file loading works even with unsupported options
output = "json"
ignore_keys_regex = "^metadata.*"
"#).unwrap();
    
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, r#"{{
        "database": {{"host": "localhost", "port": 5432}},
        "metadata": {{"version": "1.0"}},
        "cache": {{"ttl": 300}}
    }}"#).unwrap();
    writeln!(file2, r#"{{
        "database": {{"host": "localhost", "port": 5433}},
        "metadata": {{"version": "1.1"}},
        "cache": {{"ttl": 600}}
    }}"#).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--path")
        .arg("database") // CLI path filtering
        .env("DIFFX_CONFIG_PATH", &config_path)
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("port"));
    assert!(!stdout.contains("metadata"));
    assert!(!stdout.contains("cache"));
}