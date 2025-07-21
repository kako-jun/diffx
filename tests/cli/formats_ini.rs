use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_ini_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.ini")
        .arg("../tests/fixtures/file2.ini");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ section1.key2: \"value2\" -> \"new_value2\"",
        ))
        .stdout(predicates::str::contains("+ section2.key4: \"value4\""));
    Ok(())
}

#[test]
fn test_format_ini_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.ini")
        .arg("../tests/fixtures/file2.ini")
        .arg("--format")
        .arg("ini");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ section1.key2:"));
    Ok(())
}

#[test]
fn test_ini_multiple_sections() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("[database]\nhost=localhost\nport=5432\n\n[cache]\nenabled=true\nttl=3600\n")
        .write_stdin("[database]\nhost=prod-server\nport=5432\n\n[cache]\nenabled=false\nttl=7200\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ database.host:"))
        .stdout(predicates::str::contains("~ cache.enabled:"))
        .stdout(predicates::str::contains("~ cache.ttl:"));
    Ok(())
}

#[test]
fn test_ini_global_section() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("global_key=global_value\nother_key=value\n[section1]\nkey1=value1\n")
        .write_stdin("global_key=new_global_value\nother_key=value\n[section1]\nkey1=new_value1\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ global_key:"))
        .stdout(predicates::str::contains("~ section1.key1:"));
    Ok(())
}

#[test]
fn test_ini_comments_and_empty_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("; This is a comment\n[section]\nkey=value\n\n; Another comment\nkey2=value2\n")
        .write_stdin("; This is a comment\n[section]\nkey=new_value\n\n; Another comment\nkey2=value2\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ section.key:"));
    Ok(())
}

#[test]
fn test_ini_special_characters() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("[paths]\ntemp_dir=C:\\temp\nlog_file=app.log\n")
        .write_stdin("[paths]\ntemp_dir=D:\\temp\nlog_file=application.log\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ paths.temp_dir:"))
        .stdout(predicates::str::contains("~ paths.log_file:"));
    Ok(())
}

#[test]
fn test_ini_boolean_and_numeric_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("[settings]\nenabled=true\nmax_connections=100\ntimeout=30.5\n")
        .write_stdin("[settings]\nenabled=false\nmax_connections=200\ntimeout=60.0\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ settings.enabled:"))
        .stdout(predicates::str::contains("~ settings.max_connections:"))
        .stdout(predicates::str::contains("~ settings.timeout:"));
    Ok(())
}

#[test]
fn test_ini_missing_sections() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("ini");
    cmd.write_stdin("[section1]\nkey1=value1\n[section2]\nkey2=value2\n")
        .write_stdin("[section1]\nkey1=value1\n[section3]\nkey3=value3\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("- section2"))
        .stdout(predicates::str::contains("+ section3"));
    Ok(())
}
