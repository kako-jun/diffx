//! Output format tests
//! Spec: docs/specs/cli.md ## 出力制御
//!
//! -o, --output: diffx (default), json, yaml
//! --no-color: Disable ANSI colors
//! -q, --quiet: No output, exit code only
//! --brief: Only report whether files differ
//! -v, --verbose: Show detailed info on stderr

use assert_cmd::Command;
use predicates::prelude::*;

fn diffx() -> Command {
    Command::cargo_bin("diffx").unwrap()
}

fn fixtures(name: &str) -> String {
    format!("tests/fixtures/{name}")
}

// =============================================================================
// --output diffx (default)
// =============================================================================

#[test]
fn output_diffx_default() {
    // Default output uses diffx format with symbols: + - ~ !
    diffx()
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("~").or(predicate::str::contains("+")));
}

#[test]
fn output_diffx_explicit() {
    diffx()
        .arg("--output")
        .arg("diffx")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn output_diffx_symbols() {
    // Verify ~ for Modified, + for Added, - for Removed
    diffx()
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("~")); // Modified values
}

// =============================================================================
// --output json
// =============================================================================

#[test]
fn output_json() {
    diffx()
        .arg("--output")
        .arg("json")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::starts_with("["))
        .stdout(predicate::str::contains("Modified").or(predicate::str::contains("Added")));
}

#[test]
fn output_json_valid_syntax() {
    let output = diffx()
        .arg("--output")
        .arg("json")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should be valid JSON
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_ok(),
        "Output should be valid JSON: {stdout}"
    );
}

#[test]
fn output_json_no_diff() {
    let output = diffx()
        .arg("--output")
        .arg("json")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file1.json"))
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Empty array for no differences
    assert!(
        stdout.trim() == "[]" || stdout.trim().is_empty(),
        "No diff should output empty array: {stdout}"
    );
}

// =============================================================================
// --output yaml
// =============================================================================

#[test]
fn output_yaml() {
    diffx()
        .arg("--output")
        .arg("yaml")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Modified").or(predicate::str::contains("Added")));
}

// =============================================================================
// --no-color
// =============================================================================

#[test]
fn output_no_color() {
    let output = diffx()
        .arg("--no-color")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should not contain ANSI escape sequences
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI codes: {stdout}"
    );
}

// =============================================================================
// --quiet
// =============================================================================

#[test]
fn output_quiet_with_diff() {
    diffx()
        .arg("--quiet")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::is_empty());
}

#[test]
fn output_quiet_no_diff() {
    diffx()
        .arg("--quiet")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file1.json"))
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty());
}

#[test]
fn output_quiet_short_form() {
    diffx()
        .arg("-q")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::is_empty());
}

// =============================================================================
// --brief
// =============================================================================

#[test]
fn output_brief_with_diff() {
    diffx()
        .arg("--brief")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("differ"));
}

#[test]
fn output_brief_no_diff() {
    diffx()
        .arg("--brief")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file1.json"))
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty());
}

// =============================================================================
// --verbose
// =============================================================================

#[test]
fn output_verbose() {
    diffx()
        .arg("--verbose")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Input file information"))
        .stderr(predicate::str::contains("bytes"));
}

#[test]
fn output_verbose_short_form() {
    diffx()
        .arg("-v")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Input"));
}

// =============================================================================
// --help and --version
// =============================================================================

#[test]
fn output_help() {
    diffx()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("Options"));
}

#[test]
fn output_help_short() {
    diffx()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn output_version() {
    diffx()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("diffx"));
}

#[test]
fn output_version_short() {
    diffx()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("diffx"));
}
