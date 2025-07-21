use assert_cmd::Command;

#[test]
fn diffx_format_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["infrastructure.json", "infrastructure.new.json"])
        .assert()
        .success();
}

#[test]
fn diffx_format_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["api-v1.yaml", "api-v2.yaml", "--path", "paths"])
        .assert()
        .success();
}

#[test]
fn diffx_format_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["expected_output.json", "actual_output.json", "--array-id-key", "id"])
        .assert()
        .success();
}

#[test]
fn diffx_format_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config.json", "config.new.json"])
        .assert()
        .success();
}