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
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_yaml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.yaml")
        .arg("../tests/fixtures/file2.yaml");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_toml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.toml")
        .arg("../tests/fixtures/file2.toml");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age: 30 -> 31"))
        .stdout(predicate::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicate::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_basic_ini_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.ini")
        .arg("../tests/fixtures/file2.ini");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ section1.key2: \"value2\" -> \"new_value2\"",
        ))
        .stdout(predicate::str::contains("+ section2.key4: \"value4\""));
    Ok(())
}

#[test]
fn test_basic_xml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.xml")
        .arg("../tests/fixtures/file2.xml");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ item.$text: \"value2\" -> \"value3\"",
        ))
        .stdout(predicate::str::contains("~ item.@id: \"2\" -> \"3\""));
    Ok(())
}

#[test]
fn test_basic_csv_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.csv")
        .arg("../tests/fixtures/file2.csv");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ [0].header2: \"valueB\" -> \"new_valueB\"",
        ))
        .stdout(
            predicate::str::contains("+ [2]: ")
                .and(predicate::str::contains("\"header1\":\"valueE\""))
                .and(predicate::str::contains("\"header2\":\"valueF\"")),
        );
    Ok(())
}

#[test]
fn test_specify_input_format() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut cmd = diffx_cmd();
    let mut child = cmd
        .arg("-")
        .arg("../tests/fixtures/file2.json")
        .arg("--format")
        .arg("json")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(
            r#"{
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
}"#
            .as_bytes(),
        )?;
    } // stdin is dropped here, closing the pipe
    let output = child.wait_with_output()?;
    assert_eq!(output.status.code(), Some(1)); // Differences found
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(predicate::str::contains("~ age: 30 -> 31").eval(&stdout));
    assert!(predicate::str::contains("~ city: \"New York\" -> \"Boston\"").eval(&stdout));
    assert!(predicate::str::contains("~ name: \"Alice\" -> \"John\"").eval(&stdout));
    assert!(predicate::str::contains("+ items:").eval(&stdout));
    Ok(())
}

#[test]
fn test_json_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
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
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            r#"- Modified:
  - age
  - 30
  - 31"#,
        ))
        .stdout(predicate::str::contains(
            r#"- Modified:
  - city
  - New York
  - Boston"#,
        ))
        .stdout(predicate::str::contains(
            r#"- Added:
  - items[2]
  - orange"#,
        ));
    Ok(())
}

#[test]
fn test_unified_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--output")
        .arg("unified");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("-  \"age\": 30,"))
        .stdout(predicate::str::contains("+  \"age\": 31,"))
        .stdout(predicate::str::contains("-  \"city\": \"New York\","));
    Ok(())
}

#[test]
fn test_ignore_keys_regex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^age$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ age:").not())
        .stdout(predicate::str::contains(
            r#"~ city: "New York" -> "Boston""#,
        ))
        .stdout(predicate::str::contains("+ items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_epsilon_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/data1.json")
        .arg("../tests/fixtures/data2.json")
        .arg("--epsilon")
        .arg("0.00001");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty()); // No differences expected within epsilon (empty output)
    Ok(())
}

#[test]
fn test_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json")
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ [id=1].age: 25 -> 26"))
        .stdout(
            predicate::str::contains("+ [id=3]: ")
                .and(predicate::str::contains(r#""id":3"#))
                .and(predicate::str::contains(r#""name":"Charlie""#))
                .and(predicate::str::contains(r#""age":28"#)),
        )
        .stdout(predicate::str::contains("~ [0].").not()); // Ensure not comparing by index
    Ok(())
}

#[test]
fn test_directory_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("--- Comparing b.json ---"))
        .stdout(predicate::str::contains(
            "~ key3: \"value3\" -> \"new_value3\"",
        ));
    Ok(())
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
fn test_path_filtering_application() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(predicate::str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ))
        .stdout(predicate::str::contains("database").not())
        .stdout(predicate::str::contains("services").not());
    Ok(())
}

#[test]
fn test_path_filtering_services() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("services");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("~ services.auth.url: \"http://localhost:8080\" -> \"https://auth.example.com\""))
        .stdout(predicate::str::contains("~ services.cache.enabled: false -> true"))
        .stdout(predicate::str::contains("~ services.cache.url: \"redis://localhost:6379\" -> \"redis://cache.example.com:6379\""))
        .stdout(predicate::str::contains("application").not())
        .stdout(predicate::str::contains("database").not());
    Ok(())
}

#[test]
fn test_path_filtering_database() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("database");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ database.host: \"localhost\" -> \"prod-db.example.com\"",
        ))
        .stdout(predicate::str::contains(
            "~ database.name: \"myapp_dev\" -> \"myapp_prod\"",
        ))
        .stdout(predicate::str::contains("~ database.timeout: 30 -> 60"))
        .stdout(predicate::str::contains("application").not())
        .stdout(predicate::str::contains("services").not());
    Ok(())
}

#[test]
fn test_complex_regex_security_fields() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(password|secret_.*|credentials|connection_string)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicate::str::contains(
            "~ security.host: \"localhost\" -> \"prod-server.example.com\"",
        ))
        .stdout(predicate::str::contains("password").not())
        .stdout(predicate::str::contains("secret_").not())
        .stdout(predicate::str::contains("credentials").not())
        .stdout(predicate::str::contains("connection_string").not());
    Ok(())
}

#[test]
fn test_complex_regex_build_fields() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*|deploy_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicate::str::contains(
            "~ monitoring.metrics.cpu: 45.2 -> 52.1",
        ))
        .stdout(predicate::str::contains(
            "~ monitoring.metrics.memory: 78.9 -> 82.3",
        ))
        .stdout(predicate::str::contains("timestamp").not())
        .stdout(predicate::str::contains("build_").not())
        .stdout(predicate::str::contains("deploy_").not());
    Ok(())
}

#[test]
fn test_complex_regex_multiple_groups() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(password|secret_.*|timestamp|build_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ application.version: \"1.0.0\" -> \"1.1.0\"",
        ))
        .stdout(predicate::str::contains(
            "~ security.host: \"localhost\" -> \"prod-server.example.com\"",
        ))
        .stdout(predicate::str::contains("password").not())
        .stdout(predicate::str::contains("secret_").not())
        .stdout(predicate::str::contains("timestamp").not())
        .stdout(predicate::str::contains("build_").not());
    Ok(())
}

#[test]
fn test_combined_path_and_regex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--path")
        .arg("monitoring")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ monitoring.metrics.cpu: 45.2 -> 52.1",
        ))
        .stdout(predicate::str::contains(
            "~ monitoring.metrics.memory: 78.9 -> 82.3",
        ))
        .stdout(predicate::str::contains("~ monitoring.deploy_time:"))
        .stdout(predicate::str::contains("timestamp").not())
        .stdout(predicate::str::contains("build_").not())
        .stdout(predicate::str::contains("application").not())
        .stdout(predicate::str::contains("security").not());
    Ok(())
}

#[test]
fn test_combined_path_and_output_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""application.debug""#))
        .stdout(predicate::str::contains(r#"true"#))
        .stdout(predicate::str::contains(r#"false"#))
        .stdout(predicate::str::contains("database").not())
        .stdout(predicate::str::contains("services").not());
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

#[test]
fn test_config_file_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test configuration file by setting the config path env var
    // Note: Config file integration is not fully implemented yet, so this test
    // currently verifies that the command succeeds even with config path set
    let mut cmd = diffx_cmd();
    cmd.env("DIFFX_CONFIG_PATH", "../tests/fixtures/test_config.toml")
        .arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1)
        // Config file specifies output = "json", so expect JSON format
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""age""#))
        .stdout(predicate::str::contains(r#""city""#))
        .stdout(predicate::str::contains(r#""Added""#))
        .stdout(predicate::str::contains(r#""items[2]""#));
    Ok(())
}

#[test]
fn test_format_specification_with_stdin() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut cmd = diffx_cmd();
    let mut child = cmd
        .arg("-")
        .arg("../tests/fixtures/file2.ini")
        .arg("--format")
        .arg("ini")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(
            br#"[section1]
key1 = value1
key2 = value2

[section2]
key3 = value3
"#,
        )?;
    }

    let output = child.wait_with_output()?;
    assert_eq!(output.status.code(), Some(1)); // Differences found
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        predicate::str::contains("~ section1.key2: \"value2\" -> \"new_value2\"").eval(&stdout)
    );
    assert!(predicate::str::contains("+ section2.key4: \"value4\"").eval(&stdout));
    Ok(())
}

// Industry-specific scenario tests
#[test]
fn test_api_schema_comparison() -> Result<(), Box<dyn std::error::Error>> {
    // Test API schema evolution - common DevOps use case
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/api_schema_v1.json")
        .arg("../tests/fixtures/api_schema_v2.json")
        .arg("--path")
        .arg("paths")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""Added""#))
        .stdout(predicate::str::contains(r#"paths./users.post"#))
        .stdout(predicate::str::contains(r#"schema.type"#));
    Ok(())
}

#[test]
fn test_cicd_configuration_drift() -> Result<(), Box<dyn std::error::Error>> {
    // Test CI/CD configuration monitoring - ignore build metadata
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/security_config.json")
        .arg("../tests/fixtures/security_config_new.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|build_.*|deploy_.*|password|secret_.*)$")
        .arg("--output")
        .arg("yaml");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("Modified"))
        .stdout(predicate::str::contains("application.version"))
        .stdout(predicate::str::contains("security.host"))
        .stdout(predicate::str::contains("monitoring.metrics"))
        .stdout(predicate::str::contains("timestamp").not())
        .stdout(predicate::str::contains("password").not())
        .stdout(predicate::str::contains("secret_").not());
    Ok(())
}

#[test]
fn test_environment_config_comparison() -> Result<(), Box<dyn std::error::Error>> {
    // Test environment configuration comparison - focus on application settings
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--path")
        .arg("application")
        .arg("--ignore-keys-regex")
        .arg("^(host|port|password|.*_secret)$");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(predicate::str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ))
        .stdout(predicate::str::contains("host").not())
        .stdout(predicate::str::contains("port").not());
    Ok(())
}

#[test]
fn test_array_id_key_from_config() -> Result<(), Box<dyn std::error::Error>> {
    // Test that array_id_key configuration is properly loaded from config file
    let mut cmd = diffx_cmd();
    cmd.env(
        "DIFFX_CONFIG_PATH",
        "../tests/fixtures/array_id_config.toml",
    )
    .arg("../tests/fixtures/users_v1.json")
    .arg("../tests/fixtures/users_v2.json");
    cmd.assert()
        .code(1)
        // Config specifies array_id_key = "id" and output = "json"
        // Should identify array elements by ID and show semantic changes
        .stdout(predicate::str::contains(r#""Modified""#))
        .stdout(predicate::str::contains(r#""[id=2].role""#))
        .stdout(predicate::str::contains(r#""user""#))
        .stdout(predicate::str::contains(r#""moderator""#))
        .stdout(predicate::str::contains(r#""Added""#))
        .stdout(predicate::str::contains(r#""[id=3]""#));
    Ok(())
}
