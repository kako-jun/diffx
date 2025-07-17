#!/usr/bin/env python3
"""
Unified test runner for diffx-python package
Runs all test suites in organized fashion
"""
import os
import sys
import time
import importlib.util
from pathlib import Path

# ANSI color codes
class Colors:
    GREEN = '\033[32m'
    RED = '\033[31m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    CYAN = '\033[36m'
    MAGENTA = '\033[35m'
    BOLD = '\033[1m'
    RESET = '\033[0m'

def log(message, color=Colors.RESET):
    print(f"{color}{message}{Colors.RESET}")

def success(message):
    log(f"✓ {message}", Colors.GREEN)

def error(message):
    log(f"✗ {message}", Colors.RED)

def info(message):
    log(f"ℹ {message}", Colors.BLUE)

def header(message):
    log(f"\n{Colors.BOLD}{Colors.CYAN}=== {message} ==={Colors.RESET}")

def separator():
    log(f"{Colors.YELLOW}{'─' * 60}{Colors.RESET}")

def check_prerequisites():
    """Check if prerequisites are available"""
    header('Prerequisites Check')
    
    # Check if package can be imported
    try:
        import diffx
        success('diffx package available')
    except ImportError:
        error('diffx package not available')
        return False
    
    # Check if test modules exist
    test_dir = Path(__file__).parent
    required_files = [
        'test_cli.py', 'test_basic.py', 'test_binary.py',
        'test_formats.py', 'test_errors.py', 'test_features.py'
    ]
    
    for file in required_files:
        if not (test_dir / file).exists():
            error(f'{file} not found')
            return False
    
    success('All required test files found')
    return True

def import_test_module(module_name, file_path):
    """Dynamically import a test module"""
    try:
        spec = importlib.util.spec_from_file_location(module_name, file_path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module
    except Exception as e:
        error(f"Failed to import {module_name}: {e}")
        return None

def run_test_suite(name, test_function, description):
    """Run a test suite and report results"""
    header(f'{name} Tests')
    info(description)
    separator()
    
    start_time = time.time()
    success = False
    
    try:
        success = test_function()
        duration = time.time() - start_time
        
        if success:
            log(f"\n{Colors.GREEN}{Colors.BOLD}✓ {name} tests PASSED{Colors.RESET} ({duration:.3f}s)", Colors.GREEN)
        else:
            log(f"\n{Colors.RED}{Colors.BOLD}✗ {name} tests FAILED{Colors.RESET} ({duration:.3f}s)", Colors.RED)
    except Exception as err:
        duration = time.time() - start_time
        error(f"{name} tests CRASHED: {err} ({duration:.3f}s)")
        success = False
    
    separator()
    return success

def run_all_tests():
    """Run all test suites"""
    start_time = time.time()
    
    log(f"{Colors.BOLD}{Colors.MAGENTA}diffx-python Test Suite{Colors.RESET}")
    log("Running comprehensive tests for diffx Python package\n")
    
    # Check prerequisites first
    if not check_prerequisites():
        error('Prerequisites check failed. Cannot proceed with tests.')
        sys.exit(1)
    
    # Test suite definitions
    test_dir = Path(__file__).parent
    test_suites = [
        {
            'name': 'Binary',
            'file': 'test_binary.py',
            'function': 'run_binary_tests',
            'description': 'Verifies that the underlying Rust binary is properly bundled and functional'
        },
        {
            'name': 'CLI',
            'file': 'test_cli.py',
            'function': 'run_cli_tests',
            'description': 'Tests command-line interface functionality and basic commands'
        },
        {
            'name': 'Basic',
            'file': 'test_basic.py',
            'function': 'run_basic_tests',
            'description': 'Tests core diff operations and basic functionality'
        },
        {
            'name': 'Formats',
            'file': 'test_formats.py',
            'function': 'run_format_tests',
            'description': 'Tests support for various file formats (JSON, YAML, TOML, etc.)'
        },
        {
            'name': 'Features',
            'file': 'test_features.py',
            'function': 'run_feature_tests',
            'description': 'Tests advanced features and options'
        },
        {
            'name': 'Errors',
            'file': 'test_errors.py',
            'function': 'run_error_tests',
            'description': 'Tests error handling and edge cases'
        }
    ]
    
    results = []
    
    for suite in test_suites:
        # Import the test module
        module_path = test_dir / suite['file']
        module = import_test_module(suite['name'].lower(), module_path)
        
        if module is None:
            results.append({'name': suite['name'], 'success': False})
            continue
        
        # Get the test function
        test_function = getattr(module, suite['function'], None)
        if test_function is None:
            error(f"Test function {suite['function']} not found in {suite['file']}")
            results.append({'name': suite['name'], 'success': False})
            continue
        
        # Run the test suite
        success = run_test_suite(suite['name'], test_function, suite['description'])
        results.append({'name': suite['name'], 'success': success})
    
    # Summary
    total_time = time.time() - start_time
    passed = sum(1 for r in results if r['success'])
    failed = len(results) - passed
    
    header('Test Results Summary')
    
    for result in results:
        if result['success']:
            success(f"{result['name']} tests")
        else:
            error(f"{result['name']} tests")
    
    separator()
    
    if failed == 0:
        log(f"\n{Colors.BOLD}{Colors.GREEN}🎉 ALL TESTS PASSED!{Colors.RESET}", Colors.GREEN)
        log(f"{Colors.GREEN}✓ {passed}/{len(results)} test suites passed{Colors.RESET}")
        log(f"{Colors.BLUE}Total time: {total_time:.3f}s{Colors.RESET}\n")
        return True
    else:
        log(f"\n{Colors.BOLD}{Colors.RED}❌ SOME TESTS FAILED{Colors.RESET}", Colors.RED)
        log(f"{Colors.RED}✗ {failed}/{len(results)} test suites failed{Colors.RESET}")
        log(f"{Colors.GREEN}✓ {passed}/{len(results)} test suites passed{Colors.RESET}")
        log(f"{Colors.BLUE}Total time: {total_time:.3f}s{Colors.RESET}\n")
        return False

def main():
    """Main entry point"""
    try:
        success = run_all_tests()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        log(f"\n{Colors.YELLOW}Tests interrupted by user{Colors.RESET}")
        sys.exit(1)
    except Exception as e:
        error(f"Test runner crashed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()