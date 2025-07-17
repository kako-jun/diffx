use assert_cmd::prelude::*;
use predicates::str;
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
        .stdout(str::contains("diffx"))
        .stdout(str::contains("USAGE"))
        .stdout(str::contains("--ignore-case"))
        .stdout(str::contains("--ignore-whitespace"))
        .stdout(str::contains("--quiet"))
        .stdout(str::contains("--brief"))
        .stdout(str::contains("--context"));
    Ok(())
}

#[test]
fn test_version_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(str::contains("diffx"));
    Ok(())
}