//! Directory comparison tests
//! Spec: docs/specs/cli.md ## ディレクトリ比較
//!
//! -r, --recursive: Required for directory comparison
//! Behavior:
//! - Recursively collect files from both directories
//! - Match by relative path
//! - Files only in one side -> Added/Removed
//! - Files in both -> Compare contents

use assert_cmd::Command;
use predicates::prelude::*;

fn diffx() -> Command {
    Command::cargo_bin("diffx").unwrap()
}

fn fixtures(name: &str) -> String {
    format!("tests/fixtures/{}", name)
}

// =============================================================================
// -r, --recursive flag
// =============================================================================

#[test]
fn directory_requires_recursive_flag() {
    // Spec: Directory comparison without -r should fail with exit code 2
    diffx()
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("--recursive").or(predicate::str::contains("-r")));
}

#[test]
fn directory_recursive_long_form() {
    diffx()
        .arg("--recursive")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .assert()
        .failure()
        .code(1); // Should succeed and find differences
}

#[test]
fn directory_recursive_short_form() {
    diffx()
        .arg("-r")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// Directory comparison behavior
// =============================================================================

#[test]
fn directory_identical() {
    diffx()
        .arg("-r")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir1"))
        .assert()
        .success()
        .code(0);
}

#[test]
fn directory_with_differences() {
    let output = diffx()
        .arg("-r")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show file paths and differences
    assert!(!stdout.is_empty(), "Should output differences");
}

#[test]
fn directory_with_json_output() {
    diffx()
        .arg("-r")
        .arg("--output")
        .arg("json")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::starts_with("["));
}

// =============================================================================
// File vs Directory mismatch
// =============================================================================

#[test]
fn directory_file_vs_dir_error() {
    diffx()
        .arg("-r")
        .arg(fixtures("file1.json"))
        .arg(fixtures("dir1"))
        .assert()
        .failure()
        .code(2); // Should error, not compare
}

// =============================================================================
// Nested directories
// =============================================================================

#[test]
fn directory_nested_subdirs() {
    // dir1 and dir2 have nested subdir/ directories
    let output = diffx()
        .arg("-r")
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .output()
        .unwrap();

    // Should process nested files
    assert_eq!(output.status.code(), Some(1));
}
