// Directory operations

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_all_files_recursive(dir: &Path) -> Result<Vec<PathBuf>> {
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
