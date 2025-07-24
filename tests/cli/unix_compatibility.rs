use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_unix_pattern_diff_q_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -q equivalent: quiet mode exit codes only
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--quiet");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicates::str::is_empty());

    // Test with different files
    let mut cmd2 = diffx_cmd();
    cmd2.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--quiet");
    cmd2.assert()
        .code(1) // Differences found
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_brief_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff --brief equivalent: filenames only
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--brief");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains(
            "Files ../tests/fixtures/file1.json and ../tests/fixtures/file2.json differ",
        ))
        .stdout(predicates::str::contains("age").not()) // Should not show details
        .stdout(predicates::str::contains("city").not());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_i_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -i equivalent: ignore case
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json")
        .arg("--ignore-case");
    cmd.assert()
        .code(0) // No differences when ignoring case
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_w_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -w equivalent: ignore whitespace
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(0) // No differences when ignoring whitespace
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_c3_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -C3 equivalent: context lines
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("2");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicates::str::contains("-      \"port\": 5432"))
        .stdout(predicates::str::contains("+      \"port\": 5433"))
        .stdout(predicates::str::contains("\"host\": \"localhost\"")); // Context line
    Ok(())
}

#[test]
fn test_unix_combined_pattern_qiw() -> Result<(), Box<dyn std::error::Error>> {
    // Test combined pattern: diff -qiw equivalent
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--quiet")
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Still differences (different keys)
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_directory_brief_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test --brief with directory comparison
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--brief");
    cmd.assert()
        .code(1); // Differences found
    Ok(())
}

#[test]
fn test_directory_comparison_without_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_directory_comparison_with_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_directory_vs_file_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/file1.json");
    cmd.assert().code(2).stderr(
        predicates::str::contains("Cannot compare directory")
            .and(predicates::str::contains("with file"))
    );
    Ok(())
}

#[test]
fn test_recursive_compares_nested_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1);
    Ok(())
}

#[test]
fn test_non_recursive_does_not_compare_nested_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1);
    Ok(())
}
