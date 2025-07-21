use assert_cmd::Command;

#[test]
fn performance_benchmarks_example_1() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_2() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--output", "json", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_3() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--iterations", "100", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_4() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--memory-profile", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_5() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--cpu-profile", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_6() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--warmup", "10", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_7() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--no-color", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_8() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--algorithm", "myers", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_9() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--threads", "4", "file1.txt", "file2.txt"])
        .assert()
        .success();
}

#[test]
fn performance_benchmarks_example_10() {
    let mut cmd = Command::cargo_bin("diffx").unwrap();
    cmd.args(&["--benchmark", "--save-results", "benchmark_results.json", "file1.txt", "file2.txt"])
        .assert()
        .success();
}