use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_csv_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.csv")
        .arg("../tests/fixtures/file2.csv");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ [0].header2: \"valueB\" -> \"new_valueB\"",
        ))
        .stdout(
            predicates::str::contains("+ [2]: ")
                .and(predicates::str::contains("\"header1\":\"valueE\""))
                .and(predicates::str::contains("\"header2\":\"valueF\"")),
        );
    Ok(())
}
