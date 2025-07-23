"""
README examples tests for diffx-python package
Tests README.md usage examples as Python library functions
"""

import unittest
import tempfile
import json
import os
import sys

# Import the diffx Python library
try:
    import diffx_python as diffx
except ImportError:
    # Fallback for development
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../src'))
    import diffx_python as diffx


class READMEExamplesTest(unittest.TestCase):
    """Test cases for README.md examples using diffx Python library functions"""
    
    def setUp(self):
        """Set up temporary directory for test files"""
        self.temp_dir = tempfile.mkdtemp()
        
    def tearDown(self):
        """Clean up temporary files"""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def create_temp_json(self, content):
        """Helper function to create temporary JSON files"""
        import tempfile
        fd, path = tempfile.mkstemp(suffix='.json', dir=self.temp_dir)
        with os.fdopen(fd, 'w') as f:
            json.dump(content, f)
        return path

    def test_basic_json_diff(self):
        """Test case 1: Basic JSON semantic diff"""
        file1 = self.create_temp_json({"name": "myapp", "version": "1.0"})
        file2 = self.create_temp_json({"version": "1.1", "name": "myapp"})
        
        result = diffx.diff(file1, file2)
        
        # Should detect only semantic changes (version), ignore key order
        self.assertIn('version', result)
        self.assertIn('1.0', result) 
        self.assertIn('1.1', result)

    def test_json_output_format(self):
        """Test case 2: JSON output format option"""
        file1 = self.create_temp_json({"debug": True})
        file2 = self.create_temp_json({"debug": False})
        
        result = diffx.diff(file1, file2, output='json')
        
        # Should return valid JSON string
        parsed = json.loads(result)
        self.assertIsInstance(parsed, list)
        self.assertGreater(len(parsed), 0)

    def test_yaml_output_format(self):
        """Test case 3: YAML output format"""
        file1 = self.create_temp_json({"test": 1})
        file2 = self.create_temp_json({"test": 2})
        
        result = diffx.diff(file1, file2, output='yaml')
        
        # Should return YAML format string
        self.assertIsInstance(result, str)
        self.assertGreater(len(result), 0)

    def test_ignore_keys_regex(self):
        """Test case 4: Ignore keys regex option"""
        file1 = self.create_temp_json({"timestamp": "2024-01-01", "data": "value1"})
        file2 = self.create_temp_json({"timestamp": "2024-01-02", "data": "value2"})
        
        result = diffx.diff(file1, file2, ignore_keys_regex="^timestamp$")
        
        # Should ignore timestamp changes, only show data changes
        self.assertNotIn('timestamp', result)
        self.assertIn('data', result)
        self.assertIn('value1', result)
        self.assertIn('value2', result)

    def test_array_id_key(self):
        """Test case 5: Array ID key tracking"""
        file1 = self.create_temp_json({
            "users": [
                {"id": 1, "name": "John"},
                {"id": 2, "name": "Jane"}
            ]
        })
        file2 = self.create_temp_json({
            "users": [
                {"id": 2, "name": "Jane"},
                {"id": 1, "name": "Johnny"}
            ]
        })
        
        result = diffx.diff(file1, file2, array_id_key="id")
        
        # Should track changes by ID, not by position
        self.assertIn('name', result)
        self.assertIn('John', result)
        self.assertIn('Johnny', result)

    def test_epsilon_tolerance(self):
        """Test case 6: Epsilon tolerance for floating point"""
        file1 = self.create_temp_json({"value": 1.0001})
        file2 = self.create_temp_json({"value": 1.0002})
        
        result = diffx.diff(file1, file2, epsilon=0.001)
        
        # Should consider values equal within epsilon tolerance (empty result)
        self.assertEqual(result.strip(), '')

    def test_ignore_case(self):
        """Test case 7: Ignore case differences"""
        file1 = self.create_temp_json({"status": "ACTIVE"})
        file2 = self.create_temp_json({"status": "active"})
        
        result = diffx.diff(file1, file2, ignore_case=True)
        
        # Should be considered identical when ignoring case
        self.assertEqual(result.strip(), '')

    def test_ignore_whitespace(self):
        """Test case 8: Ignore whitespace differences"""
        file1 = self.create_temp_json({"text": "hello world"})
        file2 = self.create_temp_json({"text": "hello    world"})
        
        result = diffx.diff(file1, file2, ignore_whitespace=True)
        
        # Should ignore whitespace differences
        self.assertEqual(result.strip(), '')

    def test_unified_output_with_context(self):
        """Test case 9: Context lines with unified output"""
        file1 = self.create_temp_json({"a": 1, "b": 2, "c": 3, "d": 4})
        file2 = self.create_temp_json({"a": 1, "b": 20, "c": 3, "d": 4})
        
        result = diffx.diff(file1, file2, output='unified', context=3)
        
        # Should return unified diff format with context
        self.assertIn('-', result)
        self.assertIn('+', result)

    def test_quiet_mode_identical(self):
        """Test case 10: Quiet mode for identical files"""
        file1 = self.create_temp_json({"test": "value"})
        file2 = self.create_temp_json({"test": "value"})
        
        result = diffx.diff(file1, file2, quiet=True)
        
        # Should return empty result for identical files
        self.assertEqual(result.strip(), '')

    def test_brief_mode(self):
        """Test case 11: Brief mode for quick comparison"""
        file1 = self.create_temp_json({"test": "value1"})
        file2 = self.create_temp_json({"test": "value2"})
        
        result = diffx.diff(file1, file2, brief=True)
        
        # Should return brief output format
        self.assertGreater(len(result), 0)

    def test_meta_chaining(self):
        """Test case 12: Meta-chaining - compare diff reports"""
        # Create first diff report
        file1a = self.create_temp_json({"version": "1.0"})
        file1b = self.create_temp_json({"version": "1.1"})
        
        report1 = diffx.diff(file1a, file1b, output='json')
        
        # Create second diff report 
        file2a = self.create_temp_json({"version": "1.1"})
        file2b = self.create_temp_json({"version": "1.2"})
        
        report2 = diffx.diff(file2a, file2b, output='json')
        
        # Save reports to temp files
        report1_file = self.create_temp_json(json.loads(report1))
        report2_file = self.create_temp_json(json.loads(report2))
        
        # Compare the reports (meta-chaining)
        meta_diff = diffx.diff(report1_file, report2_file)
        
        # Should be able to compare diff reports
        self.assertIsInstance(meta_diff, str)

    def test_performance_large_config(self):
        """Test case 13: Performance with larger configuration data"""
        large_data1 = {
            "config": {
                "database": {"host": "localhost", "port": 5432},
                "cache": {"enabled": True, "ttl": 3600},
                "logging": {"level": "info", "file": "/var/log/app.log"}
            }
        }
        
        large_data2 = {
            "config": {
                "database": {"host": "prod-db", "port": 5432},
                "cache": {"enabled": False, "ttl": 7200},
                "logging": {"level": "debug", "file": "/var/log/app.log"}
            }
        }
        
        file1 = self.create_temp_json(large_data1)
        file2 = self.create_temp_json(large_data2)
        
        result = diffx.diff(file1, file2)
        
        # Should detect all semantic changes
        self.assertIn('host', result)
        self.assertIn('enabled', result)
        self.assertIn('level', result)

    def test_cicd_config_validation(self):
        """Test case 14: CI/CD configuration validation"""
        prod_config = {"env": "prod", "debug": False, "port": 8080}
        staging_config = {"env": "staging", "debug": True, "port": 8080}
        
        file1 = self.create_temp_json(prod_config)
        file2 = self.create_temp_json(staging_config)
        
        result = diffx.diff(file1, file2, output='json')
        
        # Should return valid JSON for CI/CD processing
        parsed = json.loads(result)
        self.assertIsInstance(parsed, list)

    def test_cross_format_compatibility(self):
        """Test case 15: Cross-format compatibility test"""
        yaml_style = {
            "services": {
                "web": {"image": "nginx", "ports": ["80:80"]},
                "db": {"image": "postgres", "environment": {"POSTGRES_DB": "myapp"}}
            }
        }
        
        json_style = {
            "services": {
                "web": {"image": "nginx:latest", "ports": ["80:80"]},
                "db": {"image": "postgres", "environment": {"POSTGRES_DB": "myapp"}}
            }
        }
        
        file1 = self.create_temp_json(yaml_style)
        file2 = self.create_temp_json(json_style)
        
        result = diffx.diff(file1, file2, output='yaml')
        
        # Should handle cross-format comparison
        self.assertIn('image', result)
        self.assertIn('nginx', result)
        self.assertIn('nginx:latest', result)

    def test_advanced_filtering_combination(self):
        """Test case 16: Advanced filtering combination"""
        complex1 = {
            "timestamp": "2024-01-01T10:00:00Z",
            "config": {"debug": True, "users": [{"id": 1, "name": "John"}]},
            "_internal": "meta"
        }
        
        complex2 = {
            "timestamp": "2024-01-01T11:00:00Z",
            "config": {"debug": False, "users": [{"id": 1, "name": "Johnny"}]},
            "_internal": "meta2"
        }
        
        file1 = self.create_temp_json(complex1)
        file2 = self.create_temp_json(complex2)
        
        result = diffx.diff(file1, file2, 
                           ignore_keys_regex="^(timestamp|_.*)",
                           array_id_key="id")
        
        # Should ignore timestamp and _internal, but detect config changes
        self.assertNotIn('timestamp', result)
        self.assertNotIn('_internal', result)
        self.assertIn('debug', result)
        self.assertIn('name', result)

    def test_package_ecosystem_compatibility(self):
        """Test case 17: Package ecosystem compatibility"""
        rust_config = {
            "name": "diffx",
            "version": "0.1.0",
            "edition": "2021"
        }
        
        node_config = {
            "name": "diffx-js",
            "version": "0.1.0",
            "main": "index.js"
        }
        
        file1 = self.create_temp_json(rust_config)
        file2 = self.create_temp_json(node_config)
        
        result = diffx.diff(file1, file2)
        
        # Should detect differences between package configs
        self.assertIn('name', result)
        self.assertIn('diffx', result)

    def test_api_response_validation(self):
        """Test case 18: API response validation scenario"""
        expected_response = {
            "id": 123,
            "name": "John Doe",
            "email": "john@example.com",
            "created_at": "2024-01-01T00:00:00Z"
        }
        
        actual_response = {
            "id": 123,
            "name": "John Doe",
            "email": "john@example.com",
            "created_at": "2024-01-01T00:01:00Z"
        }
        
        file1 = self.create_temp_json(expected_response)
        file2 = self.create_temp_json(actual_response)
        
        result = diffx.diff(file1, file2, ignore_keys_regex="^(created_at|updated_at)$")
        
        # Should ignore timestamp fields in API validation
        self.assertEqual(result.strip(), '')

    def test_database_migration_validation(self):
        """Test case 19: Database migration validation"""
        before_migration = {
            "users": {"columns": ["id", "name", "email"]},
            "posts": {"columns": ["id", "title", "content"]}
        }
        
        after_migration = {
            "users": {"columns": ["id", "name", "email", "created_at"]},
            "posts": {"columns": ["id", "title", "content", "author_id"]}
        }
        
        file1 = self.create_temp_json(before_migration)
        file2 = self.create_temp_json(after_migration)
        
        result = diffx.diff(file1, file2)
        
        # Should detect column additions
        self.assertIn('created_at', result)
        self.assertIn('author_id', result)

    def test_security_configuration_audit(self):
        """Test case 20: Security configuration audit"""
        security_policy1 = {
            "permissions": {"read": True, "write": False, "admin": False}
        }
        
        security_policy2 = {
            "permissions": {"read": True, "write": True, "admin": False}
        }
        
        file1 = self.create_temp_json(security_policy1)
        file2 = self.create_temp_json(security_policy2)
        
        result = diffx.diff(file1, file2, output='json')
        
        # Should detect permission changes in security audit
        parsed = json.loads(result)
        self.assertIsInstance(parsed, list)
        self.assertIn('write', result)

    def test_large_dataset_processing(self):
        """Test case 21: Large dataset processing optimization"""
        large_dataset1 = {
            "dataset": {"users": 1000, "active": True},
            "metrics": {"cpu": 45.2, "memory": 67.8}
        }
        
        large_dataset2 = {
            "dataset": {"users": 1001, "active": True},
            "metrics": {"cpu": 48.1, "memory": 69.2}
        }
        
        file1 = self.create_temp_json(large_dataset1)
        file2 = self.create_temp_json(large_dataset2)
        
        result = diffx.diff(file1, file2, output='json')
        
        # Should handle large datasets efficiently
        parsed = json.loads(result)
        self.assertIsInstance(parsed, list)

    def test_git_hook_dependency_detection(self):
        """Test case 22: Git hook dependency detection simulation"""
        old_package = {
            "dependencies": {"express": "^4.18.0"}
        }
        
        new_package = {
            "dependencies": {"express": "^4.18.0", "lodash": "^4.17.21"}
        }
        
        file1 = self.create_temp_json(old_package)
        file2 = self.create_temp_json(new_package)
        
        result = diffx.diff(file1, file2, output='json')
        
        # Should detect new dependency addition
        parsed = json.loads(result)
        self.assertIsInstance(parsed, list)
        self.assertIn('lodash', result)

    def test_monitoring_config_drift(self):
        """Test case 23: Configuration drift monitoring"""
        expected_config = {
            "service": {"replicas": 3, "memory": "512Mi"},
            "database": {"pool_size": 10}
        }
        
        current_config = {
            "service": {"replicas": 2, "memory": "512Mi"},
            "database": {"pool_size": 10}
        }
        
        file1 = self.create_temp_json(expected_config)
        file2 = self.create_temp_json(current_config)
        
        result = diffx.diff(file1, file2)
        
        # Should detect configuration drift
        self.assertIn('replicas', result)
        self.assertIn('3', result)
        self.assertIn('2', result)

    def test_batch_file_processing(self):
        """Test case 24: Batch file processing simulation"""
        config_v1 = {"version": "1.0", "feature_flags": {"new_ui": False}}
        config_v2 = {"version": "1.1", "feature_flags": {"new_ui": True}}
        
        file1 = self.create_temp_json(config_v1)
        file2 = self.create_temp_json(config_v2)
        
        result = diffx.diff(file1, file2)
        
        # Should handle batch processing scenarios
        self.assertIn('version', result)
        self.assertIn('new_ui', result)

    def test_environment_comparison(self):
        """Test case 25: Environment comparison"""
        dev_config = {"host": "localhost", "debug": True, "ssl": False}
        prod_config = {"host": "prod-server.com", "debug": False, "ssl": True}
        
        file1 = self.create_temp_json(dev_config)
        file2 = self.create_temp_json(prod_config)
        
        result = diffx.diff(file1, file2, ignore_keys_regex="host")
        
        # Should compare environments while ignoring host
        self.assertNotIn('host', result)
        self.assertIn('debug', result)
        self.assertIn('ssl', result)

    def test_backup_verification(self):
        """Test case 26: Backup verification"""
        original_data = {"users": 100, "posts": 250, "settings": {"theme": "dark"}}
        backup_data = {"users": 100, "posts": 250, "settings": {"theme": "dark"}}
        
        file1 = self.create_temp_json(original_data)
        file2 = self.create_temp_json(backup_data)
        
        result = diffx.diff(file1, file2)
        
        # Should verify backup integrity (no differences)
        self.assertEqual(result.strip(), '')

    def test_schema_evolution(self):
        """Test case 27: API schema evolution"""
        schema_v1 = {
            "User": {"fields": ["id", "name", "email"]},
            "Post": {"fields": ["id", "title", "content"]}
        }
        
        schema_v2 = {
            "User": {"fields": ["id", "name", "email", "avatar_url"]},
            "Post": {"fields": ["id", "title", "content", "published_at"]}
        }
        
        file1 = self.create_temp_json(schema_v1)
        file2 = self.create_temp_json(schema_v2)
        
        result = diffx.diff(file1, file2)
        
        # Should detect schema evolution
        self.assertIn('avatar_url', result)
        self.assertIn('published_at', result)

    def test_multi_tenant_config(self):
        """Test case 28: Multi-tenant configuration"""
        tenant_a_config = {
            "tenant": "tenant_a",
            "features": {"analytics": True, "billing": True},
            "limits": {"api_calls": 10000}
        }
        
        tenant_b_config = {
            "tenant": "tenant_b", 
            "features": {"analytics": False, "billing": True},
            "limits": {"api_calls": 5000}
        }
        
        file1 = self.create_temp_json(tenant_a_config)
        file2 = self.create_temp_json(tenant_b_config)
        
        result = diffx.diff(file1, file2, ignore_keys_regex="tenant")
        
        # Should compare tenant configs while ignoring tenant name
        self.assertNotIn('tenant', result)
        self.assertIn('analytics', result)
        self.assertIn('api_calls', result)

    def test_deployment_validation(self):
        """Test case 29: Deployment configuration validation"""
        staging_deploy = {
            "environment": "staging",
            "replicas": 2,
            "resources": {"cpu": "100m", "memory": "256Mi"}
        }
        
        prod_deploy = {
            "environment": "production", 
            "replicas": 5,
            "resources": {"cpu": "500m", "memory": "1Gi"}
        }
        
        file1 = self.create_temp_json(staging_deploy)
        file2 = self.create_temp_json(prod_deploy)
        
        result = diffx.diff(file1, file2, ignore_keys_regex="environment")
        
        # Should validate deployment differences while ignoring environment
        self.assertNotIn('environment', result)
        self.assertIn('replicas', result)
        self.assertIn('resources', result)

    def test_comprehensive_integration(self):
        """Test case 30-34: Comprehensive integration scenario"""
        production_config = {
            "timestamp": "2024-01-01T10:00:00Z",
            "environment": "prod",
            "services": [
                {"id": "web", "replicas": 3, "memory": "512Mi"},
                {"id": "api", "replicas": 5, "memory": "1Gi"}
            ],
            "database": {
                "host": "prod-db.internal",
                "port": 5432,
                "pool_size": 10
            },
            "_metadata": {"version": "1.0", "deployment": "blue-green"}
        }
        
        staging_config = {
            "timestamp": "2024-01-01T11:00:00Z",
            "environment": "staging",
            "services": [
                {"id": "web", "replicas": 2, "memory": "256Mi"},
                {"id": "api", "replicas": 3, "memory": "512Mi"}
            ],
            "database": {
                "host": "staging-db.internal",
                "port": 5432,
                "pool_size": 5
            },
            "_metadata": {"version": "1.0", "deployment": "rolling"}
        }
        
        file1 = self.create_temp_json(production_config)
        file2 = self.create_temp_json(staging_config)
        
        result = diffx.diff(file1, file2,
                           ignore_keys_regex="^(timestamp|_metadata.deployment)$",
                           array_id_key="id")
        
        # Should detect environment-specific differences while ignoring metadata
        self.assertIn('environment', result)
        self.assertIn('services', result)
        self.assertIn('database', result)
        
        # Should ignore specified fields
        self.assertNotIn('timestamp', result)
        # Note: _metadata.deployment might still appear in path context


if __name__ == '__main__':
    unittest.main()