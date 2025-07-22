use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_toml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.toml")
        .arg("../tests/fixtures/file2.toml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"))
        .stdout(predicates::str::contains(
            "~ city: \"New York\" -> \"Boston\"",
        ))
        .stdout(predicates::str::contains("  + items[2]: \"orange\""));
    Ok(())
}

#[test]
fn test_format_toml_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.toml")
        .arg("../tests/fixtures/file2.toml")
        .arg("--format")
        .arg("toml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ age: 30 -> 31"));
    Ok(())
}

#[test]
fn test_toml_tables_and_arrays() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "[server]\nhost = \"localhost\"\nport = 8080\n\n[[users]]\nname = \"Alice\"\nage = 30\n")?;
    fs::write(&file2_path, "[server]\nhost = \"example.com\"\nport = 8080\n\n[[users]]\nname = \"Alice\"\nage = 31\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ server.host:"))
        .stdout(predicates::str::contains("~ users[0].age:"));
    Ok(())
}

#[test]
fn test_toml_nested_tables() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "[database]\nhost = \"localhost\"\n[database.pool]\nmax_connections = 10\n")?;
    fs::write(&file2_path, "[database]\nhost = \"localhost\"\n[database.pool]\nmax_connections = 20\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ database.pool.max_connections: 10 -> 20"));
    Ok(())
}

#[test]
fn test_toml_array_of_tables() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "[[products]]\nname = \"Hammer\"\nprice = 15.99\n\n[[products]]\nname = \"Screwdriver\"\nprice = 8.50\n")?;
    fs::write(&file2_path, "[[products]]\nname = \"Hammer\"\nprice = 16.99\n\n[[products]]\nname = \"Drill\"\nprice = 45.00\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ products[0].price:"))
        .stdout(predicates::str::contains("~ products[1].name:"));
    Ok(())
}

#[test]
fn test_toml_different_data_types() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "title = \"Config\"\nenabled = true\ncount = 42\npi = 3.14\ndate = 2023-01-01T10:00:00Z\n")?;
    fs::write(&file2_path, "title = \"New Config\"\nenabled = false\ncount = 42\npi = 3.14159\ndate = 2023-01-02T10:00:00Z\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ title:"))
        .stdout(predicates::str::contains("~ enabled: true -> false"))
        .stdout(predicates::str::contains("~ pi:"))
        .stdout(predicates::str::contains("~ date:"));
    Ok(())
}

#[test]
fn test_toml_multiline_strings() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "description = \"\"\"\nThis is a\nmultiline string\n\"\"\"\n")?;
    fs::write(&file2_path, "description = \"\"\"\nThis is a\ndifferent multiline string\n\"\"\"\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ description:"));
    Ok(())
}

#[test]
fn test_toml_inline_tables() -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::tempdir;
    use std::fs;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.toml");
    let file2_path = temp_dir.path().join("file2.toml");

    fs::write(&file1_path, "point = { x = 1, y = 2 }\ncolor = { r = 255, g = 128, b = 0 }\n")?;
    fs::write(&file2_path, "point = { x = 3, y = 2 }\ncolor = { r = 255, g = 128, b = 64 }\n")?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path)
        .arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ point.x: 1 -> 3"))
        .stdout(predicates::str::contains("~ color.b: 0 -> 64"));
    Ok(())
}
