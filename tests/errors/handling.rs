use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_nonexistent_files_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("nonexistent1.json")
        .arg("nonexistent2.json");
    cmd.assert()
        .code(2) // Error exit code
        .stderr(str::contains("No such file").or(str::contains("not found")));
    Ok(())
}

#[test]
fn test_directory_vs_file_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/file1.json");
    cmd.assert().code(2).stderr(str::contains(
        "Cannot compare directory and file",
    ));
    Ok(())
}

#[test]
fn test_invalid_format_specification() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--format")
        .arg("invalid_format");
    cmd.assert()
        .code(2) // Error exit code
        .stderr(str::contains("format").or(str::contains("invalid")));
    Ok(())
}

#[test]
fn test_invalid_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("invalid_output");
    cmd.assert()
        .code(2) // Error exit code
        .stderr(str::contains("output").or(str::contains("invalid")));
    Ok(())
}