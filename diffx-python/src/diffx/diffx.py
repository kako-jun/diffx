"""
Main diffx wrapper implementation
"""

import json
import subprocess
import tempfile
import os
import platform
from pathlib import Path
from typing import Union, List, Dict, Any, Optional, Literal
from dataclasses import dataclass


# Type definitions
Format = Literal["json", "yaml", "toml", "xml", "ini", "csv"]
OutputFormat = Literal["cli", "json", "yaml", "unified"]


@dataclass
class DiffOptions:
    """Options for the diff operation"""
    format: Optional[Format] = None
    output: Optional[OutputFormat] = None
    recursive: bool = False
    path: Optional[str] = None
    ignore_keys_regex: Optional[str] = None
    epsilon: Optional[float] = None
    array_id_key: Optional[str] = None
    optimize: bool = False
    context: Optional[int] = None
    ignore_whitespace: bool = False
    ignore_case: bool = False
    quiet: bool = False
    brief: bool = False


class DiffResult:
    """Result of a diff operation when output format is 'json'"""
    def __init__(self, data: Dict[str, Any]):
        self.data = data
    
    @property
    def added(self) -> Optional[tuple]:
        """Get Added result if present"""
        return tuple(self.data["Added"]) if "Added" in self.data else None
    
    @property
    def removed(self) -> Optional[tuple]:
        """Get Removed result if present"""
        return tuple(self.data["Removed"]) if "Removed" in self.data else None
    
    @property
    def modified(self) -> Optional[tuple]:
        """Get Modified result if present"""
        return tuple(self.data["Modified"]) if "Modified" in self.data else None
    
    @property
    def type_changed(self) -> Optional[tuple]:
        """Get TypeChanged result if present"""
        return tuple(self.data["TypeChanged"]) if "TypeChanged" in self.data else None
    
    def __repr__(self) -> str:
        return f"DiffResult({self.data})"


class DiffError(Exception):
    """Error thrown when diffx command fails"""
    def __init__(self, message: str, exit_code: int, stderr: str):
        super().__init__(message)
        self.exit_code = exit_code
        self.stderr = stderr


def _get_diffx_binary_path() -> str:
    """Get the path to the diffx binary"""
    # Check if local binary exists (installed via postinstall)
    package_dir = Path(__file__).parent.parent.parent
    binary_name = "diffx.exe" if platform.system() == "Windows" else "diffx"
    local_binary_path = package_dir / "bin" / binary_name
    
    if local_binary_path.exists():
        return str(local_binary_path)
    
    # Fall back to system PATH
    return "diffx"


def _execute_diffx(args: List[str]) -> tuple[str, str]:
    """Execute diffx command and return stdout, stderr"""
    diffx_path = _get_diffx_binary_path()
    
    try:
        result = subprocess.run(
            [diffx_path] + args,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout, result.stderr
    except subprocess.CalledProcessError as e:
        raise DiffError(
            f"diffx exited with code {e.returncode}",
            e.returncode,
            e.stderr or ""
        )
    except FileNotFoundError:
        raise DiffError(
            "diffx command not found. Please install diffx CLI tool.",
            -1,
            ""
        )


def diff(
    input1: str,
    input2: str,
    options: Optional[DiffOptions] = None
) -> Union[str, List[DiffResult]]:
    """
    Compare two files or directories using diffx
    
    Args:
        input1: Path to first file/directory or '-' for stdin
        input2: Path to second file/directory
        options: Comparison options
        
    Returns:
        String output for CLI format, or list of DiffResult for JSON format
        
    Examples:
        >>> result = diff('file1.json', 'file2.json')
        >>> print(result)
        
        >>> json_result = diff('config1.yaml', 'config2.yaml', 
        ...                   DiffOptions(format='yaml', output='json'))
        >>> for diff_item in json_result:
        ...     print(diff_item)
        
        >>> dir_result = diff('dir1/', 'dir2/', 
        ...                  DiffOptions(recursive=True, path='config'))
    """
    if options is None:
        options = DiffOptions()
    
    args = [input1, input2]
    
    # Add format option
    if options.format:
        args.extend(["--format", options.format])
    
    # Add output format option
    if options.output:
        args.extend(["--output", options.output])
    
    # Add recursive option
    if options.recursive:
        args.append("--recursive")
    
    # Add path filter option
    if options.path:
        args.extend(["--path", options.path])
    
    # Add ignore keys regex option
    if options.ignore_keys_regex:
        args.extend(["--ignore-keys-regex", options.ignore_keys_regex])
    
    # Add epsilon option
    if options.epsilon is not None:
        args.extend(["--epsilon", str(options.epsilon)])
    
    # Add array ID key option
    if options.array_id_key:
        args.extend(["--array-id-key", options.array_id_key])
    
    # Add optimize option
    if options.optimize:
        args.append("--optimize")
    
    # Add context option
    if options.context is not None:
        args.extend(["--context", str(options.context)])
    
    # Add ignore whitespace option
    if options.ignore_whitespace:
        args.append("--ignore-whitespace")
    
    # Add ignore case option
    if options.ignore_case:
        args.append("--ignore-case")
    
    # Add quiet option
    if options.quiet:
        args.append("--quiet")
    
    # Add brief option
    if options.brief:
        args.append("--brief")
    
    stdout, stderr = _execute_diffx(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return [DiffResult(item) for item in json_data]
        except json.JSONDecodeError as e:
            raise DiffError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def diff_string(
    content1: str,
    content2: str,
    format: Format,
    options: Optional[DiffOptions] = None
) -> Union[str, List[DiffResult]]:
    """
    Compare two strings directly (writes to temporary files)
    
    Args:
        content1: First content string
        content2: Second content string
        format: Content format
        options: Comparison options
        
    Returns:
        String output for CLI format, or list of DiffResult for JSON format
        
    Examples:
        >>> json1 = '{"name": "Alice", "age": 30}'
        >>> json2 = '{"name": "Alice", "age": 31}'
        >>> result = diff_string(json1, json2, 'json', 
        ...                     DiffOptions(output='json'))
        >>> print(result)
    """
    if options is None:
        options = DiffOptions()
    
    # Ensure format is set
    options.format = format
    
    # Create temporary files
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_file1 = Path(tmp_dir) / f"file1.{format}"
        tmp_file2 = Path(tmp_dir) / f"file2.{format}"
        
        # Write content to temporary files
        tmp_file1.write_text(content1, encoding="utf-8")
        tmp_file2.write_text(content2, encoding="utf-8")
        
        # Perform diff
        return diff(str(tmp_file1), str(tmp_file2), options)


def is_diffx_available() -> bool:
    """
    Check if diffx command is available in the system
    
    Returns:
        True if diffx is available, False otherwise
        
    Examples:
        >>> if not is_diffx_available():
        ...     print("Please install diffx CLI tool")
        ...     exit(1)
    """
    try:
        _execute_diffx(["--version"])
        return True
    except DiffError:
        return False