#!/usr/bin/env python3
"""
Test all diffx options combinations for Python package
"""

import sys
import os
sys.path.insert(0, 'diffx-python/src')

from diffx.diffx import DiffOptions, diff, diff_string, DiffError
import tempfile
import json

# Test data
test_data1 = {
    "name": "Alice",
    "age": 30,
    "settings": {
        "theme": "dark",
        "notifications": True,
        "limits": {
            "max_files": 100,
            "timeout": 30.5
        }
    },
    "users": [
        {"id": 1, "name": "User1", "active": True},
        {"id": 2, "name": "User2", "active": False}
    ]
}

test_data2 = {
    "name": "Alice",
    "age": 31,
    "settings": {
        "theme": "light",
        "notifications": True,
        "limits": {
            "max_files": 150,
            "timeout": 30.501
        }
    },
    "users": [
        {"id": 1, "name": "User1", "active": True},
        {"id": 3, "name": "User3", "active": True}
    ]
}

def test_all_option_combinations():
    """Test all possible option combinations"""
    print("🧪 Testing all diffx option combinations...")
    
    # Create test files
    with tempfile.TemporaryDirectory() as tmp_dir:
        file1 = os.path.join(tmp_dir, "test1.json")
        file2 = os.path.join(tmp_dir, "test2.json")
        
        with open(file1, 'w') as f:
            json.dump(test_data1, f, indent=2)
        with open(file2, 'w') as f:
            json.dump(test_data2, f, indent=2)
        
        test_cases = [
            # Basic options
            ("Basic diff", DiffOptions()),
            ("JSON format", DiffOptions(format="json")),
            ("YAML format", DiffOptions(format="yaml")),
            # TOML format test skipped for JSON files - would need .toml files
            
            # Output formats
            ("CLI output", DiffOptions(output="cli")),
            ("JSON output", DiffOptions(output="json")),
            ("YAML output", DiffOptions(output="yaml")),
            ("Unified output", DiffOptions(output="unified")),
            
            # Path filtering
            ("Path filter", DiffOptions(path="settings.theme")),
            ("Nested path", DiffOptions(path="settings.limits.max_files")),
            
            # Regex ignore
            ("Ignore id keys", DiffOptions(ignore_keys_regex="^id$")),
            ("Ignore name keys", DiffOptions(ignore_keys_regex="name")),
            
            # Epsilon tolerance
            ("Epsilon 0.1", DiffOptions(epsilon=0.1)),
            ("Epsilon 0.001", DiffOptions(epsilon=0.001)),
            
            # Array ID key
            ("Array ID key", DiffOptions(array_id_key="id")),
            
            # Performance options
            ("Optimize", DiffOptions(optimize=True)),
            ("Batch size", DiffOptions(batch_size=500)),
            ("Optimize + Batch", DiffOptions(optimize=True, batch_size=500)),
            
            # Complex combinations
            ("Multi-option 1", DiffOptions(
                format="json",
                output="json",
                path="settings",
                epsilon=0.01
            )),
            ("Multi-option 2", DiffOptions(
                output="yaml",
                ignore_keys_regex="^id$",
                array_id_key="id",
                optimize=True
            )),
            ("Multi-option 3", DiffOptions(
                format="json",
                output="json",
                path="users",
                array_id_key="id",
                batch_size=100
            )),
            ("All options", DiffOptions(
                format="json",
                output="json",
                path="settings",
                ignore_keys_regex="timeout",
                epsilon=0.01,
                array_id_key="id",
                optimize=True,
                batch_size=1000
            ))
        ]
        
        success_count = 0
        total_count = len(test_cases)
        
        for name, options in test_cases:
            try:
                result = diff(file1, file2, options)
                print(f"✓ {name}: OK")
                success_count += 1
            except DiffError as e:
                print(f"✗ {name}: FAILED - {e}")
            except Exception as e:
                print(f"✗ {name}: ERROR - {e}")
        
        # Add TOML format test with proper files
        try:
            result = diff("tests/fixtures/file1.toml", "tests/fixtures/file2.toml", DiffOptions(format="toml"))
            print("✓ TOML format: OK")
            success_count += 1
        except DiffError as e:
            print(f"✗ TOML format: FAILED - {e}")
        except Exception as e:
            print(f"✗ TOML format: ERROR - {e}")
        
        total_count += 1
        
        print(f"\n📊 Results: {success_count}/{total_count} tests passed")
        return success_count == total_count

def test_string_diff_combinations():
    """Test string diff with various combinations"""
    print("\n🧪 Testing string diff combinations...")
    
    json1 = json.dumps(test_data1, indent=2)
    json2 = json.dumps(test_data2, indent=2)
    
    yaml1 = '''name: Alice
age: 30
settings:
  theme: dark
  notifications: true'''
    
    yaml2 = '''name: Alice
age: 31
settings:
  theme: light
  notifications: true'''
    
    test_cases = [
        ("String JSON", json1, json2, "json", DiffOptions()),
        ("String JSON with options", json1, json2, "json", DiffOptions(output="json", epsilon=0.1)),
        ("String YAML", yaml1, yaml2, "yaml", DiffOptions()),
        ("String YAML with options", yaml1, yaml2, "yaml", DiffOptions(output="yaml", path="settings")),
    ]
    
    success_count = 0
    total_count = len(test_cases)
    
    for name, content1, content2, format_type, options in test_cases:
        try:
            result = diff_string(content1, content2, format_type, options)
            print(f"✓ {name}: OK")
            success_count += 1
        except DiffError as e:
            print(f"✗ {name}: FAILED - {e}")
        except Exception as e:
            print(f"✗ {name}: ERROR - {e}")
    
    print(f"\n📊 String diff results: {success_count}/{total_count} tests passed")
    return success_count == total_count

def test_edge_cases():
    """Test edge cases and error conditions"""
    print("\n🧪 Testing edge cases...")
    
    test_cases = [
        ("Empty options", lambda: diff("tests/fixtures/config_v1.json", "tests/fixtures/config_v2.json", None)),
        ("Non-existent file", lambda: diff("nonexistent1.json", "nonexistent2.json", DiffOptions())),
        ("Invalid regex", lambda: diff("tests/fixtures/config_v1.json", "tests/fixtures/config_v2.json", 
                                      DiffOptions(ignore_keys_regex="[invalid"))),
    ]
    
    success_count = 0
    total_count = len(test_cases)
    
    for name, test_func in test_cases:
        try:
            if name == "Empty options":
                result = test_func()
                print(f"✓ {name}: OK (handled None options)")
                success_count += 1
            else:
                result = test_func()
                print(f"✗ {name}: Should have failed but didn't")
        except DiffError as e:
            print(f"✓ {name}: OK (properly caught error)")
            success_count += 1
        except Exception as e:
            print(f"✗ {name}: Unexpected error - {e}")
    
    print(f"\n📊 Edge case results: {success_count}/{total_count} tests passed")
    return success_count == total_count

if __name__ == "__main__":
    print("🔬 diffx Python Package - Comprehensive Option Testing")
    print("=" * 60)
    
    all_passed = True
    
    # Test all option combinations
    all_passed &= test_all_option_combinations()
    
    # Test string diff combinations  
    all_passed &= test_string_diff_combinations()
    
    # Test edge cases
    all_passed &= test_edge_cases()
    
    print("\n" + "=" * 60)
    if all_passed:
        print("🎉 All tests passed! Python package handles all option combinations correctly.")
        sys.exit(0)
    else:
        print("❌ Some tests failed. Please check the output above.")
        sys.exit(1)