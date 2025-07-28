use diffx_core::*;
use regex::Regex;
use serde_json::{json, Value};

// ============================================================================
// UNIFIED API TESTS - Core Functionality
// ============================================================================

#[test]
fn test_diff_basic_modification() {
    let old = json!({"name": "Alice", "age": 30});
    let new = json!({"name": "Alice", "age": 31});

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        DiffResult::Modified(path, old_val, new_val) => {
            assert_eq!(path, "age");
            assert_eq!(old_val, &json!(30));
            assert_eq!(new_val, &json!(31));
        }
        _ => panic!("Expected Modified result"),
    }
}

#[test]
fn test_diff_added_field() {
    let old = json!({"name": "Alice"});
    let new = json!({"name": "Alice", "age": 30});

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        DiffResult::Added(path, value) => {
            assert_eq!(path, "age");
            assert_eq!(value, &json!(30));
        }
        _ => panic!("Expected Added result"),
    }
}

#[test]
fn test_diff_removed_field() {
    let old = json!({"name": "Alice", "age": 30});
    let new = json!({"name": "Alice"});

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        DiffResult::Removed(path, value) => {
            assert_eq!(path, "age");
            assert_eq!(value, &json!(30));
        }
        _ => panic!("Expected Removed result"),
    }
}

#[test]
fn test_diff_type_changed() {
    let old = json!({"value": 123});
    let new = json!({"value": "123"});

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        DiffResult::TypeChanged(path, old_val, new_val) => {
            assert_eq!(path, "value");
            assert_eq!(old_val, &json!(123));
            assert_eq!(new_val, &json!("123"));
        }
        _ => panic!("Expected TypeChanged result"),
    }
}

#[test]
fn test_diff_no_changes() {
    let old = json!({"name": "Alice", "age": 30});
    let new = json!({"name": "Alice", "age": 30});

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 0);
}

// ============================================================================
// OPTIONS TESTING - All Options Coverage
// ============================================================================

#[test]
fn test_diff_with_epsilon() {
    let old = json!({"value": 1.0});
    let new = json!({"value": 1.001});

    let options = DiffOptions {
        epsilon: Some(0.01),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 0); // Within epsilon

    let options = DiffOptions {
        epsilon: Some(0.0001),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 1); // Outside epsilon
}

#[test]
fn test_diff_with_array_id_key() {
    let old = json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    });
    let new = json!({
        "users": [
            {"id": 2, "name": "Bob"},
            {"id": 1, "name": "Alice Updated"}
        ]
    });

    let options = DiffOptions {
        array_id_key: Some("id".to_string()),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();

    // Should detect modification of Alice's name, not array reordering
    assert_eq!(results.len(), 1);
    match &results[0] {
        DiffResult::Modified(path, _, new_val) => {
            assert!(path.contains("[id=1]"));
            assert!(path.contains("name"));
            assert_eq!(new_val, &json!("Alice Updated"));
        }
        _ => panic!("Expected Modified result"),
    }
}

#[test]
fn test_diff_with_ignore_keys_regex() {
    let old = json!({
        "data": "important",
        "timestamp": "2023-01-01",
        "debug_info": "old"
    });
    let new = json!({
        "data": "important",
        "timestamp": "2023-01-02",
        "debug_info": "new"
    });

    let regex = Regex::new(r"^(timestamp|debug_)").unwrap();
    let options = DiffOptions {
        ignore_keys_regex: Some(regex),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 0); // All changes ignored
}

#[test]
fn test_diff_with_path_filter() {
    let old = json!({
        "config": {"value": 1},
        "metadata": {"value": 2}
    });
    let new = json!({
        "config": {"value": 10},
        "metadata": {"value": 20}
    });

    let options = DiffOptions {
        path_filter: Some("config".to_string()),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 1);

    match &results[0] {
        DiffResult::Modified(path, _, _) => {
            assert!(path.contains("config"));
        }
        _ => panic!("Expected Modified result"),
    }
}

#[test]
fn test_diff_with_output_format() {
    let old = json!({"name": "Alice"});
    let new = json!({"name": "Bob"});

    // Test all output formats
    for format in OutputFormat::value_variants() {
        let options = DiffOptions {
            output_format: Some(*format),
            ..Default::default()
        };

        let results = diff(&old, &new, Some(&options)).unwrap();
        assert_eq!(results.len(), 1);

        // Test formatting
        let formatted = format_output(&results, *format).unwrap();
        assert!(!formatted.is_empty());
    }
}

#[test]
fn test_diff_with_memory_optimization() {
    let old = json!({"data": [1, 2, 3]});
    let new = json!({"data": [1, 2, 4]});

    let options = DiffOptions {
        use_memory_optimization: Some(true),
        batch_size: Some(100),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_diff_with_diffx_specific_options() {
    let old = json!({"text": "Hello World"});
    let new = json!({"text": "HELLO WORLD"});

    let diffx_options = DiffxSpecificOptions {
        ignore_case: Some(true),
        ignore_whitespace: Some(false),
        ..Default::default()
    };

    let options = DiffOptions {
        diffx_options: Some(diffx_options),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 0); // Ignored due to case insensitive
}

#[test]
fn test_diff_with_ignore_whitespace() {
    let old = json!({"text": "Hello World"});
    let new = json!({"text": "HelloWorld"});

    let diffx_options = DiffxSpecificOptions {
        ignore_whitespace: Some(true),
        ..Default::default()
    };

    let options = DiffOptions {
        diffx_options: Some(diffx_options),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();
    assert_eq!(results.len(), 0); // Ignored due to whitespace
}

// ============================================================================
// ARRAY HANDLING TESTS
// ============================================================================

#[test]
fn test_diff_arrays_by_index() {
    let old = json!([1, 2, 3]);
    let new = json!([1, 3, 4]);

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 2);
    // Should detect changes at indices 1 and 2
}

#[test]
fn test_diff_arrays_with_id_key() {
    let old = json!([
        {"id": "a", "value": 1},
        {"id": "b", "value": 2}
    ]);
    let new = json!([
        {"id": "b", "value": 20},
        {"id": "c", "value": 3}
    ]);

    let options = DiffOptions {
        array_id_key: Some("id".to_string()),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();

    // Should detect: removed 'a', modified 'b', added 'c'
    assert_eq!(results.len(), 3);
}

#[test]
fn test_diff_arrays_mixed_id_and_index() {
    let old = json!([
        {"id": "a", "value": 1},
        {"value": 2}, // No ID
        {"id": "b", "value": 3}
    ]);
    let new = json!([
        {"id": "b", "value": 30},
        {"value": 20}, // No ID
        {"id": "c", "value": 4}
    ]);

    let options = DiffOptions {
        array_id_key: Some("id".to_string()),
        ..Default::default()
    };

    let results = diff(&old, &new, Some(&options)).unwrap();

    // Should handle both ID-based and index-based comparisons
    assert!(!results.is_empty());
}

// ============================================================================
// COMPLEX DATA STRUCTURES
// ============================================================================

#[test]
fn test_diff_nested_objects() {
    let old = json!({
        "user": {
            "profile": {
                "name": "Alice",
                "settings": {
                    "theme": "dark"
                }
            }
        }
    });
    let new = json!({
        "user": {
            "profile": {
                "name": "Alice",
                "settings": {
                    "theme": "light",
                    "notifications": true
                }
            }
        }
    });

    let results = diff(&old, &new, None).unwrap();

    assert_eq!(results.len(), 2);
    // Should find theme change and notifications addition
}

#[test]
fn test_diff_large_dataset() {
    let mut old_data = serde_json::Map::new();
    let mut new_data = serde_json::Map::new();

    // Create large dataset
    for i in 0..1000 {
        old_data.insert(format!("key_{i}"), json!(i));
        new_data.insert(format!("key_{i}"), json!(i + 1));
    }

    let old = Value::Object(old_data);
    let new = Value::Object(new_data);

    let results = diff(&old, &new, None).unwrap();
    assert_eq!(results.len(), 1000);
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_diff_memory_limit_exceeded() {
    // This test simulates memory limit check
    let options = DiffOptions {
        use_memory_optimization: Some(true),
        ..Default::default()
    };

    // Create reasonably sized data that won't actually exceed limits
    let old = json!({"data": "small"});
    let new = json!({"data": "small_modified"});

    let results = diff(&old, &new, Some(&options));
    assert!(results.is_ok());
}


// ============================================================================
// OUTPUT FORMAT TESTS
// ============================================================================

#[test]
fn test_output_format_parsing() {
    assert_eq!(
        OutputFormat::parse_format("json").unwrap(),
        OutputFormat::Json
    );
    assert_eq!(
        OutputFormat::parse_format("yaml").unwrap(),
        OutputFormat::Yaml
    );
    assert_eq!(
        OutputFormat::parse_format("yml").unwrap(),
        OutputFormat::Yaml
    );
    assert_eq!(
        OutputFormat::parse_format("diffx").unwrap(),
        OutputFormat::Diffx
    );

    assert!(OutputFormat::parse_format("invalid").is_err());
}

#[test]
fn test_all_output_formats() {
    let results = vec![DiffResult::Added("test".to_string(), json!("value"))];

    for format in OutputFormat::value_variants() {
        let output = format_output(&results, *format).unwrap();
        assert!(!output.is_empty());

        match format {
            OutputFormat::Json => assert!(output.contains("{")),
            OutputFormat::Yaml => assert!(output.contains("Added")),
            OutputFormat::Diffx => assert!(output.contains("Added")),
        }
    }
}

// ============================================================================
// PARSER FUNCTION TESTS (Internal Use)
// ============================================================================

#[test]
fn test_parse_json() {
    let content = r#"{"name": "test", "value": 123}"#;
    let result = parse_json(content).unwrap();

    assert_eq!(result["name"], json!("test"));
    assert_eq!(result["value"], json!(123));
}

#[test]
fn test_parse_csv() {
    let content = "name,age\nAlice,30\nBob,25";
    let result = parse_csv(content).unwrap();

    if let Value::Array(records) = result {
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["name"], json!("Alice"));
        assert_eq!(records[0]["age"], json!("30"));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_parse_yaml() {
    let content = "name: test\nvalue: 123";
    let result = parse_yaml(content).unwrap();

    assert_eq!(result["name"], json!("test"));
    assert_eq!(result["value"], json!(123));
}

#[test]
fn test_parse_invalid_json() {
    let content = "invalid json {";
    let result = parse_json(content);
    assert!(result.is_err());
}

// ============================================================================
// UTILITY FUNCTION TESTS (Internal Use)
// ============================================================================

#[test]
fn test_value_type_name() {
    assert_eq!(value_type_name(&json!(null)), "Null");
    assert_eq!(value_type_name(&json!(true)), "Boolean");
    assert_eq!(value_type_name(&json!(123)), "Number");
    assert_eq!(value_type_name(&json!("test")), "String");
    assert_eq!(value_type_name(&json!([])), "Array");
    assert_eq!(value_type_name(&json!({})), "Object");
}

#[test]
fn test_estimate_memory_usage() {
    assert_eq!(estimate_memory_usage(&json!(null)), 0);
    assert_eq!(estimate_memory_usage(&json!(true)), 1);
    assert_eq!(estimate_memory_usage(&json!(123)), 8);
    assert!(estimate_memory_usage(&json!("test")) >= 4);
    assert!(estimate_memory_usage(&json!([])) >= 24);
    assert!(estimate_memory_usage(&json!({})) >= 24);
}

#[test]
fn test_would_exceed_memory_limit() {
    let small = json!({"test": "value"});
    let _large = json!({"data": vec!["x"; 1000000]});

    assert!(!would_exceed_memory_limit(&small, &small));
    // Note: This test depends on the actual memory calculation
}

// ============================================================================
// LIGHTWEIGHT DIFF RESULT TESTS
// ============================================================================

#[test]
fn test_lightweight_diff_result_conversion() {
    let result = DiffResult::Added("test".to_string(), json!({"key": "value"}));
    let lightweight = LightweightDiffResult::from(&result);

    match lightweight {
        LightweightDiffResult::Added(path, value_str) => {
            assert_eq!(path, "test");
            assert!(value_str.contains("key"));
            assert!(value_str.contains("value"));
        }
        _ => panic!("Expected Added result"),
    }
}
