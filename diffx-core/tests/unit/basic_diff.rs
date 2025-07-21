use diffx_core::*;
use serde_json::json;

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
    assert_eq!(
        differences[0],
        DiffResult::Modified("b".to_string(), json!(2), json!(3))
    );
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
    assert_eq!(
        differences[0],
        DiffResult::Removed("b".to_string(), json!(2))
    );
}

#[test]
fn test_diff_type_changed() {
    let v1 = json!({ "a": 1 });
    let v2 = json!({ "a": "1" });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::TypeChanged("a".to_string(), json!(1), json!("1"))
    );
}

#[test]
fn test_diff_nested_object_modified() {
    let v1 = json!({ "a": { "b": 1 } });
    let v2 = json!({ "a": { "b": 2 } });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Modified("a.b".to_string(), json!(1), json!(2))
    );
}

#[test]
fn test_diff_root_type_changed() {
    let v1 = json!(1);
    let v2 = json!("1");
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::TypeChanged("".to_string(), json!(1), json!("1"))
    );
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
    assert_eq!(
        differences[0],
        DiffResult::Modified("data".to_string(), json!("value"), json!("new_value"))
    );
}
