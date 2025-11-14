// Module declarations
mod types;
mod parser;

// Re-export public APIs
pub use types::*;
pub use parser::*;

use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// ============================================================================
// UNIFIED API - Main Function
// ============================================================================

/// Unified diff function for diffx (path-based entry point)
///
/// This is the main entry point that handles both files and directories automatically.
/// - File vs File: Regular file comparison
/// - Directory vs Directory: Recursive directory comparison  
/// - File vs Directory: Returns error
pub fn diff_paths(
    old_path: &str,
    new_path: &str,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>> {
    use std::path::Path;

    let path1 = Path::new(old_path);
    let path2 = Path::new(new_path);

    match (path1.is_dir(), path2.is_dir()) {
        (true, true) => diff_directories(path1, path2, options),
        (false, false) => diff_files(path1, path2, options),
        (true, false) => Err(anyhow!(
            "Cannot compare directory '{}' with file '{}'",
            old_path,
            new_path
        )),
        (false, true) => Err(anyhow!(
            "Cannot compare file '{}' with directory '{}'",
            old_path,
            new_path
        )),
    }
}

/// Unified diff function for diffx (Value-based)
///
/// This function operates on pre-parsed JSON values.
/// For file/directory operations, use diff_paths() instead.
pub fn diff(old: &Value, new: &Value, options: Option<&DiffOptions>) -> Result<Vec<DiffResult>> {
    let default_options = DiffOptions::default();
    let opts = options.unwrap_or(&default_options);

    // Apply memory optimization if requested
    if opts.use_memory_optimization.unwrap_or(false) {
        diff_optimized_implementation(old, new, opts)
    } else {
        diff_standard_implementation(old, new, opts)
    }
}

fn diff_standard_implementation(
    old: &Value,
    new: &Value,
    options: &DiffOptions,
) -> Result<Vec<DiffResult>> {
    let mut results = Vec::new();
    diff_recursive(old, new, "", &mut results, options);
    Ok(results)
}

fn diff_optimized_implementation(
    old: &Value,
    new: &Value,
    options: &DiffOptions,
) -> Result<Vec<DiffResult>> {
    // Check memory limits
    if would_exceed_memory_limit(old, new) {
        return Err(anyhow!("Input too large for memory optimization"));
    }

    diff_standard_implementation(old, new, options)
}

fn diff_files(
    path1: &Path,
    path2: &Path,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>> {
    // Read file contents
    let content1 = fs::read_to_string(path1)?;
    let content2 = fs::read_to_string(path2)?;

    // Detect formats based on file extensions
    let format1 = detect_format_from_path(path1);
    let format2 = detect_format_from_path(path2);

    // Parse content based on detected formats
    let value1 = parse_content_by_format(&content1, format1)?;
    let value2 = parse_content_by_format(&content2, format2)?;

    // Use existing diff implementation
    diff(&value1, &value2, options)
}

fn diff_directories(
    dir1: &Path,
    dir2: &Path,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>> {
    let mut results = Vec::new();

    // Get all files in both directories recursively
    let files1 = get_all_files_recursive(dir1)?;
    let files2 = get_all_files_recursive(dir2)?;

    // Create maps for easier lookup (relative path -> absolute path)
    let files1_map: HashMap<String, &Path> = files1
        .iter()
        .filter_map(|path| {
            path.strip_prefix(dir1)
                .ok()
                .map(|rel| (rel.to_string_lossy().to_string(), path.as_path()))
        })
        .collect();

    let files2_map: HashMap<String, &Path> = files2
        .iter()
        .filter_map(|path| {
            path.strip_prefix(dir2)
                .ok()
                .map(|rel| (rel.to_string_lossy().to_string(), path.as_path()))
        })
        .collect();

    // Find files that exist in dir1 but not in dir2 (removed)
    for (rel_path, abs_path1) in &files1_map {
        if !files2_map.contains_key(rel_path) {
            let content = fs::read_to_string(abs_path1).unwrap_or_default();
            if let Ok(value) = parse_content_by_format(&content, detect_format_from_path(abs_path1))
            {
                results.push(DiffResult::Removed(rel_path.clone(), value));
            }
        }
    }

    // Find files that exist in dir2 but not in dir1 (added)
    for (rel_path, abs_path2) in &files2_map {
        if !files1_map.contains_key(rel_path) {
            let content = fs::read_to_string(abs_path2).unwrap_or_default();
            if let Ok(value) = parse_content_by_format(&content, detect_format_from_path(abs_path2))
            {
                results.push(DiffResult::Added(rel_path.clone(), value));
            }
        }
    }

    // Find files that exist in both directories (compare contents)
    for (rel_path, abs_path1) in &files1_map {
        if let Some(abs_path2) = files2_map.get(rel_path) {
            match diff_files(abs_path1, abs_path2, options) {
                Ok(mut file_results) => {
                    // Prefix all paths with the relative path
                    for result in &mut file_results {
                        match result {
                            DiffResult::Added(path, _) => *path = format!("{rel_path}/{path}"),
                            DiffResult::Removed(path, _) => *path = format!("{rel_path}/{path}"),
                            DiffResult::Modified(path, _, _) => {
                                *path = format!("{rel_path}/{path}")
                            }
                            DiffResult::TypeChanged(path, _, _) => {
                                *path = format!("{rel_path}/{path}")
                            }
                        }
                    }
                    results.extend(file_results);
                }
                Err(_) => {
                    // If file comparison fails, skip this file
                    continue;
                }
            }
        }
    }

    Ok(results)
}

fn get_all_files_recursive(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                files.extend(get_all_files_recursive(&path)?);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}

// Helper function to add result with path filtering
fn add_diff_result(result: DiffResult, results: &mut Vec<DiffResult>, options: &DiffOptions) {
    // Apply path filter if specified
    if let Some(filter) = &options.path_filter {
        let path = match &result {
            DiffResult::Added(path, _) => path,
            DiffResult::Removed(path, _) => path,
            DiffResult::Modified(path, _, _) => path,
            DiffResult::TypeChanged(path, _, _) => path,
        };
        if !path.contains(filter) {
            return;
        }
    }
    results.push(result);
}

fn diff_recursive(
    old: &Value,
    new: &Value,
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    match (old, new) {
        (Value::Object(old_obj), Value::Object(new_obj)) => {
            diff_objects(old_obj, new_obj, path, results, options);
        }
        (Value::Array(old_arr), Value::Array(new_arr)) => {
            diff_arrays(old_arr, new_arr, path, results, options);
        }
        (Value::Number(old_num), Value::Number(new_num)) => {
            if let Some(epsilon) = options.epsilon {
                let old_f = old_num.as_f64().unwrap_or(0.0);
                let new_f = new_num.as_f64().unwrap_or(0.0);
                if (old_f - new_f).abs() > epsilon {
                    add_diff_result(
                        DiffResult::Modified(path.to_string(), old.clone(), new.clone()),
                        results,
                        options,
                    );
                }
            } else if old != new {
                add_diff_result(
                    DiffResult::Modified(path.to_string(), old.clone(), new.clone()),
                    results,
                    options,
                );
            }
        }
        (Value::String(old_str), Value::String(new_str)) => {
            let mut old_processed = old_str.clone();
            let mut new_processed = new_str.clone();

            // Apply string transformations based on options
            if let Some(diffx_opts) = &options.diffx_options {
                if diffx_opts.ignore_whitespace.unwrap_or(false) {
                    old_processed = old_processed
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    new_processed = new_processed
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                }
                if diffx_opts.ignore_case.unwrap_or(false) {
                    old_processed = old_processed.to_lowercase();
                    new_processed = new_processed.to_lowercase();
                }
            }

            if old_processed != new_processed {
                add_diff_result(
                    DiffResult::Modified(path.to_string(), old.clone(), new.clone()),
                    results,
                    options,
                );
            }
        }
        _ => {
            if old != new {
                if old.type_name() != new.type_name() {
                    add_diff_result(
                        DiffResult::TypeChanged(path.to_string(), old.clone(), new.clone()),
                        results,
                        options,
                    );
                } else {
                    // For other types, just do regular comparison
                    add_diff_result(
                        DiffResult::Modified(path.to_string(), old.clone(), new.clone()),
                        results,
                        options,
                    );
                }
            }
        }
    }
}

fn diff_objects(
    old_obj: &serde_json::Map<String, Value>,
    new_obj: &serde_json::Map<String, Value>,
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    // Handle ignore_keys_regex
    let should_ignore_key = |key: &str| -> bool {
        if let Some(regex) = &options.ignore_keys_regex {
            regex.is_match(key)
        } else {
            false
        }
    };

    // Check for removed keys
    for (key, old_value) in old_obj {
        if should_ignore_key(key) {
            continue;
        }

        let new_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{path}.{key}")
        };

        if !new_obj.contains_key(key) {
            add_diff_result(
                DiffResult::Removed(new_path, old_value.clone()),
                results,
                options,
            );
        }
    }

    // Check for added and modified keys
    for (key, new_value) in new_obj {
        if should_ignore_key(key) {
            continue;
        }

        let new_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{path}.{key}")
        };

        match old_obj.get(key) {
            None => {
                add_diff_result(
                    DiffResult::Added(new_path, new_value.clone()),
                    results,
                    options,
                );
            }
            Some(old_value) => {
                diff_recursive(old_value, new_value, &new_path, results, options);
            }
        }
    }
}

fn diff_arrays(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    if let Some(id_key) = &options.array_id_key {
        diff_arrays_with_id(old_arr, new_arr, path, results, options, id_key);
    } else {
        diff_arrays_by_index(old_arr, new_arr, path, results, options);
    }
}

fn diff_arrays_with_id(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
    id_key: &str,
) {
    let mut old_by_id: HashMap<String, (usize, &Value)> = HashMap::new();
    let mut new_by_id: HashMap<String, (usize, &Value)> = HashMap::new();
    let mut old_without_id: Vec<(usize, &Value)> = Vec::new();
    let mut new_without_id: Vec<(usize, &Value)> = Vec::new();

    // Separate items with IDs from those without
    for (index, item) in old_arr.iter().enumerate() {
        if let Some(id_value) = item.get(id_key) {
            let id_str = match id_value {
                Value::String(s) => format!("\"{s}\""), // Add quotes for strings
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => format!("{id_value:?}"),
            };
            old_by_id.insert(id_str, (index, item));
        } else {
            old_without_id.push((index, item));
        }
    }

    for (index, item) in new_arr.iter().enumerate() {
        if let Some(id_value) = item.get(id_key) {
            let id_str = match id_value {
                Value::String(s) => format!("\"{s}\""), // Add quotes for strings
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => format!("{id_value:?}"),
            };
            new_by_id.insert(id_str, (index, item));
        } else {
            new_without_id.push((index, item));
        }
    }

    // Handle items with IDs
    // Find removed items
    for (id, (_, old_item)) in &old_by_id {
        if !new_by_id.contains_key(id) {
            let item_path = if path.is_empty() {
                format!("[{id_key}={id}]")
            } else {
                format!("{path}[{id_key}={id}]")
            };
            results.push(DiffResult::Removed(item_path, (*old_item).clone()));
        }
    }

    // Find added and modified items with IDs
    for (id, (_, new_item)) in &new_by_id {
        let item_path = if path.is_empty() {
            format!("[{id_key}={id}]")
        } else {
            format!("{path}[{id_key}={id}]")
        };

        match old_by_id.get(id) {
            None => {
                results.push(DiffResult::Added(item_path, (*new_item).clone()));
            }
            Some((_, old_item)) => {
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
        }
    }

    // Handle items without IDs by index
    let max_len = old_without_id.len().max(new_without_id.len());
    for i in 0..max_len {
        match (old_without_id.get(i), new_without_id.get(i)) {
            (Some((old_index, old_item)), Some((_, new_item))) => {
                let item_path = if path.is_empty() {
                    format!("[{old_index}]")
                } else {
                    format!("{path}[{old_index}]")
                };
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
            (Some((old_index, old_item)), None) => {
                let item_path = if path.is_empty() {
                    format!("[{old_index}]")
                } else {
                    format!("{path}[{old_index}]")
                };
                results.push(DiffResult::Removed(item_path, (*old_item).clone()));
            }
            (None, Some((new_index, new_item))) => {
                let item_path = if path.is_empty() {
                    format!("[{new_index}]")
                } else {
                    format!("{path}[{new_index}]")
                };
                results.push(DiffResult::Added(item_path, (*new_item).clone()));
            }
            (None, None) => unreachable!(),
        }
    }
}

fn diff_arrays_by_index(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    let max_len = old_arr.len().max(new_arr.len());

    for i in 0..max_len {
        let item_path = format!("{path}[{i}]");

        match (old_arr.get(i), new_arr.get(i)) {
            (Some(old_item), Some(new_item)) => {
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
            (Some(old_item), None) => {
                results.push(DiffResult::Removed(item_path, old_item.clone()));
            }
            (None, Some(new_item)) => {
                results.push(DiffResult::Added(item_path, new_item.clone()));
            }
            (None, None) => unreachable!(),
        }
    }
}

// ============================================================================
// BACKWARD COMPATIBILITY FUNCTIONS - REMOVED PER UNIFIED API SPECIFICATION
// ============================================================================
// All backward compatibility functions have been removed to comply with
// the unified API design philosophy: only the single diff() function should be exposed.

// ============================================================================
// UTILITY FUNCTIONS - FOR INTERNAL USE ONLY
// ============================================================================
// These functions are public only for CLI and language bindings.
// External users should use the main diff() function.

/// Get type name of a JSON value - FOR INTERNAL USE ONLY
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

/// Estimate memory usage of a JSON value - FOR INTERNAL USE ONLY
pub fn estimate_memory_usage(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 8,
        Value::String(s) => s.len(),
        Value::Array(arr) => arr.iter().map(estimate_memory_usage).sum::<usize>() + 24,
        Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| k.len() + estimate_memory_usage(v))
                .sum::<usize>()
                + 24
        }
    }
}

/// Check if values would exceed memory limit - FOR INTERNAL USE ONLY
pub fn would_exceed_memory_limit(v1: &Value, v2: &Value) -> bool {
    const MAX_MEMORY_MB: usize = 100;
    const BYTES_PER_MB: usize = 1024 * 1024;

    let total_size = estimate_memory_usage(v1) + estimate_memory_usage(v2);
    total_size > MAX_MEMORY_MB * BYTES_PER_MB
}

/// Format output to string - FOR INTERNAL USE ONLY
pub fn format_output<T: Serialize>(results: &[T], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)
            .map_err(|e| anyhow!("JSON serialization error: {}", e)),
        OutputFormat::Yaml => {
            serde_yaml::to_string(results).map_err(|e| anyhow!("YAML serialization error: {}", e))
        }
        OutputFormat::Diffx => {
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string(result)?;
                output.push_str(&json);
                output.push('\n');
            }
            Ok(output)
        }
    }
}

/// Format DiffResult output using proper Display implementation
pub fn format_diff_output(
    results: &[DiffResult],
    format: OutputFormat,
    _options: Option<&DiffOptions>,
) -> Result<String> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)
            .map_err(|e| anyhow!("JSON serialization error: {}", e)),
        OutputFormat::Yaml => {
            let mut output = String::new();
            for result in results {
                match result {
                    DiffResult::Added(path, value) => {
                        output.push_str("- Added:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::Removed(path, value) => {
                        output.push_str("- Removed:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::Modified(path, old_value, new_value) => {
                        output.push_str("- Modified:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(old_value).unwrap_or_default().trim()
                        ));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(new_value).unwrap_or_default().trim()
                        ));
                    }
                    DiffResult::TypeChanged(path, old_value, new_value) => {
                        output.push_str("- TypeChanged:\n");
                        output.push_str(&format!("  - {path}\n"));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(old_value).unwrap_or_default().trim()
                        ));
                        output.push_str(&format!(
                            "  - {}\n",
                            serde_yaml::to_string(new_value).unwrap_or_default().trim()
                        ));
                    }
                }
            }
            Ok(output)
        }
        OutputFormat::Diffx => {
            let mut output = String::new();
            for result in results {
                output.push_str(&result.to_string());
                output.push('\n');
            }
            Ok(output)
        }
    }
}

// ============================================================================
// TRAITS
// ============================================================================

trait ValueTypeExt {
    fn type_name(&self) -> &str;
}

impl ValueTypeExt for Value {
    fn type_name(&self) -> &str {
        value_type_name(self)
    }
}

// ============================================================================
// DIRECTORY HANDLING TESTS
// ============================================================================

#[cfg(test)]
mod directory_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_diff_paths_files() {
        // Test file vs file comparison
        let temp_dir = std::env::temp_dir();
        let file1_path = temp_dir.join("diffx_test1.json");
        let file2_path = temp_dir.join("diffx_test2.json");

        fs::write(&file1_path, r#"{"name": "test", "value": 1}"#).unwrap();
        fs::write(&file2_path, r#"{"name": "test", "value": 2}"#).unwrap();

        let results = diff_paths(
            &file1_path.to_string_lossy(),
            &file2_path.to_string_lossy(),
            None,
        )
        .unwrap();

        assert_eq!(results.len(), 1);

        // Cleanup
        let _ = fs::remove_file(file1_path);
        let _ = fs::remove_file(file2_path);
    }

    #[test]
    fn test_diff_paths_file_vs_directory_error() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("diffx_test_file.json");
        let dir_path = temp_dir.join("diffx_test_dir");

        fs::write(&file_path, r#"{"test": true}"#).unwrap();
        fs::create_dir_all(&dir_path).unwrap();

        let result = diff_paths(
            &file_path.to_string_lossy(),
            &dir_path.to_string_lossy(),
            None,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot compare file"));

        // Cleanup
        let _ = fs::remove_file(file_path);
        let _ = fs::remove_dir_all(dir_path);
    }
}
