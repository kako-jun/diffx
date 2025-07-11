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
    cmd.assert().success().stdout(predicate::str::is_empty()); // No differences expected within epsilon (empty output)
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

// ===== NEW OPTIONS TESTS =====

#[test]
fn test_ignore_case_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test ignore-case option - should not show differences for case-only changes
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json")
        .arg("--ignore-case");
    cmd.assert()
        .code(0) // No differences when ignoring case
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_ignore_case_option_shows_differences_without_flag() -> Result<(), Box<dyn std::error::Error>>
{
    // Test that case differences are shown without the ignore-case flag
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains(
            "~ status: \"Active\" -> \"ACTIVE\"",
        ))
        .stdout(predicate::str::contains("~ level: \"Info\" -> \"INFO\""));
    Ok(())
}

#[test]
fn test_ignore_whitespace_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test ignore-whitespace option - should not show differences for whitespace-only changes
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(0) // No differences when ignoring whitespace
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_ignore_whitespace_option_shows_differences_without_flag(
) -> Result<(), Box<dyn std::error::Error>> {
    // Test that whitespace differences are shown without the ignore-whitespace flag
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains(
            "~ text: \"Hello  World\" -> \"Hello World\"",
        ))
        .stdout(predicate::str::contains(
            "~ message: \"Test\\tValue\" -> \"Test Value\"",
        ));
    Ok(())
}

#[test]
fn test_combined_ignore_options() -> Result<(), Box<dyn std::error::Error>> {
    // Test combining ignore-case and ignore-whitespace options
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json") // Mix case and whitespace differences
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Still differences (different keys)
        .stdout(predicate::str::contains("- level: \"Info\""))
        .stdout(predicate::str::contains("+ message: \"Test Value\""));
    Ok(())
}

#[test]
fn test_quiet_option_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test quiet option with identical files - should output nothing and exit 0
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--quiet");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_quiet_option_with_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test quiet option with different files - should output nothing and exit 1
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--quiet");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_brief_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test brief option - should only show filenames, not differences
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--brief");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains(
            "Files ../tests/fixtures/file1.json and ../tests/fixtures/file2.json differ",
        ))
        .stdout(predicate::str::contains("age").not()) // Should not show actual differences
        .stdout(predicate::str::contains("city").not());
    Ok(())
}

#[test]
fn test_brief_option_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    // Test brief option with identical files - should output nothing
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--brief");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_context_option_unified_output() -> Result<(), Box<dyn std::error::Error>> {
    // Test context option with unified output format
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("2");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("-      \"port\": 5432"))
        .stdout(predicate::str::contains("+      \"port\": 5433"))
        .stdout(predicate::str::contains("\"host\": \"localhost\"")) // Context line
        .stdout(predicate::str::contains("\"name\": \"myapp\"")); // Context line
    Ok(())
}

#[test]
fn test_context_option_zero_context() -> Result<(), Box<dyn std::error::Error>> {
    // Test context option with zero context - should show only changed lines
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("0");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("-      \"port\": 5432"))
        .stdout(predicate::str::contains("+      \"port\": 5433"))
        .stdout(predicate::str::contains("\"host\": \"localhost\"").not()) // No context
        .stdout(predicate::str::contains("\"name\": \"myapp\"").not()); // No context
    Ok(())
}

#[test]
fn test_auto_optimization_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test that small files use standard mode, large files auto-optimize
    // Since we can't easily test large files, we test that small files work normally
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_auto_optimization_on_small_files() -> Result<(), Box<dyn std::error::Error>> {
    // Test that automatic optimization works correctly on small files
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("~ age: 30 -> 31")); // Same output as standard mode
    Ok(())
}

#[test]
fn test_complex_options_combination() -> Result<(), Box<dyn std::error::Error>> {
    // Test combination of multiple new options
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|version)$")
        .arg("--path")
        .arg("application")
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(predicate::str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ));
    Ok(())
}

// ===== UNIX COMMAND PATTERN TESTS =====

#[test]
fn test_unix_pattern_diff_q_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -q equivalent: quiet mode exit codes only
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json") // Same file
        .arg("--quiet");
    cmd.assert()
        .code(0) // No differences
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    // Test with different files
    let mut cmd2 = diffx_cmd();
    cmd2.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--quiet");
    cmd2.assert()
        .code(1) // Differences found
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_brief_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff --brief equivalent: filenames only
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--brief");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains(
            "Files ../tests/fixtures/file1.json and ../tests/fixtures/file2.json differ",
        ))
        .stdout(predicate::str::contains("age").not()) // Should not show details
        .stdout(predicate::str::contains("city").not());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_i_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -i equivalent: ignore case
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json")
        .arg("--ignore-case");
    cmd.assert()
        .code(0) // No differences when ignoring case
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_w_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -w equivalent: ignore whitespace
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/whitespace_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(0) // No differences when ignoring whitespace
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_pattern_diff_c3_equivalent() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -C3 equivalent: context lines
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("2");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("-      \"port\": 5432"))
        .stdout(predicate::str::contains("+      \"port\": 5433"))
        .stdout(predicate::str::contains("\"host\": \"localhost\"")); // Context line
    Ok(())
}

#[test]
fn test_unix_combined_pattern_qiw() -> Result<(), Box<dyn std::error::Error>> {
    // Test combined pattern: diff -qiw equivalent
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/whitespace_test2.json")
        .arg("--quiet")
        .arg("--ignore-case")
        .arg("--ignore-whitespace");
    cmd.assert()
        .code(1) // Still differences (different keys)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_unix_directory_brief_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test diff -r --brief equivalent for directories
    // Note: --brief affects individual file comparison, not directory traversal
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive")
        .arg("--brief");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("Comparing")); // Directory comparison shows file names
    Ok(())
}

// ===== CI/CD AUTOMATION PATTERN TESTS =====

#[test]
fn test_cicd_deployment_validation_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test deployment validation pattern from documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-case")
        .arg("--ignore-whitespace")
        .arg("--ignore-keys-regex")
        .arg("^(environment|debug|host|port)")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("Modified").or(predicate::str::contains("[]")));
    Ok(())
}

#[test]
fn test_cicd_config_drift_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // Test configuration drift monitoring pattern
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^(hostname|instance_id|last_.*|timestamp)")
        .arg("--ignore-case")
        .arg("--quiet");
    cmd.assert()
        .code(1) // Configuration drift detected
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn test_cicd_batch_file_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Test batch file validation pattern
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let config1_path = temp_dir.path().join("config1.json");
    let config2_path = temp_dir.path().join("config2.json");

    fs::write(&config1_path, r#"{"app": "test", "version": "1.0"}"#)?;
    fs::write(&config2_path, r#"{"app": "test", "version": "1.1"}"#)?;

    // Test first file (no differences)
    let mut cmd1 = diffx_cmd();
    cmd1.arg(&config1_path).arg(&config1_path).arg("--quiet");
    cmd1.assert().code(0);

    // Test second file (differences)
    let mut cmd2 = diffx_cmd();
    cmd2.arg(&config1_path).arg(&config2_path).arg("--quiet");
    cmd2.assert().code(1);

    Ok(())
}

#[test]
fn test_git_workflow_integration_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test Git workflow integration pattern
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-whitespace")
        .arg("--context")
        .arg("2")
        .arg("--output")
        .arg("unified");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("-"))
        .stdout(predicate::str::contains("+"));
    Ok(())
}

#[test]
fn test_monitoring_script_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test monitoring script pattern with alerting
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(hostname|instance_id|last_update|timestamp)")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::starts_with("["));
    Ok(())
}

// ===== ADVANCED AUTOMATION TESTS =====

#[test]
fn test_api_contract_validation_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test API contract validation from documentation examples
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|requestId|serverId|responseTime)$")
        .arg("--ignore-case")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Differences found
        .stdout(predicate::str::contains("Modified").or(predicate::str::contains("Added")));
    Ok(())
}

#[test]
fn test_pre_commit_hook_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test pre-commit hook pattern from documentation
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(environment|debug)$")
        .arg("--quiet");
    cmd.assert().code(1); // Would trigger pre-commit warning
    Ok(())
}

#[test]
fn test_kubernetes_config_drift_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Test Kubernetes configuration drift detection pattern
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_dev.json")
        .arg("../tests/fixtures/config_prod.json")
        .arg("--ignore-keys-regex")
        .arg("^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)")
        .arg("--output")
        .arg("json");
    cmd.assert()
        .code(1) // Configuration differences
        .stdout(predicate::str::starts_with("["));
    Ok(())
}

// ===============================================
// Verbose Output Tests
// ===============================================

#[test]
fn test_verbose_basic_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Optimization enabled:"))
        .stderr(predicate::str::contains("Batch size:"))
        .stderr(predicate::str::contains("Input file information:"))
        .stderr(predicate::str::contains("Parse time:"))
        .stderr(predicate::str::contains("Diff computation time:"))
        .stderr(predicate::str::contains("Total differences found:"))
        .stderr(predicate::str::contains("Performance summary:"))
        .stderr(predicate::str::contains("Total processing time:"));
    Ok(())
}

#[test]
fn test_verbose_key_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose")
        .arg("--ignore-keys-regex")
        .arg("age");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Key filtering configuration:"))
        .stderr(predicate::str::contains("Regex pattern: age"));
    Ok(())
}

#[test]
fn test_verbose_epsilon_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose")
        .arg("--epsilon")
        .arg("0.1");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains(
            "Numerical tolerance configuration:",
        ))
        .stderr(predicate::str::contains("Epsilon value: 0.1"));
    Ok(())
}

#[test]
fn test_verbose_array_id_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/users1.json")
        .arg("../tests/fixtures/users2.json")
        .arg("--verbose")
        .arg("--array-id-key")
        .arg("id");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Array tracking configuration:"))
        .stderr(predicate::str::contains("ID key for array elements: id"));
    Ok(())
}

#[test]
fn test_verbose_path_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--verbose")
        .arg("--path")
        .arg("app"); // Use "app" path which should have differences
    cmd.assert()
        .stderr(predicate::str::contains("Path filtering results:"))
        .stderr(predicate::str::contains("Filter path: app"))
        .stderr(predicate::str::contains("Total differences before filter:"))
        .stderr(predicate::str::contains("Differences after filter:"));
    Ok(())
}

#[test]
fn test_verbose_context_display() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/context_test1.json")
        .arg("../tests/fixtures/context_test2.json")
        .arg("--verbose")
        .arg("--output")
        .arg("unified")
        .arg("--context")
        .arg("3");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Context display configuration:"))
        .stderr(predicate::str::contains("Context lines: 3"))
        .stderr(predicate::str::contains("Context display results:"))
        .stderr(predicate::str::contains("Difference blocks shown:"));
    Ok(())
}

#[test]
fn test_verbose_directory_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Directory scan results:"))
        .stderr(predicate::str::contains("Files in ../tests/fixtures/dir1:"))
        .stderr(predicate::str::contains("Files in ../tests/fixtures/dir2:"))
        .stderr(predicate::str::contains("Total files to compare:"))
        .stderr(predicate::str::contains("Directory comparison summary:"))
        .stderr(predicate::str::contains("Files compared:"))
        .stderr(predicate::str::contains("Differences found:"));
    Ok(())
}

#[test]
fn test_verbose_combined_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose")
        .arg("--ignore-keys-regex")
        .arg("nonexistent") // Use a regex that won't filter out differences
        .arg("--epsilon")
        .arg("0.01");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Key filtering configuration:"))
        .stderr(predicate::str::contains(
            "Numerical tolerance configuration:",
        ))
        .stderr(predicate::str::contains("Performance summary:"));
    Ok(())
}

#[test]
fn test_verbose_no_differences() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file1.json")
        .arg("--verbose");
    cmd.assert()
        .code(0)
        .stderr(predicate::str::contains("Total differences found: 0"))
        .stderr(predicate::str::contains("Performance summary:"));
    Ok(())
}

#[test]
fn test_verbose_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/config_v1.json")
        .arg("../tests/fixtures/config_v2.json")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Input file information:"))
        .stderr(predicate::str::contains("bytes"))
        .stderr(predicate::str::contains("Parse time:"))
        .stderr(predicate::str::contains("µs").or(predicate::str::contains("ms")))
        .stderr(predicate::str::contains("Diff computation time:"))
        .stderr(predicate::str::contains("Total processing time:"))
        .stderr(predicate::str::contains("Memory optimization:"));
    Ok(())
}

// Unix diff compatibility tests
#[test]
fn test_directory_comparison_without_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("--- Comparing"))
        .stdout(predicate::str::contains("Common subdirectories")); // Shows subdirs but doesn't compare them
    Ok(())
}

#[test]
fn test_directory_comparison_with_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("--- Comparing"));
    Ok(())
}

#[test]
fn test_directory_vs_file_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/file1.json");
    cmd.assert().code(2).stderr(predicate::str::contains(
        "Cannot compare directory and file",
    ));
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_non_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Directory scan results:"))
        .stderr(predicate::str::contains("Recursive mode: false"));
    Ok(())
}

#[test]
fn test_directory_comparison_verbose_recursive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("Directory scan results:"))
        .stderr(predicate::str::contains("Recursive mode: true"));
    Ok(())
}

#[test]
fn test_directory_with_common_subdirectories() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("Common subdirectories:"))
        .stdout(predicate::str::contains("subdir")); // Should show common subdir but not compare files inside
    Ok(())
}

#[test]
fn test_recursive_compares_nested_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2")
        .arg("--recursive");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains(
            "--- Comparing subdir/nested.json ---",
        )) // Should compare nested files
        .stdout(
            predicate::str::contains("data:")
                .and(predicate::str::contains("value1").and(predicate::str::contains("value2"))),
        );
    Ok(())
}

#[test]
fn test_non_recursive_does_not_compare_nested_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/dir1")
        .arg("../tests/fixtures/dir2");
    cmd.assert()
        .code(1)
        .stdout(predicate::str::contains("--- Comparing subdir/nested.json ---").not()); // Should NOT compare nested files
    Ok(())
}
