// Object diff functions

use super::{add_diff_result, diff_recursive};
use crate::{DiffOptions, DiffResult};
use serde_json::Value;

pub(crate) fn diff_objects(
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
