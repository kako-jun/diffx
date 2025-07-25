#[allow(unused_imports)]
use assert_cmd::prelude::*;
// Main test module for diffx
// Organized into logical test categories for better maintainability

// Core library tests - direct API testing
pub mod core;

// CLI command tests - integration testing
pub mod cli;

// Basic functionality tests - end-to-end scenarios
pub mod basic;

// Format-specific tests - file format support
pub mod formats;

// Error handling tests - edge cases and error conditions
pub mod errors;

// Feature tests - advanced functionality
pub mod features;

// Documentation examples tests - ensure docs stay current
pub mod docs_examples;

// Unit tests are now in unit.rs as a top-level integration test
// pub mod unit;

// Integration tests - test interaction between components
pub mod integration;