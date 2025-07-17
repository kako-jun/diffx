use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_version_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"));
    Ok(())
}

#[test]
fn test_help_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("diffx"))
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn test_basic_json_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_specify_input_format() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut cmd = diffx_cmd();
    let mut child = cmd
        .arg("-")
        .arg("../tests/fixtures/file2.json")
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(
            r#"{
  "name": "Alice",
  "age": 30,
  "city": "New York",
  "config": {
    "users": [
      {"id": 1, "name": "Alice"},
      {"id": 2, "name": "Bob"}
    ],
    "settings": {"theme": "dark"}
  }
}"#
            .as_bytes(),
        )?;
    } // stdin is dropped here, closing the pipe
    let output = child.wait_with_output()?;
    assert_eq!(output.status.code(), Some(1)); // Differences found
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(predicate::str::contains("~ age: 30 -> 31").eval(&stdout));
    assert!(predicate::str::contains("~ city: \"New York\" -> \"Boston\"").eval(&stdout));
    assert!(predicate::str::contains("~ name: \"Alice\" -> \"John\"").eval(&stdout));
    assert!(predicate::str::contains("+ items:").eval(&stdout));
    Ok(())
}

#[test]
fn test_format_specification_with_stdin() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut cmd = diffx_cmd();
    let mut child = cmd
        .arg("-")
        .arg("../tests/fixtures/file2.ini")
        .arg("--format")
        .arg("ini")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(
            br#"[section1]
key1 = value1
key2 = value2

[section2]
key3 = value3
"#,
        )?;
    }

    let output = child.wait_with_output()?;
    assert_eq!(output.status.code(), Some(1)); // Differences found
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        predicate::str::contains("~ section1.key2: \"value2\" -> \"new_value2\"").eval(&stdout)
    );
    assert!(predicate::str::contains("+ section2.key4: \"value4\"").eval(&stdout));
    Ok(())
}