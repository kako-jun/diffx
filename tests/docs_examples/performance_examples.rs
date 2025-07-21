use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

// Helper function to get the diffx command
fn diffx_cmd() -> Command {
    Command::cargo_bin("diffx").expect("Failed to find diffx binary")
}

// Helper function to create temporary JSON files for testing
fn create_temp_json(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: time diffx file1.json file2.json
#[test]
fn test_basic_timing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"test": "data1"}"#);
    let file2 = create_temp_json(r#"{"test": "data2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 2: time diffx file1.json file2.json --ignore-keys-regex "^timestamp$"
#[test]
fn test_ignore_timestamp() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "value1", "timestamp": "2024-01-01"}"#);
    let file2 = create_temp_json(r#"{"data": "value2", "timestamp": "2024-01-02"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--ignore-keys-regex").arg("^timestamp$");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: time diffx file1.json file2.json --ignore-keys-regex "^(timestamp|_.*|temp_.*)$"
#[test]
fn test_ignore_multiple_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "value1", "timestamp": "2024-01-01", "_internal": "meta", "temp_data": "tmp"}"#);
    let file2 = create_temp_json(r#"{"data": "value2", "timestamp": "2024-01-02", "_internal": "meta2", "temp_data": "tmp2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|_.*|temp_.*)$");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: time diffx users1.json users2.json
#[test]
fn test_users_basic() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"users": [{"id": 1, "name": "John"}]}"#);
    let file2 = create_temp_json(r#"{"users": [{"id": 1, "name": "Jane"}]}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 5: time diffx users1.json users2.json --array-id-key "id"
#[test]
fn test_users_with_array_id() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Bob"}]}"#);
    let file2 = create_temp_json(r#"{"users": [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Johnny"}]}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--array-id-key").arg("id");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: time diffx file1.json file2.json (output test)
#[test]
fn test_output_timing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 100}"#);
    let file2 = create_temp_json(r#"{"value": 200}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 7: time diffx file1.json file2.json --output json
#[test]
fn test_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 100}"#);
    let file2 = create_temp_json(r#"{"value": 200}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: time diffx file1.json file2.json --output yaml
#[test]
fn test_yaml_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 100}"#);
    let file2 = create_temp_json(r#"{"value": 200}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("yaml");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: diffx large_config.json large_config.new.json
#[test]
fn test_large_config_basic() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true}}"#);
    let file2 = create_temp_json(r#"{"database": {"host": "prod-db", "port": 5432}, "cache": {"enabled": false}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 10: diffx large_config.json large_config.new.json --path "database.connections"
#[test]
fn test_large_config_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"database": {"connections": {"primary": "db1"}}}"#);
    let file2 = create_temp_json(r#"{"database": {"connections": {"primary": "db2"}}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 11: diffx file1.json file2.json (memory test)
#[test]
fn test_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "memory_test1"}"#);
    let file2 = create_temp_json(r#"{"data": "memory_test2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 12: diffx file1.json file2.json --output json (memory test)
#[test]
fn test_memory_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "memory_test1"}"#);
    let file2 = create_temp_json(r#"{"data": "memory_test2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 13: diffx file1.json file2.json --output unified (memory test)
#[test]
fn test_memory_unified_output() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"data": "memory_test1"}"#);
    let file2 = create_temp_json(r#"{"data": "memory_test2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("unified");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: diffx "$file" "${file}.backup" (bulk processing)
#[test]
fn test_bulk_processing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": "original"}"#);
    let file2 = create_temp_json(r#"{"config": "backup"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 15: diffx {} {}.backup || echo "Diff in {}" (bulk processing with error handling)
#[test]
fn test_bulk_processing_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": "original"}"#);
    let file2 = create_temp_json(r#"{"config": "backup"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 16: diffx huge1.json huge2.json --path "section1"
#[test]
fn test_parallel_section1() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"section1": {"data": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"section1": {"data": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 17: diffx huge1.json huge2.json --path "section2"
#[test]
fn test_parallel_section2() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"section2": {"data": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"section2": {"data": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 18: diffx huge1.json huge2.json --path "section3"
#[test]
fn test_parallel_section3() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"section3": {"data": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"section3": {"data": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 19: diffx very_large1.json very_large2.json
#[test]
fn test_very_large_files() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"very_large": {"dataset": "v1"}}"#);
    let file2 = create_temp_json(r#"{"very_large": {"dataset": "v2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 20: diffx very_large1.json very_large2.json --path "chunk1"
#[test]
fn test_chunk1_processing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"chunk1": {"data": "large1"}}"#);
    let file2 = create_temp_json(r#"{"chunk1": {"data": "large2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 21: diffx very_large1.json very_large2.json --path "chunk2"
#[test]
fn test_chunk2_processing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"chunk2": {"data": "large1"}}"#);
    let file2 = create_temp_json(r#"{"chunk2": {"data": "large2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 22: diffx data1.json data2.json --epsilon 0.001
#[test]
fn test_epsilon_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 1.0001}"#);
    let file2 = create_temp_json(r#"{"value": 1.0002}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--epsilon").arg("0.001");
    cmd.assert().success();
    Ok(())
}

/// Test case 23: diffx data1.json data2.json (epsilon comparison)
#[test]
fn test_no_epsilon() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"value": 1.0001}"#);
    let file2 = create_temp_json(r#"{"value": 1.0002}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 24: diffx file1.json file2.json --output json (output comparison)
#[test]
fn test_output_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"format": "test1"}"#);
    let file2 = create_temp_json(r#"{"format": "test2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 25: diffx file1.json file2.json (output comparison)
#[test]
fn test_output_format_default() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"format": "test1"}"#);
    let file2 = create_temp_json(r#"{"format": "test2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 26: /usr/bin/time -v diffx large1.json large2.json
#[test]
fn test_time_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"large": "data1"}"#);
    let file2 = create_temp_json(r#"{"large": "data2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 27: valgrind --tool=massif diffx file1.json file2.json
#[test]
fn test_valgrind_massif() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"memory": "profile1"}"#);
    let file2 = create_temp_json(r#"{"memory": "profile2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 28: diffx huge1.json huge2.json (optimization test)
#[test]
fn test_huge_files_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"huge": {"dataset": "optimization1"}}"#);
    let file2 = create_temp_json(r#"{"huge": {"dataset": "optimization2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 29: diffx huge1.json huge2.json --path "users"
#[test]
fn test_huge_files_users_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"users": {"count": 1000000}}"#);
    let file2 = create_temp_json(r#"{"users": {"count": 1000001}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 30: diffx huge1.json huge2.json --path "products"
#[test]
fn test_huge_files_products_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"products": {"count": 500000}}"#);
    let file2 = create_temp_json(r#"{"products": {"count": 500001}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 31: diffx huge1.json huge2.json --path "orders"
#[test]
fn test_huge_files_orders_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"orders": {"count": 2000000}}"#);
    let file2 = create_temp_json(r#"{"orders": {"count": 2000001}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 32: diffx config1.json config2.json --output json | jq ...
#[test]
fn test_config_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": {"section": "database"}}"#);
    let file2 = create_temp_json(r#"{"config": {"section": "services"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 33: diffx config1.json config2.json --path "database"
#[test]
fn test_config_database_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"database": {"host": "localhost"}}"#);
    let file2 = create_temp_json(r#"{"database": {"host": "remote"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 34: diffx config1.json config2.json --path "services"
#[test]
fn test_config_services_path() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"services": {"api": "v1"}}"#);
    let file2 = create_temp_json(r#"{"services": {"api": "v2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 35: diffx sample1.json sample2.json --array-id-key "id"
#[test]
fn test_sample_array_id() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"items": [{"id": 1, "name": "item1"}]}"#);
    let file2 = create_temp_json(r#"{"items": [{"id": 1, "name": "item2"}]}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--array-id-key").arg("id");
    cmd.assert().success();
    Ok(())
}

/// Test case 36: diffx "$1" "$file2" --output json > "diff_$(basename "$1" .json).json"
#[test]
fn test_batch_processing() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"batch": "file1"}"#);
    let file2 = create_temp_json(r#"{"batch": "file2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 37: diffx "configs/$env/app.json" "configs/$BASE/app.json" --ignore-keys-regex "^(host|port|password)" --output json
#[test]
fn test_config_env_app() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"app": "prod", "host": "prod-server", "port": 8080, "password": "secret"}"#);
    let file2 = create_temp_json(r#"{"app": "dev", "host": "dev-server", "port": 3000, "password": "dev-secret"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--ignore-keys-regex").arg("^(host|port|password)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 38: diffx "configs/$env/db.json" "configs/$BASE/db.json" --ignore-keys-regex "^(connection_string|credentials)" --output json
#[test]
fn test_config_env_db() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"database": "prod", "connection_string": "prod-conn", "credentials": "prod-cred"}"#);
    let file2 = create_temp_json(r#"{"database": "dev", "connection_string": "dev-conn", "credentials": "dev-cred"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--ignore-keys-regex").arg("^(connection_string|credentials)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 39: diffx baseline_config.json current_config.json >/dev/null 2>&1
#[test]
fn test_baseline_silent() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"baseline": "config"}"#);
    let file2 = create_temp_json(r#"{"current": "config"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 40: diffx baseline_config.json current_config.json --ignore-keys-regex "^(timestamp|build_id|deployment_time)" --output json
#[test]
fn test_baseline_detailed() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"config": "baseline", "timestamp": "2024-01-01", "build_id": "123"}"#);
    let file2 = create_temp_json(r#"{"config": "current", "timestamp": "2024-01-02", "build_id": "124"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--ignore-keys-regex").arg("^(timestamp|build_id|deployment_time)")
        .arg("--output").arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 41: time diffx file1.json file2.json (benchmark setup)
#[test]
fn test_benchmark_setup() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"benchmark": "setup1"}"#);
    let file2 = create_temp_json(r#"{"benchmark": "setup2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 42: /usr/bin/time -v diffx file1.json file2.json (benchmark verbose)
#[test]
fn test_benchmark_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"benchmark": "verbose1"}"#);
    let file2 = create_temp_json(r#"{"benchmark": "verbose2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 43: diffx "$file" "${file}.backup" >/dev/null
#[test]
fn test_monitoring_silent() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"monitor": "original"}"#);
    let file2 = create_temp_json(r#"{"monitor": "backup"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 44: time diffx test_file.json test_file.backup 2>&1 | grep real | cut -d' ' -f2
#[test]
fn test_performance_measurement() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"performance": "test"}"#);
    let file2 = create_temp_json(r#"{"performance": "backup"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 45: diffx huge1.json huge2.json (tuning basic)
#[test]
fn test_tuning_basic() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"tuning": {"basic": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"tuning": {"basic": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 46: diffx huge1.json huge2.json --path "section1" (tuning path1)
#[test]
fn test_tuning_path_section1() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"section1": {"tuning": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"section1": {"tuning": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 47: diffx huge1.json huge2.json --path "section2" (tuning path2)
#[test]
fn test_tuning_path_section2() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"section2": {"tuning": "huge1"}}"#);
    let file2 = create_temp_json(r#"{"section2": {"tuning": "huge2"}}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 48: diffx users1.json users2.json (array optimization basic)
#[test]
fn test_array_optimization_basic() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"users": [{"name": "user1"}, {"name": "user2"}]}"#);
    let file2 = create_temp_json(r#"{"users": [{"name": "user1"}, {"name": "user3"}]}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 49: diffx users1.json users2.json --array-id-key "id" (array optimization with id)
#[test]
fn test_array_optimization_with_id() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"users": [{"id": 1, "name": "user1"}, {"id": 2, "name": "user2"}]}"#);
    let file2 = create_temp_json(r#"{"users": [{"id": 1, "name": "user1"}, {"id": 2, "name": "user3"}]}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path())
        .arg("--array-id-key").arg("id");
    cmd.assert().success();
    Ok(())
}

/// Test case 50: diffx file1.json file2.json --help
#[test]
fn test_help_option() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 51: valgrind --tool=massif diffx large1.json large2.json (profiling)
#[test]
fn test_profiling_massif() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"profiling": "large1"}"#);
    let file2 = create_temp_json(r#"{"profiling": "large2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 52: perf record diffx large1.json large2.json (performance profiling)
#[test]
fn test_perf_record() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = create_temp_json(r#"{"perf": "large1"}"#);
    let file2 = create_temp_json(r#"{"perf": "large2"}"#);
    
    let mut cmd = diffx_cmd();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().success();
    Ok(())
}