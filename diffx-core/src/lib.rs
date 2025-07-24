use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// UNIFIED API - Core Types
// ============================================================================

#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, Value, Value),
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
            DiffResult::Added(path, value) => {
                LightweightDiffResult::Added(path.clone(), serde_json::to_string(value).unwrap_or_default())
            }
            DiffResult::Removed(path, value) => {
                LightweightDiffResult::Removed(path.clone(), serde_json::to_string(value).unwrap_or_default())
            }
            DiffResult::Modified(path, old, new) => {
                LightweightDiffResult::Modified(
                    path.clone(),
                    serde_json::to_string(old).unwrap_or_default(),
                    serde_json::to_string(new).unwrap_or_default(),
                )
            }
            DiffResult::TypeChanged(path, old, new) => {
                LightweightDiffResult::TypeChanged(
                    path.clone(),
                    serde_json::to_string(old).unwrap_or_default(),
                    serde_json::to_string(new).unwrap_or_default(),
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    #[serde(rename = "diffx")]
    Diffx,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "unified")]
    Unified,
}

// Manual ValueEnum implementation since it's not available in core
impl OutputFormat {
    pub fn value_variants() -> &'static [Self] {
        &[Self::Diffx, Self::Json, Self::Yaml, Self::Unified]
    }
    
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "diffx" => Ok(Self::Diffx),
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            "unified" => Ok(Self::Unified),
            _ => Err(anyhow!("Invalid output format: {}", s)),
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Diffx
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiffxSpecificOptions {
    pub context_lines: Option<usize>,
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
    
    // Output control
    pub output_format: Option<OutputFormat>,
    pub show_unchanged: Option<bool>,
    pub show_types: Option<bool>,
    
    // Memory optimization
    pub use_memory_optimization: Option<bool>,
    pub batch_size: Option<usize>,
    
    // diffx-specific options
    pub diffx_options: Option<DiffxSpecificOptions>,
}

// Backward compatibility - legacy DiffConfig structure
#[derive(Debug, Clone, Default)]
pub struct DiffConfig {
    pub ignore_keys_regex: Option<Regex>,
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub use_memory_optimization: bool,
    pub batch_size: usize,
    pub ignore_whitespace: bool,
    pub ignore_case: bool,
}

impl DiffConfig {
    pub fn default() -> Self {
        Self {
            ignore_keys_regex: None,
            epsilon: None,
            array_id_key: None,
            use_memory_optimization: false,
            batch_size: 1000,
            ignore_whitespace: false,
            ignore_case: false,
        }
    }
}

impl From<&DiffConfig> for DiffOptions {
    fn from(config: &DiffConfig) -> Self {
        let mut options = DiffOptions::default();
        options.epsilon = config.epsilon;
        options.array_id_key = config.array_id_key.clone();
        options.ignore_keys_regex = config.ignore_keys_regex.clone();
        options.use_memory_optimization = Some(config.use_memory_optimization);
        options.batch_size = Some(config.batch_size);
        
        let mut diffx_options = DiffxSpecificOptions::default();
        diffx_options.ignore_whitespace = Some(config.ignore_whitespace);
        diffx_options.ignore_case = Some(config.ignore_case);
        options.diffx_options = Some(diffx_options);
        
        options
    }
}

// ============================================================================
// UNIFIED API - Main Function
// ============================================================================

/// Unified diff function for diffx
/// 
/// This is the single entry point for all diffx functionality.
/// All configuration is done through the options parameter.
pub fn diff(
    old: &Value,
    new: &Value,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>> {
    let default_options = DiffOptions::default();
    let opts = options.unwrap_or(&default_options);
    
    // Apply memory optimization if requested
    if opts.use_memory_optimization.unwrap_or(false) {
        diff_optimized_implementation(old, new, opts)
    } else {
        diff_standard_implementation(old, new, opts)
    }
}

fn diff_standard_implementation(
    old: &Value, 
    new: &Value, 
    options: &DiffOptions
) -> Result<Vec<DiffResult>> {
    let mut results = Vec::new();
    diff_recursive(old, new, "", &mut results, options);
    Ok(results)
}

fn diff_optimized_implementation(
    old: &Value,
    new: &Value, 
    options: &DiffOptions
) -> Result<Vec<DiffResult>> {
    // Check memory limits
    if would_exceed_memory_limit(old, new) {
        return Err(anyhow!("Input too large for memory optimization"));
    }
    
    diff_standard_implementation(old, new, options)
}

fn diff_recursive(
    old: &Value,
    new: &Value,
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    // Apply path filter if specified
    if let Some(filter) = &options.path_filter {
        if !path.contains(filter) {
            return;
        }
    }

    match (old, new) {
        (Value::Object(old_obj), Value::Object(new_obj)) => {
            diff_objects(old_obj, new_obj, path, results, options);
        }
        (Value::Array(old_arr), Value::Array(new_arr)) => {
            diff_arrays(old_arr, new_arr, path, results, options);
        }
        (Value::Number(old_num), Value::Number(new_num)) => {
            if let Some(epsilon) = options.epsilon {
                let old_f = old_num.as_f64().unwrap_or(0.0);
                let new_f = new_num.as_f64().unwrap_or(0.0);
                if (old_f - new_f).abs() > epsilon {
                    results.push(DiffResult::Modified(
                        path.to_string(),
                        old.clone(),
                        new.clone(),
                    ));
                }
            } else if old != new {
                results.push(DiffResult::Modified(
                    path.to_string(),
                    old.clone(),
                    new.clone(),
                ));
            }
        }
        (Value::String(old_str), Value::String(new_str)) => {
            let mut old_processed = old_str.clone();
            let mut new_processed = new_str.clone();
            
            // Apply string transformations based on options
            if let Some(diffx_opts) = &options.diffx_options {
                if diffx_opts.ignore_whitespace.unwrap_or(false) {
                    old_processed = old_processed.chars().filter(|c| !c.is_whitespace()).collect();
                    new_processed = new_processed.chars().filter(|c| !c.is_whitespace()).collect();
                }
                if diffx_opts.ignore_case.unwrap_or(false) {
                    old_processed = old_processed.to_lowercase();
                    new_processed = new_processed.to_lowercase();
                }
            }
            
            if old_processed != new_processed {
                results.push(DiffResult::Modified(
                    path.to_string(),
                    old.clone(),
                    new.clone(),
                ));
            }
        }
        _ => {
            if old != new {
                if old.type_name() != new.type_name() {
                    results.push(DiffResult::TypeChanged(
                        path.to_string(),
                        old.clone(),
                        new.clone(),
                    ));
                } else {
                    // For other types, just do regular comparison
                    results.push(DiffResult::Modified(
                        path.to_string(),
                        old.clone(),
                        new.clone(),
                    ));
                }
            }
        }
    }
}

fn diff_objects(
    old_obj: &serde_json::Map<String, Value>,
    new_obj: &serde_json::Map<String, Value>,
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    // Handle ignore_keys_regex
    let should_ignore_key = |key: &str| -> bool {
        if let Some(regex) = &options.ignore_keys_regex {
            regex.is_match(key)
        } else {
            false
        }
    };

    // Check for removed keys
    for (key, old_value) in old_obj {
        if should_ignore_key(key) {
            continue;
        }
        
        let new_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{}.{}", path, key)
        };

        if !new_obj.contains_key(key) {
            results.push(DiffResult::Removed(new_path, old_value.clone()));
        }
    }

    // Check for added and modified keys
    for (key, new_value) in new_obj {
        if should_ignore_key(key) {
            continue;
        }
        
        let new_path = if path.is_empty() {
            key.clone()
        } else {
            format!("{}.{}", path, key)
        };

        match old_obj.get(key) {
            None => {
                results.push(DiffResult::Added(new_path, new_value.clone()));
            }
            Some(old_value) => {
                diff_recursive(old_value, new_value, &new_path, results, options);
            }
        }
    }
}

fn diff_arrays(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    if let Some(id_key) = &options.array_id_key {
        diff_arrays_with_id(old_arr, new_arr, path, results, options, id_key);
    } else {
        diff_arrays_by_index(old_arr, new_arr, path, results, options);
    }
}

fn diff_arrays_with_id(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
    id_key: &str,
) {
    let mut old_by_id: HashMap<String, (usize, &Value)> = HashMap::new();
    let mut new_by_id: HashMap<String, (usize, &Value)> = HashMap::new();
    let mut old_without_id: Vec<(usize, &Value)> = Vec::new();
    let mut new_without_id: Vec<(usize, &Value)> = Vec::new();

    // Separate items with IDs from those without
    for (index, item) in old_arr.iter().enumerate() {
        if let Some(id_value) = item.get(id_key) {
            let id_str = match id_value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => format!("{:?}", id_value),
            };
            old_by_id.insert(id_str, (index, item));
        } else {
            old_without_id.push((index, item));
        }
    }

    for (index, item) in new_arr.iter().enumerate() {
        if let Some(id_value) = item.get(id_key) {
            let id_str = match id_value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => format!("{:?}", id_value),
            };
            new_by_id.insert(id_str, (index, item));
        } else {
            new_without_id.push((index, item));
        }
    }

    // Handle items with IDs
    // Find removed items
    for (id, (_, old_item)) in &old_by_id {
        if !new_by_id.contains_key(id) {
            let item_path = if path.is_empty() {
                format!("[{}={}]", id_key, id)
            } else {
                format!("{}[{}={}]", path, id_key, id)
            };
            results.push(DiffResult::Removed(item_path, (*old_item).clone()));
        }
    }

    // Find added and modified items with IDs
    for (id, (_, new_item)) in &new_by_id {
        let item_path = if path.is_empty() {
            format!("[{}={}]", id_key, id)
        } else {
            format!("{}[{}={}]", path, id_key, id)
        };
        
        match old_by_id.get(id) {
            None => {
                results.push(DiffResult::Added(item_path, (*new_item).clone()));
            }
            Some((_, old_item)) => {
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
        }
    }

    // Handle items without IDs by index
    let max_len = old_without_id.len().max(new_without_id.len());
    for i in 0..max_len {
        match (old_without_id.get(i), new_without_id.get(i)) {
            (Some((old_index, old_item)), Some((_, new_item))) => {
                let item_path = if path.is_empty() {
                    format!("[{}]", old_index)
                } else {
                    format!("{}[{}]", path, old_index)
                };
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
            (Some((old_index, old_item)), None) => {
                let item_path = if path.is_empty() {
                    format!("[{}]", old_index)
                } else {
                    format!("{}[{}]", path, old_index)
                };
                results.push(DiffResult::Removed(item_path, (*old_item).clone()));
            }
            (None, Some((new_index, new_item))) => {
                let item_path = if path.is_empty() {
                    format!("[{}]", new_index)
                } else {
                    format!("{}[{}]", path, new_index)
                };
                results.push(DiffResult::Added(item_path, (*new_item).clone()));
            }
            (None, None) => unreachable!(),
        }
    }
}

fn diff_arrays_by_index(
    old_arr: &[Value],
    new_arr: &[Value],
    path: &str,
    results: &mut Vec<DiffResult>,
    options: &DiffOptions,
) {
    let max_len = old_arr.len().max(new_arr.len());

    for i in 0..max_len {
        let item_path = format!("{}[{}]", path, i);

        match (old_arr.get(i), new_arr.get(i)) {
            (Some(old_item), Some(new_item)) => {
                diff_recursive(old_item, new_item, &item_path, results, options);
            }
            (Some(old_item), None) => {
                results.push(DiffResult::Removed(item_path, old_item.clone()));
            }
            (None, Some(new_item)) => {
                results.push(DiffResult::Added(item_path, new_item.clone()));
            }
            (None, None) => unreachable!(),
        }
    }
}

// ============================================================================
// BACKWARD COMPATIBILITY FUNCTIONS - REMOVED PER UNIFIED API SPECIFICATION
// ============================================================================
// All backward compatibility functions have been removed to comply with
// the unified API design philosophy: only the single diff() function should be exposed.

// ============================================================================
// PARSER FUNCTIONS - FOR INTERNAL USE ONLY
// ============================================================================
// These functions are public only for CLI and language bindings.
// External users should use the main diff() function with file reading.

/// Parse JSON content - FOR INTERNAL USE ONLY
/// External users should read files themselves and use diff() function
pub fn parse_json(content: &str) -> Result<Value> {
    serde_json::from_str(content).map_err(|e| anyhow!("JSON parse error: {}", e))
}

/// Parse CSV content - FOR INTERNAL USE ONLY
pub fn parse_csv(content: &str) -> Result<Value> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()?.clone();
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = serde_json::Map::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.to_string(), Value::String(field.to_string()));
            }
        }

        records.push(Value::Object(map));
    }

    Ok(Value::Array(records))
}

/// Parse YAML content - FOR INTERNAL USE ONLY
pub fn parse_yaml(content: &str) -> Result<Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow!("YAML parse error: {}", e))
}

/// Parse TOML content - FOR INTERNAL USE ONLY
pub fn parse_toml(content: &str) -> Result<Value> {
    let toml_value: toml::Value = content.parse()?;
    toml_to_json_value(toml_value)
}

fn toml_to_json_value(toml_val: toml::Value) -> Result<Value> {
    match toml_val {
        toml::Value::String(s) => Ok(Value::String(s)),
        toml::Value::Integer(i) => Ok(Value::Number(i.into())),
        toml::Value::Float(f) => Ok(Value::Number(
            serde_json::Number::from_f64(f).ok_or_else(|| anyhow!("Invalid float"))?
        )),
        toml::Value::Boolean(b) => Ok(Value::Bool(b)),
        toml::Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(toml_to_json_value(item)?);
            }
            Ok(Value::Array(json_arr))
        }
        toml::Value::Table(table) => {
            let mut json_obj = serde_json::Map::new();
            for (key, value) in table {
                json_obj.insert(key, toml_to_json_value(value)?);
            }
            Ok(Value::Object(json_obj))
        }
        toml::Value::Datetime(dt) => Ok(Value::String(dt.to_string())),
    }
}

/// Parse INI content - FOR INTERNAL USE ONLY
pub fn parse_ini(content: &str) -> Result<Value> {
    let mut result = serde_json::Map::new();
    let mut current_section = String::new();
    let mut global_section = serde_json::Map::new();

    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            result.insert(current_section.clone(), Value::Object(serde_json::Map::new()));
        } else if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos+1..].trim().to_string();
            
            if current_section.is_empty() {
                global_section.insert(key, Value::String(value));
            } else {
                if let Some(Value::Object(section)) = result.get_mut(&current_section) {
                    section.insert(key, Value::String(value));
                }
            }
        }
    }

    // Add global section if it exists
    if !global_section.is_empty() {
        result.insert("default".to_string(), Value::Object(global_section));
    }

    Ok(Value::Object(result))
}

/// Parse XML content - FOR INTERNAL USE ONLY
pub fn parse_xml(content: &str) -> Result<Value> {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::collections::HashMap;

    let mut reader = Reader::from_str(content);
    reader.trim_text(true);
    let mut current_element = String::new();
    let mut elements: HashMap<String, Vec<Value>> = HashMap::new();
    let mut current_attrs: HashMap<String, String> = HashMap::new();
    let mut text_content = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_attrs.clear();
                
                // Parse attributes
                for attr in e.attributes() {
                    if let Ok(attr) = attr {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        current_attrs.insert(key, value);
                    }
                }
                text_content.clear();
            }
            Ok(Event::Text(e)) => {
                text_content = e.unescape().unwrap_or_default().to_string();
            }
            Ok(Event::End(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag_name == current_element {
                    let mut element_value = serde_json::Map::new();
                    
                    // Add attributes
                    for (key, value) in &current_attrs {
                        element_value.insert(key.clone(), Value::String(value.clone()));
                    }
                    
                    // Add text content
                    if !text_content.trim().is_empty() {
                        element_value.insert("text".to_string(), Value::String(text_content.trim().to_string()));
                    }
                    
                    let element_obj = if element_value.is_empty() && !text_content.trim().is_empty() {
                        Value::String(text_content.trim().to_string())
                    } else if element_value.len() == 1 && element_value.contains_key("text") {
                        element_value.get("text").unwrap().clone()
                    } else {
                        Value::Object(element_value)
                    };
                    
                    elements.entry(tag_name).or_insert_with(Vec::new).push(element_obj);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow!("XML parsing error: {}", e)),
            _ => {}
        }
    }

    // Convert to JSON structure
    let mut result = serde_json::Map::new();
    for (tag, values) in elements {
        if values.len() == 1 {
            result.insert(tag, values.into_iter().next().unwrap());
        } else {
            result.insert(tag, Value::Array(values));
        }
    }

    Ok(Value::Object(result))
}

// ============================================================================
// UTILITY FUNCTIONS - FOR INTERNAL USE ONLY
// ============================================================================
// These functions are public only for CLI and language bindings.
// External users should use the main diff() function.

/// Get type name of a JSON value - FOR INTERNAL USE ONLY
pub fn value_type_name(value: &Value) -> &str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}

/// Estimate memory usage of a JSON value - FOR INTERNAL USE ONLY
pub fn estimate_memory_usage(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 8,
        Value::String(s) => s.len(),
        Value::Array(arr) => {
            arr.iter().map(estimate_memory_usage).sum::<usize>() + 24
        }
        Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| k.len() + estimate_memory_usage(v))
                .sum::<usize>() + 24
        }
    }
}

/// Check if values would exceed memory limit - FOR INTERNAL USE ONLY
pub fn would_exceed_memory_limit(v1: &Value, v2: &Value) -> bool {
    const MAX_MEMORY_MB: usize = 100;
    const BYTES_PER_MB: usize = 1024 * 1024;
    
    let total_size = estimate_memory_usage(v1) + estimate_memory_usage(v2);
    total_size > MAX_MEMORY_MB * BYTES_PER_MB
}

/// Format output to string - FOR INTERNAL USE ONLY
pub fn format_output<T: Serialize>(
    results: &[T],
    format: OutputFormat,
) -> Result<String> {
    match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(results)
                .map_err(|e| anyhow!("JSON serialization error: {}", e))
        }
        OutputFormat::Yaml => {
            serde_yaml::to_string(results)
                .map_err(|e| anyhow!("YAML serialization error: {}", e))
        }
        OutputFormat::Diffx => {
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string(result)?;
                output.push_str(&json);
                output.push('\n');
            }
            Ok(output)
        }
        OutputFormat::Unified => {
            // Simple unified diff format
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string(result)?;
                output.push_str(&format!("~ {}\n", json));
            }
            Ok(output)
        }
    }
}

// ============================================================================
// TRAITS
// ============================================================================

trait ValueTypeExt {
    fn type_name(&self) -> &str;
}

impl ValueTypeExt for Value {
    fn type_name(&self) -> &str {
        value_type_name(self)
    }
}