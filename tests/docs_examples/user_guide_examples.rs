use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_json_example_from_docs() -> Result<(), Box<dyn std::error::Error>> {
    // This should match examples shown in documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age:"))
        .stdout(predicate::str::contains("~ city:"))
        .stdout(predicate::str::contains("+ items["));
    Ok(())
}

#[test]
fn test_json_output_example_from_docs() -> Result<(), Box<dyn std::error::Error>> {
    // This should match JSON output examples in documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""Added""#));
    Ok(())
}

#[test]
fn test_ignore_keys_example_from_docs() -> Result<(), Box<dyn std::error::Error>> {
    // This should match ignore keys examples in documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(password|secret_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("password").not())
        .stdout(predicate::str::contains("secret_").not());
    Ok(())
}