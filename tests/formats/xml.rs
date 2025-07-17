use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_xml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.xml")
        .arg("../tests/fixtures/file2.xml");
    cmd.assert()
        .code(1)
        .stdout(str::contains(
            "~ item.$text: \"value2\" -> \"value3\"",
        ))
        .stdout(str::contains("~ item.@id: \"2\" -> \"3\""));
    Ok(())
}