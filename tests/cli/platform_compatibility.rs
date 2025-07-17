use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_auto_optimization_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test that small files use standard mode, large files auto-optimize
    // Since we can't easily test large files, we test that small files work normally
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_auto_optimization_on_small_files() -> Result<(), Box<dyn std::error::Error>> {
    // Test that automatic optimization works correctly on small files
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains("~ age: 30 -> 31")); // Same output as standard mode
    Ok(())
}

#[test]
fn test_complex_options_combination() -> Result<(), Box<dyn std::error::Error>> {
    // Test combination of multiple new options
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|version)$")
        .arg("--path")
        .arg("application")
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ));
    Ok(())
}