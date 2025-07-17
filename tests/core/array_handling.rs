use diffx_core::*;
use serde_json::json;

#[test]
fn test_diff_array_element_added() {
    let v1 = json!([1, 2]);
    let v2 = json!([1, 2, 3]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Added("[2]".to_string(), json!(3))
    );
}

#[test]
fn test_diff_array_element_removed() {
    let v1 = json!([1, 2, 3]);
    let v2 = json!([1, 2]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Removed("[2]".to_string(), json!(3))
    );
}

#[test]
fn test_diff_array_element_modified() {
    let v1 = json!([1, 2, 3]);
    let v2 = json!([1, 2, 4]);
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Modified("[2]".to_string(), json!(3), json!(4))
    );
}

#[test]
fn test_diff_nested_array_element_modified() {
    let v1 = json!({ "a": [1, 2, 3] });
    let v2 = json!({ "a": [1, 2, 4] });
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Modified("a[2]".to_string(), json!(3), json!(4))
    );
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
    assert!(differences.contains(&DiffResult::Modified(
        "[1].id".to_string(),
        json!(2),
        json!(3)
    )));
    assert!(differences.contains(&DiffResult::Added("[2]".to_string(), json!({"id": 4}))));
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
    assert!(differences.contains(&DiffResult::Modified(
        "[id=2].value".to_string(),
        json!("b"),
        json!("c")
    )));
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
    assert!(differences.contains(&DiffResult::Removed(
        "[id=2]".to_string(),
        json!({"id": 2, "value": "b"})
    )));
    assert!(differences.contains(&DiffResult::Added(
        "[id=3]".to_string(),
        json!({"id": 3, "value": "c"})
    )));
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
    assert!(differences.contains(&DiffResult::Modified(
        "[id=2].data.name".to_string(),
        json!("B"),
        json!("C")
    )));
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
    assert!(differences.contains(&DiffResult::Modified(
        "[1].value".to_string(),
        json!("b"),
        json!("c")
    )));
}