use diffx_core::*;
use serde_json::json;

#[test]
fn test_basic_api_usage_from_docs() {
    // This should match API examples shown in documentation
    let v1 = json!({ "name": "Alice", "age": 30 });
    let v2 = json!({ "name": "Alice", "age": 31 });
    let differences = diff(&v1, &v2, None, None, None);
    
    assert_eq!(differences.len(), 1);
    assert_eq!(
        differences[0],
        DiffResult::Modified("age".to_string(), json!(30), json!(31))
    );
}

#[test]
fn test_array_id_key_api_example_from_docs() {
    // This should match array ID key examples in documentation
    let v1 = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]);
    let v2 = json!([
        {"id": 2, "name": "Robert"},
        {"id": 1, "name": "Alice"}
    ]);
    let differences = diff(&v1, &v2, None, None, Some("id"));
    
    assert_eq!(differences.len(), 1);
    assert!(differences.contains(&DiffResult::Modified(
        "[id=2].name".to_string(),
        json!("Bob"),
        json!("Robert")
    )));
}

#[test]
fn test_epsilon_api_example_from_docs() {
    // This should match epsilon examples in documentation
    let v1 = json!({ "temperature": 23.0001 });
    let v2 = json!({ "temperature": 23.0002 });
    let differences = diff(&v1, &v2, None, Some(0.001), None);
    
    assert!(differences.is_empty()); // No differences within epsilon tolerance
}