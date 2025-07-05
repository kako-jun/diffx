use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
// use ini::Ini;
use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use quick_xml::de::from_str;

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
        let type_match = matches!((v1, v2), (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_)) | (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)));

        if !type_match {
            results.push(DiffResult::TypeChanged(
                "".to_string(),
                v1.clone(),
                v2.clone(),
            ));
            return results; // If root type changed, no further diffing needed
        } else if v1.is_object() && v2.is_object() {
            diff_objects(
                "",
                v1.as_object().unwrap(),
                v2.as_object().unwrap(),
                &mut results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
            );
        } else if v1.is_array() && v2.is_array() {
            diff_arrays(
                "",
                v1.as_array().unwrap(),
                v2.as_array().unwrap(),
                &mut results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
            );
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
            diff_objects(
                path,
                map1,
                map2,
                results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
            );
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            diff_arrays(
                path,
                arr1,
                arr2,
                results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
            );
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
        let current_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{}.{}", path, key)
        };
        if let Some(regex) = ignore_keys_regex {
            if regex.is_match(key) {
                continue;
            }
        }
        match map2.get(key) {
            Some(value2) => {
                // Recurse for nested objects/arrays
                if value1.is_object() && value2.is_object()
                    || value1.is_array() && value2.is_array()
                {
                    diff_recursive(
                        &current_path,
                        value1,
                        value2,
                        results,
                        ignore_keys_regex,
                        epsilon,
                        array_id_key,
                    );
                } else if !values_are_equal(value1, value2, epsilon) {
                    let type_match = matches!((value1, value2), (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_)) | (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)));

                    if !type_match {
                        results.push(DiffResult::TypeChanged(
                            current_path,
                            value1.clone(),
                            value2.clone(),
                        ));
                    } else {
                        results.push(DiffResult::Modified(
                            current_path,
                            value1.clone(),
                            value2.clone(),
                        ));
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
            let current_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", path, key)
            };
            results.push(DiffResult::Added(current_path, value2.clone()));
        }
    }
}

fn diff_arrays(
    path: &str,
    arr1: &[Value],
    arr2: &[Value],
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
            match map2.get(id_val) {
                Some(val2) => {
                    // Recurse for nested objects/arrays
                    if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                        diff_recursive(
                            &current_path,
                            val1,
                            val2,
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                        );
                    } else if !values_are_equal(val1, val2, epsilon) {
                        let type_match = matches!((val1, val2), (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_)) | (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)));

                        if !type_match {
                            results.push(DiffResult::TypeChanged(
                                current_path,
                                (*val1).clone(),
                                (*val2).clone(),
                            ));
                        } else {
                            results.push(DiffResult::Modified(
                                current_path,
                                (*val1).clone(),
                                (*val2).clone(),
                            ));
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
                        diff_recursive(
                            &current_path,
                            val1,
                            val2,
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                        );
                    } else if !values_are_equal(val1, val2, epsilon) {
                        let type_match = matches!((val1, val2), (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_)) | (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)));

                        if !type_match {
                            results.push(DiffResult::TypeChanged(
                                current_path,
                                (*val1).clone(),
                                (*val2).clone(),
                            ));
                        } else {
                            results.push(DiffResult::Modified(
                                current_path,
                                (*val1).clone(),
                                (*val2).clone(),
                            ));
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
                        diff_recursive(
                            &current_path,
                            val1,
                            val2,
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                        );
                    } else if !values_are_equal(val1, val2, epsilon) {
                        let type_match = matches!((val1, val2), (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_)) | (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)));

                        if !type_match {
                            results.push(DiffResult::TypeChanged(
                                current_path,
                                val1.clone(),
                                val2.clone(),
                            ));
                        } else {
                            results.push(DiffResult::Modified(
                                current_path,
                                val1.clone(),
                                val2.clone(),
                            ));
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
