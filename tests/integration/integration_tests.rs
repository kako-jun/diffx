// Integration tests for diffx components
// Test the interaction between different parts of the system

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn run_diffx_command(args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "diffx", "--"]);
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

        assert!(output.status.success());

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
            "--format",
            "json",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));

        // Verify it's valid JSON
        let _parsed: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
    }

    #[test]
    fn test_verbose_mode_integration() {
        let content1 = "line1\nline2\nline3\nline4";
        let content2 = "line1\nmodified line\nline3\nline4";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--verbose",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() > 100); // Verbose output should be substantial
    }

    #[test]
    fn test_quiet_mode_integration() {
        let content1 = "original content";
        let content2 = "modified content";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--quiet",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() < 100); // Quiet output should be minimal
    }

    #[test]
    fn test_no_color_option_integration() {
        let content1 = "test content line";
        let content2 = "modified content line";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--no-color",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Output should not contain ANSI color codes
        assert!(!stdout.contains("\x1b["));
    }

    #[test]
    fn test_brief_mode_integration() {
        let content1 = "line1\nline2\nline3\nline4\nline5";
        let content2 = "line1\nmodified\nline3\nchanged\nline5";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--brief",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() < 200); // Brief output should be short
    }

    #[test]
    fn test_output_file_integration() {
        let content1 = "original data";
        let content2 = "modified data";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);
        let output_file = NamedTempFile::new().expect("Failed to create output file");

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--output",
            output_file.path().to_str().unwrap(),
        ]);

        assert!(output.status.success());

        // Check that output file was created and has content
        let output_content =
            std::fs::read_to_string(output_file.path()).expect("Failed to read output file");
        assert!(!output_content.is_empty());
    }

    #[test]
    fn test_directory_comparison_integration() {
        let output = run_diffx_command(&["tests/fixtures/dir1", "tests/fixtures/dir2"]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("dir") || stdout.contains("file") || stdout.len() > 0);
    }

    #[test]
    fn test_multiple_format_support_integration() {
        // Test YAML files
        let yaml1 = "name: John\nage: 25\nactive: true";
        let yaml2 = "name: John\nage: 26\nactive: true";

        let file1 = create_temp_file_with_content(yaml1);
        let file2 = create_temp_file_with_content(yaml2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ]);

        assert!(output.status.success());

        // Test TOML files
        let toml1 = "[server]\nport = 8080\nhost = \"localhost\"";
        let toml2 = "[server]\nport = 8081\nhost = \"localhost\"";

        let toml_file1 = create_temp_file_with_content(toml1);
        let toml_file2 = create_temp_file_with_content(toml2);

        let toml_output = run_diffx_command(&[
            toml_file1.path().to_str().unwrap(),
            toml_file2.path().to_str().unwrap(),
        ]);

        assert!(toml_output.status.success());
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
            "--ignore-array-order",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // With array order ignored, should detect minimal or no differences
        assert!(stdout.len() < 500);
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
            "--semantic",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Semantic diff should recognize these as equivalent
        assert!(stdout.contains("equivalent") || stdout.len() < 200);
    }

    #[test]
    fn test_context_lines_integration() {
        let content1 = "line1\nline2\noriginal\nline4\nline5\nline6\nline7";
        let content2 = "line1\nline2\nmodified\nline4\nline5\nline6\nline7";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--context",
            "2",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should show context lines around the change
        assert!(stdout.contains("line2") && stdout.contains("line4"));
    }

    #[test]
    fn test_ignore_whitespace_integration() {
        let content1 = "line1\n  line2  \nline3";
        let content2 = "line1\nline2\nline3";

        let file1 = create_temp_file_with_content(content1);
        let file2 = create_temp_file_with_content(content2);

        let output = run_diffx_command(&[
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
            "--ignore-whitespace",
        ]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // With whitespace ignored, should show minimal differences
        assert!(stdout.len() < 300);
    }
}
