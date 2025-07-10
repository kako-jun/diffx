use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// Environment variables are now implemented in diffx CLI

#[test]
fn test_environment_variables_output() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_OUTPUT", "json")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Modified"));
    assert!(stdout.starts_with("["));
}

#[test]
fn test_environment_variables_ignore_keys() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{"name": "Alice", "timestamp": "2024-01-01", "_internal": "data"}}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{"name": "Alice", "timestamp": "2024-01-02", "_internal": "different"}}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_IGNORE_KEYS_REGEX", "^(timestamp|_.*)")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0)); // No differences after filtering
}

#[test]
fn test_environment_variables_epsilon() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"value": 1.0001}}"#).unwrap();
    writeln!(file2, r#"{{"value": 1.0002}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_EPSILON", "0.001")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0)); // No differences within epsilon
}

#[test]
fn test_environment_variables_combined() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{"name": "Alice", "timestamp": "2024-01-01", "value": 1.0001}}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{"name": "Bob", "timestamp": "2024-01-02", "value": 1.0002}}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .env("DIFFX_OUTPUT", "json")
        .env("DIFFX_IGNORE_KEYS_REGEX", "^timestamp$")
        .env("DIFFX_EPSILON", "0.001")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Modified"));
    assert!(stdout.contains("name"));
    assert!(!stdout.contains("timestamp"));
    assert!(!stdout.contains("value"));
}

#[test]
fn test_help_command_long() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("diffx"));
    assert!(stdout.contains("OPTIONS"));
    assert!(stdout.contains("Arguments"));
}

#[test]
fn test_help_command_short() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", "-h"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("diffx"));
}

#[test]
fn test_version_command_long() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("diffx"));
    assert!(stdout.trim().chars().next().unwrap().is_ascii_digit() || stdout.contains("0."));
}

#[test]
fn test_version_command_short() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", "-V"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("diffx"));
}

#[test]
fn test_exit_code_no_differences() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 30}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

#[test]
fn test_exit_code_differences_found() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that there is some output (either stdout or stderr)
    assert!(!stdout.is_empty() || !stderr.is_empty());
}

#[test]
fn test_exit_code_file_not_found() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg("nonexistent_file.json")
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    assert_ne!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No such file") || stderr.contains("not found"));
}

#[test]
fn test_exit_code_invalid_json() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{invalid json}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    assert_ne!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("parse") || stderr.contains("invalid"));
}

#[test]
fn test_exit_code_invalid_regex() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("[invalid")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    assert_ne!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("regex") || stderr.contains("invalid"));
}

#[test]
fn test_error_handling_nonexistent_file() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg("definitely_nonexistent_file.json")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read file") || stderr.contains("No such file"));
}

#[test]
fn test_error_handling_invalid_format() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("invalid_format")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("format"));
}

#[test]
fn test_error_handling_invalid_output_format() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("invalid_output")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("output"));
}

#[test]
fn test_error_handling_invalid_epsilon() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"value": 1.0}}"#).unwrap();
    writeln!(file2, r#"{{"value": 1.1}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--epsilon")
        .arg("invalid_number")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("epsilon") || stderr.contains("number"));
}

#[test]
fn test_error_handling_invalid_batch_size() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--batch-size")
        .arg("invalid_size")
        .output()
        .expect("Failed to execute command");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("batch"));
}

#[test]
fn test_complex_option_combination_1() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{
        "name": "Alice",
        "age": 30,
        "timestamp": "2024-01-01",
        "settings": {{"theme": "dark"}},
        "users": [{{"id": 1, "name": "User1"}}]
    }}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{
        "name": "Alice",
        "age": 31,
        "timestamp": "2024-01-02",
        "settings": {{"theme": "light"}},
        "users": [{{"id": 1, "name": "User1"}}]
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--ignore-keys-regex")
        .arg("^timestamp$")
        .arg("--array-id-key")
        .arg("id")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("["));
    assert!(stdout.contains("Modified"));
    assert!(!stdout.contains("timestamp"));
}

#[test]
fn test_complex_option_combination_2() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{
        "database": {{"host": "localhost", "port": 5432}},
        "api": {{"version": "1.0", "timeout": 30.0}},
        "metadata": {{"created": "2024-01-01"}}
    }}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{
        "database": {{"host": "localhost", "port": 5433}},
        "api": {{"version": "1.0", "timeout": 30.001}},
        "metadata": {{"created": "2024-01-02"}}
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--path")
        .arg("database")
        .arg("--epsilon")
        .arg("0.01")
        .arg("--ignore-keys-regex")
        .arg("^metadata.*")
        .arg("--output")
        .arg("yaml")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("port"));
    assert!(!stdout.contains("timeout"));
    assert!(!stdout.contains("metadata"));
}

#[test]
fn test_complex_option_combination_3() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{
        "users": [
            {{"id": 1, "name": "Alice", "last_login": "2024-01-01"}},
            {{"id": 2, "name": "Bob", "last_login": "2024-01-02"}}
        ]
    }}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{
        "users": [
            {{"id": 1, "name": "Alice", "last_login": "2024-01-03"}},
            {{"id": 3, "name": "Charlie", "last_login": "2024-01-04"}}
        ]
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--array-id-key")
        .arg("id")
        .arg("--ignore-keys-regex")
        .arg("^last_login$")
        .arg("--path")
        .arg("users")
        .arg("--optimize")
        .arg("--batch-size")
        .arg("100")
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Added"));
    assert!(stdout.contains("Removed"));
    // Note: last_login appears because entire objects are added/removed, not modified
    // ignore_keys_regex only applies to field modifications, not complete object additions/removals
}

#[test]
fn test_complex_option_combination_4() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(
        file1,
        r#"{{
        "services": {{
            "web": {{"replicas": 3, "memory": "512Mi"}},
            "db": {{"replicas": 1, "memory": "1Gi"}}
        }},
        "version": "1.0.0",
        "timestamp": "2024-01-01T00:00:00Z"
    }}"#
    )
    .unwrap();
    writeln!(
        file2,
        r#"{{
        "services": {{
            "web": {{"replicas": 5, "memory": "512Mi"}},
            "db": {{"replicas": 1, "memory": "1Gi"}}
        }},
        "version": "1.0.1",
        "timestamp": "2024-01-02T00:00:00Z"
    }}"#
    )
    .unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg("unified")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|version)$")
        .arg("--path")
        .arg("services")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("replicas"));
    assert!(!stdout.contains("timestamp"));
    assert!(!stdout.contains("version"));
}

#[test]
fn test_stdin_with_format_specification() {
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg("-")
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{\"name\": \"Alice\", \"age\": 30}")
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
}

#[test]
fn test_stdin_empty_input() {
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg("-")
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"").expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("parse") || stderr.contains("empty") || stderr.contains("invalid"));
}

#[test]
fn test_stdin_invalid_json() {
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg("-")
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{invalid json}")
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("parse") || stderr.contains("invalid"));
}

#[test]
fn test_stdin_with_output_format() {
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg("-")
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{\"name\": \"Alice\", \"age\": 30}")
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("["));
    assert!(stdout.contains("Modified"));
}

#[test]
fn test_performance_options_combination() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    // Create larger JSON for performance testing
    let data1 = serde_json::json!({
        "users": (0..100).map(|i| serde_json::json!({
            "id": i,
            "name": format!("User{}", i),
            "active": i % 2 == 0
        })).collect::<Vec<_>>()
    });

    let data2 = serde_json::json!({
        "users": (0..100).map(|i| serde_json::json!({
            "id": i,
            "name": format!("User{}", i),
            "active": i % 3 == 0
        })).collect::<Vec<_>>()
    });

    writeln!(file1, "{}", serde_json::to_string(&data1).unwrap()).unwrap();
    writeln!(file2, "{}", serde_json::to_string(&data2).unwrap()).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--optimize")
        .arg("--batch-size")
        .arg("50")
        .arg("--array-id-key")
        .arg("id")
        .arg("--path")
        .arg("users")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("active"));
}

#[test]
fn test_batch_size_without_optimize() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--batch-size")
        .arg("1000")
        .output()
        .expect("Failed to execute command");

    // Should work even without --optimize
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
}

#[test]
fn test_very_large_batch_size() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, r#"{{"name": "Alice", "age": 30}}"#).unwrap();
    writeln!(file2, r#"{{"name": "Alice", "age": 31}}"#).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--"])
        .arg(file1.path())
        .arg(file2.path())
        .arg("--format")
        .arg("json")
        .arg("--optimize")
        .arg("--batch-size")
        .arg("100000")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age"));
}
