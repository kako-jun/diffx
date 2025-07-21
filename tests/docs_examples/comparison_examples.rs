use assert_cmd::Command;

#[test]
fn comparison_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config_v1.json", "config_v2.json"])
        .assert()
        .success();
}

#[test]
fn comparison_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn comparison_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.yaml", "file2.yaml"])
        .assert()
        .success();
}

#[test]
fn comparison_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["data1.csv", "data2.csv", "--array-id-key", "id"])
        .assert()
        .success();
}

#[test]
fn comparison_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn comparison_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["-", "config.json"])
        .assert()
        .success();
}

#[test]
fn comparison_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config1.json", "config2.json", "--output", "unified"])
        .assert()
        .success();
}

#[test]
fn comparison_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["config1.json", "config2.json"])
        .assert()
        .success();
}

#[test]
fn comparison_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--output", "json"])
        .assert()
        .success();
}