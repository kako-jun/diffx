use diffx_core::*;
use regex::Regex;
use serde_json::json;

#[test]
fn test_diff_ignore_keys_regex() {
    let v1 = json!({ "id": 1, "name": "Alice", "_timestamp": "abc" });
    let v2 = json!({ "id": 2, "name": "Alice", "_timestamp": "def" });
    let regex = Regex::new(r"^_.*").unwrap();
    let mut options = DiffOptions::default();
    options.ignore_keys_regex = Some(regex);
    let differences = diff(&v1, &v2, Some(&options)).unwrap();
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified("id".to_string(), json!(1), json!(2))));

    let v3 = json!({ "id": 1, "name": "Alice", "version": "1.0" });
    let v4 = json!({ "id": 1, "name": "Bob", "version": "1.1" });
    let regex_name = Regex::new(r"^name$").unwrap();
    let mut options_name = DiffOptions::default();
    options_name.ignore_keys_regex = Some(regex_name);
    let differences_name = diff(&v3, &v4, Some(&options_name)).unwrap();
    assert_eq!(differences_name.len(), 1);
    assert!(differences_name.contains(&DiffResult::Modified(
        "version".to_string(),
        json!("1.0"),
        json!("1.1")
    )));
}

#[test]
fn test_diff_ignore_keys_regex_nested() {
    let v1 = json!({ "data": { "id": 1, "_timestamp": "abc" } });
    let v2 = json!({ "data": { "id": 2, "_timestamp": "def" } });
    let regex = Regex::new(r"^_.*").unwrap();
    let mut options = DiffOptions::default();
    options.ignore_keys_regex = Some(regex);
    let differences = diff(&v1, &v2, Some(&options)).unwrap();
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified(
        "data.id".to_string(),
        json!(1),
        json!(2)
    )));
}

#[test]
fn test_diff_epsilon_comparison() {
    let v1 = json!({ "a": 1.0, "b": 2.000001 });
    let v2 = json!({ "a": 1.0, "b": 2.000002 });
    let epsilon = Some(0.00001);
    let mut options = DiffOptions::default();
    options.epsilon = epsilon;
    let differences = diff(&v1, &v2, Some(&options)).unwrap();
    assert!(differences.is_empty());

    let v3 = json!({ "a": 1.0, "b": 2.00001 });
    let v4 = json!({ "a": 1.0, "b": 2.00003 });
    let epsilon_large = Some(0.00001);
    let mut options_large = DiffOptions::default();
    options_large.epsilon = epsilon_large;
    let differences_large = diff(&v3, &v4, Some(&options_large)).unwrap();
    assert_eq!(differences_large.len(), 1);
    assert_eq!(
        differences_large[0],
        DiffResult::Modified("b".to_string(), json!(2.00001), json!(2.00003))
    );
}

#[test]
fn test_diff_epsilon_comparison_type_mismatch() {
    let v1 = json!({ "a": 1.0 });
    let v2 = json!({ "a": "1.0" });
    let epsilon = Some(0.00001);
    let mut options = DiffOptions::default();
    options.epsilon = epsilon;
    let differences = diff(&v1, &v2, Some(&options)).unwrap();
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::TypeChanged("a".to_string(), json!(1.0), json!("1.0"))
    );
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
    let mut options = DiffOptions::default();
    options.epsilon = epsilon;
    options.array_id_key = Some("id".to_string());
    let differences = diff(&v1, &v2, Some(&options)).unwrap();
    assert!(differences.is_empty());
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
    let differences = diff(&v1, &v2, None).unwrap();
    assert_eq!(differences.len(), 4);
    assert!(differences.contains(&DiffResult::Modified(
        "config.users[1].name".to_string(),
        json!("Bob"),
        json!("Robert")
    )));
    assert!(differences.contains(&DiffResult::Added(
        "config.users[2]".to_string(),
        json!({"id": 3, "name": "Charlie"})
    )));
    assert!(differences.contains(&DiffResult::Modified(
        "config.settings.theme".to_string(),
        json!("dark"),
        json!("light")
    )));
    assert!(differences.contains(&DiffResult::Added(
        "config.settings.font_size".to_string(),
        json!(12)
    )));
}
