use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_verbose_key_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose")
        .arg("--ignore-keys-regex")
        .arg("age");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Key filtering configuration:"))
        .stderr(predicate::str::contains("Regex pattern: age"));
    Ok(())
}

#[test]
fn test_verbose_epsilon_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose")
        .arg("--epsilon")
        .arg("0.1");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains(
            "Numerical tolerance configuration:",
        ))
        .stderr(predicate::str::contains("Epsilon value: 0.1"));
    Ok(())
}

#[test]
fn test_verbose_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json")
        .arg("--verbose")
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Array tracking configuration:"))
        .stderr(predicate::str::contains("ID key for array elements: id"));
    Ok(())
}

#[test]
fn test_verbose_path_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--verbose")
        .arg("--path")
        .arg("app"); // Use "app" path which should have differences
    cmd.assert()
        .stderr(predicate::str::contains("Path filtering results:"))
        .stderr(predicate::str::contains("Filter path: app"))
        .stderr(predicate::str::contains("Total differences before filter:"))
        .stderr(predicate::str::contains("Differences after filter:"));
    Ok(())
}

#[test]
fn test_verbose_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Input file information:"))
        .stderr(predicate::str::contains("bytes"))
        .stderr(predicate::str::contains("Parse time:"))
        .stderr(predicate::str::contains("µs").or(predicate::str::contains("ms")))
        .stderr(predicate::str::contains("Diff computation time:"))
        .stderr(predicate::str::contains("Total processing time:"))
        .stderr(predicate::str::contains("Memory optimization:"));
    Ok(())
}

#[test]
fn test_verbose_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json")
        .arg("--verbose");
    cmd.assert()
        .code(0)
        .stderr(predicate::str::contains("Total differences found: 0"))
        .stderr(predicate::str::contains("Performance summary:"));
    Ok(())
}