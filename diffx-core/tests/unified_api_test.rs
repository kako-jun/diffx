use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::{json, Value};

#[test]
fn test_unified_api_basic_diff() {
    let old = json!({"a": 1, "b": 2});
    let new = json!({"a": 1, "b": 3});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Modified(path, old_val, new_val) => {
            assert_eq!(path, "b");
            assert_eq!(old_val, &json!(2));
            assert_eq!(new_val, &json!(3));
        }
        _ => panic!("Expected Modified variant"),
    }
}

#[test]
fn test_unified_api_with_options() {
    let old = json!({"name": "test", "value": 10});
    let new = json!({"name": "test", "value": 20});
    
    let options = DiffOptions {
        output_format: Some(diffx_core::OutputFormat::Json),
        show_unchanged: Some(false),
        ..Default::default()
    };
    
    let result = diff(&old, &new, Some(&options)).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Modified(path, old_val, new_val) => {
            assert_eq!(path, "value");
            assert_eq!(old_val, &json!(10));
            assert_eq!(new_val, &json!(20));
        }
        _ => panic!("Expected Modified variant"),
    }
}

#[test]
fn test_unified_api_array_diff() {
    let old = json!([1, 2, 3]);
    let new = json!([1, 2, 4]);
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Modified(path, old_val, new_val) => {
            assert_eq!(path, "[2]");
            assert_eq!(old_val, &json!(3));
            assert_eq!(new_val, &json!(4));
        }
        _ => panic!("Expected Modified variant"),
    }
}

#[test]
fn test_unified_api_added_field() {
    let old = json!({"a": 1});
    let new = json!({"a": 1, "b": 2});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Added(path, value) => {
            assert_eq!(path, "b");
            assert_eq!(value, &json!(2));
        }
        _ => panic!("Expected Added variant"),
    }
}

#[test]
fn test_unified_api_removed_field() {
    let old = json!({"a": 1, "b": 2});
    let new = json!({"a": 1});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Removed(path, value) => {
            assert_eq!(path, "b");
            assert_eq!(value, &json!(2));
        }
        _ => panic!("Expected Removed variant"),
    }
}

#[test]
fn test_unified_api_no_changes() {
    let old = json!({"a": 1, "b": 2});
    let new = json!({"a": 1, "b": 2});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_unified_api_nested_object() {
    let old = json!({"user": {"name": "John", "age": 30}});
    let new = json!({"user": {"name": "John", "age": 31}});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::Modified(path, old_val, new_val) => {
            assert_eq!(path, "user.age");
            assert_eq!(old_val, &json!(30));
            assert_eq!(new_val, &json!(31));
        }
        _ => panic!("Expected Modified variant"),
    }
}

#[test]
fn test_unified_api_type_change() {
    let old = json!({"value": 42});
    let new = json!({"value": "42"});
    
    let result = diff(&old, &new, None).unwrap();
    
    assert_eq!(result.len(), 1);
    match &result[0] {
        DiffResult::TypeChanged(path, old_val, new_val) => {
            assert_eq!(path, "value");
            assert_eq!(old_val, &json!(42));
            assert_eq!(new_val, &json!("42"));
        }
        _ => panic!("Expected TypeChanged variant"),
    }
}