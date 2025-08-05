use serde_json::{json, Value};

/// Common test fixtures shared across core/python/js tests
/// These fixtures are compatible with CLI fixtures but focused on unified API testing
pub struct TestFixtures;

impl TestFixtures {
    /// Get path to shared CLI fixtures directory
    pub fn cli_fixtures_dir() -> &'static str {
        "../tests/fixtures"
    }

    /// Load JSON file from CLI fixtures
    pub fn load_cli_fixture(filename: &str) -> Value {
        let path = format!("{}/{}", Self::cli_fixtures_dir(), filename);
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read fixture: {path}"));

        if filename.ends_with(".json") {
            serde_json::from_str(&content).unwrap()
        } else {
            panic!("Only JSON fixtures supported in unified API tests")
        }
    }

    /// Basic configuration comparison fixtures
    pub fn config_v1() -> Value {
        Self::load_cli_fixture("config_v1.json")
    }

    pub fn config_v2() -> Value {
        Self::load_cli_fixture("config_v2.json")
    }

    pub fn config_v3() -> Value {
        Self::load_cli_fixture("config_v3.json")
    }

    /// User data fixtures for array testing
    pub fn users_v1() -> Value {
        Self::load_cli_fixture("users_v1.json")
    }

    pub fn users_v2() -> Value {
        Self::load_cli_fixture("users_v2.json")
    }

    /// API schema fixtures for complex object testing
    pub fn api_schema_v1() -> Value {
        Self::load_cli_fixture("api_schema_v1.json")
    }

    pub fn api_schema_v2() -> Value {
        Self::load_cli_fixture("api_schema_v2.json")
    }

    /// Security config for sensitive data testing
    pub fn security_config() -> Value {
        Self::load_cli_fixture("security_config.json")
    }

    pub fn security_config_new() -> Value {
        Self::load_cli_fixture("security_config_new.json")
    }

    /// Inline test fixtures for unified API specific tests
    pub fn simple_object_old() -> Value {
        json!({
            "name": "diffx",
            "version": "1.0.0",
            "features": ["json", "yaml"]
        })
    }

    pub fn simple_object_new() -> Value {
        json!({
            "name": "diffx",
            "version": "1.1.0",
            "features": ["json", "yaml", "toml"],
            "author": "Claude"
        })
    }

    pub fn array_with_ids_old() -> Value {
        json!([
            {"id": 1, "name": "Alice", "role": "admin"},
            {"id": 2, "name": "Bob", "role": "user"},
            {"id": 3, "name": "Charlie", "role": "user"}
        ])
    }

    pub fn array_with_ids_new() -> Value {
        json!([
            {"id": 1, "name": "Alice", "role": "superadmin"},
            {"id": 3, "name": "Charlie", "role": "user"},
            {"id": 4, "name": "David", "role": "user"}
        ])
    }

    pub fn nested_object_old() -> Value {
        json!({
            "database": {
                "host": "localhost",
                "port": 5432,
                "config": {
                    "max_connections": 100,
                    "timeout": 30
                }
            },
            "cache": {
                "type": "redis",
                "ttl": 3600
            }
        })
    }

    pub fn nested_object_new() -> Value {
        json!({
            "database": {
                "host": "production.db",
                "port": 5432,
                "config": {
                    "max_connections": 200,
                    "timeout": 30,
                    "ssl": true
                }
            },
            "cache": {
                "type": "memcached",
                "ttl": 7200
            },
            "monitoring": {
                "enabled": true
            }
        })
    }

    pub fn numeric_precision_old() -> Value {
        json!({
            "measurements": [1.0, 2.001, 3.14160],
            "coordinates": {"x": 10.0, "y": 20.0}
        })
    }

    pub fn numeric_precision_new() -> Value {
        json!({
            "measurements": [1.001, 2.002, 3.14160],
            "coordinates": {"x": 10.001, "y": 20.001}
        })
    }

    pub fn string_comparison_old() -> Value {
        json!({
            "title": "Hello World",
            "description": "This is a test",
            "tags": ["Test", "Example"]
        })
    }

    pub fn string_comparison_new() -> Value {
        json!({
            "title": "HELLO WORLD",
            "description": "This  is  a  test",
            "tags": ["test", "example"]
        })
    }

    pub fn type_changes_old() -> Value {
        json!({
            "count": 42,
            "enabled": true,
            "data": [1, 2, 3],
            "meta": {"created": "2023-01-01"}
        })
    }

    pub fn type_changes_new() -> Value {
        json!({
            "count": "42",
            "enabled": "true",
            "data": {"0": 1, "1": 2, "2": 3},
            "meta": "metadata"
        })
    }

    /// Large dataset for performance testing
    pub fn large_dataset_old() -> Value {
        let mut items = Vec::new();
        for i in 0..1000 {
            items.push(json!({
                "id": i,
                "value": format!("item_{}", i),
                "metadata": {
                    "created": "2023-01-01",
                    "score": i as f64 * 0.1
                }
            }));
        }
        json!({"items": items})
    }

    pub fn large_dataset_new() -> Value {
        let mut items = Vec::new();
        for i in 0..1000 {
            items.push(json!({
                "id": i,
                "value": format!("item_{}_updated", i),
                "metadata": {
                    "created": "2023-01-01",
                    "updated": "2023-01-02",
                    "score": i as f64 * 0.1 + 1.0
                }
            }));
        }
        json!({"items": items})
    }
}

/// Helper functions for test data generation
pub mod generators {
    use super::*;

    pub fn generate_array_without_ids(size: usize) -> Value {
        let items: Vec<Value> = (0..size)
            .map(|i| json!({"index": i, "value": format!("item_{}", i)}))
            .collect();
        json!(items)
    }

    pub fn generate_array_with_ids(size: usize, id_key: &str) -> Value {
        let items: Vec<Value> = (0..size)
            .map(|i| json!({id_key: i, "value": format!("item_{}", i)}))
            .collect();
        json!(items)
    }

    pub fn generate_deep_nested_object(depth: usize) -> Value {
        fn create_nested(current_depth: usize, max_depth: usize) -> Value {
            if current_depth >= max_depth {
                json!({"value": current_depth})
            } else {
                json!({
                    "level": current_depth,
                    "nested": create_nested(current_depth + 1, max_depth)
                })
            }
        }
        create_nested(0, depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_fixtures_loading() {
        // Test that we can load existing CLI fixtures
        let config_v1 = TestFixtures::config_v1();
        let config_v2 = TestFixtures::config_v2();

        assert!(config_v1.is_object());
        assert!(config_v2.is_object());
        assert_ne!(config_v1, config_v2);
    }

    #[test]
    fn test_inline_fixtures() {
        let old = TestFixtures::simple_object_old();
        let new = TestFixtures::simple_object_new();

        assert_eq!(old["name"], json!("diffx"));
        assert_eq!(new["name"], json!("diffx"));
        assert_ne!(old["version"], new["version"]);
    }

    #[test]
    fn test_fixture_generators() {
        let arr = generators::generate_array_with_ids(5, "id");
        if let Value::Array(items) = arr {
            assert_eq!(items.len(), 5);
            assert_eq!(items[0]["id"], json!(0));
        } else {
            panic!("Expected array");
        }

        let nested = generators::generate_deep_nested_object(3);
        // The structure is {"level": 0, "nested": {"level": 1, "nested": {"level": 2, "nested": {"value": 3}}}}
        assert!(nested["nested"]["nested"]["nested"]["value"].is_number());
    }
}
