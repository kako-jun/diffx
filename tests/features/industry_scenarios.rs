use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_api_schema_comparison() -> Result<(), Box<dyn std::error::Error>> {
    // Test API schema evolution - common DevOps use case
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/api_schema_v1.json")
        .arg("../tests/fixtures/api_schema_v2.json")
        .arg("--path")
        .arg("paths")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(str::contains(r#""Modified""#))
        .stdout(str::contains(r#""Added""#))
        .stdout(str::contains(r#"paths./users.post"#))
        .stdout(str::contains(r#"schema.type"#));
    Ok(())
}

#[test]
fn test_cicd_configuration_drift() -> Result<(), Box<dyn std::error::Error>> {
    // Test CI/CD configuration monitoring - ignore build metadata
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*|deploy_.*|password|secret_.*)$")
        .arg("--output")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(str::contains("Modified"))
        .stdout(str::contains("application.version"))
        .stdout(str::contains("security.host"))
        .stdout(str::contains("monitoring.metrics"))
        .stdout(str::contains("timestamp").not())
        .stdout(str::contains("password").not())
        .stdout(str::contains("secret_").not());
    Ok(())
}

#[test]
fn test_environment_config_comparison() -> Result<(), Box<dyn std::error::Error>> {
    // Test environment configuration comparison - focus on application settings
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application")
        .arg("--ignore-keys-regex")
        .arg("^(host|port|password|.*_secret)$");
    cmd.assert()
        .code(1)
        .stdout(str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ))
        .stdout(str::contains("host").not())
        .stdout(str::contains("port").not());
    Ok(())
}

#[test]
fn test_api_contract_validation_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test API contract validation from documentation examples
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|requestId|serverId|responseTime)$")
        .arg("--ignore-case")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(str::contains("Modified").or(str::contains("Added")));
    Ok(())
}

#[test]
fn test_kubernetes_config_drift_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test Kubernetes configuration drift detection pattern
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Configuration differences
        .stdout(str::starts_with("["));
    Ok(())
}