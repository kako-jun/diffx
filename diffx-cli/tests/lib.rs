#[allow(unused_imports)]
use assert_cmd::prelude::*;
// Main test module for diffx-cli
// Integration tests for CLI binary

// CLI command tests
pub mod cli;

// Documentation examples tests - ensure docs stay current
pub mod docs_examples;

// Integration tests - test CLI binary behavior
pub mod integration;