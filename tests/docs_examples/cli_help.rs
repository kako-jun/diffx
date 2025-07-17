use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_help_output_matches_docs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"))
        .stdout(predicate::str::contains("USAGE"))
        .stdout(predicate::str::contains("--ignore-case"))
        .stdout(predicate::str::contains("--ignore-whitespace"))
        .stdout(predicate::str::contains("--quiet"))
        .stdout(predicate::str::contains("--brief"))
        .stdout(predicate::str::contains("--context"));
    Ok(())
}

#[test]
fn test_version_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"));
    Ok(())
}