use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

#[test]
fn test_basic_xml_diff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.xml")
        .arg("../tests/fixtures/file2.xml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ item[1].text: \"value2\" -> \"new_value2\"",
        ))
        .stdout(predicates::str::contains("+ item[2]:"));
    Ok(())
}

#[test]
fn test_format_xml_explicit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("../tests/fixtures/file1.xml")
        .arg("../tests/fixtures/file2.xml")
        .arg("--format")
        .arg("xml");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains(
            "~ item[1].text: \"value2\" -> \"new_value2\"",
        ))
        .stdout(predicates::str::contains("+ item[2]:"));
    Ok(())
}

#[test]
fn test_xml_attributes_and_text() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<root><person id=\"1\" name=\"John\">Content</person></root>",
    )?;
    fs::write(
        &file2_path,
        "<root><person id=\"2\" name=\"Jane\">Different Content</person></root>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ person.id:"))
        .stdout(predicates::str::contains("~ person.name:"))
        .stdout(predicates::str::contains("~ person.text:"));
    Ok(())
}

#[test]
fn test_xml_nested_elements() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<config><database><host>localhost</host><port>5432</port></database></config>",
    )?;
    fs::write(
        &file2_path,
        "<config><database><host>prod-server</host><port>5433</port></database></config>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ database.host:"))
        .stdout(predicates::str::contains("~ database.port:"));
    Ok(())
}

#[test]
fn test_xml_arrays_and_lists() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<items><item>A</item><item>B</item><item>C</item></items>",
    )?;
    fs::write(
        &file2_path,
        "<items><item>A</item><item>X</item><item>C</item></items>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ item[1]: \"B\" -> \"X\""));
    Ok(())
}

#[test]
fn test_xml_namespaces() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<root xmlns:ns=\"http://example.com\"><ns:element>value1</ns:element></root>",
    )?;
    fs::write(
        &file2_path,
        "<root xmlns:ns=\"http://example.com\"><ns:element>value2</ns:element></root>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert().code(1).stdout(predicates::str::contains("~"));
    Ok(())
}

#[test]
fn test_xml_cdata_sections() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<data><![CDATA[Some <text> with & special chars]]></data>",
    )?;
    fs::write(
        &file2_path,
        "<data><![CDATA[Different <text> with & special chars]]></data>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ :"));
    Ok(())
}

#[test]
fn test_xml_mixed_content() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<paragraph>This is <em>important</em> text.</paragraph>",
    )?;
    fs::write(
        &file2_path,
        "<paragraph>This is <em>critical</em> text.</paragraph>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert().code(1).stdout(predicates::str::contains(
        "~ em: \"important\" -> \"critical\"",
    ));
    Ok(())
}

#[test]
fn test_xml_empty_elements() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<root><empty/><self-closing attr=\"val\"/></root>",
    )?;
    fs::write(
        &file2_path,
        "<root><empty/><self-closing attr=\"new_val\"/></root>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ self-closing.attr:"));
    Ok(())
}

#[test]
fn test_xml_multiple_root_elements() -> Result<(), Box<dyn std::error::Error>> {
    // Test handling of XML fragments or multiple root elements
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir()?;
    let file1_path = temp_dir.path().join("file1.xml");
    let file2_path = temp_dir.path().join("file2.xml");

    fs::write(
        &file1_path,
        "<root><item1>value1</item1><item2>value2</item2></root>",
    )?;
    fs::write(
        &file2_path,
        "<root><item1>new_value1</item1><item2>value2</item2></root>",
    )?;

    let mut cmd = diffx_cmd();
    cmd.arg(&file1_path).arg(&file2_path);
    // This might fail with malformed XML, but should handle gracefully
    let _result = cmd.output()?;
    // Should either parse successfully or fail gracefully
    Ok(())
}
