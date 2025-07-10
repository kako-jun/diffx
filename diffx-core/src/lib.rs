use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
// use ini::Ini;
use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use quick_xml::de::from_str;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
// Removed ProgressReporter - Unix tools should be pipe-friendly

#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, Value, Value),
}

/// Lightweight diff result that doesn't clone values unnecessarily
#[derive(Debug, PartialEq, Serialize)]
pub enum LightweightDiffResult {
    Added(String, String),               // path, serialized value
    Removed(String, String),             // path, serialized value
    Modified(String, String, String),    // path, old_value, new_value
    TypeChanged(String, String, String), // path, old_value, new_value
}

impl From<&DiffResult> for LightweightDiffResult {
    fn from(diff: &DiffResult) -> Self {
        match diff {
            DiffResult::Added(path, value) => {
                LightweightDiffResult::Added(path.clone(), value.to_string())
            }
            DiffResult::Removed(path, value) => {
                LightweightDiffResult::Removed(path.clone(), value.to_string())
            }
            DiffResult::Modified(path, old, new) => {
                LightweightDiffResult::Modified(path.clone(), old.to_string(), new.to_string())
            }
            DiffResult::TypeChanged(path, old, new) => {
                LightweightDiffResult::TypeChanged(path.clone(), old.to_string(), new.to_string())
            }
        }
    }
}

/// Configuration for diff operations - essential options only
#[derive(Debug, Clone)]
pub struct DiffConfig {
    pub ignore_keys_regex: Option<regex::Regex>,
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub use_memory_optimization: bool, // Explicit choice
    pub batch_size: usize,
    pub ignore_whitespace: bool,
    pub ignore_case: bool,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            ignore_keys_regex: None,
            epsilon: None,
            array_id_key: None,
            use_memory_optimization: false, // Conservative default
            batch_size: 1000,
            ignore_whitespace: false,
            ignore_case: false,
        }
    }
}

// Removed estimate_item_count - no longer needed without progress reporting

/// Standard diff function - predictable, no automatic optimization
pub fn diff_standard(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult> {
    diff_standard_implementation(
        v1,
        v2,
        ignore_keys_regex,
        epsilon,
        array_id_key,
        false,
        false,
    )
}

/// Standard diff function with configuration support
pub fn diff_standard_with_config(v1: &Value, v2: &Value, config: &DiffConfig) -> Vec<DiffResult> {
    diff_standard_implementation(
        v1,
        v2,
        config.ignore_keys_regex.as_ref(),
        config.epsilon,
        config.array_id_key.as_deref(),
        config.ignore_whitespace,
        config.ignore_case,
    )
}

/// Standard diff function - clean, predictable output
fn diff_standard_implementation(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
) -> Vec<DiffResult> {
    let mut results = Vec::new();

    // Handle root level type or value change first
    if !values_are_equal_with_config(v1, v2, epsilon, ignore_whitespace, ignore_case) {
        let type_match = matches!(
            (v1, v2),
            (Value::Null, Value::Null)
                | (Value::Bool(_), Value::Bool(_))
                | (Value::Number(_), Value::Number(_))
                | (Value::String(_), Value::String(_))
                | (Value::Array(_), Value::Array(_))
                | (Value::Object(_), Value::Object(_))
        );

        if !type_match {
            results.push(DiffResult::TypeChanged(
                "".to_string(),
                v1.clone(),
                v2.clone(),
            ));
            return results;
        } else if v1.is_object() && v2.is_object() {
            diff_objects(
                "",
                v1.as_object().unwrap(),
                v2.as_object().unwrap(),
                &mut results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
                ignore_whitespace,
                ignore_case,
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
                ignore_whitespace,
                ignore_case,
            );
        } else {
            results.push(DiffResult::Modified("".to_string(), v1.clone(), v2.clone()));
        }
    }

    results
}

/// Memory-optimized diff function - explicitly requested optimization
pub fn diff_optimized(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult> {
    let mut results = Vec::new();
    memory_efficient_diff(
        v1,
        v2,
        &mut results,
        ignore_keys_regex,
        epsilon,
        array_id_key,
        false,
        false,
    );
    results
}

/// Memory-optimized diff function with configuration support
pub fn diff_optimized_with_config(v1: &Value, v2: &Value, config: &DiffConfig) -> Vec<DiffResult> {
    let mut results = Vec::new();
    memory_efficient_diff(
        v1,
        v2,
        &mut results,
        config.ignore_keys_regex.as_ref(),
        config.epsilon,
        config.array_id_key.as_deref(),
        config.ignore_whitespace,
        config.ignore_case,
    );
    results
}

/// Enhanced diff function with explicit configuration
pub fn diff_with_config(v1: &Value, v2: &Value, config: &DiffConfig) -> Vec<DiffResult> {
    // Explicit choice: user decides which algorithm to use
    if config.use_memory_optimization {
        diff_optimized_with_config(v1, v2, config)
    } else {
        diff_standard_with_config(v1, v2, config)
    }
}

/// Backward compatible diff function - uses standard algorithm
pub fn diff(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult> {
    // Always use standard algorithm for predictable behavior
    diff_standard(v1, v2, ignore_keys_regex, epsilon, array_id_key)
}

#[allow(clippy::too_many_arguments)]
fn diff_recursive(
    path: &str,
    v1: &Value,
    v2: &Value,
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
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
                ignore_whitespace,
                ignore_case,
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
                ignore_whitespace,
                ignore_case,
            );
        }
        _ => { /* Should not happen if called correctly from diff_objects/diff_arrays */ }
    }
}

#[allow(clippy::too_many_arguments)]
fn diff_objects(
    path: &str,
    map1: &serde_json::Map<String, Value>,
    map2: &serde_json::Map<String, Value>,
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
) {
    // Check for modified or removed keys
    for (key, value1) in map1 {
        let current_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{path}.{key}")
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
                        ignore_whitespace,
                        ignore_case,
                    );
                } else if !values_are_equal_with_config(
                    value1,
                    value2,
                    epsilon,
                    ignore_whitespace,
                    ignore_case,
                ) {
                    let type_match = matches!(
                        (value1, value2),
                        (Value::Null, Value::Null)
                            | (Value::Bool(_), Value::Bool(_))
                            | (Value::Number(_), Value::Number(_))
                            | (Value::String(_), Value::String(_))
                            | (Value::Array(_), Value::Array(_))
                            | (Value::Object(_), Value::Object(_))
                    );

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
                (*key).clone()
            } else {
                format!("{path}.{key}")
            };
            results.push(DiffResult::Added(current_path, value2.clone()));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn diff_arrays(
    path: &str,
    arr1: &[Value],
    arr2: &[Value],
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
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
            let current_path = format!("{path}[{id_key}={id_val}]");
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
                            ignore_whitespace,
                            ignore_case,
                        );
                    } else if !values_are_equal_with_config(
                        val1,
                        val2,
                        epsilon,
                        ignore_whitespace,
                        ignore_case,
                    ) {
                        let type_match = matches!(
                            (val1, val2),
                            (Value::Null, Value::Null)
                                | (Value::Bool(_), Value::Bool(_))
                                | (Value::Number(_), Value::Number(_))
                                | (Value::String(_), Value::String(_))
                                | (Value::Array(_), Value::Array(_))
                                | (Value::Object(_), Value::Object(_))
                        );

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
                let current_path = format!("{path}[{id_key}={id_val}]");
                results.push(DiffResult::Added(current_path, val2.clone()));
            }
        }

        // Handle elements without ID using index-based comparison
        let max_len = no_id_elements1.len().max(no_id_elements2.len());
        for i in 0..max_len {
            match (no_id_elements1.get(i), no_id_elements2.get(i)) {
                (Some((idx1, val1)), Some((_idx2, val2))) => {
                    let current_path = format!("{path}[{idx1}]");
                    if val1.is_object() && val2.is_object() || val1.is_array() && val2.is_array() {
                        diff_recursive(
                            &current_path,
                            val1,
                            val2,
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                            ignore_whitespace,
                            ignore_case,
                        );
                    } else if !values_are_equal_with_config(
                        val1,
                        val2,
                        epsilon,
                        ignore_whitespace,
                        ignore_case,
                    ) {
                        let type_match = matches!(
                            (val1, val2),
                            (Value::Null, Value::Null)
                                | (Value::Bool(_), Value::Bool(_))
                                | (Value::Number(_), Value::Number(_))
                                | (Value::String(_), Value::String(_))
                                | (Value::Array(_), Value::Array(_))
                                | (Value::Object(_), Value::Object(_))
                        );

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
                    let current_path = format!("{path}[{idx1}]");
                    results.push(DiffResult::Removed(current_path, (*val1).clone()));
                }
                (None, Some((idx2, val2))) => {
                    let current_path = format!("{path}[{idx2}]");
                    results.push(DiffResult::Added(current_path, (*val2).clone()));
                }
                (None, None) => break,
            }
        }
    } else {
        // Fallback to index-based comparison if no id_key is provided
        let max_len = arr1.len().max(arr2.len());
        for i in 0..max_len {
            let current_path = format!("{path}[{i}]");
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
                            ignore_whitespace,
                            ignore_case,
                        );
                    } else if !values_are_equal_with_config(
                        val1,
                        val2,
                        epsilon,
                        ignore_whitespace,
                        ignore_case,
                    ) {
                        let type_match = matches!(
                            (val1, val2),
                            (Value::Null, Value::Null)
                                | (Value::Bool(_), Value::Bool(_))
                                | (Value::Number(_), Value::Number(_))
                                | (Value::String(_), Value::String(_))
                                | (Value::Array(_), Value::Array(_))
                                | (Value::Object(_), Value::Object(_))
                        );

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

fn values_are_equal_with_config(
    v1: &Value,
    v2: &Value,
    epsilon: Option<f64>,
    ignore_whitespace: bool,
    ignore_case: bool,
) -> bool {
    // Handle numeric comparisons with epsilon
    if let (Some(e), Value::Number(n1), Value::Number(n2)) = (epsilon, v1, v2) {
        if let (Some(f1), Some(f2)) = (n1.as_f64(), n2.as_f64()) {
            return (f1 - f2).abs() < e;
        }
    }

    // Handle string comparisons with ignore options
    if let (Value::String(s1), Value::String(s2)) = (v1, v2) {
        let mut str1 = s1.as_str();
        let mut str2 = s2.as_str();

        let owned_s1;
        let owned_s2;

        // Apply whitespace normalization if needed
        if ignore_whitespace {
            owned_s1 = normalize_whitespace(str1);
            owned_s2 = normalize_whitespace(str2);
            str1 = &owned_s1;
            str2 = &owned_s2;
        }

        // Apply case normalization if needed
        if ignore_case {
            return str1.to_lowercase() == str2.to_lowercase();
        } else {
            return str1 == str2;
        }
    }

    // Default comparison for all other types
    v1 == v2
}

fn normalize_whitespace(s: &str) -> String {
    // Replace all whitespace sequences with single spaces and trim
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
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

/// Get approximate memory usage of a Value in bytes
pub fn estimate_memory_usage(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 8, // Approximate for f64
        Value::String(s) => s.len(),
        Value::Array(arr) => {
            arr.iter().map(estimate_memory_usage).sum::<usize>() + (arr.len() * 8)
            // Vec overhead
        }
        Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| k.len() + estimate_memory_usage(v))
                .sum::<usize>()
                + (obj.len() * 16) // Map overhead
        }
    }
}

/// Check if processing these values would exceed memory limits
pub fn would_exceed_memory_limit(v1: &Value, v2: &Value) -> bool {
    const MAX_MEMORY_USAGE: usize = 1024 * 1024 * 1024; // 1GB limit

    let usage1 = estimate_memory_usage(v1);
    let usage2 = estimate_memory_usage(v2);

    // Account for diff results and temporary data (multiply by 3)
    (usage1 + usage2) * 3 > MAX_MEMORY_USAGE
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

/// Parse large files with streaming support to reduce memory usage
/// Returns None if file is too large (>100MB) and should use streaming diff
pub fn parse_large_file<P: AsRef<Path>>(path: P) -> Result<Option<Value>> {
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    // 100MB threshold for streaming
    const MAX_MEMORY_SIZE: u64 = 100 * 1024 * 1024;

    if file_size > MAX_MEMORY_SIZE {
        return Ok(None); // Signal that streaming should be used
    }

    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    // Auto-detect format from file extension
    let path_str = path.as_ref().to_string_lossy();
    if path_str.ends_with(".json") {
        Ok(Some(serde_json::from_str(&content)?))
    } else if path_str.ends_with(".yaml") || path_str.ends_with(".yml") {
        Ok(Some(serde_yml::from_str(&content)?))
    } else if path_str.ends_with(".toml") {
        Ok(Some(toml::from_str(&content)?))
    } else {
        Err(anyhow!("Unsupported file format for large file parsing"))
    }
}

/// Memory-efficient diff for large files using streaming approach
pub fn diff_large_files<P: AsRef<Path>>(
    path1: P,
    path2: P,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Result<Vec<DiffResult>> {
    // Try to parse normally first
    let v1_opt = parse_large_file(&path1)?;
    let v2_opt = parse_large_file(&path2)?;

    match (v1_opt, v2_opt) {
        (Some(v1), Some(v2)) => {
            // Both files are small enough for in-memory processing
            Ok(diff(&v1, &v2, ignore_keys_regex, epsilon, array_id_key))
        }
        _ => {
            // At least one file is too large, use streaming diff
            streaming_diff(&path1, &path2, ignore_keys_regex, epsilon, array_id_key)
        }
    }
}

/// Streaming diff implementation for very large files
fn streaming_diff<P: AsRef<Path>>(
    path1: P,
    path2: P,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Result<Vec<DiffResult>> {
    // For now, implement a simplified version that chunks the files
    // This is a placeholder for more sophisticated streaming logic
    let mut results = Vec::new();

    // Read files in chunks and compare
    let file1 = File::open(&path1)?;
    let file2 = File::open(&path2)?;

    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    // Read entire files (for now - this would be optimized further)
    reader1.read_to_string(&mut buffer1)?;
    reader2.read_to_string(&mut buffer2)?;

    // Parse with reduced memory footprint
    let v1: Value = serde_json::from_str(&buffer1)
        .or_else(|_| serde_yml::from_str(&buffer1))
        .or_else(|_| toml::from_str(&buffer1))
        .map_err(|e| anyhow!("Failed to parse file 1: {}", e))?;

    let v2: Value = serde_json::from_str(&buffer2)
        .or_else(|_| serde_yml::from_str(&buffer2))
        .or_else(|_| toml::from_str(&buffer2))
        .map_err(|e| anyhow!("Failed to parse file 2: {}", e))?;

    // Clear buffers to free memory
    drop(buffer1);
    drop(buffer2);

    // Use optimized diff with memory-conscious approach
    memory_efficient_diff(
        &v1,
        &v2,
        &mut results,
        ignore_keys_regex,
        epsilon,
        array_id_key,
        false, // ignore_whitespace - not supported in streaming mode
        false, // ignore_case - not supported in streaming mode
    );

    Ok(results)
}

/// Memory-efficient diff implementation that processes data in chunks
#[allow(clippy::too_many_arguments)]
fn memory_efficient_diff(
    v1: &Value,
    v2: &Value,
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
) {
    // Process diff without cloning large values when possible
    if !values_are_equal_with_config(v1, v2, epsilon, ignore_whitespace, ignore_case) {
        let type_match = matches!(
            (v1, v2),
            (Value::Null, Value::Null)
                | (Value::Bool(_), Value::Bool(_))
                | (Value::Number(_), Value::Number(_))
                | (Value::Array(_), Value::Array(_))
                | (Value::Object(_), Value::Object(_))
        );

        if !type_match {
            results.push(DiffResult::TypeChanged(
                "".to_string(),
                v1.clone(),
                v2.clone(),
            ));
        } else if v1.is_object() && v2.is_object() {
            memory_efficient_diff_objects(
                "",
                v1.as_object().unwrap(),
                v2.as_object().unwrap(),
                results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
                ignore_whitespace,
                ignore_case,
            );
        } else if v1.is_array() && v2.is_array() {
            memory_efficient_diff_arrays(
                "",
                v1.as_array().unwrap(),
                v2.as_array().unwrap(),
                results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
                ignore_whitespace,
                ignore_case,
            );
        } else {
            results.push(DiffResult::Modified("".to_string(), v1.clone(), v2.clone()));
        }
    }
}

/// Memory-efficient object comparison
#[allow(clippy::too_many_arguments)]
fn memory_efficient_diff_objects(
    path: &str,
    map1: &serde_json::Map<String, Value>,
    map2: &serde_json::Map<String, Value>,
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
) {
    // Process keys in batches to limit memory usage
    const BATCH_SIZE: usize = 1000;

    let keys1: Vec<_> = map1.keys().collect();
    let keys2: Vec<_> = map2.keys().collect();

    // Process in batches
    for chunk in keys1.chunks(BATCH_SIZE) {
        for key in chunk {
            if let Some(regex) = ignore_keys_regex {
                if regex.is_match(key) {
                    continue;
                }
            }

            let current_path = if path.is_empty() {
                (*key).clone()
            } else {
                format!("{path}.{key}")
            };

            match (map1.get(*key), map2.get(*key)) {
                (Some(value1), Some(value2)) => {
                    if value1.is_object() && value2.is_object() {
                        memory_efficient_diff_objects(
                            &current_path,
                            value1.as_object().unwrap(),
                            value2.as_object().unwrap(),
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                            ignore_whitespace,
                            ignore_case,
                        );
                    } else if value1.is_array() && value2.is_array() {
                        memory_efficient_diff_arrays(
                            &current_path,
                            value1.as_array().unwrap(),
                            value2.as_array().unwrap(),
                            results,
                            ignore_keys_regex,
                            epsilon,
                            array_id_key,
                            ignore_whitespace,
                            ignore_case,
                        );
                    } else if !values_are_equal_with_config(
                        value1,
                        value2,
                        epsilon,
                        ignore_whitespace,
                        ignore_case,
                    ) {
                        let type_match = matches!(
                            (value1, value2),
                            (Value::Null, Value::Null)
                                | (Value::Bool(_), Value::Bool(_))
                                | (Value::Number(_), Value::Number(_))
                                | (Value::String(_), Value::String(_))
                                | (Value::Array(_), Value::Array(_))
                                | (Value::Object(_), Value::Object(_))
                        );

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
                (Some(value1), None) => {
                    results.push(DiffResult::Removed(current_path, value1.clone()));
                }
                (None, Some(_)) => {
                    // Will be handled in the "added" phase
                }
                (None, None) => {
                    // Should not happen
                }
            }
        }
    }

    // Process added keys
    for chunk in keys2.chunks(BATCH_SIZE) {
        for key in chunk {
            if !map1.contains_key(*key) {
                let current_path = if path.is_empty() {
                    (*key).clone()
                } else {
                    format!("{path}.{key}")
                };
                if let Some(value2) = map2.get(*key) {
                    results.push(DiffResult::Added(current_path, value2.clone()));
                }
            }
        }
    }
}

/// Memory-efficient array comparison
#[allow(clippy::too_many_arguments)]
fn memory_efficient_diff_arrays(
    path: &str,
    arr1: &[Value],
    arr2: &[Value],
    results: &mut Vec<DiffResult>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    ignore_whitespace: bool,
    ignore_case: bool,
) {
    // Use the existing array diff logic but with batching for very large arrays
    const BATCH_SIZE: usize = 10000;

    if arr1.len() > BATCH_SIZE || arr2.len() > BATCH_SIZE {
        // Process large arrays in chunks
        let max_len = arr1.len().max(arr2.len());
        for chunk_start in (0..max_len).step_by(BATCH_SIZE) {
            let chunk_end = (chunk_start + BATCH_SIZE).min(max_len);
            let chunk1 = arr1.get(chunk_start..chunk_end).unwrap_or(&[]);
            let chunk2 = arr2.get(chunk_start..chunk_end).unwrap_or(&[]);

            // Process this chunk using existing logic
            diff_arrays(
                path,
                chunk1,
                chunk2,
                results,
                ignore_keys_regex,
                epsilon,
                array_id_key,
                ignore_whitespace,
                ignore_case,
            );
        }
    } else {
        // Use existing implementation for smaller arrays
        diff_arrays(
            path,
            arr1,
            arr2,
            results,
            ignore_keys_regex,
            epsilon,
            array_id_key,
            ignore_whitespace,
            ignore_case,
        );
    }
}

// API is already public
