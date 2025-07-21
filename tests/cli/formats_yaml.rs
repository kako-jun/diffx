use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_yaml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.yaml")
        .arg("../tests/fixtures/file2.yaml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"))
        .stdout(predicates::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicates::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_format_yaml_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.yaml")
        .arg("../tests/fixtures/file2.yaml")
        .arg("--format")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_yaml_multiline_strings() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("yaml");
    cmd.write_stdin("description: |\n  Multi-line\n  text here\n")
        .write_stdin("description: |\n  Different\n  text here\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ description:"));
    Ok(())
}

#[test]
fn test_yaml_arrays_and_objects() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("yaml");
    cmd.write_stdin("items:\n  - name: A\n    value: 1\n  - name: B\n    value: 2\n")
        .write_stdin("items:\n  - name: A\n    value: 10\n  - name: C\n    value: 3\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~"));
    Ok(())
}

#[test]
fn test_yaml_with_special_chars() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("yaml");
    cmd.write_stdin("message: 'Text with: colons and \"quotes\"'\n")
        .write_stdin("message: 'Different text'\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ message:"));
    Ok(())
}
