#[allow(unused_imports)]
use assert_cmd::prelude::*;
// Original CLI tests
pub mod basic_commands;
pub mod no_color_option;
pub mod options;
pub mod output_formats;
pub mod platform_compatibility;
pub mod unix_compatibility;

// Migrated from basic/
pub mod basic_automation;
pub mod basic_directory_comparison;
pub mod basic_functionality;

// Migrated from errors/
pub mod errors_edge_cases;
pub mod errors_handling;

// Migrated from features/
pub mod features_advanced_filtering;
pub mod features_industry_scenarios;
pub mod features_optimization_performance;
pub mod features_semantic_diffx;
pub mod features_verbose_output;

// Migrated from formats/
pub mod formats_csv;
pub mod formats_ini;
pub mod formats_json;
pub mod formats_toml;
pub mod formats_xml;
pub mod formats_yaml;
