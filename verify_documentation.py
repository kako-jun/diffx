#!/usr/bin/env python3
"""
Verify that documented command examples work as expected.
This script will systematically test each documented example.
"""

import subprocess
import json
import tempfile
import os
import sys
from pathlib import Path

def run_diffx_command(args, input_data=None):
    """Run diffx command and return stdout, stderr, and exit code."""
    cmd = ["cargo", "run", "--bin", "diffx", "--"] + args
    
    if input_data:
        process = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        stdout, stderr = process.communicate(input=input_data)
        return stdout, stderr, process.returncode
    else:
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.stdout, result.stderr, result.returncode

def create_test_files():
    """Create test files for documentation examples."""
    # Create temporary directory
    test_dir = Path("test_docs_verification")
    test_dir.mkdir(exist_ok=True)
    
    # Basic JSON files
    (test_dir / "config1.json").write_text('{"name": "Alice", "age": 30}')
    (test_dir / "config2.json").write_text('{"name": "Alice", "age": 31}')
    
    # Files with timestamps
    (test_dir / "file1.json").write_text('{"name": "Alice", "timestamp": "2024-01-01"}')
    (test_dir / "file2.json").write_text('{"name": "Alice", "timestamp": "2024-01-02"}')
    
    # Files with arrays
    (test_dir / "users1.json").write_text('{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}')
    (test_dir / "users2.json").write_text('{"users": [{"id": 1, "name": "Alice"}, {"id": 3, "name": "Charlie"}]}')
    
    # Files with floating point numbers
    (test_dir / "metrics1.json").write_text('{"value": 1.0001}')
    (test_dir / "metrics2.json").write_text('{"value": 1.0002}')
    
    # YAML files
    (test_dir / "config1.yaml").write_text('name: Alice\nage: 30')
    (test_dir / "config2.yaml").write_text('name: Alice\nage: 31')
    
    # TOML files
    (test_dir / "config1.toml").write_text('name = "Alice"\nage = 30')
    (test_dir / "config2.toml").write_text('name = "Alice"\nage = 31')
    
    return test_dir

def test_basic_usage():
    """Test basic usage examples from documentation."""
    print("🧪 Testing Basic Usage Examples")
    
    test_dir = create_test_files()
    
    # Test 1: Basic JSON comparison
    print("  Test 1: Basic JSON comparison")
    stdout, stderr, code = run_diffx_command([
        str(test_dir / "config1.json"),
        str(test_dir / "config2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Stdout: {stdout.strip()}")
    print(f"    Stderr: {stderr.strip()}")
    
    # Expected: Exit code 1 (differences found), output should show age difference
    expected_in_output = ["age", "30", "31"]
    actual_results = {
        "exit_code": code,
        "has_age_diff": "age" in stdout,
        "has_old_value": "30" in stdout,
        "has_new_value": "31" in stdout
    }
    
    print(f"    Expected: Exit code 1, output contains age/30/31")
    print(f"    Actual: {actual_results}")
    print()
    
    return actual_results

def test_help_version():
    """Test help and version commands."""
    print("🧪 Testing Help and Version Commands")
    
    # Test help command
    print("  Test 1: --help")
    stdout, stderr, code = run_diffx_command(["--help"])
    print(f"    Exit code: {code}")
    print(f"    Contains 'Usage:': {'Usage:' in stdout}")
    print(f"    Contains 'diffx': {'diffx' in stdout}")
    print()
    
    # Test version command
    print("  Test 2: --version")
    stdout, stderr, code = run_diffx_command(["--version"])
    print(f"    Exit code: {code}")
    print(f"    Contains 'diffx': {'diffx' in stdout}")
    print(f"    Output: {stdout.strip()}")
    print()

def test_format_options():
    """Test format specification options."""
    print("🧪 Testing Format Options")
    
    test_dir = create_test_files()
    
    # Test explicit JSON format
    print("  Test 1: --format json")
    stdout, stderr, code = run_diffx_command([
        "--format", "json",
        str(test_dir / "config1.json"),
        str(test_dir / "config2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()
    
    # Test YAML format
    print("  Test 2: YAML files")
    stdout, stderr, code = run_diffx_command([
        str(test_dir / "config1.yaml"),
        str(test_dir / "config2.yaml")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()

def test_output_formats():
    """Test output format options."""
    print("🧪 Testing Output Formats")
    
    test_dir = create_test_files()
    
    # Test JSON output
    print("  Test 1: --output json")
    stdout, stderr, code = run_diffx_command([
        "--output", "json",
        str(test_dir / "config1.json"),
        str(test_dir / "config2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Starts with '[': {stdout.strip().startswith('[')}")
    print(f"    Contains 'Modified': {'Modified' in stdout}")
    print(f"    Output: {stdout.strip()}")
    print()
    
    # Test YAML output
    print("  Test 2: --output yaml")
    stdout, stderr, code = run_diffx_command([
        "--output", "yaml",
        str(test_dir / "config1.json"),
        str(test_dir / "config2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()

def test_advanced_options():
    """Test advanced options."""
    print("🧪 Testing Advanced Options")
    
    test_dir = create_test_files()
    
    # Test ignore keys regex
    print("  Test 1: --ignore-keys-regex")
    stdout, stderr, code = run_diffx_command([
        "--ignore-keys-regex", "^timestamp$",
        str(test_dir / "file1.json"),
        str(test_dir / "file2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()
    
    # Test epsilon
    print("  Test 2: --epsilon")
    stdout, stderr, code = run_diffx_command([
        "--epsilon", "0.001",
        str(test_dir / "metrics1.json"),
        str(test_dir / "metrics2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()
    
    # Test array ID key
    print("  Test 3: --array-id-key")
    stdout, stderr, code = run_diffx_command([
        "--array-id-key", "id",
        str(test_dir / "users1.json"),
        str(test_dir / "users2.json")
    ])
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print()

def test_stdin_usage():
    """Test stdin usage."""
    print("🧪 Testing Stdin Usage")
    
    test_dir = create_test_files()
    
    # Test stdin input
    print("  Test 1: stdin input")
    stdout, stderr, code = run_diffx_command([
        "-",
        str(test_dir / "config2.json"),
        "--format", "json"
    ], input_data='{"name": "Alice", "age": 30}')
    
    print(f"    Exit code: {code}")
    print(f"    Output: {stdout.strip()}")
    print(f"    Stderr: {stderr.strip()}")
    print()

def main():
    """Main function to run all tests."""
    print("📋 Documentation Verification Report")
    print("=" * 50)
    
    try:
        test_basic_usage()
        test_help_version()
        test_format_options()
        test_output_formats()
        test_advanced_options()
        test_stdin_usage()
        
        print("✅ Verification complete. Check results above.")
        
    except Exception as e:
        print(f"❌ Error during verification: {e}")
        sys.exit(1)
    
    finally:
        # Clean up
        import shutil
        if Path("test_docs_verification").exists():
            shutil.rmtree("test_docs_verification")

if __name__ == "__main__":
    main()