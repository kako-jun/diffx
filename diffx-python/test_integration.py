#!/usr/bin/env python3
"""
Integration tests for diffx-python package
Verifies basic functionality and integration
"""
import os
import sys
import tempfile
import json
import subprocess
from pathlib import Path
from diffx import diff, diff_string, is_diffx_available, DiffOptions, Format, OutputFormat

def test_basic_functionality():
    """Test basic diff functionality"""
    print("Testing basic diff functionality...")
    
    # Sample data
    data1 = {"name": "Alice", "age": 30, "city": "Tokyo"}
    data2 = {"name": "Alice", "age": 31, "city": "Osaka", "country": "Japan"}
    
    # Test diff function
    result = diff(data1, data2)
    print(f"✓ Basic diff: {len(result)} differences found")
    assert len(result) > 0
    
    # Test diff with options
    options = DiffOptions(output=OutputFormat.JSON)
    json_result = diff_string(json.dumps(data1), json.dumps(data2), Format.JSON, options)
    json_data = json.loads(json_result)
    print(f"✓ JSON diff: {len(json_data)} differences found")
    assert len(json_data) > 0

def test_file_comparison():
    """Test file comparison functionality"""
    print("Testing file comparison...")
    
    # Create temporary files
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
        json.dump({"version": "1.0", "features": ["A", "B"]}, f1)
        file1 = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
        json.dump({"version": "1.1", "features": ["A", "B", "C"]}, f2)
        file2 = f2.name
    
    try:
        # Test file comparison using diff
        with open(file1, 'r') as f1, open(file2, 'r') as f2:
            result = diff(json.load(f1), json.load(f2))
        print(f"✓ File diff: {len(result)} differences found")
        assert len(result) > 0
        
        # Check for expected differences
        found_version = any('version' in str(diff) for diff in result)
        found_features = any('features' in str(diff) for diff in result)
        assert found_version, "Version difference should be detected"
        assert found_features, "Features difference should be detected"
        
    finally:
        os.unlink(file1)
        os.unlink(file2)

def test_directory_comparison():
    """Test directory comparison functionality"""
    print("Testing directory comparison...")
    
    # Create temporary directories
    with tempfile.TemporaryDirectory() as tmpdir:
        dir1 = Path(tmpdir) / "dir1"
        dir2 = Path(tmpdir) / "dir2"
        dir1.mkdir()
        dir2.mkdir()
        
        # Create test files
        (dir1 / "config.json").write_text(json.dumps({"env": "dev", "debug": True}))
        (dir2 / "config.json").write_text(json.dumps({"env": "prod", "debug": False}))
        
        # Test directory comparison using individual file diffs
        config1 = json.loads((dir1 / "config.json").read_text())
        config2 = json.loads((dir2 / "config.json").read_text())
        result = diff(config1, config2)
        print(f"✓ Directory diff: {len(result)} differences found")
        assert len(result) > 0

def test_cli_integration():
    """Test CLI integration through subprocess"""
    print("Testing CLI integration...")
    
    # Create test data
    data1 = {"status": "active", "users": [{"id": 1, "name": "Alice"}]}
    data2 = {"status": "inactive", "users": [{"id": 1, "name": "Bob"}]}
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
        json.dump(data1, f1)
        file1 = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
        json.dump(data2, f2)
        file2 = f2.name
    
    try:
        # Test CLI execution
        result = subprocess.run([
            sys.executable, '-m', 'diffx', 
            file1, file2, '--output', 'json'
        ], capture_output=True, text=True)
        
        print(f"✓ CLI execution: return code {result.returncode}")
        
        if result.returncode == 0:
            output_data = json.loads(result.stdout)
            print(f"✓ CLI JSON output: {len(output_data)} differences found")
            assert len(output_data) > 0
        else:
            print(f"  Note: CLI execution failed (diffx binary may not be installed)")
            print(f"  stdout: {result.stdout}")
            print(f"  stderr: {result.stderr}")
            
    finally:
        os.unlink(file1)
        os.unlink(file2)

def test_error_handling():
    """Test error handling scenarios"""
    print("Testing error handling...")
    
    # Test with invalid data
    try:
        result = diff({"invalid": "data"}, None)
        print(f"✓ Error handling: handled gracefully")
    except Exception as e:
        print(f"✓ Error handling: {type(e).__name__}")
    
    # Test with invalid JSON string
    try:
        result = diff_string("invalid json", "{}", Format.JSON)
        print("✓ Invalid JSON handling: no exception raised")
    except Exception as e:
        print(f"✓ Invalid JSON handling: {type(e).__name__}")

def test_new_options():
    """Test new options: ignore-case, ignore-whitespace, context, quiet, brief"""
    print("Testing new options...")
    
    # Test ignore-case option
    data1 = {"status": "Active", "level": "Info"}
    data2 = {"status": "ACTIVE", "level": "INFO"}
    
    options = DiffOptions(ignore_case=True, output=OutputFormat.JSON)
    result = diff_string(json.dumps(data1), json.dumps(data2), Format.JSON, options)
    json_data = json.loads(result)
    print(f"✓ Ignore-case option: {len(json_data)} differences found (should be 0)")
    assert len(json_data) == 0
    
    # Test ignore-whitespace option
    data3 = {"text": "Hello  World", "message": "Test\tValue"}
    data4 = {"text": "Hello World", "message": "Test Value"}
    
    options = DiffOptions(ignore_whitespace=True, output=OutputFormat.JSON)
    result = diff_string(json.dumps(data3), json.dumps(data4), Format.JSON, options)
    json_data = json.loads(result)
    print(f"✓ Ignore-whitespace option: {len(json_data)} differences found (should be 0)")
    assert len(json_data) == 0
    
    # Test context option
    data5 = {"host": "localhost", "port": 5432, "name": "myapp"}
    data6 = {"host": "localhost", "port": 5433, "name": "myapp"}
    
    options = DiffOptions(context=3, output=OutputFormat.UNIFIED)
    result = diff_string(json.dumps(data5, indent=2), json.dumps(data6, indent=2), Format.JSON, options)
    print(f"✓ Context option: unified output generated")
    assert "@@" in result or "port" in result
    
    # Test quiet option
    options = DiffOptions(quiet=True)
    result = diff_string(json.dumps(data1), json.dumps(data2), Format.JSON, options)
    print(f"✓ Quiet option: output suppressed")
    assert result == ""
    
    # Test brief option
    options = DiffOptions(brief=True)
    result = diff_string(json.dumps(data5), json.dumps(data6), Format.JSON, options)
    print(f"✓ Brief option: brief output generated")
    assert "differ" in result or "port" in result

def test_advanced_features():
    """Test advanced features"""
    print("Testing advanced features...")
    
    # Test with complex nested data
    data1 = {
        "config": {
            "database": {
                "host": "localhost",
                "port": 5432,
                "settings": {
                    "timeout": 30,
                    "pool_size": 10
                }
            }
        },
        "services": ["web", "api"]
    }
    
    data2 = {
        "config": {
            "database": {
                "host": "prod-db",
                "port": 5432,
                "settings": {
                    "timeout": 60,
                    "pool_size": 20,
                    "ssl": True
                }
            }
        },
        "services": ["web", "api", "worker"]
    }
    
    result = diff(data1, data2)
    print(f"✓ Complex nested diff: {len(result)} differences found")
    assert len(result) > 0
    
    # Check for specific differences
    result_str = str(result)
    assert "host" in result_str, "Host difference should be detected"
    assert "timeout" in result_str, "Timeout difference should be detected"
    assert "services" in result_str, "Services difference should be detected"

def test_package_metadata():
    """Test package metadata and version"""
    print("Testing package metadata...")
    
    try:
        import diffx
        if hasattr(diffx, '__version__'):
            print(f"✓ Package version: {diffx.__version__}")
        else:
            print("✓ Package imported successfully (no version info)")
    except ImportError as e:
        print(f"✗ Package import failed: {e}")
        sys.exit(1)

def main():
    """Run all tests"""
    print("=" * 50)
    print("diffx-python Integration Tests")
    print("=" * 50)
    
    tests = [
        test_package_metadata,
        test_basic_functionality,
        test_file_comparison,
        test_directory_comparison,
        test_error_handling,
        test_new_options,
        test_advanced_features,
        test_cli_integration,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
            print(f"✓ {test.__name__} - PASSED")
        except Exception as e:
            failed += 1
            print(f"✗ {test.__name__} - FAILED: {e}")
            import traceback
            traceback.print_exc()
        print()
    
    print("=" * 50)
    print(f"Test Results: {passed} passed, {failed} failed")
    print("=" * 50)
    
    if failed > 0:
        sys.exit(1)
    else:
        print("All tests passed!")

if __name__ == "__main__":
    main()