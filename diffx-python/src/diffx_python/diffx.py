"""
Unified diffx Python API - single function interface
"""

from typing import List, Dict, Any, Optional, Union
try:
    from .diffx_python import (
        diff_py, parse_json_py, parse_csv_py, parse_yaml_py, 
        parse_toml_py, parse_ini_py, parse_xml_py, format_output_py
    )
except ImportError:
    # Fallback for development/testing
    try:
        from diffx_python import (
            diff_py, parse_json_py, parse_csv_py, parse_yaml_py,
            parse_toml_py, parse_ini_py, parse_xml_py, format_output_py
        )
    except ImportError as e:
        raise ImportError(
            f"Could not import diffx-python native module: {e}. "
            "Make sure the package is properly installed."
        ) from e


class DiffError(Exception):
    """Error thrown when diff operation fails"""
    pass


# ============================================================================
# UNIFIED API - Main Function
# ============================================================================


def diff(old: Union[Dict, List, str, int, float, bool, None], 
         new: Union[Dict, List, str, int, float, bool, None], 
         **kwargs) -> List[Dict[str, Any]]:
    """
    Unified diff function for comparing two JSON-like structures.
    
    This is the main entry point for diffx functionality in Python.
    All configuration is done through keyword arguments.
    
    Args:
        old: The old data structure (dict, list, or primitive)
        new: The new data structure (dict, list, or primitive)
        **kwargs: Optional parameters for comparison:
            epsilon (float): Numerical comparison tolerance
            array_id_key (str): Key to use for array element identification
            ignore_keys_regex (str): Regex pattern for keys to ignore
            path_filter (str): Only show differences in paths containing this string
            output_format (str): Output format ("diffx", "json", "yaml", "unified")
            show_unchanged (bool): Show unchanged values as well
            show_types (bool): Show type information in output
            use_memory_optimization (bool): Enable memory optimization for large files
            batch_size (int): Batch size for memory optimization
            context_lines (int): Number of context lines for diff output
            ignore_whitespace (bool): Ignore whitespace differences
            ignore_case (bool): Ignore case differences
            brief_mode (bool): Report only whether files differ
            quiet_mode (bool): Suppress normal output; return only exit status
    
    Returns:
        List[Dict]: List of differences found, each containing:
            - "type": "Added", "Removed", "Modified", or "TypeChanged"
            - "path": Path where the difference occurred
            - "value" (for Added/Removed): The value that was added/removed
            - "old_value", "new_value" (for Modified/TypeChanged): The old and new values
    
    Examples:
        >>> old = {"name": "Alice", "age": 30}
        >>> new = {"name": "Alice", "age": 31}
        >>> result = diff(old, new)
        >>> print(result)
        [{"type": "Modified", "path": "age", "old_value": 30, "new_value": 31}]
        
        >>> # With options
        >>> result = diff(old, new, epsilon=0.1, show_types=True)
        
        >>> # Array comparison with ID
        >>> old_list = [{"id": "1", "name": "Alice"}, {"id": "2", "name": "Bob"}]
        >>> new_list = [{"id": "1", "name": "Alice"}, {"id": "2", "name": "Bobby"}]
        >>> result = diff(old_list, new_list, array_id_key="id")
    """
    try:
        return diff_py(old, new, **kwargs)
    except Exception as e:
        raise DiffError(f"Diff operation failed: {e}") from e


# ============================================================================
# UNIFIED API - Parser Functions
# ============================================================================

def parse_json(content: str) -> Union[Dict, List, str, int, float, bool, None]:
    """
    Parse JSON string to Python object.
    
    Args:
        content: JSON string content
        
    Returns:
        Parsed Python object (dict, list, or primitive)
        
    Examples:
        >>> data = parse_json('{"name": "Alice", "age": 30}')
        >>> print(data)
        {'name': 'Alice', 'age': 30}
    """
    try:
        return parse_json_py(content)
    except Exception as e:
        raise DiffError(f"JSON parse failed: {e}") from e


def parse_csv(content: str) -> List[Dict[str, str]]:
    """
    Parse CSV string to Python list of dicts.
    
    Args:
        content: CSV string content
        
    Returns:
        List of dictionaries representing CSV rows
        
    Examples:
        >>> csv_content = "name,age\\nAlice,30\\nBob,25"
        >>> data = parse_csv(csv_content)
        >>> print(data)
        [{'name': 'Alice', 'age': '30'}, {'name': 'Bob', 'age': '25'}]
    """
    try:
        return parse_csv_py(content)
    except Exception as e:
        raise DiffError(f"CSV parse failed: {e}") from e


def parse_yaml(content: str) -> Union[Dict, List, str, int, float, bool, None]:
    """
    Parse YAML string to Python object.
    
    Args:
        content: YAML string content
        
    Returns:
        Parsed Python object (dict, list, or primitive)
        
    Examples:
        >>> yaml_content = "name: Alice\\nage: 30"
        >>> data = parse_yaml(yaml_content)
        >>> print(data)
        {'name': 'Alice', 'age': 30}
    """
    try:
        return parse_yaml_py(content)
    except Exception as e:
        raise DiffError(f"YAML parse failed: {e}") from e


def parse_toml(content: str) -> Dict[str, Any]:
    """
    Parse TOML string to Python dict.
    
    Args:
        content: TOML string content
        
    Returns:
        Parsed Python dictionary
        
    Examples:
        >>> toml_content = "name = 'Alice'\\nage = 30"
        >>> data = parse_toml(toml_content)
        >>> print(data)
        {'name': 'Alice', 'age': 30}
    """
    try:
        return parse_toml_py(content)
    except Exception as e:
        raise DiffError(f"TOML parse failed: {e}") from e


def parse_ini(content: str) -> Dict[str, Any]:
    """
    Parse INI string to Python dict.
    
    Args:
        content: INI string content
        
    Returns:
        Parsed Python dictionary
        
    Examples:
        >>> ini_content = "[section]\\nname = Alice\\nage = 30"
        >>> data = parse_ini(ini_content)
        >>> print(data)
        {'section': {'name': 'Alice', 'age': '30'}}
    """
    try:
        return parse_ini_py(content)
    except Exception as e:
        raise DiffError(f"INI parse failed: {e}") from e


def parse_xml(content: str) -> Dict[str, Any]:
    """
    Parse XML string to Python dict.
    
    Args:
        content: XML string content
        
    Returns:
        Parsed Python dictionary
        
    Examples:
        >>> xml_content = "<root><name>Alice</name><age>30</age></root>"
        >>> data = parse_xml(xml_content)
        >>> print(data)
        {'root': {'name': 'Alice', 'age': '30'}}
    """
    try:
        return parse_xml_py(content)
    except Exception as e:
        raise DiffError(f"XML parse failed: {e}") from e


# ============================================================================
# UNIFIED API - Utility Functions
# ============================================================================

def format_output(results: List[Dict[str, Any]], format: str = "diffx") -> str:
    """
    Format diff results to string output.
    
    Args:
        results: List of diff results from diff() function
        format: Output format ("diffx", "json", "yaml", "unified")
        
    Returns:
        Formatted string output
        
    Examples:
        >>> old = {"name": "Alice", "age": 30}
        >>> new = {"name": "Alice", "age": 31}
        >>> results = diff(old, new)
        >>> formatted = format_output(results, "json")
        >>> print(formatted)
    """
    try:
        return format_output_py(results, format)
    except Exception as e:
        raise DiffError(f"Format output failed: {e}") from e


def diff_files(file1_path: str, file2_path: str, **kwargs) -> List[Dict[str, Any]]:
    """
    Compare two files directly.
    
    Args:
        file1_path: Path to first file
        file2_path: Path to second file
        **kwargs: Same options as diff() function
        
    Returns:
        List of differences found
        
    Examples:
        >>> result = diff_files("config1.json", "config2.json")
        >>> print(result)
        
        >>> result = diff_files("data1.yaml", "data2.yaml", epsilon=0.1)
    """
    import json
    from pathlib import Path
    
    # Read files
    try:
        path1 = Path(file1_path)
        path2 = Path(file2_path)
        
        content1 = path1.read_text(encoding='utf-8')
        content2 = path2.read_text(encoding='utf-8')
        
        # Auto-detect format from file extension
        ext1 = path1.suffix.lower()
        ext2 = path2.suffix.lower()
        
        # Parse based on file extension
        if ext1 in ['.json']:
            data1 = parse_json(content1)
        elif ext1 in ['.yaml', '.yml']:
            data1 = parse_yaml(content1)
        elif ext1 in ['.toml']:
            data1 = parse_toml(content1)
        elif ext1 in ['.ini', '.cfg']:
            data1 = parse_ini(content1)
        elif ext1 in ['.xml']:
            data1 = parse_xml(content1)
        elif ext1 in ['.csv']:
            data1 = parse_csv(content1)
        else:
            # Try JSON as fallback
            try:
                data1 = parse_json(content1)
            except:
                raise DiffError(f"Unsupported file format: {ext1}")
        
        if ext2 in ['.json']:
            data2 = parse_json(content2)
        elif ext2 in ['.yaml', '.yml']:
            data2 = parse_yaml(content2)
        elif ext2 in ['.toml']:
            data2 = parse_toml(content2)
        elif ext2 in ['.ini', '.cfg']:
            data2 = parse_ini(content2)
        elif ext2 in ['.xml']:
            data2 = parse_xml(content2)
        elif ext2 in ['.csv']:
            data2 = parse_csv(content2)
        else:
            # Try JSON as fallback
            try:
                data2 = parse_json(content2)
            except:
                raise DiffError(f"Unsupported file format: {ext2}")
        
        return diff(data1, data2, **kwargs)
        
    except FileNotFoundError as e:
        raise DiffError(f"File not found: {e}") from e
    except Exception as e:
        raise DiffError(f"Failed to compare files: {e}") from e


def diff_strings(content1: str, content2: str, format: str, **kwargs) -> List[Dict[str, Any]]:
    """
    Compare two string contents directly.
    
    Args:
        content1: First content string
        content2: Second content string  
        format: Content format ("json", "yaml", "toml", "ini", "xml", "csv")
        **kwargs: Same options as diff() function
        
    Returns:
        List of differences found
        
    Examples:
        >>> json1 = '{"name": "Alice", "age": 30}'
        >>> json2 = '{"name": "Alice", "age": 31}'
        >>> result = diff_strings(json1, json2, "json")
        >>> print(result)
    """
    try:
        # Parse based on format
        if format == "json":
            data1 = parse_json(content1)
            data2 = parse_json(content2)
        elif format in ["yaml", "yml"]:
            data1 = parse_yaml(content1)
            data2 = parse_yaml(content2)
        elif format == "toml":
            data1 = parse_toml(content1)
            data2 = parse_toml(content2)
        elif format in ["ini", "cfg"]:
            data1 = parse_ini(content1)
            data2 = parse_ini(content2)  
        elif format == "xml":
            data1 = parse_xml(content1)
            data2 = parse_xml(content2)
        elif format == "csv":
            data1 = parse_csv(content1)
            data2 = parse_csv(content2)
        else:
            raise DiffError(f"Unsupported format: {format}")
            
        return diff(data1, data2, **kwargs)
        
    except Exception as e:
        raise DiffError(f"Failed to compare strings: {e}") from e


# ============================================================================
# VERSION INFO
# ============================================================================

__version__ = "0.5.7"
