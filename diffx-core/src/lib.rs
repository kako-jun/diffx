use serde::Serialize;
use serde_json::Value;
use regex::Regex;
use std::collections::HashMap;
// use ini::Ini;
use anyhow::{Result, anyhow};
use quick_xml::de::from_str;
use csv::ReaderBuilder;

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
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult> {
    let mut results = Vec::new();

    // Handle root level type or value change first
    if !values_are_equal(v1, v2, epsilon) {
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
            diff_objects("", v1.as_object().unwrap(), v2.as_object().unwrap(), &mut results, ignore_keys_regex, epsilon, array_id_key);
        } else if v1.is_array() && v2.is_array() {
            diff_arrays("", v1.as_array().unwrap(), v2.as_array().unwrap(), &mut results, ignore_keys_regex, epsilon, array_id_key);
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
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) {
    match (v1, v2) {
        (Value::Object(map1), Value::Object(map2)) => {
            diff_objects(path, map1, map2, results, ignore_keys_regex, epsilon, array_id_key);
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            diff_arrays(path, arr1, arr2, results, ignore_keys_regex, epsilon, array_id_key);
        }
        _ => { /* Should not happen if called correctly from diff_objects/diff_arrays */ }
    }
}

fn diff_objects(
    path: &str,
    map1: &serde_json::Map<String, Value>,
    map2: &serde_json::Map<String, Value>,
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) {
    // Check for modified or removed keys
    for (key, value1) in map1 {
        let current_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
        if let Some(regex) = ignore_keys_regex {
            if regex.is_match(key) {
                continue;
            }
        }
        match map2.get(key) {
            Some(value2) => {
                // Recurse for nested objects/arrays
                if value1.is_object() && value2.is_object() || value1.is_array() && value2.is_array() {
                    diff_recursive(&current_path, value1, value2, results, ignore_keys_regex, epsilon, array_id_key);
                } else if !values_are_equal(value1, value2, epsilon) {
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
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) {
    if let Some(id_key) = array_id_key {
        let mut map1: HashMap<Value, &Value> = HashMap::new();
        let mut no_id_elements1: Vec<(usize, &Value)> = Vec::new();
        for (i, val) in arr1.iter().enumerate() {
            if let Some(id_val) = val.get(id_key) {
                map1.insert(id_val.clone(), val);
            } else {
                no_id_elements1.push((i, val));
            }
        }

        let mut map2: HashMap<Value, &Value> = HashMap::new();
        let mut no_id_elements2: Vec<(usize, &Value)> = Vec::new();
        for (i, val) in arr2.iter().enumerate() {
            if let Some(id_val) = val.get(id_key) {
                map2.insert(id_val.clone(), val);
            } else {
                no_id_elements2.push((i, val));
            }
        }

        // Check for modified or removed elements
        for (id_val, val1) in &map1 {
            let current_path = format!("{}[{}={}]", path, id_key, id_val);
            match map2.get(&id_val) {
                Some(val2) => {
                    // Recurse for nested objects/arrays
                    if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                        diff_recursive(&current_path, val1, val2, results, ignore_keys_regex, epsilon, array_id_key);
                    } else if !values_are_equal(val1, val2, epsilon) {
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
                            results.push(DiffResult::TypeChanged(current_path, (*val1).clone(), (*val2).clone()));
                        } else {
                            results.push(DiffResult::Modified(current_path, (*val1).clone(), (*val2).clone()));
                        }
                    }
                }
                None => {
                    results.push(DiffResult::Removed(current_path, (*val1).clone()));
                }
            }
        }

        // Check for added elements with ID
        for (id_val, val2) in map2 {
            if !map1.contains_key(&id_val) {
                let current_path = format!("{}[{}={}]", path, id_key, id_val);
                results.push(DiffResult::Added(current_path, val2.clone()));
            }
        }

        // Handle elements without ID using index-based comparison
        let max_len = no_id_elements1.len().max(no_id_elements2.len());
        for i in 0..max_len {
            match (no_id_elements1.get(i), no_id_elements2.get(i)) {
                (Some((idx1, val1)), Some((_idx2, val2))) => {
                    let current_path = format!("{}[{}]", path, idx1);
                    if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                        diff_recursive(&current_path, val1, val2, results, ignore_keys_regex, epsilon, array_id_key);
                    } else if !values_are_equal(val1, val2, epsilon) {
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
                            results.push(DiffResult::TypeChanged(current_path, (*val1).clone(), (*val2).clone()));
                        } else {
                            results.push(DiffResult::Modified(current_path, (*val1).clone(), (*val2).clone()));
                        }
                    }
                }
                (Some((idx1, val1)), None) => {
                    let current_path = format!("{}[{}]", path, idx1);
                    results.push(DiffResult::Removed(current_path, (*val1).clone()));
                }
                (None, Some((idx2, val2))) => {
                    let current_path = format!("{}[{}]", path, idx2);
                    results.push(DiffResult::Added(current_path, (*val2).clone()));
                }
                (None, None) => break,
            }
        }
    } else {
        // Fallback to index-based comparison if no id_key is provided
        let max_len = arr1.len().max(arr2.len());
        for i in 0..max_len {
            let current_path = format!("{}[{}]", path, i);
            match (arr1.get(i), arr2.get(i)) {
                (Some(val1), Some(val2)) => {
                    // Recurse for nested objects/arrays within arrays
                    if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                        diff_recursive(&current_path, val1, val2, results, ignore_keys_regex, epsilon, array_id_key);
                    } else if !values_are_equal(val1, val2, epsilon) {
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
}

fn values_are_equal(v1: &Value, v2: &Value, epsilon: Option<f64>) -> bool {
    if let (Some(e), Value::Number(n1), Value::Number(n2)) = (epsilon, v1, v2) {
        if let (Some(f1), Some(f2)) = (n1.as_f64(), n2.as_f64()) {
            return (f1 - f2).abs() < e;
        }
    }
    v1 == v2
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

pub fn parse_ini(content: &str) -> Result<Value> {
    use configparser::ini::Ini;
    
    let mut ini = Ini::new();
    ini.read(content.to_string())
        .map_err(|e| anyhow!("Failed to parse INI: {}", e))?;
    
    let mut root_map = serde_json::Map::new();

    for section_name in ini.sections() {
        let mut section_map = serde_json::Map::new();
        
        if let Some(section) = ini.get_map_ref().get(&section_name) {
            for (key, value) in section {
                if let Some(v) = value {
                    section_map.insert(key.clone(), Value::String(v.clone()));
                } else {
                    section_map.insert(key.clone(), Value::Null);
                }
            }
        }
        
        root_map.insert(section_name, Value::Object(section_map));
    }

    Ok(Value::Object(root_map))
}

pub fn parse_xml(content: &str) -> Result<Value> {
    let value: Value = from_str(content)?;
    Ok(value)
}

pub fn parse_csv(content: &str) -> Result<Value> {
    let mut reader = ReaderBuilder::new().from_reader(content.as_bytes());
    let mut records = Vec::new();

    let headers = reader.headers()?.clone();
    let has_headers = !headers.is_empty();

    for result in reader.into_records() {
        let record = result?;
        if has_headers {
            let mut obj = serde_json::Map::new();
            for (i, header) in headers.iter().enumerate() {
                if let Some(value) = record.get(i) {
                    obj.insert(header.to_string(), Value::String(value.to_string()));
                }
            }
            records.push(Value::Object(obj));
        } else {
            let mut arr = Vec::new();
            for field in record.iter() {
                arr.push(Value::String(field.to_string()));
            }
            records.push(Value::Array(arr));
        }
    }
    Ok(Value::Array(records))
}

#[cfg(test)]
mod tests {
    use super::*;
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
            "key_global": "value_global",
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
            "root": {
                "item": [
                    {
                        "#text": "value1",
                        "@id": "1"
                    },
                    {
                        "#text": "value2",
                        "@id": "2"
                    }
                ]
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
            "data": {
                "name": "test"
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
            ["valueA", "valueB"],
            ["valueC", "valueD"]
        ]);
        let parsed = parse_csv(csv_content).unwrap();
        assert_eq!(parsed, expected);
    }
}
