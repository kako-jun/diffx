use assert_cmd::Command;

#[test]
fn faq_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["-", "other_data.json", "--format", "json"])
        .assert()
        .success();
}

#[test]
fn faq_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--ignore-keys-regex", "^id$|^timestamp$"])
        .assert()
        .success();
}

#[test]
fn faq_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["data1.json", "data2.json", "--epsilon", "0.00001"])
        .assert()
        .success();
}

#[test]
fn faq_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["users1.json", "users2.json", "--array-id-key", "uuid"])
        .assert()
        .success();
}

#[test]
fn faq_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.json", "file2.json", "--output", "json"])
        .assert()
        .success();
}

#[test]
fn faq_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.yaml", "file2.yaml", "--format", "yaml"])
        .assert()
        .success();
}