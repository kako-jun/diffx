// Module declarations
mod diff;
mod io;
mod parser;
mod types;
mod utils;

// Re-export public APIs
pub use diff::{diff, diff_paths};
pub use parser::*;
pub use types::*;
pub use utils::*;
