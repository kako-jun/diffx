use assert_cmd::prelude::*;
use predicates::str;
use std::process::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
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
        .stdout(str::contains("~ age:").not())
        .stdout(str::contains(
            r#"~ city: "New York" -> "Boston""#,
        ))
        .stdout(str::contains("+ items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_epsilon_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/data1.json")
        .arg("../tests/fixtures/data2.json")
        .arg("--epsilon")
        .arg("0.00001");
    cmd.assert().success().stdout(str::is_empty()); // No differences expected within epsilon (empty output)
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
        .stdout(str::contains("~ [id=1].age: 25 -> 26"))
        .stdout(
            str::contains("+ [id=3]: ")
                .and(str::contains(r#""id":3"#))
                .and(str::contains(r#""name":"Charlie""#))
                .and(str::contains(r#""age":28"#)),
        )
        .stdout(str::contains("~ [0].").not()); // Ensure not comparing by index
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
        .stdout(str::contains(
            "~ application.debug: true -> false",
        ))
        .stdout(str::contains(
            "~ application.environment: \"development\" -> \"production\"",
        ))
        .stdout(str::contains("database").not())
        .stdout(str::contains("services").not());
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
        .stdout(str::contains("~ services.auth.url: \"http://localhost:8080\" -> \"https://auth.example.com\""))
        .stdout(str::contains("~ services.cache.enabled: false -> true"))
        .stdout(str::contains("~ services.cache.url: \"redis://localhost:6379\" -> \"redis://cache.example.com:6379\""))
        .stdout(str::contains("application").not())
        .stdout(str::contains("database").not());
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
        .stdout(str::contains(
            "~ database.host: \"localhost\" -> \"prod-db.example.com\"",
        ))
        .stdout(str::contains(
            "~ database.name: \"myapp_dev\" -> \"myapp_prod\"",
        ))
        .stdout(str::contains("~ database.timeout: 30 -> 60"))
        .stdout(str::contains("application").not())
        .stdout(str::contains("services").not());
    Ok(())
}

#[test]
fn test_ignore_case_option() -> Result<(), Box<dyn std::error::Error>> {
    // Test ignore-case option - should not show differences for case-only changes
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/case_test1.json")
        .arg("../tests/fixtures/case_test2.json")
        .arg("--ignore-case");
    cmd.assert()
        .code(0) // No differences when ignoring case
        .stdout(str::is_empty());
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
        .stdout(str::contains(
            "~ status: \"Active\" -> \"ACTIVE\"",
        ))
        .stdout(str::contains("~ level: \"Info\" -> \"INFO\""));
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
        .stdout(str::is_empty());
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
        .stdout(str::contains(
            "~ text: \"Hello  World\" -> \"Hello World\"",
        ))
        .stdout(str::contains(
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
        .stdout(str::contains("- level: \"Info\""))
        .stdout(str::contains("+ message: \"Test Value\""));
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
        .stdout(str::is_empty())
        .stderr(str::is_empty());
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
        .stdout(str::is_empty())
        .stderr(str::is_empty());
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
        .stdout(str::contains(
            "Files ../tests/fixtures/file1.json and ../tests/fixtures/file2.json differ",
        ))
        .stdout(str::contains("age").not()) // Should not show actual differences
        .stdout(str::contains("city").not());
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
        .stdout(str::is_empty());
    Ok(())
}

#[test]
fn test_verbose_basic_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.json")
        .arg("../tests/fixtures/file2.json")
        .arg("--verbose");
    cmd.assert()
        .code(1)
        .stderr(str::contains("Optimization enabled:"))
        .stderr(str::contains("Batch size:"))
        .stderr(str::contains("Input file information:"))
        .stderr(str::contains("Parse time:"))
        .stderr(str::contains("Diff computation time:"))
        .stderr(str::contains("Total differences found:"))
        .stderr(str::contains("Performance summary:"))
        .stderr(str::contains("Total processing time:"));
    Ok(())
}