#[allow(unused_imports)]
use assert_cmd::prelude::*;
// Integration tests for diffx components
// Test the interaction between different parts of the system

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn run_diffx_command(args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "diffx", "--"]);
    command.args(args);
    command.output().expect("Failed to execute diffx command")
}

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_basic_file_diff_integration() {
        let content1 = r#"{"version": "1.0", "data": {"count": 10}}"#;
        let content2 = r#"{"version": "1.1", "data": {"count": 15}}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ]);

        // diffx returns exit code 1 when differences are found (like Unix diff)
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("version") || stdout.contains("count") || stdout.contains("diff"));
    }

    #[test]
    fn test_json_format_integration() {
        let content1 = r#"{"users": [{"id": 1, "name": "Alice", "active": true}]}"#;
        let content2 = r#"{"users": [{"id": 1, "name": "Alice", "active": false}]}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "-o",
            "json",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));

        // Verify it's valid JSON
        let _parsed: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
    }

    #[test]
    fn test_verbose_mode_integration() {
        let content1 = r#"{"name": "test", "value": 1}"#;
        let content2 = r#"{"name": "test", "value": 2}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--verbose",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty()); // Verbose output should have content
    }

    #[test]
    fn test_quiet_mode_integration() {
        let content1 = r#"{"value": 1}"#;
        let content2 = r#"{"value": 2}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--quiet",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty()); // Quiet mode should produce no output
    }

    #[test]
    fn test_no_color_option_integration() {
        let content1 = r#"{"test": "content"}"#;
        let content2 = r#"{"test": "modified"}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--no-color",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Output should not contain ANSI color codes
        assert!(!stdout.contains("\x1b["));
    }

    #[test]
    fn test_brief_mode_integration() {
        let content1 = r#"{"a": 1, "b": 2, "c": 3}"#;
        let content2 = r#"{"a": 1, "b": 5, "c": 3}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--brief",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() < 200); // Brief output should be short
    }

    #[test]
    fn test_output_format_integration() {
        let content1 = r#"{"data": "original"}"#;
        let content2 = r#"{"data": "modified"}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "-o",
            "json",
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        // Output should be in JSON format
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
        let _parsed: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
    }

    #[test]
    fn test_directory_comparison_integration() {
        // Use absolute paths or paths relative to project root
        let dir1 = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests/fixtures/dir1");
        let dir2 = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests/fixtures/dir2");

        let output = run_diffx_command(&[
            dir1.to_str().unwrap(),
            dir2.to_str().unwrap(),
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
    }

    #[test]
    fn test_multiple_format_support_integration() {
        // Test YAML files
        let yaml1 = "name: John\nage: 25\nactive: true";
        let yaml2 = "name: John\nage: 26\nactive: true";

        let mut file1 = NamedTempFile::with_suffix(".yaml").expect("Failed to create temp file");
        file1.write_all(yaml1.as_bytes()).expect("Failed to write to temp file");
        let mut file2 = NamedTempFile::with_suffix(".yaml").expect("Failed to create temp file");
        file2.write_all(yaml2.as_bytes()).expect("Failed to write to temp file");

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        // Test TOML files
        let toml1 = "[server]\nport = 8080\nhost = \"localhost\"";
        let toml2 = "[server]\nport = 8081\nhost = \"localhost\"";

        let mut toml_file1 = NamedTempFile::with_suffix(".toml").expect("Failed to create temp file");
        toml_file1.write_all(toml1.as_bytes()).expect("Failed to write to temp file");
        let mut toml_file2 = NamedTempFile::with_suffix(".toml").expect("Failed to create temp file");
        toml_file2.write_all(toml2.as_bytes()).expect("Failed to write to temp file");

        let toml_output = run_diffx_command(&[
            toml_file1.path().to_str().unwrap(),
            toml_file2.path().to_str().unwrap(),
        ]);

        // Exit code 1 when differences found
        assert_eq!(toml_output.status.code(), Some(1));
    }

    #[test]
    fn test_array_handling_integration() {
        let content1 = r#"{"items": [{"id": 1, "name": "A"}, {"id": 2, "name": "B"}]}"#;
        let content2 = r#"{"items": [{"id": 2, "name": "B"}, {"id": 1, "name": "A"}]}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--array-id-key",
            "id",
        ]);

        // With array ID key, should detect no differences
        assert_eq!(output.status.code(), Some(0));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty() || stdout.contains("No differences"));
    }

    #[test]
    fn test_semantic_diff_integration() {
        let content1 = r#"{"config": {"enabled": true, "timeout": 30}}"#;
        let content2 = r#"{"config": {"timeout": 30, "enabled": true}}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ]);

        // diffx detects no semantic differences for reordered JSON objects
        assert_eq!(output.status.code(), Some(0));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty() || stdout.contains("No differences"));
    }

    #[test]
    fn test_context_lines_integration() {
        // Context lines are for unified format, but diffx uses structured format
        // Test with actual structured data
        let content1 = r#"{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}"#;
        let content2 = r#"{"a": 1, "b": 2, "c": 99, "d": 4, "e": 5}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ]);

        // Exit code 1 when differences found
        assert_eq!(output.status.code(), Some(1));

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should show the difference
        assert!(stdout.contains("c"));
    }

    #[test]
    fn test_ignore_whitespace_integration() {
        let content1 = r#"{"name":   "test"  ,  "value":  1   }"#;
        let content2 = r#"{"name":"test","value":1}"#;

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--ignore-whitespace",
        ]);

        // With whitespace ignored in JSON parsing, should detect no differences
        assert_eq!(output.status.code(), Some(0));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty() || stdout.contains("No differences"));
    }
}
