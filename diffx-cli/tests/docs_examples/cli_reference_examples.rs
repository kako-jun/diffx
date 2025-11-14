#[allow(unused_imports)]
use assert_cmd::prelude::*;
use assert_cmd::Command;

#[test]
fn cli_reference_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "json", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json", "--path", "database"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-keys-regex", "^(timestamp|createdAt|updatedAt)$"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users.json", "users.updated.json", "--array-id-key", "id"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["configs/", "configs.backup/", "--recursive"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["data.yaml", "data.updated.yaml", "--ignore-case", "--ignore-whitespace", "--epsilon", "0.001", "--ignore-keys-regex", "^(timestamp|version)$"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["expected_config.json", "actual_config.json", "--ignore-keys-regex", "^(deployment_time|build_id)", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_10() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--help"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_11() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--version"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_12() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--verbose"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_13() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--quiet"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_14() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--no-color"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_15() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--color", "always"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_16() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--color", "never"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_17() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--color", "auto"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_18() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--context", "3"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_19() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--unified"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_20() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--side-by-side"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_21() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-case"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_22() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-whitespace"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_23() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-blank-lines"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_24() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-trailing-whitespace"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_25() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-all-space"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_26() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--epsilon", "0.01"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_27() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-keys", "timestamp,version"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_28() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-values", "null"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_29() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--include-only", "data,config"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_30() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--max-depth", "5"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_31() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--show-unchanged"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_32() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--show-types"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_33() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--line-numbers"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_34() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--word-diff"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_35() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--char-diff"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_36() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["configs/", "configs.backup/", "--exclude", "*.log"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_37() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["configs/", "configs.backup/", "--include", "*.json"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_38() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["configs/", "configs.backup/", "--follow-symlinks"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_39() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--threads", "4"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_40() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["large_file1.json", "large_file2.json", "--memory-limit", "1G"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_41() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--cache", "enable"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_42() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--streaming"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_43() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--config"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_44() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--list-formats"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_45() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--examples"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_46() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--completions", "bash"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_47() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["api_v1.json", "api_v2.json", "--ignore-keys-regex", "^(version|timestamp)$"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_48() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["schema_old.json", "schema_new.json", "--show-types"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_49() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_dev.yml", "config_prod.yml", "--ignore-keys", "environment,debug"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_50() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users_backup.json", "users_current.json", "--array-id-key", "user_id"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_51() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["metrics.json", "metrics_new.json", "--epsilon", "0.001"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_52() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["build.json", "build_new.json", "--ignore-keys-regex", "^(build_time|git_hash)$"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_53() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["test_results.xml", "test_results_new.xml", "--format", "xml"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_54() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["packages.json", "packages_updated.json", "--path", "dependencies"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_55() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--algorithm", "myers"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_56() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--algorithm", "patience"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_57() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--algorithm", "histogram"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_58() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--benchmark"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_59() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--profile"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_60() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--debug"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_61() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--trace"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_62() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--timing"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_63() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--stats"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_64() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--output-file", "results.json"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_65() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--patch-format"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_66() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--summary-only"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_67() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--exit-code"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_68() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--machine-readable"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_69() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--check-syntax"])
        .assert()
        .success();
}

#[test]
fn cli_reference_example_70() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--validate"])
        .assert()
        .success();
}