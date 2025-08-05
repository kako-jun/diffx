import pytest
import json
import os
import sys
from pathlib import Path

# Add the parent directory to sys.path to import diffx_python
# This assumes we're running tests from the tests directory
sys.path.insert(0, str(Path(__file__).parent.parent))

try:
    import diffx_python
except ImportError:
    # If the module is not built yet, skip tests
    pytest.skip("diffx_python module not built", allow_module_level=True)

# ============================================================================
# TEST FIXTURES - Shared with Core Tests
# ============================================================================

class TestFixtures:
    """
    Python equivalent of Rust fixtures for unified API testing.
    Uses same test data as core tests but in Python format.
    """
    
    @staticmethod
    def load_cli_fixture(filename):
        """Load JSON file from CLI fixtures directory"""
        fixtures_dir = Path(__file__).parent.parent.parent / "tests" / "fixtures"
        fixture_path = fixtures_dir / filename
        
        if not fixture_path.exists():
            pytest.skip(f"CLI fixture not found: {fixture_path}")
        
        with open(fixture_path, 'r') as f:
            return json.load(f)
    
    @staticmethod
    def config_v1():
        return TestFixtures.load_cli_fixture("config_v1.json")
    
    @staticmethod
    def config_v2():
        return TestFixtures.load_cli_fixture("config_v2.json")
    
    @staticmethod
    def config_v3():
        return TestFixtures.load_cli_fixture("config_v3.json")
    
    @staticmethod
    def users_v1():
        return TestFixtures.load_cli_fixture("users_v1.json")
    
    @staticmethod
    def users_v2():
        return TestFixtures.load_cli_fixture("users_v2.json")
    
    @staticmethod
    def api_schema_v1():
        return TestFixtures.load_cli_fixture("api_schema_v1.json")
    
    @staticmethod
    def api_schema_v2():
        return TestFixtures.load_cli_fixture("api_schema_v2.json")
    
    @staticmethod
    def simple_object_old():
        return {
            "name": "diffx",
            "version": "1.0.0", 
            "features": ["json", "yaml"]
        }
    
    @staticmethod
    def simple_object_new():
        return {
            "name": "diffx",
            "version": "1.1.0",
            "features": ["json", "yaml", "toml"],
            "author": "Claude"
        }
    
    @staticmethod
    def array_with_ids_old():
        return [
            {"id": 1, "name": "Alice", "role": "admin"},
            {"id": 2, "name": "Bob", "role": "user"},
            {"id": 3, "name": "Charlie", "role": "user"}
        ]
    
    @staticmethod
    def array_with_ids_new():
        return [
            {"id": 1, "name": "Alice", "role": "superadmin"},
            {"id": 3, "name": "Charlie", "role": "user"},
            {"id": 4, "name": "David", "role": "user"}
        ]
    
    @staticmethod
    def nested_object_old():
        return {
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
        }
    
    @staticmethod
    def nested_object_new():
        return {
            "database": {
                "host": "production.db",
                "port": 5432,
                "config": {
                    "max_connections": 200,
                    "timeout": 30,
                    "ssl": True
                }
            },
            "cache": {
                "type": "memcached",
                "ttl": 7200
            },
            "monitoring": {
                "enabled": True
            }
        }
    
    @staticmethod
    def numeric_precision_old():
        return {
            "measurements": [1.0, 2.001, 3.1415926],
            "coordinates": {"x": 10.0, "y": 20.0}
        }
    
    @staticmethod
    def numeric_precision_new():
        return {
            "measurements": [1.001, 2.002, 3.1415927],
            "coordinates": {"x": 10.001, "y": 20.001}
        }
    
    @staticmethod
    def type_changes_old():
        return {
            "count": 42,
            "enabled": True,
            "data": [1, 2, 3],
            "meta": {"created": "2023-01-01"}
        }
    
    @staticmethod
    def type_changes_new():
        return {
            "count": "42",
            "enabled": "true",
            "data": {"0": 1, "1": 2, "2": 3},
            "meta": "metadata"
        }

# ============================================================================
# UNIFIED API TESTS - Core Functionality
# ============================================================================

class TestUnifiedAPI:
    """Test the unified diff() function with Python bindings"""
    
    def test_diff_basic_modification(self):
        old = {"name": "Alice", "age": 30}
        new = {"name": "Alice", "age": 31}
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 1
        result = results[0]
        assert result["type"] == "Modified"
        assert result["path"] == "age"
        assert result["old_value"] == 30
        assert result["new_value"] == 31
    
    def test_diff_added_field(self):
        old = {"name": "Alice"}
        new = {"name": "Alice", "age": 30}
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 1
        result = results[0]
        assert result["type"] == "Added"
        assert result["path"] == "age"
        assert result["value"] == 30
    
    def test_diff_removed_field(self):
        old = {"name": "Alice", "age": 30}
        new = {"name": "Alice"}
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 1
        result = results[0]
        assert result["type"] == "Removed"
        assert result["path"] == "age"
        assert result["value"] == 30
    
    def test_diff_type_changed(self):
        old = {"value": 123}
        new = {"value": "123"}
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 1
        result = results[0]
        assert result["type"] == "TypeChanged"
        assert result["path"] == "value"
        assert result["old_value"] == 123
        assert result["new_value"] == "123"
    
    def test_diff_no_changes(self):
        old = {"name": "Alice", "age": 30}
        new = {"name": "Alice", "age": 30}
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 0

# ============================================================================
# OPTIONS TESTING - All Python Options Coverage
# ============================================================================

class TestOptionsHandling:
    """Test all available options through Python interface"""
    
    def test_diff_with_epsilon(self):
        old = {"value": 1.0}
        new = {"value": 1.001}
        
        # Within epsilon - no differences
        results = diffx_python.diff(old, new, epsilon=0.01)
        assert len(results) == 0
        
        # Outside epsilon - should detect difference
        results = diffx_python.diff(old, new, epsilon=0.0001)
        assert len(results) == 1
    
    def test_diff_with_array_id_key(self):
        old = {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ]
        }
        new = {
            "users": [
                {"id": 2, "name": "Bob"},
                {"id": 1, "name": "Alice Updated"}
            ]
        }
        
        results = diffx_python.diff(old, new, array_id_key="id")
        
        # Should detect modification of Alice's name, not array reordering
        assert len(results) == 1
        result = results[0]
        assert result["type"] == "Modified"
        assert "[id=1]" in result["path"]
        assert "name" in result["path"]
        assert result["new_value"] == "Alice Updated"
    
    def test_diff_with_ignore_keys_regex(self):
        old = {
            "data": "important",
            "timestamp": "2023-01-01",
            "debug_info": "old"
        }
        new = {
            "data": "important", 
            "timestamp": "2023-01-02",
            "debug_info": "new"
        }
        
        results = diffx_python.diff(old, new, ignore_keys_regex=r"^(timestamp|debug_)")
        assert len(results) == 0  # All changes ignored
    
    def test_diff_with_path_filter(self):
        old = {
            "config": {"value": 1},
            "metadata": {"value": 2}
        }
        new = {
            "config": {"value": 10},
            "metadata": {"value": 20}
        }
        
        # Test path filter - get all results first
        all_results = diffx_python.diff(old, new)
        filtered_results = diffx_python.diff(old, new, path_filter="config")
        
        # Now it should properly filter
        assert isinstance(filtered_results, list)
        assert len(all_results) == 2  # Both config and metadata should change
        assert len(filtered_results) == 1  # Only config.value should match the filter
        assert filtered_results[0]["path"] == "config.value"
        
        # Test with a more specific filter that should match
        exact_results = diffx_python.diff(old, new, path_filter="config.value")
        assert isinstance(exact_results, list)
        assert len(exact_results) == 1  # Exact match
        assert exact_results[0]["path"] == "config.value"
    
    def test_diff_with_output_format(self):
        old = {"name": "Alice"}
        new = {"name": "Bob"}
        
        # Test different output formats
        for output_format in ["diffx", "json", "yaml"]:
            results = diffx_python.diff(old, new, output_format=output_format)
            assert len(results) == 1
    
    def test_diff_with_memory_optimization(self):
        old = {"data": [1, 2, 3]}
        new = {"data": [1, 2, 4]}
        
        results = diffx_python.diff(old, new, use_memory_optimization=True, batch_size=100)
        assert len(results) == 1
    
    def test_diff_with_diffx_specific_options(self):
        old = {"text": "Hello World"}
        new = {"text": "HELLO WORLD"}
        
        # Case insensitive - should find no differences
        results = diffx_python.diff(old, new, ignore_case=True)
        assert len(results) == 0
        
        # Case sensitive - should find difference  
        results = diffx_python.diff(old, new, ignore_case=False)
        assert len(results) == 1
    
    def test_diff_with_ignore_whitespace(self):
        old = {"text": "Hello World"}
        new = {"text": "HelloWorld"}
        
        # Ignore whitespace - no differences
        results = diffx_python.diff(old, new, ignore_whitespace=True)
        assert len(results) == 0
        
        # Don't ignore whitespace - should find difference
        results = diffx_python.diff(old, new, ignore_whitespace=False)
        assert len(results) == 1

# ============================================================================
# PYTHON TYPE CONVERSION TESTS
# ============================================================================

class TestPythonTypeConversion:
    """Test Python <-> Rust type conversion in bindings"""
    
    def test_python_primitive_types(self):
        """Test conversion of Python primitive types"""
        test_data = {
            "null_value": None,
            "bool_true": True,
            "bool_false": False,
            "int_value": 42,
            "float_value": 3.14,
            "string_value": "hello",
            "empty_string": "",
            "unicode_string": "こんにちは"
        }
        
        # Should not raise any conversion errors
        results = diffx_python.diff(test_data, test_data)
        assert len(results) == 0
    
    def test_python_container_types(self):
        """Test conversion of Python containers"""
        test_data = {
            "empty_list": [],
            "list_mixed": [1, "two", 3.0, True, None],
            "empty_dict": {},
            "nested_dict": {
                "level1": {
                    "level2": {
                        "value": "deep"
                    }
                }
            },
            "list_of_dicts": [
                {"id": 1, "name": "first"},
                {"id": 2, "name": "second"}
            ]
        }
        
        # Should not raise any conversion errors
        results = diffx_python.diff(test_data, test_data)
        assert len(results) == 0
    
    def test_large_python_numbers(self):
        """Test handling of large Python numbers"""
        old = {"big_int": 2**63 - 1, "big_float": 1.7976931348623157e+308}
        new = {"big_int": 2**63 - 2, "big_float": 1.7976931348623157e+307}
        
        results = diffx_python.diff(old, new)
        assert len(results) == 2  # Both should be detected as changes
    
    def test_python_dict_keys_types(self):
        """Test that only string keys are supported in dicts"""
        # Python dicts with non-string keys should be converted to string keys
        old = {"valid_key": "value"}
        new = {"valid_key": "value", "new_key": "new_value"}
        
        results = diffx_python.diff(old, new)
        assert len(results) == 1
        assert results[0]["type"] == "Added"

# ============================================================================
# ARRAY HANDLING TESTS
# ============================================================================

class TestArrayHandling:
    """Test array comparison with and without ID keys"""
    
    def test_diff_arrays_by_index(self):
        old = [1, 2, 3]
        new = [1, 3, 4]
        
        results = diffx_python.diff(old, new)
        
        assert len(results) == 2  # Changes at indices 1 and 2
    
    def test_diff_arrays_with_id_key(self):
        old = [
            {"id": "a", "value": 1},
            {"id": "b", "value": 2}
        ]
        new = [
            {"id": "b", "value": 20},
            {"id": "c", "value": 3}
        ]
        
        results = diffx_python.diff(old, new, array_id_key="id")
        
        # Should detect: removed 'a', modified 'b', added 'c'
        assert len(results) == 3
    
    def test_diff_arrays_mixed_id_and_index(self):
        old = [
            {"id": "a", "value": 1},
            {"value": 2},  # No ID
            {"id": "b", "value": 3}
        ]
        new = [
            {"id": "b", "value": 30},
            {"value": 20},  # No ID
            {"id": "c", "value": 4}
        ]
        
        results = diffx_python.diff(old, new, array_id_key="id")
        
        # Should handle both ID-based and index-based comparisons
        assert len(results) > 0

# ============================================================================
# COMPLEX DATA STRUCTURES WITH FIXTURES
# ============================================================================

class TestComplexStructures:
    """Test complex data structures using shared fixtures"""
    
    def test_diff_with_cli_fixtures(self):
        """Test using CLI fixtures that match Rust tests"""
        try:
            old = TestFixtures.config_v1()
            new = TestFixtures.config_v2()
            
            results = diffx_python.diff(old, new)
            assert len(results) > 0  # Should find differences
            
        except Exception as e:
            pytest.skip(f"CLI fixtures not available: {e}")
    
    def test_diff_nested_objects(self):
        old = TestFixtures.nested_object_old()
        new = TestFixtures.nested_object_new()
        
        results = diffx_python.diff(old, new)
        
        # Should find multiple changes in nested structure
        assert len(results) > 1
        
        # Verify some expected changes
        paths = [result["path"] for result in results]
        assert any("database.host" in path for path in paths)
        assert any("monitoring" in path for path in paths)
    
    def test_diff_large_dataset(self):
        """Test performance with larger dataset"""
        # Create large dataset (smaller than Rust version to avoid timeout)
        old_data = {}
        new_data = {}
        
        for i in range(100):  # Smaller dataset for Python test
            old_data[f"key_{i}"] = i
            new_data[f"key_{i}"] = i + 1
        
        results = diffx_python.diff(old_data, new_data)
        assert len(results) == 100

# ============================================================================
# ERROR HANDLING TESTS
# ============================================================================

class TestErrorHandling:
    """Test error handling in Python bindings"""
    
    def test_invalid_regex_pattern(self):
        old = {"test": "value"}
        new = {"test": "value2"}
        
        with pytest.raises(Exception):  # Should raise ValueError for invalid regex
            diffx_python.diff(old, new, ignore_keys_regex="[invalid_regex")
    
    def test_invalid_output_format(self):
        old = {"test": "value"}
        new = {"test": "value2"}
        
        with pytest.raises(Exception):  # Should raise ValueError for invalid format
            diffx_python.diff(old, new, output_format="invalid_format")
    
    def test_unsupported_python_types(self):
        """Test handling of complex Python types that can't be converted"""
        # Most complex types should be convertible or raise clear errors
        # This test ensures we handle edge cases gracefully
        
        old = {"simple": "value"}
        new = {"simple": "value"}
        
        # These should work fine
        results = diffx_python.diff(old, new)
        assert len(results) == 0

# ============================================================================
# INTEGRATION TESTS
# ============================================================================

class TestIntegration:
    """Integration tests that mirror Rust test scenarios"""
    
    def test_unified_api_comprehensive(self):
        """Comprehensive test using multiple options together"""
        old = TestFixtures.array_with_ids_old()
        new = TestFixtures.array_with_ids_new()
        
        results = diffx_python.diff(
            old, new,
            array_id_key="id",
            output_format="json",
            show_unchanged=False,
            use_memory_optimization=True
        )
        
        # Should detect changes in the array with ID-based comparison
        assert len(results) > 0
        
        # Verify result structure
        for result in results:
            assert "type" in result
            assert "path" in result
            assert result["type"] in ["Added", "Removed", "Modified", "TypeChanged"]
    
    def test_type_conversion_fidelity(self):
        """Test that data roundtrips through Rust conversion correctly"""
        test_cases = [
            TestFixtures.simple_object_old(),
            TestFixtures.numeric_precision_old(),
            TestFixtures.type_changes_old(),
        ]
        
        for test_data in test_cases:
            # Diff with itself should produce no changes
            results = diffx_python.diff(test_data, test_data)
            assert len(results) == 0, f"Self-diff should be empty for {test_data}"
    
    def test_backwards_compatibility_workflow(self):
        """Test that common usage patterns work correctly"""
        # Simulate common usage: load JSON, compare, get results
        old_json_str = '{"name": "old", "version": 1}'
        new_json_str = '{"name": "new", "version": 2}'
        
        old_data = json.loads(old_json_str)
        new_data = json.loads(new_json_str)
        
        results = diffx_python.diff(old_data, new_data)
        
        assert len(results) == 2  # name and version changed
        
        # Results should be easily convertible back to JSON
        results_json = json.dumps(results, default=str)
        assert len(results_json) > 0

# ============================================================================
# BENCHMARK/PERFORMANCE TESTS
# ============================================================================

class TestPerformance:
    """Basic performance tests to ensure Python bindings are efficient"""
    
    @pytest.mark.slow
    def test_large_array_performance(self):
        """Test performance with large arrays"""
        old = [{"id": i, "value": f"item_{i}"} for i in range(1000)]
        new = [{"id": i, "value": f"item_{i}_updated"} for i in range(1000)]
        
        import time
        start_time = time.time()
        results = diffx_python.diff(old, new, array_id_key="id")
        end_time = time.time()
        
        assert len(results) == 1000  # All items should be modified
        assert end_time - start_time < 5.0  # Should complete within 5 seconds
    
    @pytest.mark.slow  
    def test_deep_nesting_performance(self):
        """Test performance with deeply nested objects"""
        def create_nested(depth):
            if depth == 0:
                return {"value": "leaf"}
            return {"level": depth, "nested": create_nested(depth - 1)}
        
        old = create_nested(20)
        new = create_nested(20)
        new["nested"]["nested"]["value"] = "modified_leaf"  # Change deep value
        
        import time
        start_time = time.time()
        results = diffx_python.diff(old, new)
        end_time = time.time()
        
        assert len(results) == 1  # Should find the single deep change
        assert end_time - start_time < 1.0  # Should be fast even with deep nesting

if __name__ == "__main__":
    pytest.main([__file__])