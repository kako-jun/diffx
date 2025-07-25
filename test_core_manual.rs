use diffx_core::*;
use serde_json::json;

fn main() {
    println!("Testing diffx-core diff function...");
    
    // Test 1: No changes
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 1, "b": 2 });
    let differences = diff(&v1, &v2, None, None, None);
    println!("Test 1 (no changes): {} differences", differences.len());
    assert!(differences.is_empty(), "Expected no differences");
    
    // Test 2: Value modified
    let v1 = json!({ "a": 1, "b": 2 });
    let v2 = json!({ "a": 1, "b": 3 });
    let differences = diff(&v1, &v2, None, None, None);
    println!("Test 2 (value modified): {} differences", differences.len());
    assert_eq!(differences.len(), 1, "Expected 1 difference");
    
    // Test 3: Key added
    let v1 = json!({ "a": 1 });
    let v2 = json!({ "a": 1, "b": 2 });
    let differences = diff(&v1, &v2, None, None, None);
    println!("Test 3 (key added): {} differences", differences.len());
    assert_eq!(differences.len(), 1, "Expected 1 difference");
    
    println!("✅ All diffx-core tests passed!");
}