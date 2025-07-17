use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_toml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.toml")
        .arg("../tests/fixtures/file2.toml");
    cmd.assert()
        .code(1)
        .stdout(str::contains("~ age: 30 -> 31"))
        .stdout(str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(str::contains("  + items[2]: \"orange\""));
    Ok(())
}