use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_json_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(r#""Modified""#))
        .stdout(predicates::str::contains(r#""age""#))
        .stdout(predicates::str::contains(r#""city""#))
        .stdout(predicates::str::contains(r#""New York""#))
        .stdout(predicates::str::contains(r#""Boston""#))
        .stdout(predicates::str::contains(r#""Added""#))
        .stdout(predicates::str::contains(r#""items[2]""#))
        .stdout(predicates::str::contains(r#""orange""#));
    Ok(())
}

#[test]
fn test_yaml_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            r#"- Modified:
  - age
  - 30
  - 31"#,
        ))
        .stdout(predicates::str::contains(
            r#"- Modified:
  - city
  - New York
  - Boston"#,
        ))
        .stdout(predicates::str::contains(
            r#"- Added:
  - items[2]
  - orange"#,
        ));
    Ok(())
}


