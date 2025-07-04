use diffx_core::*;
use serde_json::json;
use regex::Regex;

#[test]
fn test_diff_no_changes() {
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 1, "b": 2 });
    let differences = diff(&v1, &v2, None, None, None);
    assert!(differences.is_empty());
}

#[test]
fn test_diff_value_modified() {
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 1, "b": 3 });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Modified("b".to_string(), json!(2), json!(3)));
}

#[test]
fn test_diff_key_added() {
    let v1 = json!({ "a": 1 });
    let v2 = json!({ "a": 1, "b": 2 });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Added("b".to_string(), json!(2)));
}

#[test]
fn test_diff_key_removed() {
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 1 });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Removed("b".to_string(), json!(2)));
}

#[test]
fn test_diff_type_changed() {
    let v1 = json!({ "a": 1 });
    let v2 = json!({ "a": "1" });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::TypeChanged("a".to_string(), json!(1), json!("1")));
}

#[test]
fn test_diff_nested_object_modified() {
    let v1 = json!({ "a": { "b": 1 } });
    let v2 = json!({ "a": { "b": 2 } });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Modified("a.b".to_string(), json!(1), json!(2)));
}

#[test]
fn test_diff_array_element_added() {
    let v1 = json!([1, 2]);
    let v2 = json!([1, 2, 3]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Added("[2]".to_string(), json!(3)));
}

#[test]
fn test_diff_array_element_removed() {
    let v1 = json!([1, 2, 3]);
    let v2 = json!([1, 2]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Removed("[2]".to_string(), json!(3)));
}

#[test]
fn test_diff_array_element_modified() {
    let v1 = json!([1, 2, 3]);
    let v2 = json!([1, 2, 4]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Modified("[2]".to_string(), json!(3), json!(4)));
}

#[test]
fn test_diff_nested_array_element_modified() {
    let v1 = json!({ "a": [1, 2, 3] });
    let v2 = json!({ "a": [1, 2, 4] });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Modified("a[2]".to_string(), json!(3), json!(4)));
}

#[test]
fn test_diff_root_type_changed() {
    let v1 = json!(1);
    let v2 = json!("1");
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::TypeChanged("".to_string(), json!(1), json!("1")));
}

#[test]
fn test_diff_nested_object_and_array() {
    let v1 = json!({
        "config": {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ],
            "settings": {"theme": "dark"}
        }
    });
    let v2 = json!({
        "config": {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Robert"},
                {"id": 3, "name": "Charlie"}
            ],
            "settings": {"theme": "light", "font_size": 12}
        }
    });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 4);
    assert!(differences.contains(&DiffResult::Modified("config.users[1].name".to_string(), json!("Bob"), json!("Robert"))));
    assert!(differences.contains(&DiffResult::Added("config.users[2]".to_string(), json!({"id": 3, "name": "Charlie"}))));
    assert!(differences.contains(&DiffResult::Modified("config.settings.theme".to_string(), json!("dark"), json!("light"))));
    assert!(differences.contains(&DiffResult::Added("config.settings.font_size".to_string(), json!(12))));
}

#[test]
fn test_diff_empty_objects_and_arrays() {
    let v1 = json!({
        "empty_obj": {},
        "empty_arr": [],
        "data": "value"
    });
    let v2 = json!({
        "empty_obj": {},
        "empty_arr": [],
        "data": "new_value"
    });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::Modified("data".to_string(), json!("value"), json!("new_value")));
}

#[test]
fn test_diff_root_array_changes() {
    let v1 = json!([
        {"id": 1},
        {"id": 2}
    ]);
    let v2 = json!([
        {"id": 1},
        {"id": 3},
        {"id": 4}
    ]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 2);
    assert!(differences.contains(&DiffResult::Modified("[1].id".to_string(), json!(2), json!(3))));
    assert!(differences.contains(&DiffResult::Added("[2]".to_string(), json!({"id": 4}))));
}

#[test]
fn test_diff_ignore_keys_regex() {
    let v1 = json!({ "id": 1, "name": "Alice", "_timestamp": "abc" });
    let v2 = json!({ "id": 2, "name": "Alice", "_timestamp": "def" });
    let regex = Regex::new(r"^_.*").unwrap();
    let differences = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("id".to_string(), json!(1), json!(2))));

    let v3 = json!({ "id": 1, "name": "Alice", "version": "1.0" });
    let v4 = json!({ "id": 1, "name": "Bob", "version": "1.1" });
    let regex_name = Regex::new(r"^name$").unwrap();
    let differences_name = diff(&v3, &v4, Some(&regex_name), None, None);
    assert_eq!(differences_name.len(), 1);
    assert!(differences_name.contains(&DiffResult::Modified("version".to_string(), json!("1.0"), json!("1.1"))));
}

#[test]
fn test_diff_ignore_keys_regex_nested() {
    let v1 = json!({ "data": { "id": 1, "_timestamp": "abc" } });
    let v2 = json!({ "data": { "id": 2, "_timestamp": "def" } });
    let regex = Regex::new(r"^_.*").unwrap();
    let differences = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("data.id".to_string(), json!(1), json!(2))));
}

#[test]
fn test_diff_epsilon_comparison() {
    let v1 = json!({ "a": 1.0, "b": 2.000001 });
    let v2 = json!({ "a": 1.0, "b": 2.000002 });
    let epsilon = Some(0.00001);
    let differences = diff(&v1, &v2, None, epsilon, None);
    assert!(differences.is_empty());

    let v3 = json!({ "a": 1.0, "b": 2.00001 });
    let v4 = json!({ "a": 1.0, "b": 2.00003 });
    let epsilon_large = Some(0.00001);
    let differences_large = diff(&v3, &v4, None, epsilon_large, None);
    assert_eq!(differences_large.len(), 1);
    assert_eq!(differences_large[0], DiffResult::Modified("b".to_string(), json!(2.00001), json!(2.00003)));
}

#[test]
fn test_diff_epsilon_comparison_type_mismatch() {
    let v1 = json!({ "a": 1.0 });
    let v2 = json!({ "a": "1.0" });
    let epsilon = Some(0.00001);
    let differences = diff(&v1, &v2, None, epsilon, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0], DiffResult::TypeChanged("a".to_string(), json!(1.0), json!("1.0")));
}

#[test]
fn test_diff_array_id_key_modified() {
    let v1 = json!([
        {"id": 1, "value": "a"},
        {"id": 2, "value": "b"}
    ]);
    let v2 = json!([
        {"id": 2, "value": "c"},
        {"id": 1, "value": "a"}
    ]);
    let differences = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("[id=2].value".to_string(), json!("b"), json!("c"))));
}

#[test]
fn test_diff_array_id_key_added_removed() {
    let v1 = json!([
        {"id": 1, "value": "a"},
        {"id": 2, "value": "b"}
    ]);
    let v2 = json!([
        {"id": 1, "value": "a"},
        {"id": 3, "value": "c"}
    ]);
    let differences = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(differences.len(), 2);
    assert!(differences.contains(&DiffResult::Removed("[id=2]".to_string(), json!({"id": 2, "value": "b"}))));
    assert!(differences.contains(&DiffResult::Added("[id=3]".to_string(), json!({"id": 3, "value": "c"}))));
}

#[test]
fn test_diff_array_id_key_nested_change() {
    let v1 = json!([
        {"id": 1, "data": {"name": "A"}},
        {"id": 2, "data": {"name": "B"}}
    ]);
    let v2 = json!([
        {"id": 2, "data": {"name": "C"}},
        {"id": 1, "data": {"name": "A"}}
    ]);
    let differences = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("[id=2].data.name".to_string(), json!("B"), json!("C"))));
}

#[test]
fn test_diff_array_id_key_no_id_in_element() {
    let v1 = json!([
        {"id": 1, "value": "a"},
        {"value": "b"}
    ]);
    let v2 = json!([
        {"id": 1, "value": "a"},
        {"value": "c"}
    ]);
    // Elements without the id_key should be compared by index
    let differences = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("[1].value".to_string(), json!("b"), json!("c"))));
}

#[test]
fn test_diff_array_id_key_with_epsilon() {
    let v1 = json!([
        {"id": 1, "value": 1.000001},
        {"id": 2, "value": 2.0}
    ]);
    let v2 = json!([
        {"id": 1, "value": 1.000002},
        {"id": 2, "value": 2.0}
    ]);
    let epsilon = Some(0.00001);
    let differences = diff(&v1, &v2, None, epsilon, Some("id"));
    assert!(differences.is_empty());
}

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