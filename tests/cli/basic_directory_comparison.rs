use assert_cmd::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_directory_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_non_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--verbose");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--verbose");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_directory_with_common_subdirectories() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("b.json"))
        .stdout(predicates::str::contains("config.json"))
        .stdout(predicates::str::contains("subdir/nested.json")); // Shows files in subdirectories
    Ok(())
}
