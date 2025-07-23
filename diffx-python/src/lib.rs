use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyAny};
use diffx_core::{
    diff, parse_json, parse_csv, parse_yaml, parse_toml, parse_ini, parse_xml,
    DiffOptions, DiffxSpecificOptions, OutputFormat, format_output, DiffResult
};
use regex::Regex;
use serde_json::Value;

/// Unified diff function for Python
/// 
/// Args:
///     old: Dict - The old JSON-like structure
///     new: Dict - The new JSON-like structure
///     **options: Various options for comparison
///         epsilon: float - Numerical comparison tolerance
///         array_id_key: str - Key to use for array element identification
///         ignore_keys_regex: str - Regex pattern for keys to ignore
///         path_filter: str - Only show differences in paths containing this string
///         output_format: str - Output format ("diffx", "json", "yaml", "unified")
///         show_unchanged: bool - Show unchanged values as well
///         show_types: bool - Show type information in output
///         use_memory_optimization: bool - Enable memory optimization for large files
///         batch_size: int - Batch size for memory optimization
///         context_lines: int - Number of context lines for diff output
///         ignore_whitespace: bool - Ignore whitespace differences
///         ignore_case: bool - Ignore case differences
///         brief_mode: bool - Report only whether files differ
///         quiet_mode: bool - Suppress normal output; return only exit status
/// 
/// Returns:
///     List[Dict] - List of differences found
#[pyfunction]
#[pyo3(signature = (old, new, **kwargs))]
fn diff_py(py: Python, old: &Bound<'_, PyAny>, new: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyObject> {
    // Convert Python objects to JSON Values
    let old_json = python_to_json_value(old)?;
    let new_json = python_to_json_value(new)?;
    
    // Build options from kwargs
    let options = build_options_from_kwargs(kwargs)?;
    
    // Perform diff
    let results = diff(&old_json, &new_json, Some(&options))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Diff error: {}", e)))?;
    
    // Convert results to Python objects
    let py_results = PyList::empty_bound(py);
    for result in results {
        let py_result = diff_result_to_python(py, &result)?;
        py_results.append(py_result)?;
    }
    
    Ok(py_results.into())
}

/// Parse JSON string to Python dict
#[pyfunction]
fn parse_json_py(py: Python, content: &str) -> PyResult<PyObject> {
    let json_value = parse_json(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {}", e)))?;
    
    json_value_to_python(py, &json_value)
}

/// Parse CSV string to Python list of dicts
#[pyfunction] 
fn parse_csv_py(py: Python, content: &str) -> PyResult<PyObject> {
    let csv_value = parse_csv(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("CSV parse error: {}", e)))?;
    
    json_value_to_python(py, &csv_value)
}

/// Parse YAML string to Python dict
#[pyfunction]
fn parse_yaml_py(py: Python, content: &str) -> PyResult<PyObject> {
    let yaml_value = parse_yaml(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("YAML parse error: {}", e)))?;
    
    json_value_to_python(py, &yaml_value)
}

/// Parse TOML string to Python dict
#[pyfunction]
fn parse_toml_py(py: Python, content: &str) -> PyResult<PyObject> {
    let toml_value = parse_toml(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("TOML parse error: {}", e)))?;
    
    json_value_to_python(py, &toml_value)
}

/// Parse INI string to Python dict
#[pyfunction]
fn parse_ini_py(py: Python, content: &str) -> PyResult<PyObject> {
    let ini_value = parse_ini(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("INI parse error: {}", e)))?;
    
    json_value_to_python(py, &ini_value)
}

/// Parse XML string to Python dict
#[pyfunction]
fn parse_xml_py(py: Python, content: &str) -> PyResult<PyObject> {
    let xml_value = parse_xml(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("XML parse error: {}", e)))?;
    
    json_value_to_python(py, &xml_value)
}

/// Format diff results
#[pyfunction]
#[pyo3(signature = (results, format = "diffx"))]
fn format_output_py(results: &Bound<'_, PyList>, format: &str) -> PyResult<String> {
    // Convert Python list back to Rust Vec<DiffResult>
    let mut rust_results = Vec::new();
    for item in results.iter() {
        let diff_result = python_to_diff_result(&item)?;
        rust_results.push(diff_result);
    }
    
    let output_format = OutputFormat::from_str(format)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid format: {}", e)))?;
    
    format_output(&rust_results, output_format)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Format error: {}", e)))
}

// Helper functions for Python <-> Rust conversion

fn python_to_json_value(py_obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if py_obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = py_obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = py_obj.extract::<i64>() {
        Ok(Value::Number(i.into()))
    } else if let Ok(f) = py_obj.extract::<f64>() {
        Ok(Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into())))
    } else if let Ok(s) = py_obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(list) = py_obj.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(dict) = py_obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            let json_value = python_to_json_value(&value)?;
            map.insert(key_str, json_value);
        }
        Ok(Value::Object(map))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported Python type"))
    }
}

fn json_value_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(py.None())
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty_bound(py);
            for item in arr {
                let py_item = json_value_to_python(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into())
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (key, value) in obj {
                let py_value = json_value_to_python(py, value)?;
                py_dict.set_item(key, py_value)?;
            }
            Ok(py_dict.into())
        }
    }
}

fn diff_result_to_python(py: Python, result: &DiffResult) -> PyResult<PyObject> {
    let py_dict = PyDict::new_bound(py);
    
    match result {
        DiffResult::Added(path, value) => {
            py_dict.set_item("type", "Added")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("value", json_value_to_python(py, value)?)?;
        }
        DiffResult::Removed(path, value) => {
            py_dict.set_item("type", "Removed")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("value", json_value_to_python(py, value)?)?;
        }
        DiffResult::Modified(path, old_val, new_val) => {
            py_dict.set_item("type", "Modified")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("old_value", json_value_to_python(py, old_val)?)?;
            py_dict.set_item("new_value", json_value_to_python(py, new_val)?)?;
        }
        DiffResult::TypeChanged(path, old_val, new_val) => {
            py_dict.set_item("type", "TypeChanged")?;
            py_dict.set_item("path", path)?;
            py_dict.set_item("old_value", json_value_to_python(py, old_val)?)?;
            py_dict.set_item("new_value", json_value_to_python(py, new_val)?)?;
        }
    }
    
    Ok(py_dict.into())
}

fn python_to_diff_result(py_obj: &Bound<'_, PyAny>) -> PyResult<DiffResult> {
    let dict = py_obj.downcast::<PyDict>()?;
    let result_type: String = dict.get_item("type")?.unwrap().extract()?;
    let path: String = dict.get_item("path")?.unwrap().extract()?;
    
    match result_type.as_str() {
        "Added" => {
            let value = python_to_json_value(&dict.get_item("value")?.unwrap())?;
            Ok(DiffResult::Added(path, value))
        }
        "Removed" => {
            let value = python_to_json_value(&dict.get_item("value")?.unwrap())?;
            Ok(DiffResult::Removed(path, value))
        }
        "Modified" => {
            let old_value = python_to_json_value(&dict.get_item("old_value")?.unwrap())?;
            let new_value = python_to_json_value(&dict.get_item("new_value")?.unwrap())?;
            Ok(DiffResult::Modified(path, old_value, new_value))
        }
        "TypeChanged" => {
            let old_value = python_to_json_value(&dict.get_item("old_value")?.unwrap())?;
            let new_value = python_to_json_value(&dict.get_item("new_value")?.unwrap())?;
            Ok(DiffResult::TypeChanged(path, old_value, new_value))
        }
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid diff result type"))
    }
}

fn build_options_from_kwargs(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<DiffOptions> {
    let mut options = DiffOptions::default();
    
    if let Some(kwargs) = kwargs {
        // Core options
        if let Some(epsilon) = kwargs.get_item("epsilon")? {
            options.epsilon = Some(epsilon.extract::<f64>()?);
        }
        
        if let Some(array_id_key) = kwargs.get_item("array_id_key")? {
            options.array_id_key = Some(array_id_key.extract::<String>()?);
        }
        
        if let Some(ignore_keys_regex) = kwargs.get_item("ignore_keys_regex")? {
            let pattern: String = ignore_keys_regex.extract()?;
            let regex = Regex::new(&pattern)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {}", e)))?;
            options.ignore_keys_regex = Some(regex);
        }
        
        if let Some(path_filter) = kwargs.get_item("path_filter")? {
            options.path_filter = Some(path_filter.extract::<String>()?);
        }
        
        if let Some(output_format) = kwargs.get_item("output_format")? {
            let format_str: String = output_format.extract()?;
            let format = OutputFormat::from_str(&format_str)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid output format: {}", e)))?;
            options.output_format = Some(format);
        }
        
        if let Some(show_unchanged) = kwargs.get_item("show_unchanged")? {
            options.show_unchanged = Some(show_unchanged.extract::<bool>()?);
        }
        
        if let Some(show_types) = kwargs.get_item("show_types")? {
            options.show_types = Some(show_types.extract::<bool>()?);
        }
        
        if let Some(use_memory_optimization) = kwargs.get_item("use_memory_optimization")? {
            options.use_memory_optimization = Some(use_memory_optimization.extract::<bool>()?);
        }
        
        if let Some(batch_size) = kwargs.get_item("batch_size")? {
            options.batch_size = Some(batch_size.extract::<usize>()?);
        }
        
        // diffx-specific options
        let mut diffx_options = DiffxSpecificOptions::default();
        let mut has_diffx_options = false;
        
        if let Some(context_lines) = kwargs.get_item("context_lines")? {
            diffx_options.context_lines = Some(context_lines.extract::<usize>()?);
            has_diffx_options = true;
        }
        
        if let Some(ignore_whitespace) = kwargs.get_item("ignore_whitespace")? {
            diffx_options.ignore_whitespace = Some(ignore_whitespace.extract::<bool>()?);
            has_diffx_options = true;
        }
        
        if let Some(ignore_case) = kwargs.get_item("ignore_case")? {
            diffx_options.ignore_case = Some(ignore_case.extract::<bool>()?);
            has_diffx_options = true;
        }
        
        if let Some(brief_mode) = kwargs.get_item("brief_mode")? {
            diffx_options.brief_mode = Some(brief_mode.extract::<bool>()?);
            has_diffx_options = true;
        }
        
        if let Some(quiet_mode) = kwargs.get_item("quiet_mode")? {
            diffx_options.quiet_mode = Some(quiet_mode.extract::<bool>()?);
            has_diffx_options = true;
        }
        
        if has_diffx_options {
            options.diffx_options = Some(diffx_options);
        }
    }
    
    Ok(options)
}

/// Python module for diffx
#[pymodule]
fn diffx_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diff_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_csv_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_yaml_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_toml_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_ini_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_xml_py, m)?)?;
    m.add_function(wrap_pyfunction!(format_output_py, m)?)?;
    
    // Add version
    m.add("__version__", "0.5.7")?;
    
    Ok(())
}