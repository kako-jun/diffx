#[allow(unused_imports)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_complex_regex_security_fields() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(password|secret_.*|credentials|connection_string)$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicates::str::contains(
            "~ security.host: \"localhost\" -> \"prod-server.example.com\"",
        ))
        .stdout(predicates::str::contains("password").not())
        .stdout(predicates::str::contains("secret_").not())
        .stdout(predicates::str::contains("credentials").not())
        .stdout(predicates::str::contains("connection_string").not());
    Ok(())
}

#[test]
fn test_complex_regex_build_fields() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*|deploy_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicates::str::contains(
            "~ monitoring.metrics.cpu: 45.2 -> 52.1",
        ))
        .stdout(predicates::str::contains(
            "~ monitoring.metrics.memory: 78.9 -> 82.3",
        ))
        .stdout(predicates::str::contains("timestamp").not())
        .stdout(predicates::str::contains("build_").not())
        .stdout(predicates::str::contains("deploy_").not());
    Ok(())
}

#[test]
fn test_complex_regex_multiple_groups() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(password|secret_.*|timestamp|build_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicates::str::contains(
            "~ security.host: \"localhost\" -> \"prod-server.example.com\"",
        ))
        .stdout(predicates::str::contains("password").not())
        .stdout(predicates::str::contains("secret_").not())
        .stdout(predicates::str::contains("timestamp").not())
        .stdout(predicates::str::contains("build_").not());
    Ok(())
}

#[test]
fn test_combined_path_and_regex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--path")
        .arg("monitoring")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ monitoring.metrics.cpu: 45.2 -> 52.1",
        ))
        .stdout(predicates::str::contains(
            "~ monitoring.metrics.memory: 78.9 -> 82.3",
        ))
        .stdout(predicates::str::contains("~ monitoring.deploy_time:"))
        .stdout(predicates::str::contains("timestamp").not())
        .stdout(predicates::str::contains("build_").not())
        .stdout(predicates::str::contains("application").not())
        .stdout(predicates::str::contains("security").not());
    Ok(())
}

#[test]
fn test_combined_path_and_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(r#""Modified""#))
        .stdout(predicates::str::contains(r#""application.debug""#))
        .stdout(predicates::str::contains(r#"true"#))
        .stdout(predicates::str::contains(r#"false"#))
        .stdout(predicates::str::contains("database").not())
        .stdout(predicates::str::contains("services").not());
    Ok(())
}
