#[allow(unused_imports)]
use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_ignore_keys_regex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^age$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age:").not())
        .stdout(predicates::str::contains(
            r#"~ city: "New York" -> "Boston""#,
        ))
        .stdout(predicates::str::contains("+ items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_ignore_keys_regex_multiple_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^(age|city)$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age:").not())
        .stdout(predicates::str::contains("~ city:").not())
        .stdout(predicates::str::contains("+ items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_ignore_keys_regex_wildcard() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json")
        .arg("--ignore-keys-regex")
        .arg(".*e$"); // Matches keys ending with 'e' like 'name', 'age'
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ name:").not())
        .stdout(predicates::str::contains("~ age:").not());
    Ok(())
}

#[test]
fn test_ignore_keys_regex_case_sensitive() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"name": "John", "Name": "John"}"#)?;
    fs::write(&file2_path, r#"{"name": "Jane", "Name": "Jane"}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--ignore-keys-regex")
        .arg("^Name$"); // Capital N
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ name:")) // Should show lowercase
        .stdout(predicates::str::contains("~ Name:").not()); // Should ignore uppercase
    Ok(())
}

#[test]
fn test_ignore_keys_regex_nested_keys() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("debug|environment");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ application.debug:").not())
        .stdout(predicates::str::contains("~ application.environment:").not())
        .stdout(predicates::str::contains("~ database.host:")); // Should still show other changes
    Ok(())
}

#[test]
fn test_ignore_keys_regex_invalid_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("[invalid"); // Invalid regex
    let result = cmd.output()?;
    // Should handle invalid regex gracefully with error message
    assert!(!result.status.success()); // Should fail with meaningful error
    Ok(())
}

#[test]
fn test_epsilon_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/data1.json")
        .arg("../tests/fixtures/data2.json")
        .arg("--epsilon")
        .arg("0.00001");
    cmd.assert().success().stdout(predicates::str::is_empty()); // No differences expected within epsilon (empty output)
    Ok(())
}

#[test]
fn test_epsilon_different_precisions() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"value": 1.05}"#)?;
    fs::write(&file2_path, r#"{"value": 1.14}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--epsilon")
        .arg("0.1");
    cmd.assert().success().stdout(predicates::str::is_empty()); // Within epsilon
    Ok(())
}

#[test]
fn test_epsilon_exceeds_threshold() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"value": 1.0}"#)?;
    fs::write(&file2_path, r#"{"value": 1.5}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--epsilon")
        .arg("0.01");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ value: 1.0 -> 1.5")); // Exceeds epsilon
    Ok(())
}

#[test]
fn test_epsilon_negative_numbers() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"temp": -10.2}"#)?;
    fs::write(&file2_path, r#"{"temp": -10.6}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--epsilon")
        .arg("0.5");
    cmd.assert().success().stdout(predicates::str::is_empty()); // Within epsilon for negative numbers
    Ok(())
}

#[test]
fn test_epsilon_zero_values() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"zero": 0.0}"#)?;
    fs::write(&file2_path, r#"{"zero": 0.0005}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--epsilon")
        .arg("0.001");
    cmd.assert().success().stdout(predicates::str::is_empty()); // Small difference from zero
    Ok(())
}

#[test]
fn test_epsilon_very_large_numbers() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(&file1_path, r#"{"big": 1000000000000}"#)?;
    fs::write(&file2_path, r#"{"big": 1000000500000}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--epsilon")
        .arg("1000000");
    cmd.assert().success().stdout(predicates::str::is_empty()); // Large epsilon for large numbers
    Ok(())
}

#[test]
fn test_epsilon_invalid_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--epsilon")
        .arg("invalid");
    let result = cmd.output()?;
    // Should handle invalid epsilon value gracefully
    assert!(!result.status.success());
    Ok(())
}

#[test]
fn test_epsilon_negative_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--epsilon")
        .arg("-0.1");
    let _result = cmd.output()?;
    // Should handle negative epsilon appropriately (either error or absolute value)
    // The exact behavior depends on implementation
    Ok(())
}

#[test]
fn test_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json")
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [id=1].age: 25 -> 26"))
        .stdout(
            predicates::str::contains("+ [id=3]: ")
                .and(predicates::str::contains(r#""id":3"#))
                .and(predicates::str::contains(r#""name":"Charlie""#))
                .and(predicates::str::contains(r#""age":28"#)),
        )
        .stdout(predicates::str::contains("~ [0].").not()); // Ensure not comparing by index
    Ok(())
}

#[test]
fn test_array_id_key_missing_id() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(
        &file1_path,
        r#"{"items": [{"name": "A"}, {"uuid": "123", "name": "B"}]}"#,
    )?;
    fs::write(&file2_path, r#"{"items": [{"uuid": "123", "name": "C"}]}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--array-id-key")
        .arg("uuid");
    // Should handle objects without the specified ID key gracefully
    cmd.assert().code(1);
    Ok(())
}

#[test]
fn test_array_id_key_duplicate_ids() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(
        &file1_path,
        r#"{"items": [{"id": 1, "name": "A"}, {"id": 1, "name": "B"}]}"#,
    )?;
    fs::write(&file2_path, r#"{"items": [{"id": 1, "name": "C"}]}"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--array-id-key")
        .arg("id");
    // Should handle duplicate IDs appropriately
    cmd.assert().code(1);
    Ok(())
}

#[test]
fn test_array_id_key_different_types() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(
        &file1_path,
        r#"{"items": [{"key": "str1", "val": 1}, {"key": 123, "val": 2}]}"#,
    )?;
    fs::write(
        &file2_path,
        r#"{"items": [{"key": "str1", "val": 10}, {"key": 123, "val": 20}]}"#,
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--array-id-key")
        .arg("key");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("[key=\"str1\"]"))
        .stdout(predicates::str::contains("[key=123]"));
    Ok(())
}

#[test]
fn test_array_id_key_nested_arrays() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    fs::write(
        &file1_path,
        r#"{"groups": [{"id": "A", "users": [{"id": 1, "name": "John"}]}]}"#,
    )?;
    fs::write(
        &file2_path,
        r#"{"groups": [{"id": "A", "users": [{"id": 1, "name": "Jane"}]}]}"#,
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path)
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("[id=\"A\"]"))
        .stdout(predicates::str::contains("[id=1]"));
    Ok(())
}

#[test]
fn test_path_filtering_application() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(predicates::str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ))
        .stdout(predicates::str::contains("database").not())
        .stdout(predicates::str::contains("services").not());
    Ok(())
}

#[test]
fn test_path_filtering_services() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("services");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ services.auth.url: \"http://localhost:8080\" -> \"https://auth.example.com\""))
        .stdout(predicates::str::contains("~ services.cache.enabled: false -> true"))
        .stdout(predicates::str::contains("~ services.cache.url: \"redis://localhost:6379\" -> \"redis://cache.example.com:6379\""))
        .stdout(predicates::str::contains("application").not())
        .stdout(predicates::str::contains("database").not());
    Ok(())
}

#[test]
fn test_path_filtering_database() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("database");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ database.host: \"localhost\" -> \"prod-db.example.com\"",
        ))
        .stdout(predicates::str::contains(
            "~ database.name: \"myapp_dev\" -> \"myapp_prod\"",
        ))
        .stdout(predicates::str::contains("~ database.timeout: 30 -> 60"))
        .stdout(predicates::str::contains("application").not())
        .stdout(predicates::str::contains("services").not());
    Ok(())
}

#[test]
fn test_ignore_case_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test ignore-case option - should not show differences for case-only changes
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json")
        .arg("--ignore-case");
    cmd.assert()
        .code(0) // No differences when ignoring case
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_ignore_case_option_shows_differences_without_flag() -> Result<(), Box<dyn std::error::Error>>
{
    // Test that case differences are shown without the ignore-case flag
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains(
            "~ status: \"Active\" -> \"ACTIVE\"",
        ))
        .stdout(predicates::str::contains("~ level: \"Info\" -> \"INFO\""));
    Ok(())
}

#[test]
fn test_ignore_whitespace_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test ignore-whitespace option - should not show differences for whitespace-only changes
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(0) // No differences when ignoring whitespace
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_ignore_whitespace_option_shows_differences_without_flag(
) -> Result<(), Box<dyn std::error::Error>> {
    // Test that whitespace differences are shown without the ignore-whitespace flag
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains(
            "~ text: \"Hello  World\" -> \"Hello World\"",
        ))
        .stdout(predicates::str::contains(
            "~ message: \"Test\\tValue\" -> \"Test Value\"",
        ));
    Ok(())
}

#[test]
fn test_combined_ignore_options() -> Result<(), Box<dyn std::error::Error>> {
    // Test combining ignore-case and ignore-whitespace options
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json") // Mix case and whitespace differences
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Still differences (different keys)
        .stdout(predicates::str::contains("- level: \"Info\""))
        .stdout(predicates::str::contains("+ message: \"Test Value\""));
    Ok(())
}

#[test]
fn test_quiet_option_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test quiet option with identical files - should output nothing and exit 0
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--quiet");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_quiet_option_with_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test quiet option with different files - should output nothing and exit 1
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--quiet");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_brief_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test brief option - should only show filenames, not differences
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--brief");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains(
            "Files ../tests/fixtures/file1.json and ../tests/fixtures/file2.json differ",
        ))
        .stdout(predicates::str::contains("age").not()) // Should not show actual differences
        .stdout(predicates::str::contains("city").not());
    Ok(())
}

#[test]
fn test_brief_option_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test brief option with identical files - should output nothing
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--brief");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_verbose_basic_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicates::str::contains("Optimization enabled:"))
        .stderr(predicates::str::contains("Batch size:"))
        .stderr(predicates::str::contains("Input file information:"))
        .stderr(predicates::str::contains("Parse time:"))
        .stderr(predicates::str::contains("Diff computation time:"))
        .stderr(predicates::str::contains("Total differences found:"))
        .stderr(predicates::str::contains("Performance summary:"))
        .stderr(predicates::str::contains("Total processing time:"));
    Ok(())
}

#[test]
fn test_format_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test --format option with various formats
    for format in ["json", "yaml", "toml", "ini", "xml", "csv"] {
        let mut cmd = diffx_cmd();
        cmd.arg("../tests/fixtures/file1.json")
            .arg("../tests/fixtures/file2.json")
            .arg("--format")
            .arg(format);

        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Should recognize format option
            assert!(
                !stderr.contains("unrecognized"),
                "Format {format} should be recognized"
            );
        }
    }
    Ok(())
}

#[test]
fn test_output_format_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test --output option with various output formats
    for output_format in ["cli", "json", "yaml", "unified"] {
        let mut cmd = diffx_cmd();
        cmd.arg("../tests/fixtures/file1.json")
            .arg("../tests/fixtures/file2.json")
            .arg("--output")
            .arg(output_format);

        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !stderr.contains("unrecognized"),
                "Output format {output_format} should be recognized"
            );
        }
    }
    Ok(())
}

#[test]
fn test_version_option() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.3.2").or(predicate::str::contains("diffx")));
    Ok(())
}

#[test]
fn test_version_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-V");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.3.2").or(predicate::str::contains("diffx")));
    Ok(())
}

#[test]
fn test_version_with_other_args() -> Result<(), Box<dyn std::error::Error>> {
    // Test that version option takes precedence
    let mut cmd = diffx_cmd();
    cmd.arg("--version").arg("file1.json").arg("file2.json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"));
    Ok(())
}

#[test]
fn test_help_option() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Arguments:"))
        .stdout(predicate::str::contains("Options:"));
    Ok(())
}

#[test]
fn test_help_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-h");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"))
        .stdout(predicate::str::contains("Usage:"));
    Ok(())
}

#[test]
fn test_help_comprehensive_content() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--output"))
        .stdout(predicate::str::contains("--epsilon"))
        .stdout(predicate::str::contains("--ignore-keys-regex"))
        .stdout(predicate::str::contains("--array-id-key"))
        .stdout(predicate::str::contains("--no-color"));
    Ok(())
}

#[test]
fn test_help_with_other_args() -> Result<(), Box<dyn std::error::Error>> {
    // Test that help option takes precedence
    let mut cmd = diffx_cmd();
    cmd.arg("--help").arg("file1.json").arg("file2.json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
    Ok(())
}
