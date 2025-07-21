use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

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

#[test]
fn test_format_csv_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.csv")
        .arg("../tests/fixtures/file2.csv")
        .arg("--format")
        .arg("csv");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].header2:"));
    Ok(())
}

#[test]
fn test_csv_with_quotes_and_commas() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("name,description\nProduct A,\"High quality, reliable\"\n")
        .write_stdin("name,description\nProduct A,\"Premium quality, durable\"\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].description:"));
    Ok(())
}

#[test]
fn test_csv_with_headers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("id,name,age\n1,John,30\n2,Jane,25\n")
        .write_stdin("id,name,age\n1,John,31\n2,Jane,25\n3,Bob,28\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].age: \"30\" -> \"31\""))
        .stdout(predicates::str::contains("+ [2]:"));
    Ok(())
}

#[test]
fn test_csv_missing_columns() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("name,age\nAlice,30\nBob,25\n")
        .write_stdin("name,age,city\nAlice,30,NYC\nBob,25,LA\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("+ [0].city:"))
        .stdout(predicates::str::contains("+ [1].city:"));
    Ok(())
}

#[test]
fn test_csv_empty_fields() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("name,email,phone\nJohn,,555-0123\n")
        .write_stdin("name,email,phone\nJohn,john@email.com,555-0123\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].email: \"\" -> \"john@email.com\""));
    Ok(())
}

#[test]
fn test_csv_different_row_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("id,name\n1,Alice\n2,Bob\n3,Charlie\n")
        .write_stdin("id,name\n1,Alice\n2,Bob\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("- [2]:"));
    Ok(())
}

#[test]
fn test_csv_special_characters() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("name,notes\n\"O'Neil\",\"Line1\nLine2\"\n")
        .write_stdin("name,notes\n\"O'Neil\",\"Line1\nLine3\"\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].notes:"));
    Ok(())
}

#[test]
fn test_csv_numeric_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("csv");
    cmd.write_stdin("item,price,quantity\nApple,1.25,10\nBanana,0.85,15\n")
        .write_stdin("item,price,quantity\nApple,1.30,10\nBanana,0.85,20\n");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ [0].price: \"1.25\" -> \"1.30\""))
        .stdout(predicates::str::contains("~ [1].quantity: \"15\" -> \"20\""));
    Ok(())
}
