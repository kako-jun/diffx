// Diff module - handles difference detection between values

mod arrays;
mod core;
mod objects;
mod recursive;

// Re-export public API
pub use core::{diff, diff_paths};

// Re-export for internal use within diffx-core
pub(crate) use arrays::diff_arrays;
pub(crate) use objects::diff_objects;
pub(crate) use recursive::{add_diff_result, diff_recursive};
