// Module declarations
mod types;
mod parser;
mod diff;
mod io;
mod utils;

// Re-export public APIs
pub use types::*;
pub use parser::*;
pub use diff::{diff_paths, diff};
pub use utils::*;
