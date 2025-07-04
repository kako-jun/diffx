use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_json_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.json").arg("../examples/file2.json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains("~ city: \"New York\" -> \"Boston\""))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_yaml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.yaml").arg("../examples/file2.yaml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains("~ city: \"New York\" -> \"Boston\""))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_toml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.toml").arg("../examples/file2.toml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains("~ city: \"New York\" -> \"Boston\""))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_ini_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.ini").arg("../examples/file2.ini");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ section1.key2: \"value2\" -> \"new_value2\""))
        .stdout(predicate::str::contains("+ section2.key4: \"value4\""));
    Ok(())
}

#[test]
fn test_basic_xml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.xml").arg("../examples/file2.xml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ item.$text: \"value2\" -> \"value3\""))
        .stdout(predicate::str::contains("~ item.@id: \"2\" -> \"3\""));
    Ok(())
}

#[test]
fn test_basic_csv_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.csv").arg("../examples/file2.csv");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ [0].header2: \"valueB\" -> \"new_valueB\""))
        .stdout(predicate::str::contains("+ [2]: ").and(predicate::str::contains("\"header1\":\"valueE\"")).and(predicate::str::contains("\"header2\":\"valueF\"")));
    Ok(())
}

#[test]
fn test_specify_input_format() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut cmd = diffx_cmd();
    let mut child = cmd.arg("-").arg("../examples/file2.json").arg("--format").arg("json").stdin(std::process::Stdio::piped()).spawn()?;
    let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
    stdin.write_all(r#"{
  "name": "Alice",
  "age": 30,
  "city": "New York",
  "config": {
    "users": [
      {"id": 1, "name": "Alice"},
      {"id": 2, "name": "Bob"}
    ],
    "settings": {"theme": "dark"}
  }
}"#.as_bytes())?;
    let _ = stdin; // Close stdin to signal EOF
    let output = child.wait_with_output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(predicate::str::contains("~ age: 30 -> 31").eval(&stdout));
    assert!(predicate::str::contains("~ city: \"New York\" -> \"Boston\"").eval(&stdout));
    assert!(predicate::str::contains("~ name: \"Alice\" -> \"John\"").eval(&stdout));
    assert!(predicate::str::contains("+ items").eval(&stdout));
    Ok(())
}

#[test]
fn test_json_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.json").arg("../examples/file2.json").arg("--output").arg("json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""age""#))
        .stdout(predicate::str::contains(r#""city""#))
        .stdout(predicate::str::contains(r#""New York""#))
        .stdout(predicate::str::contains(r#""Boston""#))
        .stdout(predicate::str::contains(r#""Added""#))
        .stdout(predicate::str::contains(r#""items[2]""#))
        .stdout(predicate::str::contains(r#""orange""#));
    Ok(())
}

#[test]
fn test_yaml_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.json").arg("../examples/file2.json").arg("--output").arg("yaml");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#"- Modified:
  - age
  - 30
  - 31"#))
        .stdout(predicate::str::contains(r#"- Modified:
  - city
  - New York
  - Boston"#))
        .stdout(predicate::str::contains(r#"- Added:
  - items[2]
  - orange"#))
        .stdout(predicate::str::contains(r#"- Modified:
  - config.users[1].name
  - Bob
  - Robert"#))
        .stdout(predicate::str::contains(r#"- Added:
  - config.users[2]
  - id: 3
    name: Charlie"#).or(predicate::str::contains(r#"- Added:
  - config.users[2]
  - name: Charlie
    id: 3"#)));
    Ok(())
}

#[test]
fn test_unified_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.json").arg("../examples/file2.json").arg("--output").arg("unified");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("-   \"age\": 30,"))
        .stdout(predicate::str::contains("+   \"age\": 31,"))
        .stdout(predicate::str::contains("-   \"city\": \"New York\","));
    Ok(())
}

#[test]
fn test_ignore_keys_regex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/file1.json").arg("../examples/file2.json").arg("--ignore-keys-regex").arg("^age$");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ age:").not())
        .stdout(predicate::str::contains(r#"~ city: "New York" -> "Boston""#))
        .stdout(predicate::str::contains("+ items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_epsilon_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/data1.json").arg("../examples/data2.json").arg("--epsilon").arg("0.00001");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No differences found.")); // No differences expected within epsilon
    Ok(())
}

#[test]
fn test_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/users1.json").arg("../examples/users2.json").arg("--array-id-key").arg("id");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("~ [id=1].age: 25 -> 26"))
                .stdout(predicate::str::contains("+ [id=3]: ").and(predicate::str::contains(r#""id":3"#)).and(predicate::str::contains(r#""name":"Charlie""#)).and(predicate::str::contains(r#""age":28"#)))
        .stdout(predicate::str::contains("~ [0].").not()); // Ensure not comparing by index
    Ok(())
}

#[test]
fn test_directory_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../examples/dir1").arg("../examples/dir2").arg("--recursive");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--- Comparing b.json ---"))
        .stdout(predicate::str::contains("~ key3: \"value3\" -> \"new_value3\""));
    Ok(())
}

#[test]
fn test_meta_chaining() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Generate diff_report_v1.json
    let mut cmd1 = diffx_cmd();
    cmd1.arg("../examples/config_v1.json").arg("../examples/config_v2.json").arg("--output").arg("json");
    let output1 = cmd1.output()?.stdout;
    std::fs::write("../examples/diff_report_v1.json", output1)?;

    // Step 2: Generate diff_report_v2.json
    let mut cmd2 = diffx_cmd();
    cmd2.arg("../examples/config_v2.json").arg("../examples/config_v3.json").arg("--output").arg("json");
    let output2 = cmd2.output()?.stdout;
    std::fs::write("../examples/diff_report_v2.json", output2)?;

    // Step 3: Compare the two diff reports
    let mut cmd3 = diffx_cmd();
    cmd3.arg("../examples/diff_report_v1.json").arg("../examples/diff_report_v2.json");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains(r#""Modified": ["app.version", "1.1", "1.2"]"#))
        .stdout(predicate::str::contains(r#""Modified": ["app.settings.log_level", "debug", "info"]"#))
        .stdout(predicate::str::contains(r#""Added": ["features[2]", "featureD"]"#));

    // Clean up generated diff report files
    std::fs::remove_file("../examples/diff_report_v1.json")?;
    std::fs::remove_file("../examples/diff_report_v2.json")?;

    Ok(())
}
