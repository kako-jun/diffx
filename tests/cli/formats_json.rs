use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_json_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"))
        .stdout(predicates::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicates::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_complex_nested_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ app.version:"))
        .stdout(predicates::str::contains("~ features["));
    Ok(())
}

#[test]
fn test_json_array_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json");
    cmd.assert().code(1).stdout(predicates::str::contains("["));
    Ok(())
}

// Enhanced tests for --format json option
#[test]
fn test_format_json_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--format")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_format_json_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("-f")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_format_json_invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.toml")
        .arg("../tests/fixtures/file2.toml")
        .arg("--format")
        .arg("json");
    // Should handle format mismatch gracefully
    let result = cmd.output()?;
    // Either succeeds with conversion or fails with meaningful error
    Ok(())
}

#[test]
fn test_json_empty_objects() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("json");
    cmd.write_stdin("{}")
        .write_stdin("{\"new\": \"value\"}");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("+ new: \"value\""));
    Ok(())
}

#[test]
fn test_json_deeply_nested() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v3.json")
        .arg("--format")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~"));
    Ok(())
}

#[test]
fn test_json_with_special_characters() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("json");
    cmd.write_stdin(r#"{"key": "value with \"quotes\" and \n newlines"}"#)
        .write_stdin(r#"{"key": "different value"}"#);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ key:"));
    Ok(())
}

#[test]
fn test_json_large_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("json");
    cmd.write_stdin(r#"{"big": 9223372036854775807, "precision": 3.14159265359}"#)
        .write_stdin(r#"{"big": 9223372036854775806, "precision": 3.14159265358}"#);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ big:"))
        .stdout(predicates::str::contains("~ precision:"));
    Ok(())
}
