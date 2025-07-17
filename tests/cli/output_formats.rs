use assert_cmd::prelude::*;
use predicates::prelude::*;
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
        .stdout(predicates::str::contains(r#""Modified""#))
        .stdout(predicates::str::contains(r#""age""#))
        .stdout(predicates::str::contains(r#""city""#))
        .stdout(predicates::str::contains(r#""New York""#))
        .stdout(predicates::str::contains(r#""Boston""#))
        .stdout(predicates::str::contains(r#""Added""#))
        .stdout(predicates::str::contains(r#""items[2]""#))
        .stdout(predicates::str::contains(r#""orange""#));
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
        .stdout(predicates::str::contains(
            r#"- Modified:
  - age
  - 30
  - 31"#,
        ))
        .stdout(predicates::str::contains(
            r#"- Modified:
  - city
  - New York
  - Boston"#,
        ))
        .stdout(predicates::str::contains(
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
        .stdout(predicates::str::contains("-  \"age\": 30,"))
        .stdout(predicates::str::contains("+  \"age\": 31,"))
        .stdout(predicates::str::contains("-  \"city\": \"New York\","));
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
        .stdout(predicates::str::contains("-      \"port\": 5432"))
        .stdout(predicates::str::contains("+      \"port\": 5433"))
        .stdout(predicates::str::contains("\"host\": \"localhost\"")) // Context line
        .stdout(predicates::str::contains("\"name\": \"myapp\"")); // Context line
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
        .stdout(predicates::str::contains("-      \"port\": 5432"))
        .stdout(predicates::str::contains("+      \"port\": 5433"))
        .stdout(predicates::str::contains("\"host\": \"localhost\"").not()) // No context
        .stdout(predicates::str::contains("\"name\": \"myapp\"").not()); // No context
    Ok(())
}
