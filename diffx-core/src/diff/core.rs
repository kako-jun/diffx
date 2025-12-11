// Core diff functions: diff_paths, diff, diff_files, diff_directories

use crate::{detect_format_from_path, parse_content_by_format, DiffOptions, DiffResult};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::diff_recursive;
use crate::io::get_all_files_recursive;

/// Unified diff function for diffx (path-based entry point)
///
/// This is the main entry point that handles both files and directories.
/// - File vs File: Regular file comparison
/// - Directory vs Directory: Requires --recursive flag, otherwise error
/// - File vs Directory: Returns error
pub fn diff_paths(
    old_path: &str,
    new_path: &str,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>> {
    let path1 = Path::new(old_path);
    let path2 = Path::new(new_path);

    let recursive = options.and_then(|o| o.recursive).unwrap_or(false);

    match (path1.is_dir(), path2.is_dir()) {
        (true, true) => {
            if recursive {
                diff_directories(path1, path2, options)
            } else {
                Err(anyhow!(
                    "Both paths are directories. Use --recursive (-r) to compare directories."
                ))
            }
        }
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

    let mut results = Vec::new();
    diff_recursive(old, new, "", &mut results, opts);
    Ok(results)
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

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
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
