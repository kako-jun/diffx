use diffx_core::{
    diff as core_diff, format_output as core_format_output, parse_csv as core_parse_csv,
    parse_ini as core_parse_ini, parse_json as core_parse_json, parse_toml as core_parse_toml,
    parse_xml as core_parse_xml, parse_yaml as core_parse_yaml, DiffOptions, DiffResult,
    DiffxSpecificOptions, OutputFormat,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use regex::Regex;

#[napi(object)]
pub struct JsDiffOptions {
    /// Numerical comparison tolerance
    pub epsilon: Option<f64>,

    /// Key to use for array element identification
    pub array_id_key: Option<String>,

    /// Regex pattern for keys to ignore
    pub ignore_keys_regex: Option<String>,

    /// Only show differences in paths containing this string
    pub path_filter: Option<String>,

    /// Output format
    pub output_format: Option<String>,

    /// Show unchanged values as well
    pub show_unchanged: Option<bool>,

    /// Show type information in output
    pub show_types: Option<bool>,

    /// Enable memory optimization for large files
    pub use_memory_optimization: Option<bool>,

    /// Batch size for memory optimization
    pub batch_size: Option<u32>,

    // diffx-specific options
    /// Ignore whitespace differences
    pub ignore_whitespace: Option<bool>,

    /// Ignore case differences
    pub ignore_case: Option<bool>,

    /// Report only whether files differ
    pub brief_mode: Option<bool>,

    /// Suppress normal output; return only exit status
    pub quiet_mode: Option<bool>,
}

#[napi(object)]
pub struct JsDiffResult {
    /// Type of difference ('Added', 'Removed', 'Modified', 'TypeChanged')
    pub diff_type: String,

    /// Path to the changed element
    pub path: String,

    /// Old value (for Modified/TypeChanged)
    pub old_value: Option<serde_json::Value>,

    /// New value (for Modified/TypeChanged/Added)
    pub new_value: Option<serde_json::Value>,

    /// Value (for Removed)
    pub value: Option<serde_json::Value>,
}

/// Unified diff function for JavaScript/Node.js
///
/// Compare two JavaScript objects or values and return differences.
///
/// # Arguments
///
/// * `old` - The old value (JavaScript object, array, or primitive)
/// * `new` - The new value (JavaScript object, array, or primitive)
/// * `options` - Optional configuration object
///
/// # Returns
///
/// Array of difference objects
///
/// # Example
///
/// ```javascript
/// const { diff } = require('diffx-js');
///
/// const old = { a: 1, b: 2 };
/// const new = { a: 1, b: 3 };
/// const result = diff(old, new);
/// console.log(result); // [{ type: 'Modified', path: 'b', oldValue: 2, newValue: 3 }]
/// ```
#[napi]
pub fn diff(
    old: serde_json::Value,
    new: serde_json::Value,
    options: Option<JsDiffOptions>,
) -> Result<Vec<JsDiffResult>> {
    // Convert options
    let rust_options = options.map(build_diff_options).transpose()?;

    // Perform diff
    let results = core_diff(&old, &new, rust_options.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Diff error: {e}")))?;

    // Convert results to JavaScript objects
    let js_results = results
        .into_iter()
        .map(convert_diff_result)
        .collect::<Result<Vec<_>>>()?;

    Ok(js_results)
}

/// Parse JSON string to JavaScript object
///
/// # Arguments
///
/// * `content` - JSON string to parse
///
/// # Returns
///
/// Parsed JavaScript object
#[napi]
pub fn parse_json(content: String) -> Result<serde_json::Value> {
    core_parse_json(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("JSON parse error: {e}")))
}

/// Parse CSV string to JavaScript array of objects
///
/// # Arguments
///
/// * `content` - CSV string to parse
///
/// # Returns
///
/// Array of JavaScript objects representing CSV rows
#[napi]
pub fn parse_csv(content: String) -> Result<serde_json::Value> {
    core_parse_csv(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("CSV parse error: {e}")))
}

/// Parse YAML string to JavaScript object
///
/// # Arguments
///
/// * `content` - YAML string to parse
///
/// # Returns
///
/// Parsed JavaScript object
#[napi]
pub fn parse_yaml(content: String) -> Result<serde_json::Value> {
    core_parse_yaml(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("YAML parse error: {e}")))
}

/// Parse TOML string to JavaScript object
///
/// # Arguments
///
/// * `content` - TOML string to parse
///
/// # Returns
///
/// Parsed JavaScript object
#[napi]
pub fn parse_toml(content: String) -> Result<serde_json::Value> {
    core_parse_toml(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("TOML parse error: {e}")))
}

/// Parse INI string to JavaScript object
///
/// # Arguments
///
/// * `content` - INI string to parse
///
/// # Returns
///
/// Parsed JavaScript object
#[napi]
pub fn parse_ini(content: String) -> Result<serde_json::Value> {
    core_parse_ini(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("INI parse error: {e}")))
}

/// Parse XML string to JavaScript object
///
/// # Arguments
///
/// * `content` - XML string to parse
///
/// # Returns
///
/// Parsed JavaScript object
#[napi]
pub fn parse_xml(content: String) -> Result<serde_json::Value> {
    core_parse_xml(&content)
        .map_err(|e| Error::new(Status::InvalidArg, format!("XML parse error: {e}")))
}

/// Format diff results as string
///
/// # Arguments
///
/// * `results` - Array of diff results
/// * `format` - Output format ("diffx", "json", "yaml")
///
/// # Returns
///
/// Formatted string output
#[napi]
pub fn format_output(results: Vec<JsDiffResult>, format: String) -> Result<String> {
    // Convert JS results back to Rust DiffResult
    let rust_results = results
        .into_iter()
        .map(convert_js_diff_result)
        .collect::<Result<Vec<_>>>()?;

    let output_format = OutputFormat::parse_format(&format)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid format: {e}")))?;

    core_format_output(&rust_results, output_format)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Format error: {e}")))
}

// Helper functions

fn build_diff_options(js_options: JsDiffOptions) -> Result<DiffOptions> {
    let mut options = DiffOptions::default();

    // Core options
    if let Some(epsilon) = js_options.epsilon {
        options.epsilon = Some(epsilon);
    }

    if let Some(array_id_key) = js_options.array_id_key {
        options.array_id_key = Some(array_id_key);
    }

    if let Some(ignore_keys_regex) = js_options.ignore_keys_regex {
        let regex = Regex::new(&ignore_keys_regex)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid regex: {e}")))?;
        options.ignore_keys_regex = Some(regex);
    }

    if let Some(path_filter) = js_options.path_filter {
        options.path_filter = Some(path_filter);
    }

    if let Some(output_format) = js_options.output_format {
        let format = OutputFormat::parse_format(&output_format)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid output format: {e}")))?;
        options.output_format = Some(format);
    }

    if let Some(show_unchanged) = js_options.show_unchanged {
        options.show_unchanged = Some(show_unchanged);
    }

    if let Some(show_types) = js_options.show_types {
        options.show_types = Some(show_types);
    }

    if let Some(use_memory_optimization) = js_options.use_memory_optimization {
        options.use_memory_optimization = Some(use_memory_optimization);
    }

    if let Some(batch_size) = js_options.batch_size {
        options.batch_size = Some(batch_size as usize);
    }

    // diffx-specific options
    let mut diffx_options = DiffxSpecificOptions::default();
    let mut has_diffx_options = false;

    // context_lines field has been removed from DiffxSpecificOptions

    if let Some(ignore_whitespace) = js_options.ignore_whitespace {
        diffx_options.ignore_whitespace = Some(ignore_whitespace);
        has_diffx_options = true;
    }

    if let Some(ignore_case) = js_options.ignore_case {
        diffx_options.ignore_case = Some(ignore_case);
        has_diffx_options = true;
    }

    if let Some(brief_mode) = js_options.brief_mode {
        diffx_options.brief_mode = Some(brief_mode);
        has_diffx_options = true;
    }

    if let Some(quiet_mode) = js_options.quiet_mode {
        diffx_options.quiet_mode = Some(quiet_mode);
        has_diffx_options = true;
    }

    if has_diffx_options {
        options.diffx_options = Some(diffx_options);
    }

    Ok(options)
}

fn convert_diff_result(result: DiffResult) -> Result<JsDiffResult> {
    match result {
        DiffResult::Added(path, value) => Ok(JsDiffResult {
            diff_type: "Added".to_string(),
            path,
            old_value: None,
            new_value: Some(value),
            value: None,
        }),
        DiffResult::Removed(path, value) => Ok(JsDiffResult {
            diff_type: "Removed".to_string(),
            path,
            old_value: None,
            new_value: None,
            value: Some(value),
        }),
        DiffResult::Modified(path, old_val, new_val) => Ok(JsDiffResult {
            diff_type: "Modified".to_string(),
            path,
            old_value: Some(old_val),
            new_value: Some(new_val),
            value: None,
        }),
        DiffResult::TypeChanged(path, old_val, new_val) => Ok(JsDiffResult {
            diff_type: "TypeChanged".to_string(),
            path,
            old_value: Some(old_val),
            new_value: Some(new_val),
            value: None,
        }),
    }
}

fn convert_js_diff_result(js_result: JsDiffResult) -> Result<DiffResult> {
    match js_result.diff_type.as_str() {
        "Added" => {
            let value = js_result.new_value.ok_or_else(|| {
                Error::new(Status::InvalidArg, "Added result must have new_value")
            })?;
            Ok(DiffResult::Added(js_result.path, value))
        }
        "Removed" => {
            let value = js_result
                .value
                .ok_or_else(|| Error::new(Status::InvalidArg, "Removed result must have value"))?;
            Ok(DiffResult::Removed(js_result.path, value))
        }
        "Modified" => {
            let old_value = js_result.old_value.ok_or_else(|| {
                Error::new(Status::InvalidArg, "Modified result must have old_value")
            })?;
            let new_value = js_result.new_value.ok_or_else(|| {
                Error::new(Status::InvalidArg, "Modified result must have new_value")
            })?;
            Ok(DiffResult::Modified(js_result.path, old_value, new_value))
        }
        "TypeChanged" => {
            let old_value = js_result.old_value.ok_or_else(|| {
                Error::new(Status::InvalidArg, "TypeChanged result must have old_value")
            })?;
            let new_value = js_result.new_value.ok_or_else(|| {
                Error::new(Status::InvalidArg, "TypeChanged result must have new_value")
            })?;
            Ok(DiffResult::TypeChanged(
                js_result.path,
                old_value,
                new_value,
            ))
        }
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid diff result type: {}", js_result.diff_type),
        )),
    }
}
