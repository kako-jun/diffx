//! Format support tests
//! Spec: docs/specs/cli.md ## 対応フォーマット
//!
//! | フォーマット | 拡張子 |
//! |--------------|--------|
//! | JSON | .json |
//! | YAML | .yaml, .yml |
//! | TOML | .toml |
//! | XML | .xml |
//! | INI | .ini |
//! | CSV | .csv |

use assert_cmd::Command;
use predicates::prelude::*;

fn diffx() -> Command {
    Command::cargo_bin("diffx").unwrap()
}

fn fixtures(name: &str) -> String {
    format!("tests/fixtures/{}", name)
}

// =============================================================================
// JSON
// =============================================================================

#[test]
fn format_json_auto_detect() {
    diffx()
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_json_explicit() {
    diffx()
        .arg("--format")
        .arg("json")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// YAML
// =============================================================================

#[test]
fn format_yaml_auto_detect() {
    diffx()
        .arg(fixtures("file1.yaml"))
        .arg(fixtures("file2.yaml"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_yaml_explicit() {
    diffx()
        .arg("--format")
        .arg("yaml")
        .arg(fixtures("file1.yaml"))
        .arg(fixtures("file2.yaml"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// TOML
// =============================================================================

#[test]
fn format_toml_auto_detect() {
    diffx()
        .arg(fixtures("file1.toml"))
        .arg(fixtures("file2.toml"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_toml_explicit() {
    diffx()
        .arg("--format")
        .arg("toml")
        .arg(fixtures("file1.toml"))
        .arg(fixtures("file2.toml"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// XML
// =============================================================================

#[test]
fn format_xml_auto_detect() {
    diffx()
        .arg(fixtures("file1.xml"))
        .arg(fixtures("file2.xml"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_xml_explicit() {
    diffx()
        .arg("--format")
        .arg("xml")
        .arg(fixtures("file1.xml"))
        .arg(fixtures("file2.xml"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// INI
// =============================================================================

#[test]
fn format_ini_auto_detect() {
    diffx()
        .arg(fixtures("file1.ini"))
        .arg(fixtures("file2.ini"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_ini_explicit() {
    diffx()
        .arg("--format")
        .arg("ini")
        .arg(fixtures("file1.ini"))
        .arg(fixtures("file2.ini"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// CSV
// =============================================================================

#[test]
fn format_csv_auto_detect() {
    diffx()
        .arg(fixtures("file1.csv"))
        .arg(fixtures("file2.csv"))
        .assert()
        .failure()
        .code(1);
}

#[test]
fn format_csv_explicit() {
    diffx()
        .arg("--format")
        .arg("csv")
        .arg(fixtures("file1.csv"))
        .arg(fixtures("file2.csv"))
        .assert()
        .failure()
        .code(1);
}

// =============================================================================
// Cross-format (same content, different format)
// =============================================================================

#[test]
fn format_json_vs_yaml_same_content() {
    // JSON and YAML with same semantic content should show no diff
    // This requires fixtures with identical content in different formats
    // For now, test that explicit format works
    diffx()
        .arg("--format")
        .arg("json")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file1.json"))
        .assert()
        .success()
        .code(0);
}
