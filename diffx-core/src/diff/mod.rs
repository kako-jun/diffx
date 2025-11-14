// Diff module - handles difference detection between values

mod core;
mod arrays;
mod objects;
mod recursive;

// Re-export for internal use within diffx-core
pub(crate) use arrays::diff_arrays;
pub(crate) use objects::diff_objects;
pub(crate) use recursive::{diff_recursive, add_diff_result};
