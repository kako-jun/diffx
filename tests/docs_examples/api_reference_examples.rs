use assert_cmd::Command;

#[test]
fn api_reference_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "ini", "file1.ini", "file2.ini"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "xml", "file1.xml", "file2.xml"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--format", "csv", "file1.csv", "file2.csv"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--type-info", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--epsilon", "0.001", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--ignore-regex", "pattern", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--array-id", "id", "file1.json", "file2.json"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--pipeline", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_10() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--custom-processor", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_11() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--async", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_12() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--error-handling", "file1.ini", "file2.ini"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_13() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--robust", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_14() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--large-data", "large_file1.txt", "large_file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_15() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--test-mode", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_16() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--epsilon-test", "0.001", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_17() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--diff-result", "added", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_18() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--diff-result", "modified", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_19() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--diff-result", "type-changed", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_20() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--parse-ini", "file.ini"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_21() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--parse-xml", "file.xml"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_22() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--parse-csv", "file.csv"])
        .assert()
        .success();
}

#[test]
fn api_reference_example_23() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--value-type-name", "file1.txt", "file2.txt"])
        .assert()
        .success();
}