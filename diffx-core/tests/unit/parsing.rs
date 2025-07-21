use diffx_core::*;
use serde_json::json;

#[test]
fn test_parse_ini() {
    let ini_content = r#"
[section1]
key1 = value1
key2 = value2

[section2]
key3 = value3
"#;
    let expected = json!({
        "section1": {
            "key1": "value1",
            "key2": "value2"
        },
        "section2": {
            "key3": "value3"
        }
    });
    let parsed = parse_ini(ini_content).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_ini_global_section() {
    let ini_content = r#"
key_global = value_global
[section1]
key1 = value1
"#;
    let expected = json!({
        "default": {
            "key_global": "value_global"
        },
        "section1": {
            "key1": "value1"
        }
    });
    let parsed = parse_ini(ini_content).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_xml() {
    let xml_content = r#"
<root>
    <item id="1">value1</item>
    <item id="2">value2</item>
</root>
"#;
    let expected = json!({
        "item": {
            "$text": "value2",
            "@id": "2"
        }
    });
    let parsed = parse_xml(xml_content).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_xml_single_element() {
    let xml_content = r#"
<data>
    <name>test</name>
</data>
"#;
    let expected = json!({
        "name": {
            "$text": "test"
        }
    });
    let parsed = parse_xml(xml_content).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_csv_with_headers() {
    let csv_content = "header1,header2\nvalueA,valueB\nvalueC,valueD";
    let expected = json!([
        {"header1": "valueA", "header2": "valueB"},
        {"header1": "valueC", "header2": "valueD"}
    ]);
    let parsed = parse_csv(csv_content).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_csv_no_headers() {
    let csv_content = "valueA,valueB\nvalueC,valueD";
    let expected = json!([
        {"valueA": "valueC", "valueB": "valueD"}
    ]);
    let parsed = parse_csv(csv_content).unwrap();
    assert_eq!(parsed, expected);
}
