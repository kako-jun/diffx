#!/usr/bin/env python3
"""Manual test script for diffx Python package."""

import json
import tempfile
from pathlib import Path

# Test both new and legacy APIs
import diffx
from diffx import run_diffx, DiffOptions

def test_availability():
    """Test if diffx is available."""
    print("🧪 Testing diffx availability...")
    available = diffx.is_diffx_available()
    print(f"OK: diffx available: {available}")
    assert available, "diffx should be available"

def test_string_comparison():
    """Test string comparison with new API."""
    print("\n🧪 Testing string comparison (new API)...")
    
    json1 = json.dumps({"name": "Alice", "age": 30, "city": "Tokyo"}, indent=2)
    json2 = json.dumps({"name": "Alice", "age": 31, "city": "Tokyo", "country": "Japan"}, indent=2)
    
    # Test JSON output
    result = diffx.diff_string(json1, json2, 'json', DiffOptions(output='json'))
    print(f"OK: JSON diff result: {len(result)} differences")
    
    for diff_item in result:
        if diff_item.added:
            print(f"  Added: {diff_item.added}")
        elif diff_item.modified:
            print(f"  Modified: {diff_item.modified}")
    
    # Test CLI output
    cli_result = diffx.diff_string(json1, json2, 'json')
    print(f"OK: CLI output:\n{cli_result}")

def test_legacy_api():
    """Test legacy API for backward compatibility."""
    print("\n🧪 Testing legacy API...")
    
    # Create temporary files
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_path = Path(tmp_dir)
        file1 = tmp_path / "test1.json"
        file2 = tmp_path / "test2.json"
        
        file1.write_text('{"name": "Alice", "age": 30}')
        file2.write_text('{"name": "Alice", "age": 31}')
        
        result = run_diffx([str(file1), str(file2)])
        print(f"OK: Legacy API - Return code: {result.returncode}")
        print(f"OK: Legacy API - Output:\n{result.stdout}")

def test_file_comparison():
    """Test file comparison."""
    print("\n🧪 Testing file comparison...")
    
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_path = Path(tmp_dir)
        file1 = tmp_path / "config1.json"
        file2 = tmp_path / "config2.json"
        
        config1 = {
            "database": {"host": "localhost", "port": 5432},
            "cache": {"enabled": True, "ttl": 3600}
        }
        config2 = {
            "database": {"host": "prod.example.com", "port": 5432},
            "cache": {"enabled": True, "ttl": 7200},
            "logging": {"level": "info"}
        }
        
        file1.write_text(json.dumps(config1, indent=2))
        file2.write_text(json.dumps(config2, indent=2))
        
        # Test with path filtering
        result = diffx.diff(
            str(file1), str(file2),
            DiffOptions(output='json', path='database')
        )
        print(f"OK: File diff (database only): {len(result)} differences")
        for diff_item in result:
            if diff_item.modified:
                print(f"  Modified: {diff_item.modified}")

def test_yaml_comparison():
    """Test YAML comparison."""
    print("\n🧪 Testing YAML comparison...")
    
    yaml1 = """
name: Alice
age: 30
hobbies:
  - reading
  - coding
"""
    
    yaml2 = """
name: Alice
age: 31
hobbies:
  - reading
  - music
  - coding
"""
    
    result = diffx.diff_string(yaml1, yaml2, 'yaml')
    print(f"OK: YAML diff:\n{result}")

def test_error_handling():
    """Test error handling."""
    print("\n🧪 Testing error handling...")
    
    try:
        diffx.diff('nonexistent1.json', 'nonexistent2.json')
        print("ERROR: Should have raised an error")
        assert False, "Should have raised DiffError"
    except diffx.DiffError as e:
        print(f"OK: Error handling works: {e}")

def main():
    """Run all tests."""
    print("diffx Python Package Manual Test Suite\n")
    
    test_availability()
    test_string_comparison()
    test_legacy_api()
    test_file_comparison()
    test_yaml_comparison()
    test_error_handling()
    
    print("\nAll tests completed successfully!")

if __name__ == "__main__":
    main()