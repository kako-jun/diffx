use diffx_core::*;
use serde_json::json;
// Large data performance tests

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_large_array_performance() {
    // Create large arrays (10,000 elements)
    let mut large_array1 = Vec::new();
    let mut large_array2 = Vec::new();

    for i in 0..10000 {
        let item = json!({
            "id": i,
            "name": format!("item_{i}"),
            "value": i * 2,
            "metadata": {
                "category": format!("cat_{}", i % 10),
                "priority": i % 5
            }
        });
        large_array1.push(item.clone());
        large_array2.push(item);
    }

    // Modify a few elements
    large_array2[1000]["value"] = json!(9999);
    large_array2[5000]["name"] = json!("modified_item");
    large_array2[9000]["metadata"]["priority"] = json!(10);

    let v1 = json!({"items": large_array1});
    let v2 = json!({"items": large_array2});

    // Test standard mode
    let start = std::time::Instant::now();
    let standard_results = diff(&v1, &v2, None, None, None);
    let standard_duration = start.elapsed();

    // Test optimized mode
    let config = DiffConfig {
        use_memory_optimization: true,
        batch_size: 1000,
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let optimized_results = diff_with_config(&v1, &v2, &config);
    let optimized_duration = start.elapsed();

    // Results should be equivalent
    assert_eq!(standard_results.len(), optimized_results.len());
    assert_eq!(standard_results.len(), 3); // 3 modifications

    // Print performance comparison
    println!("Standard mode: {standard_duration:?}");
    println!("Optimized mode: {optimized_duration:?}");
    println!(
        "Speedup: {:.2}x",
        standard_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64
    );
}

#[test]
fn test_deep_nested_structure_performance() {
    fn create_deep_structure(depth: usize, base_value: i32) -> serde_json::Value {
        if depth == 0 {
            return json!({"value": base_value, "data": format!("data_{base_value}")});
        }

        let mut obj = serde_json::Map::new();
        for i in 0..3 {
            let key = format!("level_{i}");
            obj.insert(key, create_deep_structure(depth - 1, base_value * 3 + i));
        }
        json!(obj)
    }

    // Create deep nested structures (8 levels deep)
    let v1 = create_deep_structure(8, 1);
    let mut v2 = v1.clone();

    // Modify a deep value
    v2["level_1"]["level_2"]["level_1"]["level_0"]["value"] = json!(9999);

    let config_standard = DiffConfig {
        use_memory_optimization: false,
        ..Default::default()
    };

    let config_optimized = DiffConfig {
        use_memory_optimization: true,
        batch_size: 100,
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let standard_results = diff_with_config(&v1, &v2, &config_standard);
    let standard_duration = start.elapsed();

    let start = std::time::Instant::now();
    let optimized_results = diff_with_config(&v1, &v2, &config_optimized);
    let optimized_duration = start.elapsed();

    // Results should be equivalent
    assert_eq!(standard_results.len(), optimized_results.len());
    assert!(!standard_results.is_empty());

    println!("Deep structure - Standard: {standard_duration:?}");
    println!("Deep structure - Optimized: {optimized_duration:?}");
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_memory_usage_estimation() {
    let small_value = json!({"key": "value"});
    let medium_value = json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ],
        "config": {
            "database": {"host": "localhost", "port": 5432},
            "cache": {"enabled": true}
        }
    });

    let mut large_object = serde_json::Map::new();
    for i in 0..1000 {
        large_object.insert(
            format!("key_{i}"),
            json!({
                "value": i,
                "data": format!("data_string_{i}"),
                "nested": {
                    "field1": i * 2,
                    "field2": format!("nested_{i}")
                }
            }),
        );
    }
    let large_value = json!(large_object);

    let small_usage = estimate_memory_usage(&small_value);
    let medium_usage = estimate_memory_usage(&medium_value);
    let large_usage = estimate_memory_usage(&large_value);

    assert!(small_usage > 0);
    assert!(medium_usage > small_usage);
    assert!(large_usage > medium_usage);

    println!("Small value memory usage: {small_usage} bytes");
    println!("Medium value memory usage: {medium_usage} bytes");
    println!("Large value memory usage: {large_usage} bytes");

    // Test memory limit check
    assert!(!would_exceed_memory_limit(&small_value, &small_value));
    assert!(!would_exceed_memory_limit(&medium_value, &medium_value));

    // Large values might or might not exceed limit depending on implementation
    let exceeds = would_exceed_memory_limit(&large_value, &large_value);
    println!("Large value exceeds memory limit: {exceeds}");
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_batch_processing_effectiveness() {
    // Create data that benefits from batching
    let mut large_map1 = serde_json::Map::new();
    let mut large_map2 = serde_json::Map::new();

    for i in 0..5000 {
        let key = format!("field_{i:04}");
        let value1 = json!({
            "id": i,
            "data": format!("original_{i}"),
            "timestamp": format!("2023-01-01T{:02}:00:00Z", i % 24)
        });

        let mut value2 = value1.clone();
        // Modify every 100th item
        if i % 100 == 0 {
            value2["data"] = json!(format!("modified_{i}"));
        }

        large_map1.insert(key.clone(), value1);
        large_map2.insert(key, value2);
    }

    let v1 = json!(large_map1);
    let v2 = json!(large_map2);

    // Test different batch sizes
    let batch_sizes = vec![100, 500, 1000, 2000];

    for &batch_size in &batch_sizes {
        let config = DiffConfig {
            use_memory_optimization: true,
            batch_size,
            ..Default::default()
        };

        let start = std::time::Instant::now();
        let results = diff_with_config(&v1, &v2, &config);
        let duration = start.elapsed();

        // Should find 50 differences (every 100th of 5000 items)
        assert_eq!(results.len(), 50);

        println!("Batch size {batch_size}: {duration:?}");
    }
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_array_id_key_performance() {
    // Create large array with ID keys
    let mut users1 = Vec::new();
    let mut users2 = Vec::new();

    for i in 0..1000 {
        let user = json!({
            "id": i,
            "username": format!("user_{i}"),
            "email": format!("user{i}@example.com"),
            "profile": {
                "age": 20 + (i % 50),
                "location": format!("City {}", i % 10)
            }
        });
        users1.push(user.clone());
        users2.push(user);
    }

    // Shuffle array and modify some entries
    users2.reverse(); // Completely different order
    users2[100]["profile"]["age"] = json!(99);
    users2[500]["email"] = json!("changed@example.com");

    let v1 = json!({"users": users1});
    let v2 = json!({"users": users2});

    // Test without ID key (positional comparison)
    let start = std::time::Instant::now();
    let positional_results = diff(&v1, &v2, None, None, None);
    let positional_duration = start.elapsed();

    // Test with ID key (smart comparison)
    let start = std::time::Instant::now();
    let id_results = diff(&v1, &v2, None, None, Some("id"));
    let id_duration = start.elapsed();

    // ID-based comparison should find fewer differences
    assert!(id_results.len() < positional_results.len());
    assert_eq!(id_results.len(), 2); // Only 2 actual changes

    println!(
        "Positional comparison: {} diffs in {positional_duration:?}",
        positional_results.len()
    );
    println!(
        "ID-based comparison: {} diffs in {id_duration:?}",
        id_results.len()
    );

    // Test optimized mode with ID key
    let config = DiffConfig {
        use_memory_optimization: true,
        array_id_key: Some("id".to_string()),
        batch_size: 200,
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let optimized_results = diff_with_config(&v1, &v2, &config);
    let optimized_duration = start.elapsed();

    assert_eq!(optimized_results.len(), 2); // Same as non-optimized ID-based
    println!(
        "Optimized ID-based: {} diffs in {optimized_duration:?}",
        optimized_results.len()
    );
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_regex_filtering_performance() {
    // Create data with many keys that will be filtered
    let mut data1 = serde_json::Map::new();
    let mut data2 = serde_json::Map::new();

    for i in 0..1000 {
        // Add keys that should be ignored
        data1.insert(
            format!("_internal_{i}"),
            json!(format!("internal_value_{i}")),
        );
        data1.insert(
            format!("timestamp_{i}"),
            json!(format!("2023-01-01T{:02}:00:00Z", i % 24)),
        );

        // Add keys that should be compared
        data1.insert(format!("config_{i}"), json!({"value": i}));

        // Same for data2, but with changes
        data2.insert(
            format!("_internal_{i}"),
            json!(format!("different_internal_{i}")),
        ); // Should be ignored
        data2.insert(
            format!("timestamp_{i}"),
            json!(format!("2023-01-02T{:02}:00:00Z", i % 24)),
        ); // Should be ignored
        data2.insert(
            format!("config_{i}"),
            json!({"value": if i % 10 == 0 { i + 1 } else { i }}),
        ); // Should be compared
    }

    let v1 = json!(data1);
    let v2 = json!(data2);

    let regex = regex::Regex::new(r"^(_.*|timestamp_.*)").unwrap();

    // Test without regex (all differences)
    let start = std::time::Instant::now();
    let all_results = diff(&v1, &v2, None, None, None);
    let all_duration = start.elapsed();

    // Test with regex filtering
    let start = std::time::Instant::now();
    let filtered_results = diff(&v1, &v2, Some(&regex), None, None);
    let filtered_duration = start.elapsed();

    // Filtered should have fewer differences
    assert!(filtered_results.len() < all_results.len());
    assert_eq!(filtered_results.len(), 100); // Only config changes (every 10th of 1000)

    println!("All differences: {} in {all_duration:?}", all_results.len());
    println!(
        "Filtered differences: {} in {filtered_duration:?}",
        filtered_results.len()
    );

    // Test optimized mode with regex
    let config = DiffConfig {
        use_memory_optimization: true,
        ignore_keys_regex: Some(regex),
        batch_size: 200,
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let optimized_results = diff_with_config(&v1, &v2, &config);
    let optimized_duration = start.elapsed();

    assert_eq!(optimized_results.len(), 100); // Same as non-optimized filtered
    println!(
        "Optimized filtered: {} in {optimized_duration:?}",
        optimized_results.len()
    );
}
