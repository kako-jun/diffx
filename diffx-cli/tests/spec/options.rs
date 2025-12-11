//! Comparison options tests
//! Spec: docs/specs/cli.md ## 比較オプション
//!
//! --epsilon: Numeric tolerance
//! --ignore-keys-regex: Skip keys matching pattern
//! --array-id-key: Match array elements by key
//! --ignore-whitespace: Ignore whitespace in strings
//! --ignore-case: Case-insensitive string comparison
//! --path: Filter output by path

use assert_cmd::Command;
use predicates::prelude::*;

fn diffx() -> Command {
    Command::cargo_bin("diffx").unwrap()
}

fn fixtures(name: &str) -> String {
    format!("tests/fixtures/{name}")
}

// =============================================================================
// --epsilon
// =============================================================================

#[test]
fn option_epsilon_within_tolerance() {
    // Create temp files with numbers differing by small amount
    std::fs::write("/tmp/epsilon1.json", r#"{"value": 1.0}"#).unwrap();
    std::fs::write("/tmp/epsilon2.json", r#"{"value": 1.0005}"#).unwrap();

    diffx()
        .arg("--epsilon")
        .arg("0.001")
        .arg("/tmp/epsilon1.json")
        .arg("/tmp/epsilon2.json")
        .assert()
        .success()
        .code(0); // No diff within tolerance
}

#[test]
fn option_epsilon_outside_tolerance() {
    std::fs::write("/tmp/epsilon_out1.json", r#"{"value": 1.0}"#).unwrap();
    std::fs::write("/tmp/epsilon_out2.json", r#"{"value": 2.0}"#).unwrap();

    diffx()
        .arg("--epsilon")
        .arg("0.001")
        .arg("/tmp/epsilon_out1.json")
        .arg("/tmp/epsilon_out2.json")
        .assert()
        .failure()
        .code(1); // Diff detected (1.0 difference >> 0.001 tolerance)
}

#[test]
fn option_epsilon_applies_to_integers() {
    // Spec: integers are also converted to f64
    std::fs::write("/tmp/epsilon_int1.json", r#"{"count": 100}"#).unwrap();
    std::fs::write("/tmp/epsilon_int2.json", r#"{"count": 100.0005}"#).unwrap();

    diffx()
        .arg("--epsilon")
        .arg("0.001")
        .arg("/tmp/epsilon_int1.json")
        .arg("/tmp/epsilon_int2.json")
        .assert()
        .success()
        .code(0);
}

// =============================================================================
// --ignore-keys-regex
// =============================================================================

#[test]
fn option_ignore_keys_regex_simple() {
    std::fs::write(
        "/tmp/ignore1.json",
        r#"{"name": "Alice", "timestamp": "2024-01-01"}"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/ignore2.json",
        r#"{"name": "Alice", "timestamp": "2024-12-31"}"#,
    )
    .unwrap();

    diffx()
        .arg("--ignore-keys-regex")
        .arg("^timestamp$")
        .arg("/tmp/ignore1.json")
        .arg("/tmp/ignore2.json")
        .assert()
        .success()
        .code(0); // timestamp ignored, name same
}

#[test]
fn option_ignore_keys_regex_nested() {
    // Spec: recursively applied to nested objects
    std::fs::write(
        "/tmp/nested1.json",
        r#"{"data": {"timestamp": "old", "value": 1}}"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/nested2.json",
        r#"{"data": {"timestamp": "new", "value": 1}}"#,
    )
    .unwrap();

    diffx()
        .arg("--ignore-keys-regex")
        .arg("^timestamp$")
        .arg("/tmp/nested1.json")
        .arg("/tmp/nested2.json")
        .assert()
        .success()
        .code(0); // nested timestamp also ignored
}

#[test]
fn option_ignore_keys_regex_multiple_patterns() {
    std::fs::write(
        "/tmp/multi1.json",
        r#"{"name": "A", "created_at": "x", "updated_at": "y"}"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/multi2.json",
        r#"{"name": "A", "created_at": "a", "updated_at": "b"}"#,
    )
    .unwrap();

    diffx()
        .arg("--ignore-keys-regex")
        .arg("^(created_at|updated_at)$")
        .arg("/tmp/multi1.json")
        .arg("/tmp/multi2.json")
        .assert()
        .success()
        .code(0);
}

// =============================================================================
// --array-id-key
// =============================================================================

#[test]
fn option_array_id_key_reorder_no_diff() {
    // Same elements in different order should have no diff
    std::fs::write(
        "/tmp/arr1.json",
        r#"[{"id": 1, "name": "A"}, {"id": 2, "name": "B"}]"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/arr2.json",
        r#"[{"id": 2, "name": "B"}, {"id": 1, "name": "A"}]"#,
    )
    .unwrap();

    diffx()
        .arg("--array-id-key")
        .arg("id")
        .arg("/tmp/arr1.json")
        .arg("/tmp/arr2.json")
        .assert()
        .success()
        .code(0);
}

#[test]
fn option_array_id_key_with_changes() {
    std::fs::write("/tmp/arr_chg1.json", r#"[{"id": 1, "name": "Alice"}]"#).unwrap();
    std::fs::write("/tmp/arr_chg2.json", r#"[{"id": 1, "name": "Bob"}]"#).unwrap();

    diffx()
        .arg("--array-id-key")
        .arg("id")
        .arg("/tmp/arr_chg1.json")
        .arg("/tmp/arr_chg2.json")
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("~").or(predicate::str::contains("name")));
}

#[test]
fn option_array_id_key_fallback_to_index() {
    // Elements without ID fall back to index comparison
    std::fs::write(
        "/tmp/arr_noid1.json",
        r#"[{"id": 1, "x": 1}, {"name": "NoID"}]"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/arr_noid2.json",
        r#"[{"id": 1, "x": 1}, {"name": "Changed"}]"#,
    )
    .unwrap();

    diffx()
        .arg("--array-id-key")
        .arg("id")
        .arg("/tmp/arr_noid1.json")
        .arg("/tmp/arr_noid2.json")
        .assert()
        .failure()
        .code(1); // Second element compared by index, shows diff
}

// =============================================================================
// --ignore-whitespace
// =============================================================================

#[test]
fn option_ignore_whitespace() {
    std::fs::write("/tmp/ws1.json", r#"{"text": "hello world"}"#).unwrap();
    std::fs::write("/tmp/ws2.json", r#"{"text": "helloworld"}"#).unwrap();

    diffx()
        .arg("--ignore-whitespace")
        .arg("/tmp/ws1.json")
        .arg("/tmp/ws2.json")
        .assert()
        .success()
        .code(0);
}

#[test]
fn option_ignore_whitespace_leading_trailing() {
    std::fs::write("/tmp/ws_lt1.json", r#"{"text": "  abc  "}"#).unwrap();
    std::fs::write("/tmp/ws_lt2.json", r#"{"text": "abc"}"#).unwrap();

    diffx()
        .arg("--ignore-whitespace")
        .arg("/tmp/ws_lt1.json")
        .arg("/tmp/ws_lt2.json")
        .assert()
        .success()
        .code(0);
}

#[test]
fn option_ignore_whitespace_without_flag() {
    std::fs::write("/tmp/ws_no1.json", r#"{"text": "hello world"}"#).unwrap();
    std::fs::write("/tmp/ws_no2.json", r#"{"text": "helloworld"}"#).unwrap();

    diffx()
        .arg("/tmp/ws_no1.json")
        .arg("/tmp/ws_no2.json")
        .assert()
        .failure()
        .code(1); // Diff detected without flag
}

// =============================================================================
// --ignore-case
// =============================================================================

#[test]
fn option_ignore_case() {
    std::fs::write("/tmp/case1.json", r#"{"name": "Hello"}"#).unwrap();
    std::fs::write("/tmp/case2.json", r#"{"name": "hello"}"#).unwrap();

    diffx()
        .arg("--ignore-case")
        .arg("/tmp/case1.json")
        .arg("/tmp/case2.json")
        .assert()
        .success()
        .code(0);
}

#[test]
fn option_ignore_case_keys_not_affected() {
    // Spec: case-insensitivity only applies to values, not keys
    std::fs::write("/tmp/case_key1.json", r#"{"Name": "test"}"#).unwrap();
    std::fs::write("/tmp/case_key2.json", r#"{"name": "test"}"#).unwrap();

    diffx()
        .arg("--ignore-case")
        .arg("/tmp/case_key1.json")
        .arg("/tmp/case_key2.json")
        .assert()
        .failure()
        .code(1); // Different keys, even with --ignore-case
}

#[test]
fn option_ignore_case_without_flag() {
    std::fs::write("/tmp/case_no1.json", r#"{"name": "Hello"}"#).unwrap();
    std::fs::write("/tmp/case_no2.json", r#"{"name": "hello"}"#).unwrap();

    diffx()
        .arg("/tmp/case_no1.json")
        .arg("/tmp/case_no2.json")
        .assert()
        .failure()
        .code(1); // Diff detected without flag
}

// =============================================================================
// --path (filter)
// =============================================================================

#[test]
fn option_path_filter() {
    std::fs::write(
        "/tmp/path1.json",
        r#"{"database": {"host": "a"}, "cache": {"ttl": 1}}"#,
    )
    .unwrap();
    std::fs::write(
        "/tmp/path2.json",
        r#"{"database": {"host": "b"}, "cache": {"ttl": 2}}"#,
    )
    .unwrap();

    let output = diffx()
        .arg("--path")
        .arg("database")
        .arg("/tmp/path1.json")
        .arg("/tmp/path2.json")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("database") || stdout.contains("host"),
        "Should show database changes: {stdout}"
    );
    // cache changes should be filtered out
}

#[test]
fn option_path_filter_no_match() {
    std::fs::write("/tmp/path_no1.json", r#"{"foo": 1}"#).unwrap();
    std::fs::write("/tmp/path_no2.json", r#"{"foo": 2}"#).unwrap();

    diffx()
        .arg("--path")
        .arg("nonexistent")
        .arg("/tmp/path_no1.json")
        .arg("/tmp/path_no2.json")
        .assert()
        .success()
        .code(0); // No matching paths, no output
}

// =============================================================================
// Combined options
// =============================================================================

#[test]
fn option_combined_ignore_case_and_whitespace() {
    std::fs::write("/tmp/combined1.json", r#"{"text": "Hello World"}"#).unwrap();
    std::fs::write("/tmp/combined2.json", r#"{"text": "helloworld"}"#).unwrap();

    diffx()
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("/tmp/combined1.json")
        .arg("/tmp/combined2.json")
        .assert()
        .success()
        .code(0);
}
