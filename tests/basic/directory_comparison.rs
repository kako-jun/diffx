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
        .arg("../tests/fixtures/dir2")
        .arg("--recursive");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("--- Comparing b.json ---"))
        .stdout(predicates::str::contains(
            "~ key3: \"value3\" -> \"new_value3\"",
        ));
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_non_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicates::str::contains("Directory scan results:"))
        .stderr(predicates::str::contains("Recursive mode: false"));
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicates::str::contains("Directory scan results:"))
        .stderr(predicates::str::contains("Recursive mode: true"));
    Ok(())
}

#[test]
fn test_directory_with_common_subdirectories() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("Common subdirectories:"))
        .stdout(predicates::str::contains("subdir")); // Should show common subdir but not compare files inside
    Ok(())
}
