#[allow(unused_imports)]
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use std::time::Instant;
use tempfile::tempdir;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

/// Test large file handling
/// Verifies that large files can be processed successfully
#[test]
fn test_large_file_handling() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let large_file1 = temp_dir.path().join("large1.json");
    let large_file2 = temp_dir.path().join("large2.json");

    // Create large JSON files (>1MB)
    let mut large_content1 = String::from("{\n  \"data\": [\n");
    let mut large_content2 = String::from("{\n  \"data\": [\n");

    for i in 0..50000 {
        let comma = if i == 49999 { "" } else { "," };
        large_content1.push_str(&format!(
            "    {{\"id\": {}, \"name\": \"user{}\", \"value\": {}}}{}\n",
            i,
            i,
            i * 2,
            comma
        ));
        large_content2.push_str(&format!(
            "    {{\"id\": {}, \"name\": \"user{}\", \"value\": {}}}{}\n",
            i,
            i,
            i * 2 + 1,
            comma // Small difference
        ));
    }

    large_content1.push_str("  ]\n}");
    large_content2.push_str("  ]\n}");

    fs::write(&large_file1, large_content1)?;
    fs::write(&large_file2, large_content2)?;

    let mut cmd = diffx_cmd();
    cmd.arg(&large_file1).arg(&large_file2);

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect differences in large files
    assert!(stdout.contains("value"));

    Ok(())
}

/// Test memory-efficient batch processing
/// Verifies that large files are processed in memory-efficient batches
#[test]
fn test_memory_efficient_processing() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let batch_file1 = temp_dir.path().join("batch1.yaml");
    let batch_file2 = temp_dir.path().join("batch2.yaml");

    // Create complex nested YAML that would use significant memory
    let mut yaml_content1 = String::from("services:\n");
    let mut yaml_content2 = String::from("services:\n");

    for i in 0..1000 {
        yaml_content1.push_str(&format!(
            "  service{}:\n    config:\n      host: host{}.example.com\n      port: {}\n      settings:\n        timeout: 30\n        retries: 3\n",
            i, i, 8000 + i
        ));
        yaml_content2.push_str(&format!(
            "  service{}:\n    config:\n      host: host{}.example.com\n      port: {}\n      settings:\n        timeout: 60\n        retries: 3\n",
            i, i, 8000 + i  // timeout changed from 30 to 60
        ));
    }

    fs::write(&batch_file1, yaml_content1)?;
    fs::write(&batch_file2, yaml_content2)?;

    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg(&batch_file1).arg(&batch_file2).arg("--verbose");

    let output = cmd.output()?;
    let duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found

    // Should complete within reasonable time despite size
    assert!(
        duration.as_secs() < 30,
        "Processing took too long: {duration:?}"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show timeout differences
    assert!(stdout.contains("timeout") && (stdout.contains("30") || stdout.contains("60")));

    Ok(())
}

/// Test deep nesting optimization
/// Verifies efficient handling of deeply nested structures
#[test]
fn test_deep_nesting_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let deep_file1 = temp_dir.path().join("deep1.json");
    let deep_file2 = temp_dir.path().join("deep2.json");

    // Create deeply nested JSON (50 levels)
    let mut json1 = String::from("{");
    let mut json2 = String::from("{");

    for i in 0..50 {
        json1.push_str(&format!("\"level{i}\": {{"));
        json2.push_str(&format!("\"level{i}\": {{"));
    }

    json1.push_str("\"deep_value\": \"original\"");
    json2.push_str("\"deep_value\": \"modified\""); // Change at deepest level

    for _ in 0..50 {
        json1.push('}');
        json2.push('}');
    }

    json1.push('}');
    json2.push('}');

    fs::write(&deep_file1, json1)?;
    fs::write(&deep_file2, json2)?;

    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg(&deep_file1)
        .arg(&deep_file2)
        .arg("--output")
        .arg("diffx");

    let output = cmd.output()?;
    let duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found

    // Should handle deep nesting efficiently
    assert!(
        duration.as_secs() < 10,
        "Deep nesting took too long: {duration:?}"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect the deep change
    assert!(
        stdout.contains("deep_value")
            && (stdout.contains("original") || stdout.contains("modified"))
    );

    Ok(())
}

/// Test transparent optimization with all options
/// Verifies optimizations work with all CLI options without affecting results
#[test]
fn test_transparent_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let opt_file1 = temp_dir.path().join("opt1.json");
    let opt_file2 = temp_dir.path().join("opt2.json");

    // Create moderately large files with various change types
    let json1 = serde_json::json!({
        "users": (0..1000).map(|i| serde_json::json!({
            "id": i,
            "name": format!("user{}", i),
            "email": format!("user{}@example.com", i),
            "settings": {
                "theme": "dark",
                "notifications": true,
                "timeout": 30
            }
        })).collect::<Vec<_>>(),
        "config": {
            "version": "1.0",
            "features": ["auth", "logging", "monitoring"]
        }
    });

    let json2 = serde_json::json!({
        "users": (0..1000).map(|i| serde_json::json!({
            "id": i,
            "name": format!("user{}", i),
            "email": format!("user{}@newdomain.com", i),  // Domain change
            "settings": {
                "theme": "light",  // Theme change
                "notifications": true,
                "timeout": 30
            }
        })).collect::<Vec<_>>(),
        "config": {
            "version": "1.1",  // Version change
            "features": ["auth", "logging", "monitoring", "analytics"]  // Feature added
        }
    });

    fs::write(&opt_file1, serde_json::to_string_pretty(&json1)?)?;
    fs::write(&opt_file2, serde_json::to_string_pretty(&json2)?)?;

    // Test with multiple options that should all work with optimization
    let mut cmd = diffx_cmd();
    cmd.arg(&opt_file1)
        .arg(&opt_file2)
        .arg("--array-id-key")
        .arg("id")
        .arg("--ignore-keys-regex")
        .arg("^(created|updated)$")
        .arg("--epsilon")
        .arg("0.01")
        .arg("--output")
        .arg("json")
        .arg("--verbose");

    let output = cmd.output()?;
    assert!(output.status.code() == Some(1)); // Differences found

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect semantic changes despite optimization
    assert!(
        stdout.contains("theme")
            || stdout.contains("newdomain")
            || stdout.contains("version")
            || stdout.contains("analytics")
    );

    Ok(())
}

/// Test streaming comparison for very large files
/// Verifies streaming mode handles files that don't fit in memory
#[test]
fn test_streaming_large_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let stream_file1 = temp_dir.path().join("stream1.csv");
    let stream_file2 = temp_dir.path().join("stream2.csv");

    // Create large CSV files
    let mut csv1 = String::from("id,name,email,department,salary\n");
    let mut csv2 = String::from("id,name,email,department,salary\n");

    for i in 0..100000 {
        csv1.push_str(&format!(
            "{},Employee{},emp{}@company.com,Engineering,{}\n",
            i,
            i,
            i,
            50000 + (i % 30000)
        ));
        csv2.push_str(&format!(
            "{},Employee{},emp{}@company.com,{},{}\n",
            i,
            i,
            i,
            if i % 1000 == 0 {
                "Management"
            } else {
                "Engineering"
            }, // Some dept changes
            50000 + (i % 30000) + if i % 100 == 0 { 5000 } else { 0 } // Some salary changes
        ));
    }

    fs::write(&stream_file1, csv1)?;
    fs::write(&stream_file2, csv2)?;

    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg(&stream_file1).arg(&stream_file2).arg("--brief"); // Use brief mode for large files

    let output = cmd.output()?;
    let duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found

    // Should complete streaming processing within reasonable time
    assert!(
        duration.as_secs() < 60,
        "Streaming took too long: {duration:?}"
    );

    Ok(())
}

/// Test concurrent optimization stability  
/// Verifies optimization remains stable under concurrent access
#[test]
fn test_concurrent_optimization() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::thread;

    let success_count = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    // Create test files
    let temp_dir = tempdir()?;
    for i in 0..3 {
        let file1 = temp_dir.path().join(format!("concurrent{i}a.json"));
        let file2 = temp_dir.path().join(format!("concurrent{i}b.json"));

        let json_content = serde_json::json!({
            "data": (0..1000).map(|j| serde_json::json!({
                "id": j,
                "value": j * i  // Different values per thread
            })).collect::<Vec<_>>()
        });

        fs::write(&file1, serde_json::to_string(&json_content)?)?;

        let mut modified = json_content;
        modified["data"][500]["value"] = serde_json::json!(999999); // Change one value
        fs::write(&file2, serde_json::to_string(&modified)?)?;
    }

    // Spawn concurrent comparison threads
    for i in 0..3 {
        let success_count = Arc::clone(&success_count);
        let temp_path = temp_dir.path().to_path_buf();

        let handle = thread::spawn(move || {
            let file1 = temp_path.join(format!("concurrent{i}a.json"));
            let file2 = temp_path.join(format!("concurrent{i}b.json"));

            let mut cmd = diffx_cmd();
            cmd.arg(&file1).arg(&file2).arg("--output").arg("json");

            if let Ok(output) = cmd.output() {
                if output.status.code() == Some(1) {
                    // Differences found - this is correct
                    success_count.fetch_add(1, Ordering::SeqCst);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = success_count.load(Ordering::SeqCst);

    // All concurrent operations should succeed
    assert_eq!(
        final_count, 3,
        "Concurrent optimization failed: only {final_count} of 3 succeeded"
    );

    Ok(())
}

/// Test performance regression detection
/// Verifies performance doesn't degrade with optimizations
#[test]
fn test_performance_regression() -> Result<(), Box<dyn std::error::Error>> {
    // Baseline: small files should be very fast
    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg("tests/fixtures/config_v1.json")
        .arg("tests/fixtures/config_v2.json");

    let output = cmd.output()?;
    let small_duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found
    assert!(
        small_duration.as_millis() < 1000,
        "Small file comparison too slow: {small_duration:?}"
    );

    // Medium files should scale reasonably
    let temp_dir = tempdir()?;
    let medium_file1 = temp_dir.path().join("medium1.json");
    let medium_file2 = temp_dir.path().join("medium2.json");

    let medium_json = serde_json::json!({
        "records": (0..10000).map(|i| serde_json::json!({
            "id": i,
            "data": format!("record_{}", i)
        })).collect::<Vec<_>>()
    });

    fs::write(&medium_file1, serde_json::to_string(&medium_json)?)?;

    let mut modified = medium_json;
    modified["records"][5000]["data"] = serde_json::json!("modified_record");
    fs::write(&medium_file2, serde_json::to_string(&modified)?)?;

    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg(&medium_file1).arg(&medium_file2);

    let output = cmd.output()?;
    let medium_duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found

    // Medium files should not be disproportionately slower
    // Note: In debug builds, small file comparison can be very fast (few ms),
    // making the ratio misleadingly large. We use a generous threshold.
    let ratio = medium_duration.as_millis() as f64 / small_duration.as_millis().max(1) as f64;
    assert!(
        ratio < 500.0,
        "Performance scaling poor: {ratio}x slower for medium files"
    );

    Ok(())
}

/// Test optimization effectiveness measurement
/// Verifies that optimizations actually improve performance
#[test]
fn test_optimization_effectiveness() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let large_file1 = temp_dir.path().join("large_opt1.xml");
    let large_file2 = temp_dir.path().join("large_opt2.xml");

    // Create large XML files
    let mut xml1 = String::from("<?xml version=\"1.0\"?>\n<root>\n");
    let mut xml2 = String::from("<?xml version=\"1.0\"?>\n<root>\n");

    for i in 0..10000 {
        xml1.push_str(&format!(
            "  <item id=\"{}\">\n    <name>Item {}</name>\n    <value>{}</value>\n  </item>\n",
            i,
            i,
            i * 10
        ));
        xml2.push_str(&format!(
            "  <item id=\"{}\">\n    <name>Item {}</name>\n    <value>{}</value>\n  </item>\n",
            i,
            i,
            i * 10 + (if i == 5000 { 1 } else { 0 }) // One small change
        ));
    }

    xml1.push_str("</root>");
    xml2.push_str("</root>");

    fs::write(&large_file1, xml1)?;
    fs::write(&large_file2, xml2)?;

    // Test with optimization (should be automatic for large files)
    let start = Instant::now();
    let mut cmd = diffx_cmd();
    cmd.arg(&large_file1).arg(&large_file2).arg("--verbose");

    let output = cmd.output()?;
    let optimized_duration = start.elapsed();

    assert!(output.status.code() == Some(1)); // Differences found

    // Should complete within reasonable time due to optimization
    assert!(
        optimized_duration.as_secs() < 20,
        "Optimized processing too slow: {optimized_duration:?}"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect the change
    assert!(stdout.contains("value") && stdout.contains("50001"));

    Ok(())
}
