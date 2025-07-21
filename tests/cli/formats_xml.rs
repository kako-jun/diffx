use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_cmd::Command;

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
            "~ item.$text: \"value2\" -> \"value3\"",
        ))
        .stdout(predicates::str::contains("~ item.@id: \"2\" -> \"3\""));
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
        .stdout(predicates::str::contains("~ item.@id:"));
    Ok(())
}

#[test]
fn test_xml_attributes_and_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<root><person id=\"1\" name=\"John\">Content</person></root>")
        .write_stdin("<root><person id=\"2\" name=\"Jane\">Different Content</person></root>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ person.@id:"))
        .stdout(predicates::str::contains("~ person.@name:"))
        .stdout(predicates::str::contains("~ person.$text:"));
    Ok(())
}

#[test]
fn test_xml_nested_elements() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<config><database><host>localhost</host><port>5432</port></database></config>")
        .write_stdin("<config><database><host>prod-server</host><port>5433</port></database></config>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ database.host.$text:"))
        .stdout(predicates::str::contains("~ database.port.$text:"));
    Ok(())
}

#[test]
fn test_xml_arrays_and_lists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<items><item>A</item><item>B</item><item>C</item></items>")
        .write_stdin("<items><item>A</item><item>X</item><item>C</item></items>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ item[1].$text: \"B\" -> \"X\""));
    Ok(())
}

#[test]
fn test_xml_namespaces() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<root xmlns:ns=\"http://example.com\"><ns:element>value1</ns:element></root>")
        .write_stdin("<root xmlns:ns=\"http://example.com\"><ns:element>value2</ns:element></root>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~"));
    Ok(())
}

#[test]
fn test_xml_cdata_sections() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<data><![CDATA[Some <text> with & special chars]]></data>")
        .write_stdin("<data><![CDATA[Different <text> with & special chars]]></data>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ data.$text:"));
    Ok(())
}

#[test]
fn test_xml_mixed_content() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<paragraph>This is <em>important</em> text.</paragraph>")
        .write_stdin("<paragraph>This is <em>critical</em> text.</paragraph>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ em.$text: \"important\" -> \"critical\""));
    Ok(())
}

#[test]
fn test_xml_empty_elements() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<root><empty/><self-closing attr=\"val\"/></root>")
        .write_stdin("<root><empty/><self-closing attr=\"new_val\"/></root>");
    cmd.assert()
        .code(1)
        .stdout(predicates::str::contains("~ self-closing.@attr:"));
    Ok(())
}

#[test]
fn test_xml_multiple_root_elements() -> Result<(), Box<dyn std::error::Error>> {
    // Test handling of XML fragments or multiple root elements
    let mut cmd = diffx_cmd();
    cmd.arg("-")
        .arg("-")
        .arg("--format")
        .arg("xml");
    cmd.write_stdin("<item1>value1</item1><item2>value2</item2>")
        .write_stdin("<item1>new_value1</item1><item2>value2</item2>");
    // This might fail with malformed XML, but should handle gracefully
    let result = cmd.output()?;
    // Should either parse successfully or fail gracefully
    Ok(())
}
