use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("binary exists")
}

#[test]
fn test_optimize_flag_basic() {
    let data1 = serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice", "score": 100},
            {"id": 2, "name": "Bob", "score": 200}
        ]
    });
    
    let data2 = serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice", "score": 150},  // Changed
            {"id": 2, "name": "Bob", "score": 200}
        ]
    });

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string_pretty(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string_pretty(&data2).unwrap()).unwrap();

    // Test optimize flag works
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .assert()
        .success()
        .stdout(predicate::str::contains("users[0].score"));
}

#[test]
fn test_optimize_with_batch_size() {
    let mut large_array = Vec::new();
    for i in 0..1000 {
        large_array.push(serde_json::json!({
            "id": i,
            "name": format!("user_{}", i),
            "data": format!("value_{}", i)
        }));
    }
    
    let data1 = serde_json::json!({"items": large_array});
    let mut data2 = data1.clone();
    data2["items"][500]["name"] = serde_json::json!("modified_user");

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string(&data2).unwrap()).unwrap();

    // Test optimize with custom batch size
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--batch-size")
        .arg("500")
        .assert()
        .success()
        .stdout(predicate::str::contains("items[500].name"));
}

#[test]
fn test_optimize_with_array_id_key() {
    let data1 = serde_json::json!({
        "products": [
            {"sku": "ABC123", "name": "Product A", "price": 100},
            {"sku": "DEF456", "name": "Product B", "price": 200}
        ]
    });
    
    let data2 = serde_json::json!({
        "products": [
            {"sku": "DEF456", "name": "Product B", "price": 250},  // Price changed, order changed
            {"sku": "ABC123", "name": "Product A", "price": 100}
        ]
    });

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string_pretty(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string_pretty(&data2).unwrap()).unwrap();

    // Test optimize with array ID key
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--array-id-key")
        .arg("sku")
        .assert()
        .success()
        .stdout(predicate::str::contains("products[sku=\"DEF456\"].price"));
}

#[test]
fn test_optimize_with_path_filter() {
    let data1 = serde_json::json!({
        "config": {
            "database": {"host": "localhost", "port": 5432},
            "cache": {"enabled": true, "ttl": 300}
        },
        "other": {"value": 123}
    });
    
    let data2 = serde_json::json!({
        "config": {
            "database": {"host": "localhost", "port": 5433},  // Changed
            "cache": {"enabled": true, "ttl": 300}
        },
        "other": {"value": 456}  // Changed but filtered out
    });

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string_pretty(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string_pretty(&data2).unwrap()).unwrap();

    // Test optimize with path filtering
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--path")
        .arg("config.database")
        .assert()
        .success()
        .stdout(predicate::str::contains("config.database.port"))
        .stdout(predicate::str::contains("other").not());
}

#[test]
fn test_optimize_with_ignore_regex() {
    let data1 = serde_json::json!({
        "config": {"host": "localhost", "port": 5432},
        "timestamp": "2023-01-01T00:00:00Z",
        "_internal": "ignore_me"
    });
    
    let data2 = serde_json::json!({
        "config": {"host": "remotehost", "port": 5432},  // Changed
        "timestamp": "2023-01-02T00:00:00Z",  // Changed but ignored
        "_internal": "different_value"  // Changed but ignored
    });

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string_pretty(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string_pretty(&data2).unwrap()).unwrap();

    // Test optimize with regex ignore
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--ignore-keys-regex")
        .arg("^(timestamp|_.*)")
        .assert()
        .success()
        .stdout(predicate::str::contains("config.host"))
        .stdout(predicate::str::contains("timestamp").not())
        .stdout(predicate::str::contains("_internal").not());
}

#[test]
fn test_optimize_json_output() {
    let data1 = serde_json::json!({"key": "value1"});
    let data2 = serde_json::json!({"key": "value2"});

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string(&data2).unwrap()).unwrap();

    // Test optimize with JSON output
    let output = diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--output")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_output: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(json_output.is_array());
    assert!(!json_output.as_array().unwrap().is_empty());
}

#[test]
fn test_optimize_directory_comparison() {
    let temp_dir1 = tempfile::tempdir().unwrap();
    let temp_dir2 = tempfile::tempdir().unwrap();
    
    // Create test files
    let file1_content = serde_json::json!({"config": {"value": 1}});
    let file2_content = serde_json::json!({"config": {"value": 2}});
    
    fs::write(
        temp_dir1.path().join("config.json"),
        serde_json::to_string(&file1_content).unwrap()
    ).unwrap();
    
    fs::write(
        temp_dir2.path().join("config.json"),
        serde_json::to_string(&file2_content).unwrap()
    ).unwrap();

    // Test optimize with directory comparison
    diffx_cmd()
        .arg(temp_dir1.path())
        .arg(temp_dir2.path())
        .arg("--recursive")
        .arg("--optimize")
        .assert()
        .success()
        .stdout(predicate::str::contains("config.value"));
}

#[test]
fn test_standard_vs_optimize_same_results() {
    let data1 = serde_json::json!({
        "array": [
            {"id": 1, "value": "a"},
            {"id": 2, "value": "b"}
        ],
        "config": {"setting": true}
    });
    
    let data2 = serde_json::json!({
        "array": [
            {"id": 1, "value": "changed"},
            {"id": 2, "value": "b"}
        ],
        "config": {"setting": false}
    });

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string(&data2).unwrap()).unwrap();

    // Get standard mode output
    let standard_output = diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--output")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Get optimized mode output
    let optimized_output = diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--optimize")
        .arg("--output")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Parse and compare (results should be functionally equivalent)
    let standard_json: serde_json::Value = serde_json::from_slice(&standard_output).unwrap();
    let optimized_json: serde_json::Value = serde_json::from_slice(&optimized_output).unwrap();
    
    assert_eq!(standard_json.as_array().unwrap().len(), optimized_json.as_array().unwrap().len());
}

#[test]
fn test_batch_size_without_optimize_ignored() {
    let data1 = serde_json::json!({"key": "value1"});
    let data2 = serde_json::json!({"key": "value2"});

    let file1 = NamedTempFile::with_suffix(".json").unwrap();
    let file2 = NamedTempFile::with_suffix(".json").unwrap();
    
    fs::write(&file1, serde_json::to_string(&data1).unwrap()).unwrap();
    fs::write(&file2, serde_json::to_string(&data2).unwrap()).unwrap();

    // Batch size without optimize should work (but be ignored)
    diffx_cmd()
        .arg(file1.path())
        .arg(file2.path())
        .arg("--batch-size")
        .arg("2000")
        .assert()
        .success()
        .stdout(predicate::str::contains("key"));
}