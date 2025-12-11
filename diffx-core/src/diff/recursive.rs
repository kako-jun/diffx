// Recursive diff functions

use crate::{value_type_name, DiffOptions, DiffResult};
use serde_json::Value;

// Import diff_objects and diff_arrays from sibling modules
use super::{diff_arrays, diff_objects};

// Helper function to add result with path filtering
pub(crate) fn add_diff_result(
    result: DiffResult,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
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

pub(crate) fn diff_recursive(
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
                if value_type_name(old) != value_type_name(new) {
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
