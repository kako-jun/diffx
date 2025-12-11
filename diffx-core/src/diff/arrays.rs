// Array diff functions

use super::diff_recursive;
use crate::{DiffOptions, DiffResult};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn diff_arrays(
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
