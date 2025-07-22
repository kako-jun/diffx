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
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.yaml");
    let file2_path = temp_dir.path().join("file2.yaml");

    fs::write(&file1_path, "description: |\n  Multi-line\n  text here\n")?;
    fs::write(&file2_path, "description: |\n  Different\n  text here\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ description:"));
    Ok(())
}

#[test]
fn test_yaml_arrays_and_objects() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.yaml");
    let file2_path = temp_dir.path().join("file2.yaml");

    fs::write(&file1_path, "items:\n  - name: A\n    value: 1\n  - name: B\n    value: 2\n")?;
    fs::write(&file2_path, "items:\n  - name: A\n    value: 10\n  - name: C\n    value: 3\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~"));
    Ok(())
}

#[test]
fn test_yaml_with_special_chars() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.yaml");
    let file2_path = temp_dir.path().join("file2.yaml");

    fs::write(&file1_path, "message: 'Text with: colons and \"quotes\"'\n")?;
    fs::write(&file2_path, "message: 'Different text'\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ message:"));
    Ok(())
}
