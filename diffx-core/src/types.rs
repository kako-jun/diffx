use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, Value, Value),
}

impl std::fmt::Display for DiffResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffResult::Added(key, value) => {
                write!(f, "  + {key}: {value}")
            }
            DiffResult::Removed(key, value) => {
                write!(f, "  - {key}: {value}")
            }
            DiffResult::Modified(key, value1, value2) => {
                write!(f, "  ~ {key}: {value1} -> {value2}")
            }
            DiffResult::TypeChanged(key, value1, value2) => {
                write!(f, "  # {key}: {value1} -> {value2} (type changed)")
            }
        }
    }
}

// Lightweight diff result for memory-constrained operations
#[derive(Debug, PartialEq, Serialize)]
pub enum LightweightDiffResult {
    Added(String, String),
    Removed(String, String),
    Modified(String, String, String),
    TypeChanged(String, String, String),
}

impl From<&DiffResult> for LightweightDiffResult {
    fn from(result: &DiffResult) -> Self {
        match result {
            DiffResult::Added(path, value) => LightweightDiffResult::Added(
                path.clone(),
                serde_json::to_string(value).unwrap_or_default(),
            ),
            DiffResult::Removed(path, value) => LightweightDiffResult::Removed(
                path.clone(),
                serde_json::to_string(value).unwrap_or_default(),
            ),
            DiffResult::Modified(path, old, new) => LightweightDiffResult::Modified(
                path.clone(),
                serde_json::to_string(old).unwrap_or_default(),
                serde_json::to_string(new).unwrap_or_default(),
            ),
            DiffResult::TypeChanged(path, old, new) => LightweightDiffResult::TypeChanged(
                path.clone(),
                serde_json::to_string(old).unwrap_or_default(),
                serde_json::to_string(new).unwrap_or_default(),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[serde(rename = "diffx")]
    #[default]
    Diffx,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
}

// Manual ValueEnum implementation since it's not available in core
impl OutputFormat {
    pub fn value_variants() -> &'static [Self] {
        &[Self::Diffx, Self::Json, Self::Yaml]
    }

    pub fn parse_format(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "diffx" => Ok(Self::Diffx),
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            _ => Err(anyhow!("Invalid output format: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiffxSpecificOptions {
    pub ignore_whitespace: Option<bool>,
    pub ignore_case: Option<bool>,
    pub brief_mode: Option<bool>,
    pub quiet_mode: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct DiffOptions {
    // Core comparison options
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub ignore_keys_regex: Option<Regex>,
    pub path_filter: Option<String>,

    // Directory comparison
    pub recursive: Option<bool>,

    // Output control
    pub output_format: Option<OutputFormat>,

    // diffx-specific options
    pub diffx_options: Option<DiffxSpecificOptions>,
}
