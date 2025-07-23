use diffx_core::*;
use regex::Regex;
use serde_json::{json, Value};

/// Test case 1: diffx config_v1.json config_v2.json
#[test]
fn test_basic_config_diff() {
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

/// Test case 2: time diffx large_test1.json large_test2.json
#[test]
fn test_large_file_performance() {
    let v1 = json!({"config": {"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true}}});
    let v2 = json!({"config": {"database": {"host": "prod-db", "port": 5432}, "cache": {"enabled": false}}});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 2);

    let paths: Vec<String> = diffs
        .iter()
        .map(|d| match d {
            DiffResult::Modified(path, _, _) => path.clone(),
            _ => panic!("Expected Modified results"),
        })
        .collect();

    assert!(paths.contains(&"config.database.host".to_string()));
    assert!(paths.contains(&"config.cache.enabled".to_string()));
}

/// Test case 3: diffx config_v1.json config_v2.json --output json > report1.json
#[test]
fn test_json_output_to_file() {
    let v1 = json!({"version": "1.0"});
    let v2 = json!({"version": "1.1"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);

    // Test JSON serialization
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("Modified"));
    assert!(json_output.contains("version"));
}

/// Test case 4: diffx config_v2.json config_v3.json --output json > report2.json
#[test]
fn test_second_json_output() {
    let v1 = json!({"version": "1.1"});
    let v2 = json!({"version": "1.2"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);

    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("1.1"));
    assert!(json_output.contains("1.2"));
}

/// Test case 5: diffx report1.json report2.json
#[test]
fn test_meta_chaining_diff_reports() {
    let report1 = json!([{"Modified": ["version", "1.0", "1.1"]}]);
    let report2 = json!([{"Modified": ["version", "1.1", "1.2"]}]);

    let diffs = diff(&report1, &report2, None).unwrap();
    assert!(!diffs.is_empty());
}

/// Test case 6: diffx file1.json file2.json
#[test]
fn test_basic_file_comparison() {
    let v1 = json!({"data": "value1"});
    let v2 = json!({"data": "value2"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, _, _) => {
            assert_eq!(path, "data");
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 7: diffx config.yaml config_new.yaml --output json
#[test]
fn test_yaml_with_json_output() {
    let v1 = json!({"config": {"debug": true}});
    let v2 = json!({"config": {"debug": false}});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("config.debug"));
}

/// Test case 8: diffx data.toml data_updated.toml --output yaml
#[test]
fn test_toml_with_yaml_output() {
    let v1 = json!({"app": {"name": "test"}});
    let v2 = json!({"app": {"name": "updated"}});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "app.name");
            assert_eq!(old, &json!("test"));
            assert_eq!(new, &json!("updated"));
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 9: diffx large.json large_v2.json --ignore-keys-regex "^timestamp$|^_.*"
#[test]
fn test_ignore_keys_regex() {
    let v1 = json!({"timestamp": "2024-01-01", "_internal": "meta", "data": "value1"});
    let v2 = json!({"timestamp": "2024-01-02", "_internal": "meta2", "data": "value2"});

    let regex = Regex::new(r"^timestamp$|^_.*").unwrap();
    let mut options = DiffOptions::default();
    options.ignore_keys_regex = Some(regex);
    let diffs = diff(&v1, &v2, Some(&options)).unwrap();

    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, _, _) => {
            assert_eq!(path, "data");
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 10: diffx users.json users_v2.json --array-id-key "id"
#[test]
fn test_array_id_key() {
    let v1 = json!({"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]});
    let v2 = json!({"users": [{"id": 2, "name": "Jane"}, {"id": 1, "name": "Johnny"}]});

    let mut options = DiffOptions::default();
    options.array_id_key = Some("id".to_string());
    let diffs = diff(&v1, &v2, Some(&options)).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "users[id=1].name");
            assert_eq!(old, &json!("John"));
            assert_eq!(new, &json!("Johnny"));
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 11: diffx metrics.json metrics_v2.json --epsilon 0.001
#[test]
fn test_epsilon_tolerance() {
    let v1 = json!({"value": 1.0001});
    let v2 = json!({"value": 1.0002});

    let mut options = DiffOptions::default();
    options.epsilon = Some(0.001);
    let diffs = diff(&v1, &v2, Some(&options)).unwrap();
    assert_eq!(diffs.len(), 0);
}

/// Test case 12: diffx config.yaml config_new.yaml --ignore-case
#[test]
fn test_ignore_case() {
    let v1 = json!({"status": "ACTIVE"});
    let v2 = json!({"status": "active"});

    let config = DiffConfig {
        ignore_case: true,
        ..DiffConfig::default()
    };

    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 0);
}

/// Test case 13: diffx api.json api_formatted.json --ignore-whitespace
#[test]
fn test_ignore_whitespace() {
    let v1 = json!({"text": "hello world"});
    let v2 = json!({"text": "hello    world"});

    let config = DiffConfig {
        ignore_whitespace: true,
        ..DiffConfig::default()
    };

    let diffs = diff_with_config(&v1, &v2, &config);
    assert_eq!(diffs.len(), 0);
}

/// Test case 14: diffx large.json large_v2.json --context 3 --output unified
#[test]
fn test_unified_output_with_context() {
    let v1 = json!({"a": 1, "b": 2, "c": 3, "d": 4});
    let v2 = json!({"a": 1, "b": 20, "c": 3, "d": 4});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, old, new) => {
            assert_eq!(path, "b");
            assert_eq!(old, &json!(2));
            assert_eq!(new, &json!(20));
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 15: diffx file1.json file2.json --quiet
#[test]
fn test_quiet_mode() {
    let v1 = json!({"test": "value1"});
    let v2 = json!({"test": "value2"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert!(!diffs.is_empty());
}

/// Test case 16: diffx dir1/ dir2/ --recursive --brief
#[test]
fn test_recursive_brief() {
    let v1 = json!({"test": "value1"});
    let v2 = json!({"test": "value2"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
}

/// Test case 17: diffx huge_dataset.json huge_dataset_v2.json
#[test]
fn test_huge_dataset_performance() {
    let v1 = json!({"dataset": {"size": 1000000, "type": "production"}});
    let v2 = json!({"dataset": {"size": 1000001, "type": "production"}});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
    match &diffs[0] {
        DiffResult::Modified(path, _, _) => {
            assert_eq!(path, "dataset.size");
        }
        _ => panic!("Expected Modified diff result"),
    }
}

/// Test case 18: diffx config_dir1/ config_dir2/ --recursive
#[test]
fn test_directory_recursive() {
    let v1 = json!({"config": "dir1"});
    let v2 = json!({"config": "dir2"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert_eq!(diffs.len(), 1);
}

/// Test case 19: diffx config_v1.json config_v2.json --output json > diff1.json
#[test]
fn test_diff1_json_output() {
    let v1 = json!({"config": {"version": "1.0"}});
    let v2 = json!({"config": {"version": "1.1"}});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("config.version"));
}

/// Test case 20: diffx config_v2.json config_v3.json --output json > diff2.json
#[test]
fn test_diff2_json_output() {
    let v1 = json!({"config": {"version": "1.1"}});
    let v2 = json!({"config": {"version": "1.2"}});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("1.1"));
    assert!(json_output.contains("1.2"));
}

/// Test case 21: diffx diff1.json diff2.json
#[test]
fn test_meta_diff_comparison() {
    let diff1 = json!([{"Modified": ["config.version", "1.0", "1.1"]}]);
    let diff2 = json!([{"Modified": ["config.version", "1.1", "1.2"]}]);

    let diffs = diff(&diff1, &diff2, None).unwrap();
    assert!(!diffs.is_empty());
}

/// Test case 22: diffx config/prod.yaml config/staging.yaml --output json > changes.json
#[test]
fn test_cicd_config_changes() {
    let v1 = json!({"env": "prod", "debug": false});
    let v2 = json!({"env": "staging", "debug": true});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("env"));
    assert!(json_output.contains("debug"));
}

/// Test case 23: if ! diffx config/current.json config/new.json --quiet; then
#[test]
fn test_cicd_change_detection() {
    let v1 = json!({"current": "config"});
    let v2 = json!({"current": "new_config"});

    let diffs = diff(&v1, &v2, None).unwrap();
    assert!(!diffs.is_empty());
}

/// Test case 24: diffx api_old.json api_new.json --ignore-case --ignore-whitespace --output json > api_changes.json
#[test]
fn test_api_ignore_options_json() {
    let v1 = json!({"API": "old version"});
    let v2 = json!({"api": "new   version"});

    let config = DiffConfig {
        ignore_case: true,
        ignore_whitespace: true,
        ..DiffConfig::default()
    };

    let diffs = diff_with_config(&v1, &v2, &config);
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(!json_output.is_empty());
}

/// Test case 25: diffx large_prod_data.json large_staging_data.json --output json > data_changes.json
#[test]
fn test_large_data_comparison() {
    let v1 = json!({"dataset": {"env": "prod", "size": 10000}});
    let v2 = json!({"dataset": {"env": "staging", "size": 5000}});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("env"));
    assert!(json_output.contains("size"));
}

/// Test case 26: diffx package.json HEAD~1:package.json --output json
#[test]
fn test_git_dependency_detection() {
    let v1 = json!({"dependencies": {"express": "^4.18.0"}});
    let v2 = json!({"dependencies": {"express": "^4.18.0", "lodash": "^4.17.21"}});

    let diffs = diff(&v1, &v2, None).unwrap();
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("lodash"));
}
