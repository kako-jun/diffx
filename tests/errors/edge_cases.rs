use assert_cmd::prelude::*;
use predicates::prelude::*;
use predicates::ord::predicate;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_empty_files() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let empty1_path = temp_dir.path().join("empty1.txt");
    let empty2_path = temp_dir.path().join("empty2.txt");

    fs::write(&empty1_path, "")?;
    fs::write(&empty2_path, "")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&empty1_path).arg(&empty2_path);
    // Empty files should be handled gracefully, either success or appropriate error
    cmd.assert().code(predicate::in_iter(vec![0, 3])); // Accept both success and file error codes

    Ok(())
}

#[test]
fn test_very_large_input() -> Result<(), Box<dyn std::error::Error>> {
    // This test would typically involve creating large test data
    // For now, we'll just test that the command doesn't crash with normal sized data
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--verbose");
    cmd.assert()
        .stderr(predicates::str::contains("Performance summary:"));
    Ok(())
}

#[test]
fn test_malformed_json() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let malformed_path = temp_dir.path().join("malformed.json");

    fs::write(&malformed_path, r#"{"invalid": json content"#)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&malformed_path).arg("../tests/fixtures/file1.json");
    cmd.assert()
        .code(2) // Error exit code
        .stderr(predicates::str::contains("parse").or(predicates::str::contains("invalid")));

    Ok(())
}
