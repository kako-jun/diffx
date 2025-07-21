// Unit tests for diffx core components
// Test individual functions and modules in isolation

use diffx_core::{
    diff, diff_optimized, diff_standard, diff_with_config, estimate_memory_usage, parse_csv,
    parse_ini, parse_xml, value_type_name, would_exceed_memory_limit, DiffConfig, DiffResult,
    LightweightDiffResult,
};
use regex::Regex;
use serde_json::Value;

#[test]
fn test_diff_result_serialization() {
    let result = DiffResult::Added(
        "test_key".to_string(),
        Value::String("test_value".to_string()),
    );
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(serialized.contains("Added"));
    assert!(serialized.contains("test_key"));
    assert!(serialized.contains("test_value"));
}

#[test]
fn test_diff_result_modified() {
    let old_value = Value::Number(42.into());
    let new_value = Value::Number(84.into());
    let result =
        DiffResult::Modified("score".to_string(), old_value.clone(), new_value.clone());

    match result {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "score");
            assert_eq!(old, old_value);
            assert_eq!(new, new_value);
        }
        _ => panic!("Expected Modified result"),
    }
}

#[test]
fn test_diff_result_type_changed() {
    let old_value = Value::String("42".to_string());
    let new_value = Value::Number(42.into());
    let result =
        DiffResult::TypeChanged("value".to_string(), old_value.clone(), new_value.clone());

    match result {
        DiffResult::TypeChanged(path, old, new) => {
            assert_eq!(path, "value");
            assert_eq!(old, old_value);
            assert_eq!(new, new_value);
        }
        _ => panic!("Expected TypeChanged result"),
    }
}

#[test]
fn test_lightweight_diff_result_conversion() {
    let result = DiffResult::Added("key".to_string(), Value::String("value".to_string()));
    let lightweight: LightweightDiffResult = (&result).into();

    match lightweight {
        LightweightDiffResult::Added(path, value) => {
            assert_eq!(path, "key");
            assert_eq!(value, "\"value\"");
        }
        _ => panic!("Expected Added lightweight result"),
    }
}

#[test]
fn test_diff_config_default() {
    let config = DiffConfig::default();
    assert!(config.ignore_keys_regex.is_none());
    assert!(config.epsilon.is_none());
    assert!(config.array_id_key.is_none());
    assert!(!config.use_memory_optimization);
    assert_eq!(config.batch_size, 1000);
    assert!(!config.ignore_whitespace);
    assert!(!config.ignore_case);
}

#[test]
fn test_diff_config_custom() {
    let regex = Regex::new("test").unwrap();
    let config = DiffConfig {
        ignore_keys_regex: Some(regex),
        epsilon: Some(0.001),
        array_id_key: Some("id".to_string()),
        use_memory_optimization: true,
        batch_size: 500,
        ignore_whitespace: true,
        ignore_case: true,
    };

    assert!(config.ignore_keys_regex.is_some());
    assert_eq!(config.epsilon, Some(0.001));
    assert_eq!(config.array_id_key, Some("id".to_string()));
    assert!(config.use_memory_optimization);
    assert_eq!(config.batch_size, 500);
    assert!(config.ignore_whitespace);
    assert!(config.ignore_case);
}

#[test]
fn test_diff_identical_values() {
    let value1 = serde_json::json!({"name": "Alice", "age": 30});
    let value2 = serde_json::json!({"name": "Alice", "age": 30});

    let results = diff(&value1, &value2, None, None, None);
    assert!(results.is_empty());
}

#[test]
fn test_diff_simple_modification() {
    let value1 = serde_json::json!({"name": "Alice", "age": 30});
    let value2 = serde_json::json!({"name": "Alice", "age": 31});

    let results = diff(&value1, &value2, None, None, None);
    assert_eq!(results.len(), 1);

    match &results[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "age");
            assert_eq!(old, &Value::Number(30.into()));
            assert_eq!(new, &Value::Number(31.into()));
        }
        _ => panic!("Expected Modified result"),
    }
}

#[test]
fn test_diff_with_epsilon() {
    let value1 = serde_json::json!({"value": 1.0001});
    let value2 = serde_json::json!({"value": 1.0002});

    // Without epsilon - should detect difference
    let results_strict = diff(&value1, &value2, None, None, None);
    assert_eq!(results_strict.len(), 1);

    // With epsilon - should ignore small difference
    let results_loose = diff(&value1, &value2, None, Some(0.001), None);
    assert_eq!(results_loose.len(), 0);
}

#[test]
fn test_diff_standard_vs_optimized() {
    let value1 = serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice", "age": 30},
            {"id": 2, "name": "Bob", "age": 25}
        ]
    });
    let value2 = serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice", "age": 31},
            {"id": 2, "name": "Bob", "age": 25}
        ]
    });

    let results_standard = diff_standard(&value1, &value2, None, None, None);
    let results_optimized = diff_optimized(&value1, &value2, None, None, None);

    // Both algorithms should produce the same results
    assert_eq!(results_standard.len(), results_optimized.len());
    assert_eq!(results_standard.len(), 1);
}

#[test]
fn test_value_type_name() {
    assert_eq!(value_type_name(&Value::Null), "Null");
    assert_eq!(value_type_name(&Value::Bool(true)), "Boolean");
    assert_eq!(value_type_name(&Value::Number(42.into())), "Number");
    assert_eq!(
        value_type_name(&Value::String("test".to_string())),
        "String"
    );
    assert_eq!(value_type_name(&serde_json::json!([])), "Array");
    assert_eq!(value_type_name(&serde_json::json!({})), "Object");
}

#[test]
fn test_estimate_memory_usage() {
    // Test basic types
    assert_eq!(estimate_memory_usage(&Value::Null), 0);
    assert_eq!(estimate_memory_usage(&Value::Bool(true)), 1);
    assert_eq!(estimate_memory_usage(&Value::Number(42.into())), 8);

    // Test string
    let string_val = Value::String("hello".to_string());
    assert_eq!(estimate_memory_usage(&string_val), 5);

    // Test array
    let array_val = serde_json::json!([1, 2, 3]);
    let array_usage = estimate_memory_usage(&array_val);
    assert!(array_usage > 24); // 3 * 8 + overhead

    // Test object
    let object_val = serde_json::json!({"key": "value"});
    let object_usage = estimate_memory_usage(&object_val);
    assert!(object_usage > 8); // key length + value length + overhead
}

#[test]
fn test_would_exceed_memory_limit() {
    // Small values should not exceed limit
    let small_value = serde_json::json!({"test": "value"});
    assert!(!would_exceed_memory_limit(&small_value, &small_value));

    // Very large theoretical values would exceed limit
    // (We can't actually create such large values in a unit test)
    let medium_value = serde_json::json!({
        "data": vec!["test"; 1000]
    });
    assert!(!would_exceed_memory_limit(&medium_value, &medium_value));
}

#[test]
fn test_parse_csv() {
    let csv_data = "name,age,city\nAlice,30,New York\nBob,25,Boston";
    let result = parse_csv(csv_data).unwrap();

    assert!(result.is_array());
    let array = result.as_array().unwrap();
    assert_eq!(array.len(), 2);

    // Check first record
    let first_record = &array[0];
    assert!(first_record.is_object());
    let first_obj = first_record.as_object().unwrap();
    assert_eq!(
        first_obj.get("name").unwrap(),
        &Value::String("Alice".to_string())
    );
    assert_eq!(
        first_obj.get("age").unwrap(),
        &Value::String("30".to_string())
    );
    assert_eq!(
        first_obj.get("city").unwrap(),
        &Value::String("New York".to_string())
    );
}

#[test]
fn test_parse_xml() {
    let xml_data = r#"<root><item id="1"><name>Test</name><value>123</value></item></root>"#;
    let result = parse_xml(xml_data);

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.is_object());
}

#[test]
fn test_parse_ini() {
    let ini_data = r#"
[section1]
key1=value1
key2=value2

[section2]
key3=value3
"#;
    let result = parse_ini(ini_data);

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.is_object());

    let obj = parsed.as_object().unwrap();
    assert!(obj.contains_key("section1"));
    assert!(obj.contains_key("section2"));
}

#[test]
fn test_diff_array_modification() {
    let value1 = serde_json::json!([1, 2, 3]);
    let value2 = serde_json::json!([1, 2, 4]);

    let results = diff(&value1, &value2, None, None, None);
    assert_eq!(results.len(), 1);

    match &results[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "[2]");
            assert_eq!(old, &Value::Number(3.into()));
            assert_eq!(new, &Value::Number(4.into()));
        }
        _ => panic!("Expected Modified result for array element"),
    }
}

#[test]
fn test_diff_with_config_ignore_whitespace() {
    let value1 = serde_json::json!({"text": "Hello  World"});
    let value2 = serde_json::json!({"text": "Hello World"});

    let config = DiffConfig {
        ignore_whitespace: true,
        ..Default::default()
    };

    let results = diff_with_config(&value1, &value2, &config);
    assert_eq!(results.len(), 0); // Should ignore whitespace differences
}

#[test]
fn test_diff_with_config_ignore_case() {
    let value1 = serde_json::json!({"text": "Hello"});
    let value2 = serde_json::json!({"text": "hello"});

    let config = DiffConfig {
        ignore_case: true,
        ..Default::default()
    };

    let results = diff_with_config(&value1, &value2, &config);
    assert_eq!(results.len(), 0); // Should ignore case differences
}