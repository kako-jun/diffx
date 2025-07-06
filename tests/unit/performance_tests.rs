use diffx_core::*;
use serde_json::json;

#[test]
fn test_diff_config_standard_mode() {
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 2, "b": 2 });

    let config = DiffConfig {
        use_memory_optimization: false,
        ..Default::default()
    };

    let differences = diff_with_config(&v1, &v2, &config);
    assert_eq!(differences.len(), 1);
}

#[test]
fn test_diff_config_optimized_mode() {
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 2, "b": 2 });

    let config = DiffConfig {
        use_memory_optimization: true,
        ..Default::default()
    };

    let differences = diff_with_config(&v1, &v2, &config);
    assert_eq!(differences.len(), 1);
}

// Removed progress reporter tests - Unix tools should be pipe-friendly

#[test]
fn test_diff_config_with_array_changes() {
    let v1 = json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
            {"id": 3, "name": "Charlie"}
        ]
    });

    let v2 = json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Robert"},  // Changed
            {"id": 3, "name": "Charlie"},
            {"id": 4, "name": "David"}    // Added
        ]
    });

    let config = DiffConfig {
        use_memory_optimization: false,
        ..Default::default()
    };

    let differences = diff_with_config(&v1, &v2, &config);
    assert!(!differences.is_empty());
}

#[test]
fn test_diff_config_optimized_with_changes() {
    let v1 = json!({
        "data": {
            "items": [1, 2, 3, 4, 5]
        }
    });

    let v2 = json!({
        "data": {
            "items": [1, 2, 6, 4, 5]  // Changed 3 -> 6
        }
    });

    let config = DiffConfig {
        use_memory_optimization: true,
        ..Default::default()
    };

    let differences = diff_with_config(&v1, &v2, &config);
    assert!(!differences.is_empty());
}

#[test]
fn test_memory_usage_estimation() {
    let simple_value = json!(42);
    let complex_value = json!({
        "array": [1, 2, 3],
        "object": {"nested": "value"},
        "string": "hello world"
    });

    let simple_usage = estimate_memory_usage(&simple_value);
    let complex_usage = estimate_memory_usage(&complex_value);

    assert!(complex_usage > simple_usage);
    assert!(simple_usage > 0);
}

#[test]
fn test_diff_algorithms_produce_same_results() {
    let v1 = json!({
        "config": {
            "database": {
                "host": "localhost",
                "port": 5432
            },
            "cache": {
                "enabled": true,
                "ttl": 300
            }
        }
    });

    let v2 = json!({
        "config": {
            "database": {
                "host": "localhost",
                "port": 5433  // Changed
            },
            "cache": {
                "enabled": false,  // Changed
                "ttl": 300
            },
            "logging": {  // Added
                "level": "info"
            }
        }
    });

    // Test both algorithms produce the same results
    let standard_results = diff_standard(&v1, &v2, None, None, None);
    let optimized_results = diff_optimized(&v1, &v2, None, None, None);

    // Both should find the same differences
    assert_eq!(standard_results.len(), optimized_results.len());

    // Results should be functionally equivalent (may differ in order)
    assert!(!standard_results.is_empty());
    assert!(!optimized_results.is_empty());
}

#[test]
fn test_backward_compatibility() {
    let v1 = json!({ "old": "value" });
    let v2 = json!({ "new": "value" });

    // Original diff function should still work exactly as before
    let differences = diff(&v1, &v2, None, None, None);
    assert_eq!(differences.len(), 2); // One removed, one added
}
