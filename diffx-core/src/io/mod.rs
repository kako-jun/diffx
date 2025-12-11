// IO module - handles file and directory operations

mod directories;
mod files;

// Re-export for internal use within diffx-core
pub(crate) use directories::get_all_files_recursive;
