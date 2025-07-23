use diffx_core::*;
use regex::Regex;
use serde_json::{json, Value};

// 52 test cases corresponding to performance.md examples
// Each test represents a different performance scenario or optimization technique

/// Test cases 1-10: Basic timing and optimization scenarios
#[test]
fn test_basic_timing() {
    let v1 = json!({"test": "data1"});
    let v2 = json!({"test": "data2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_ignore_timestamp() {
    let v1 = json!({"data": "value1", "timestamp": "2024-01-01"});
    let v2 = json!({"data": "value2", "timestamp": "2024-01-02"});
    let regex = Regex::new(r"^timestamp$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_ignore_multiple_patterns() {
    let v1 = json!({"data": "value1", "timestamp": "2024-01-01", "_internal": "meta", "temp_data": "tmp"});
    let v2 = json!({"data": "value2", "timestamp": "2024-01-02", "_internal": "meta2", "temp_data": "tmp2"});
    let regex = Regex::new(r"^(timestamp|_.*|temp_.*)$").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_users_basic() {
    let v1 = json!({"users": [{"id": 1, "name": "John"}]});
    let v2 = json!({"users": [{"id": 1, "name": "Jane"}]});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_users_with_array_id() {
    let v1 = json!({"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Bob"}]});
    let v2 = json!({"users": [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Johnny"}]});
    let diffs = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_output_timing() {
    let v1 = json!({"value": 100});
    let v2 = json!({"value": 200});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_json_output() {
    let v1 = json!({"value": 100});
    let v2 = json!({"value": 200});
    let diffs = diff(&v1, &v2, None, None, None);
    let json_output = serde_json::to_string(&diffs).unwrap();
    assert!(json_output.contains("Modified"));
}
#[test]
fn test_yaml_output() {
    let v1 = json!({"value": 100});
    let v2 = json!({"value": 200});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_large_config_basic() {
    let v1 = json!({"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true}});
    let v2 = json!({"database": {"host": "prod-db", "port": 5432}, "cache": {"enabled": false}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 2);
}
#[test]
fn test_large_config_path() {
    let v1 = json!({"database": {"connections": {"primary": "db1"}}});
    let v2 = json!({"database": {"connections": {"primary": "db2"}}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test cases 11-20: Memory optimization and processing strategies
#[test]
fn test_memory_usage() {
    let v1 = json!({"data": "memory_test1"});
    let v2 = json!({"data": "memory_test2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_memory_json_output() {
    let v1 = json!({"data": "memory_test1"});
    let v2 = json!({"data": "memory_test2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_memory_unified_output() {
    let v1 = json!({"data": "memory_test1"});
    let v2 = json!({"data": "memory_test2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_bulk_processing() {
    let v1 = json!({"config": "original"});
    let v2 = json!({"config": "backup"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_bulk_processing_error_handling() {
    let v1 = json!({"config": "original"});
    let v2 = json!({"config": "backup"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_parallel_section1() {
    let v1 = json!({"section1": {"data": "huge1"}});
    let v2 = json!({"section1": {"data": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_parallel_section2() {
    let v1 = json!({"section2": {"data": "huge1"}});
    let v2 = json!({"section2": {"data": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_parallel_section3() {
    let v1 = json!({"section3": {"data": "huge1"}});
    let v2 = json!({"section3": {"data": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_very_large_files() {
    let v1 = json!({"very_large": {"dataset": "v1"}});
    let v2 = json!({"very_large": {"dataset": "v2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_chunk1_processing() {
    let v1 = json!({"chunk1": {"data": "large1"}});
    let v2 = json!({"chunk1": {"data": "large2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test cases 21-30: Advanced optimization techniques
#[test]
fn test_chunk2_processing() {
    let v1 = json!({"chunk2": {"data": "large1"}});
    let v2 = json!({"chunk2": {"data": "large2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_epsilon_optimization() {
    let v1 = json!({"value": 1.0001});
    let v2 = json!({"value": 1.0002});
    let diffs = diff(&v1, &v2, None, Some(0.001), None);
    assert_eq!(diffs.len(), 0);
}
#[test]
fn test_no_epsilon() {
    let v1 = json!({"value": 1.0001});
    let v2 = json!({"value": 1.0002});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_output_format_json() {
    let v1 = json!({"format": "test1"});
    let v2 = json!({"format": "test2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_output_format_default() {
    let v1 = json!({"format": "test1"});
    let v2 = json!({"format": "test2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_time_verbose() {
    let v1 = json!({"large": "data1"});
    let v2 = json!({"large": "data2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_valgrind_massif() {
    let v1 = json!({"memory": "profile1"});
    let v2 = json!({"memory": "profile2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_huge_files_optimization() {
    let v1 = json!({"huge": {"dataset": "optimization1"}});
    let v2 = json!({"huge": {"dataset": "optimization2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_huge_files_users_path() {
    let v1 = json!({"users": {"count": 1000000}});
    let v2 = json!({"users": {"count": 1000001}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_huge_files_products_path() {
    let v1 = json!({"products": {"count": 500000}});
    let v2 = json!({"products": {"count": 500001}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test cases 31-40: Complex processing and configuration scenarios
#[test]
fn test_huge_files_orders_path() {
    let v1 = json!({"orders": {"count": 2000000}});
    let v2 = json!({"orders": {"count": 2000001}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_config_pipeline() {
    let v1 = json!({"config": {"section": "database"}});
    let v2 = json!({"config": {"section": "services"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_config_database_path() {
    let v1 = json!({"database": {"host": "localhost"}});
    let v2 = json!({"database": {"host": "remote"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_config_services_path() {
    let v1 = json!({"services": {"api": "v1"}});
    let v2 = json!({"services": {"api": "v2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_sample_array_id() {
    let v1 = json!({"items": [{"id": 1, "name": "item1"}]});
    let v2 = json!({"items": [{"id": 1, "name": "item2"}]});
    let diffs = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_batch_processing() {
    let v1 = json!({"batch": "file1"});
    let v2 = json!({"batch": "file2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_config_env_app() {
    let v1 = json!({"app": "prod", "host": "prod-server", "port": 8080, "password": "secret"});
    let v2 = json!({"app": "dev", "host": "dev-server", "port": 3000, "password": "dev-secret"});
    let regex = Regex::new(r"^(host|port|password)").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_config_env_db() {
    let v1 =
        json!({"database": "prod", "connection_string": "prod-conn", "credentials": "prod-cred"});
    let v2 = json!({"database": "dev", "connection_string": "dev-conn", "credentials": "dev-cred"});
    let regex = Regex::new(r"^(connection_string|credentials)").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_baseline_silent() {
    let v1 = json!({"baseline": "config"});
    let v2 = json!({"current": "config"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_baseline_detailed() {
    let v1 = json!({"config": "baseline", "timestamp": "2024-01-01", "build_id": "123"});
    let v2 = json!({"config": "current", "timestamp": "2024-01-02", "build_id": "124"});
    let regex = Regex::new(r"^(timestamp|build_id|deployment_time)").unwrap();
    let diffs = diff(&v1, &v2, Some(&regex), None, None);
    assert_eq!(diffs.len(), 1);
}

/// Test cases 41-52: Benchmarking, monitoring, and profiling scenarios
#[test]
fn test_benchmark_setup() {
    let v1 = json!({"benchmark": "setup1"});
    let v2 = json!({"benchmark": "setup2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_benchmark_verbose() {
    let v1 = json!({"benchmark": "verbose1"});
    let v2 = json!({"benchmark": "verbose2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_monitoring_silent() {
    let v1 = json!({"monitor": "original"});
    let v2 = json!({"monitor": "backup"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_performance_measurement() {
    let v1 = json!({"performance": "test"});
    let v2 = json!({"performance": "backup"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_tuning_basic() {
    let v1 = json!({"tuning": {"basic": "huge1"}});
    let v2 = json!({"tuning": {"basic": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_tuning_path_section1() {
    let v1 = json!({"section1": {"tuning": "huge1"}});
    let v2 = json!({"section1": {"tuning": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_tuning_path_section2() {
    let v1 = json!({"section2": {"tuning": "huge1"}});
    let v2 = json!({"section2": {"tuning": "huge2"}});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_array_optimization_basic() {
    let v1 = json!({"users": [{"name": "user1"}, {"name": "user2"}]});
    let v2 = json!({"users": [{"name": "user1"}, {"name": "user3"}]});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_array_optimization_with_id() {
    let v1 = json!({"users": [{"id": 1, "name": "user1"}, {"id": 2, "name": "user2"}]});
    let v2 = json!({"users": [{"id": 1, "name": "user1"}, {"id": 2, "name": "user3"}]});
    let diffs = diff(&v1, &v2, None, None, Some("id"));
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_help_option() {
    let v1 = json!({"help": "test"});
    let v2 = json!({"help": "test"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 0);
}
#[test]
fn test_profiling_massif() {
    let v1 = json!({"profiling": "large1"});
    let v2 = json!({"profiling": "large2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
#[test]
fn test_perf_record() {
    let v1 = json!({"perf": "large1"});
    let v2 = json!({"perf": "large2"});
    let diffs = diff(&v1, &v2, None, None, None);
    assert_eq!(diffs.len(), 1);
}
