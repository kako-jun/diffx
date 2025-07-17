use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_meta_chaining() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure test output directory exists
    std::fs::create_dir_all("../tests/output")?;

    // Step 1: Generate diff_report_v1.json
    let mut cmd1 = diffx_cmd();
    cmd1.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--output")
        .arg("json");
    let output1 = cmd1.output()?.stdout;
    std::fs::write("../tests/output/diff_report_v1.json", output1)?;

    // Step 2: Generate diff_report_v2.json
    let mut cmd2 = diffx_cmd();
    cmd2.arg("../tests/fixtures/config_v2.json")
        .arg("../tests/fixtures/config_v3.json")
        .arg("--output")
        .arg("json");
    let output2 = cmd2.output()?.stdout;
    std::fs::write("../tests/output/diff_report_v2.json", output2)?;

    // Step 3: Compare the two diff reports
    let mut cmd3 = diffx_cmd();
    cmd3.arg("../tests/output/diff_report_v1.json")
        .arg("../tests/output/diff_report_v2.json");
    cmd3.assert()
        .code(1)
        .stdout(predicate::str::contains(
            r#"~ [1].Modified[1]: "1.0" -> "1.1""#,
        ))
        .stdout(predicate::str::contains(
            r#"~ [1].Modified[2]: "1.1" -> "1.2""#,
        ))
        .stdout(predicate::str::contains(
            r#"+ [2]: {"Added":["features[2]","featureD"]}"#,
        ));

    // Clean up generated diff report files
    std::fs::remove_file("../tests/output/diff_report_v1.json")?;
    std::fs::remove_file("../tests/output/diff_report_v2.json")?;

    Ok(())
}

#[test]
fn test_combined_array_id_and_epsilon() -> Result<(), Box<dyn std::error::Error>> {
    // Create test data with small floating point differences
    let test_data1 = r#"{
  "records": [
    {"id": 1, "value": 10.00001, "name": "item1"},
    {"id": 2, "value": 20.00002, "name": "item2"}
  ]
}"#;
    let test_data2 = r#"{
  "records": [
    {"id": 1, "value": 10.00003, "name": "item1"},
    {"id": 2, "value": 20.00004, "name": "item2_updated"}
  ]
}"#;

    std::fs::write("../tests/fixtures/array_epsilon1.json", test_data1)?;
    std::fs::write("../tests/fixtures/array_epsilon2.json", test_data2)?;

    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/array_epsilon1.json")
        .arg("../tests/fixtures/array_epsilon2.json")
        .arg("--array-id-key")
        .arg("id")
        .arg("--epsilon")
        .arg("0.0001");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ records[id=2].name: \"item2\" -> \"item2_updated\"",
        ))
        .stdout(predicate::str::contains("value").not()); // Values should be ignored due to epsilon

    // Clean up
    std::fs::remove_file("../tests/fixtures/array_epsilon1.json")?;
    std::fs::remove_file("../tests/fixtures/array_epsilon2.json")?;
    Ok(())
}