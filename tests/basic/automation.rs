use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_cicd_deployment_validation_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test deployment validation pattern from documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("--ignore-keys-regex")
        .arg("^(environment|debug|host|port)")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains("Modified").or(predicates::str::contains("[]")));
    Ok(())
}

#[test]
fn test_cicd_config_drift_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // Test configuration drift monitoring pattern
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^(hostname|instance_id|last_.*|timestamp)")
        .arg("--ignore-case")
        .arg("--quiet");
    cmd.assert()
        .code(1) // Configuration drift detected
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_cicd_batch_file_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Test batch file validation pattern
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let config1_path = temp_dir.path().join("config1.json");
    let config2_path = temp_dir.path().join("config2.json");

    fs::write(&config1_path, r#"{"app": "test", "version": "1.0"}"#)?;
    fs::write(&config2_path, r#"{"app": "test", "version": "1.1"}"#)?;

    // Test first file (no differences)
    let mut cmd1 = diffx_cmd();
    cmd1.arg(&config1_path).arg(&config1_path).arg("--quiet");
    cmd1.assert().code(0);

    // Test second file (differences)
    let mut cmd2 = diffx_cmd();
    cmd2.arg(&config1_path).arg(&config2_path).arg("--quiet");
    cmd2.assert().code(1);

    Ok(())
}
