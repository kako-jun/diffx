use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_json_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(str::contains(r#""Modified""#))
        .stdout(str::contains(r#""age""#))
        .stdout(str::contains(r#""city""#))
        .stdout(str::contains(r#""New York""#))
        .stdout(str::contains(r#""Boston""#))
        .stdout(str::contains(r#""Added""#))
        .stdout(str::contains(r#""items[2]""#))
        .stdout(str::contains(r#""orange""#));
    Ok(())
}

#[test]
fn test_yaml_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(str::contains(
            r#"- Modified:
  - age
  - 30
  - 31"#,
        ))
        .stdout(str::contains(
            r#"- Modified:
  - city
  - New York
  - Boston"#,
        ))
        .stdout(str::contains(
            r#"- Added:
  - items[2]
  - orange"#,
        ));
    Ok(())
}

#[test]
fn test_unified_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("unified");
    cmd.assert()
        .code(1)
        .stdout(str::contains("-  \"age\": 30,"))
        .stdout(str::contains("+  \"age\": 31,"))
        .stdout(str::contains("-  \"city\": \"New York\","));
    Ok(())
}

#[test]
fn test_context_option_unified_output() -> Result<(), Box<dyn std::error::Error>> {
    // Test context option with unified output format
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("2");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains("-      \"port\": 5432"))
        .stdout(str::contains("+      \"port\": 5433"))
        .stdout(str::contains("\"host\": \"localhost\"")) // Context line
        .stdout(str::contains("\"name\": \"myapp\"")); // Context line
    Ok(())
}

#[test]
fn test_context_option_zero_context() -> Result<(), Box<dyn std::error::Error>> {
    // Test context option with zero context - should show only changed lines
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("0");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains("-      \"port\": 5432"))
        .stdout(str::contains("+      \"port\": 5433"))
        .stdout(str::contains("\"host\": \"localhost\"").not()) // No context
        .stdout(str::contains("\"name\": \"myapp\"").not()); // No context
    Ok(())
}