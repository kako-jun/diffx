use serde::Serialize;
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, Value, Value),
}

pub fn diff(
    v1: &Value,
    v2: &Value,
) -> Vec<DiffResult> {
    let mut results = Vec::new();

    // Handle root level type or value change first
    if v1 != v2 {
        let type_match = match (v1, v2) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(_), Value::Bool(_)) => true,
            (Value::Number(_), Value::Number(_)) => true,
            (Value::String(_), Value::String(_)) => true,
            (Value::Array(_), Value::Array(_)) => true,
            (Value::Object(_), Value::Object(_)) => true,
            _ => false,
        };

        if !type_match {
            results.push(DiffResult::TypeChanged("".to_string(), v1.clone(), v2.clone()));
            return results; // If root type changed, no further diffing needed
        } else if v1.is_object() && v2.is_object() {
            diff_objects("", v1.as_object().unwrap(), v2.as_object().unwrap(), &mut results);
        } else if v1.is_array() && v2.is_array() {
            diff_arrays("", v1.as_array().unwrap(), v2.as_array().unwrap(), &mut results);
        } else {
            // Simple value modification at root
            results.push(DiffResult::Modified("".to_string(), v1.clone(), v2.clone()));
            return results;
        }
    }

    results
}

fn diff_recursive(
    path: &str,
    v1: &Value,
    v2: &Value,
    results: &mut Vec<DiffResult>,
) {
    match (v1, v2) {
        (Value::Object(map1), Value::Object(map2)) => {
            diff_objects(path, map1, map2, results);
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            diff_arrays(path, arr1, arr2, results);
        }
        _ => { /* Should not happen if called correctly from diff_objects/diff_arrays */ }
    }
}

fn diff_objects(
    path: &str,
    map1: &serde_json::Map<String, Value>,
    map2: &serde_json::Map<String, Value>,
    results: &mut Vec<DiffResult>,
) {
    // Check for modified or removed keys
    for (key, value1) in map1 {
        let current_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
        match map2.get(key) {
            Some(value2) => {
                // Recurse for nested objects/arrays
                if value1.is_object() && value2.is_object() || value1.is_array() && value2.is_array() {
                    diff_recursive(&current_path, value1, value2, results);
                } else if value1 != value2 {
                    let type_match = match (value1, value2) {
                        (Value::Null, Value::Null) => true,
                        (Value::Bool(_), Value::Bool(_)) => true,
                        (Value::Number(_), Value::Number(_)) => true,
                        (Value::String(_), Value::String(_)) => true,
                        (Value::Array(_), Value::Array(_)) => true,
                        (Value::Object(_), Value::Object(_)) => true,
                        _ => false,
                    };

                    if !type_match {
                        results.push(DiffResult::TypeChanged(current_path, value1.clone(), value2.clone()));
                    } else {
                        results.push(DiffResult::Modified(current_path, value1.clone(), value2.clone()));
                    }
                }
            }
            None => {
                results.push(DiffResult::Removed(current_path, value1.clone()));
            }
        }
    }

    // Check for added keys
    for (key, value2) in map2 {
        if !map1.contains_key(key) {
            let current_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
            results.push(DiffResult::Added(current_path, value2.clone()));
        }
    }
}

fn diff_arrays(
    path: &str,
    arr1: &Vec<Value>,
    arr2: &Vec<Value>,
    results: &mut Vec<DiffResult>,
) {
    let max_len = arr1.len().max(arr2.len());
    for i in 0..max_len {
        let current_path = format!("{}[{}]", path, i);
        match (arr1.get(i), arr2.get(i)) {
            (Some(val1), Some(val2)) => {
                // Recurse for nested objects/arrays within arrays
                if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                    diff_recursive(&current_path, val1, val2, results);
                } else if val1 != val2 {
                    let type_match = match (val1, val2) {
                        (Value::Null, Value::Null) => true,
                        (Value::Bool(_), Value::Bool(_)) => true,
                        (Value::Number(_), Value::Number(_)) => true,
                        (Value::String(_), Value::String(_)) => true,
                        (Value::Array(_), Value::Array(_)) => true,
                        (Value::Object(_), Value::Object(_)) => true,
                        _ => false,
                    };

                    if !type_match {
                        results.push(DiffResult::TypeChanged(current_path, val1.clone(), val2.clone()));
                    } else {
                        results.push(DiffResult::Modified(current_path, val1.clone(), val2.clone()));
                    }
                }
            }
            (Some(val1), None) => {
                results.push(DiffResult::Removed(current_path, val1.clone()));
            }
            (None, Some(val2)) => {
                results.push(DiffResult::Added(current_path, val2.clone()));
            }
            (None, None) => { /* Should not happen */ }
        }
    }
}

pub fn value_type_name(value: &Value) -> &str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_diff_no_changes() {
        let v1 = json!({ "a": 1, "b": 2 });
        let v2 = json!({ "a": 1, "b": 2 });
        let differences = diff(&v1, &v2);
        assert!(differences.is_empty());
    }

    #[test]
    fn test_diff_value_modified() {
        let v1 = json!({ "a": 1, "b": 2 });
        let v2 = json!({ "a": 1, "b": 3 });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Modified("b".to_string(), json!(2), json!(3)));
    }

    #[test]
    fn test_diff_key_added() {
        let v1 = json!({ "a": 1 });
        let v2 = json!({ "a": 1, "b": 2 });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Added("b".to_string(), json!(2)));
    }

    #[test]
    fn test_diff_key_removed() {
        let v1 = json!({ "a": 1, "b": 2 });
        let v2 = json!({ "a": 1 });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Removed("b".to_string(), json!(2)));
    }

    #[test]
    fn test_diff_type_changed() {
        let v1 = json!({ "a": 1 });
        let v2 = json!({ "a": "1" });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::TypeChanged("a".to_string(), json!(1), json!("1")));
    }

    #[test]
    fn test_diff_nested_object_modified() {
        let v1 = json!({ "a": { "b": 1 } });
        let v2 = json!({ "a": { "b": 2 } });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Modified("a.b".to_string(), json!(1), json!(2)));
    }

    #[test]
    fn test_diff_array_element_added() {
        let v1 = json!([1, 2]);
        let v2 = json!([1, 2, 3]);
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Added("[2]".to_string(), json!(3)));
    }

    #[test]
    fn test_diff_array_element_removed() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([1, 2]);
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Removed("[2]".to_string(), json!(3)));
    }

    #[test]
    fn test_diff_array_element_modified() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([1, 2, 4]);
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Modified("[2]".to_string(), json!(3), json!(4)));
    }

    #[test]
    fn test_diff_nested_array_element_modified() {
        let v1 = json!({ "a": [1, 2, 3] });
        let v2 = json!({ "a": [1, 2, 4] });
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 1);
        assert_eq!(differences[0], DiffResult::Modified("a[2]".to_string(), json!(3), json!(4)));
    }

    #[test]
    fn test_diff_root_type_changed() {
        let v1 = json!(1);
        let v2 = json!("1");
        let differences = diff(&v1, &v2);
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
        let differences = diff(&v1, &v2);
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
        let differences = diff(&v1, &v2);
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
        let differences = diff(&v1, &v2);
        assert_eq!(differences.len(), 2);
        assert!(differences.contains(&DiffResult::Modified("[1].id".to_string(), json!(2), json!(3))));
        assert!(differences.contains(&DiffResult::Added("[2]".to_string(), json!({"id": 4}))));
    }
}