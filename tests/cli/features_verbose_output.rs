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
        .stderr(predicates::str::contains("Key filtering configuration:"))
        .stderr(predicates::str::contains("Regex pattern: age"));
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
        .stderr(predicates::str::contains(
            "Numerical tolerance configuration:",
        ))
        .stderr(predicates::str::contains("Epsilon value: 0.1"));
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
        .stderr(predicates::str::contains("Array tracking configuration:"))
        .stderr(predicates::str::contains("ID key for array elements: id"));
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
        .stderr(predicates::str::contains("Path filtering results:"))
        .stderr(predicates::str::contains("Filter path: app"))
        .stderr(predicates::str::contains(
            "Total differences before filter:",
        ))
        .stderr(predicates::str::contains("Differences after filter:"));
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
        .stderr(predicates::str::contains("Input file information:"))
        .stderr(predicates::str::contains("bytes"))
        .stderr(predicates::str::contains("Parse time:"))
        .stderr(predicates::str::contains("µs").or(predicates::str::contains("ms")))
        .stderr(predicates::str::contains("Diff computation time:"))
        .stderr(predicates::str::contains("Total processing time:"))
        .stderr(predicates::str::contains("Memory optimization:"));
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
        .stderr(predicates::str::contains("Total differences found: 0"))
        .stderr(predicates::str::contains("Performance summary:"));
    Ok(())
}
