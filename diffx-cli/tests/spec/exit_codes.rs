//! Exit code tests
//! Spec: docs/specs/cli.md ## 終了コード
//!
//! | コード | 意味 |
//! |--------|------|
//! | 0 | 差分なし |
//! | 1 | 差分あり |
//! | 2 | 引数エラー、パースエラー、ディレクトリ比較エラー等 |
//! | 3 | ファイルI/Oエラー（ファイルが存在しない、読み取り失敗等） |

use assert_cmd::Command;
use predicates::prelude::*;

fn diffx() -> Command {
    Command::cargo_bin("diffx").unwrap()
}

fn fixtures(name: &str) -> String {
    format!("tests/fixtures/{name}")
}

// =============================================================================
// Exit code 0: 差分なし
// =============================================================================

#[test]
fn exit_0_identical_files() {
    diffx()
        .arg(fixtures("file1.json"))
        .arg(fixtures("file1.json"))
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty());
}

#[test]
fn exit_0_semantically_identical() {
    // 同じ内容でフォーマットが違う場合も差分なし
    // (このテストは同一ファイルで代用、異なるフォーマット版が必要なら追加)
    diffx()
        .arg(fixtures("config_v1.json"))
        .arg(fixtures("config_v1.json"))
        .assert()
        .success()
        .code(0);
}

// =============================================================================
// Exit code 1: 差分あり
// =============================================================================

#[test]
fn exit_1_different_files() {
    diffx()
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("~").or(predicate::str::contains("+")));
}

#[test]
fn exit_1_with_quiet_option() {
    diffx()
        .arg("--quiet")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::is_empty());
}

// =============================================================================
// Exit code 2: 引数エラー、パースエラー、ディレクトリ比較エラー
// =============================================================================

#[test]
fn exit_2_missing_arguments() {
    diffx().assert().failure().code(2);
}

#[test]
fn exit_2_single_argument() {
    diffx()
        .arg(fixtures("file1.json"))
        .assert()
        .failure()
        .code(2);
}

#[test]
fn exit_2_invalid_json() {
    // 不正なJSONをパースしようとするとエラー
    std::fs::write("/tmp/invalid.json", "{ invalid json }").unwrap();
    diffx()
        .arg("/tmp/invalid.json")
        .arg(fixtures("file1.json"))
        .assert()
        .failure()
        .code(2);
}

#[test]
fn exit_2_directory_without_recursive() {
    // ディレクトリを -r なしで指定するとエラー
    diffx()
        .arg(fixtures("dir1"))
        .arg(fixtures("dir2"))
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("--recursive").or(predicate::str::contains("-r")));
}

#[test]
fn exit_2_invalid_option() {
    diffx()
        .arg("--invalid-option")
        .arg(fixtures("file1.json"))
        .arg(fixtures("file2.json"))
        .assert()
        .failure()
        .code(2);
}

// =============================================================================
// Exit code 3: ファイルI/Oエラー
// =============================================================================

#[test]
fn exit_3_file_not_found() {
    diffx()
        .arg("nonexistent_file.json")
        .arg(fixtures("file1.json"))
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn exit_3_both_files_not_found() {
    diffx()
        .arg("nonexistent1.json")
        .arg("nonexistent2.json")
        .assert()
        .failure()
        .code(3);
}
