use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_ini_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.ini")
        .arg("../tests/fixtures/file2.ini");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ section1.key2: \"value2\" -> \"new_value2\"",
        ))
        .stdout(predicate::str::contains("+ section2.key4: \"value4\""));
    Ok(())
}