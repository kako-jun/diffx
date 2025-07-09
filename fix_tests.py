#!/usr/bin/env python3
"""
Fix test files by:
1. Changing .success() to .code(1) for tests that expect differences
2. Adding --format json for temporary files without extensions
3. Keeping .success() for tests that expect no differences
"""

import os
import re
import glob

def fix_test_file(file_path):
    """Fix a single test file"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern 1: Fix .success() to .code(1) for tests that expect differences
    # Look for patterns where stdout contains diff-like output
    patterns_expecting_differences = [
        r'\.stdout\(predicate::str::contains\("~ ',
        r'\.stdout\(predicate::str::contains\("  \+ ',
        r'\.stdout\(predicate::str::contains\("  - ',
        r'\.stdout\(predicate::str::contains\(r#""Modified""#',
        r'\.stdout\(predicate::str::contains\(r#""Added""#',
        r'\.stdout\(predicate::str::contains\(r#""Removed""#',
        r'\.stdout\(predicate::str::contains\("Modified"',
        r'\.stdout\(predicate::str::contains\("Added"',
        r'\.stdout\(predicate::str::contains\("Removed"',
    ]
    
    # Find tests that expect differences
    tests_expecting_differences = set()
    for pattern in patterns_expecting_differences:
        matches = re.finditer(pattern, content, re.MULTILINE)
        for match in matches:
            # Find the test function containing this match
            before_match = content[:match.start()]
            test_start = before_match.rfind('fn test_')
            if test_start != -1:
                test_name_match = re.search(r'fn (test_\w+)', content[test_start:])
                if test_name_match:
                    tests_expecting_differences.add(test_name_match.group(1))
    
    # Replace .success() with .code(1) for tests expecting differences
    def replace_success_with_code1(match):
        full_match = match.group(0)
        function_name = match.group(1)
        if function_name in tests_expecting_differences:
            return full_match.replace('.success()', '.code(1)')
        return full_match
    
    # Find and replace .success() calls in tests expecting differences
    test_pattern = r'(fn test_\w+\([^{]*\{[^}]*?cmd\.assert\(\)\s*\.success\(\))'
    content = re.sub(test_pattern, replace_success_with_code1, content, flags=re.DOTALL)
    
    # Pattern 2: Add --format json for temporary files
    # Look for patterns where NamedTempFile is used
    if 'NamedTempFile' in content:
        # Add --format json before other arguments if not already present
        def add_format_json(match):
            full_match = match.group(0)
            if '--format' not in full_match:
                # Insert --format json after the file paths
                lines = full_match.split('\n')
                new_lines = []
                for i, line in enumerate(lines):
                    new_lines.append(line)
                    if '.arg(file2.path())' in line and '--format' not in '\n'.join(lines[i:]):
                        new_lines.append('        .arg("--format")')
                        new_lines.append('        .arg("json")')
                return '\n'.join(new_lines)
            return full_match
        
        # Find Command::new patterns with temp files
        temp_file_pattern = r'(let output = Command::new\("cargo"\)[^;]*?\.arg\(file2\.path\(\)\)[^;]*?\.output\(\))'
        content = re.sub(temp_file_pattern, add_format_json, content, flags=re.DOTALL)
    
    # Save the file if it was modified
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed {file_path}")
        return True
    
    return False

def main():
    """Fix all test files"""
    test_files = glob.glob('tests/integration/*.rs')
    
    fixed_count = 0
    for file_path in test_files:
        if fix_test_file(file_path):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == '__main__':
    main()