use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

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
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_complex_nested_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ app.version:"))
        .stdout(predicate::str::contains("~ features["));
    Ok(())
}

#[test]
fn test_json_array_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("["));
    Ok(())
}