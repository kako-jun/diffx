#[allow(unused_imports)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

// Helper function to create temporary JSON files for testing
fn create_temp_json(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: diffx config1.json config2.json
#[test]
fn test_index_semantic_diff() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"name": "myapp", "version": "1.0"}"#);
    let file2 = create_temp_json(r#"{"version": "1.1", "name": "myapp"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("version:"))
        .stdout(predicates::str::contains("1.0"))
        .stdout(predicates::str::contains("1.1"));
    
    Ok(())
}