use diffx_core::*;
use serde_json::{json, Value};

/// Test case 1: diffx config1.json config2.json
#[test]
fn test_index_semantic_diff() {
    let v1 = json!({"name": "myapp", "version": "1.0"});
    let v2 = json!({"version": "1.1", "name": "myapp"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "version");
            assert_eq!(old, &json!("1.0"));
            assert_eq!(new, &json!("1.1"));
        }
        _ => panic!("Expected Modified diff result"),
    }
}
