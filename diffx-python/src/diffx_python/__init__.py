"""
diffx - Blazing fast semantic diff for JSON/YAML/TOML/XML/INI/CSV

This package provides a Python wrapper around the diffx Rust binary,
optimized for semantic file comparison across multiple formats.
"""

import json
import subprocess
import sys
import shutil
from pathlib import Path
from typing import Any, Dict, List, Optional, Union
from dataclasses import dataclass
from enum import Enum

# Version management
try:
    from importlib.metadata import version
    __version__ = version("diffx-python")
except ImportError:
    # Fallback for Python < 3.8
    try:
        import pkg_resources
        __version__ = pkg_resources.get_distribution("diffx-python").version
    except Exception:
        __version__ = "unknown"

class Format(Enum):
    """Supported input formats for diffx_python."""
    JSON = "json"
    YAML = "yaml"
    TOML = "toml"
    XML = "xml"
    INI = "ini"
    CSV = "csv"

class OutputFormat(Enum):
    """Supported output formats for diffx results."""
    CLI = "cli"
    JSON = "json"
    YAML = "yaml"
    UNIFIED = "unified"

@dataclass
class DiffOptions:
    """Configuration options for diffx comparison."""
    
    # Basic options
    format: Optional[Format] = None
    output: Optional[OutputFormat] = None
    path: Optional[str] = None
    ignore_keys_regex: Optional[str] = None
    epsilon: Optional[float] = None
    array_id_key: Optional[str] = None
    context: Optional[int] = None
    ignore_whitespace: bool = False
    ignore_case: bool = False
    quiet: bool = False
    brief: bool = False
    verbose: bool = False
    
    def to_args(self) -> List[str]:
        """Convert options to command line arguments."""
        args = []
        
        if self.format:
            args.extend(["--format", self.format.value])
        if self.output:
            args.extend(["--output", self.output.value])
        if self.path:
            args.extend(["--path", self.path])
        if self.ignore_keys_regex:
            args.extend(["--ignore-keys-regex", self.ignore_keys_regex])
        if self.epsilon is not None:
            args.extend(["--epsilon", str(self.epsilon)])
        if self.array_id_key:
            args.extend(["--array-id-key", self.array_id_key])
        if self.context is not None:
            args.extend(["--context", str(self.context)])
        if self.ignore_whitespace:
            args.append("--ignore-whitespace")
        if self.ignore_case:
            args.append("--ignore-case")
        if self.quiet:
            args.append("--quiet")
        if self.brief:
            args.append("--brief")
        if self.verbose:
            args.append("--verbose")
            
        return args

class DiffxError(Exception):
    """Base exception for diffx-related errors."""
    pass

class DiffResult:
    """Result from diffx comparison."""
    
    def __init__(self, raw_output: str, format_type: str = "cli", return_code: int = 0):
        self.raw_output = raw_output
        self.format_type = format_type
        self.return_code = return_code
        self._parsed_data = None
        
    @property
    def data(self) -> Any:
        """Get parsed data (JSON objects for JSON output, raw string otherwise)."""
        if self._parsed_data is None:
            if self.format_type == "json" and self.raw_output.strip():
                try:
                    self._parsed_data = json.loads(self.raw_output)
                except json.JSONDecodeError:
                    self._parsed_data = self.raw_output
            else:
                self._parsed_data = self.raw_output
        return self._parsed_data
    
    @property
    def is_json(self) -> bool:
        """True if result is in JSON format."""
        return self.format_type == "json" and isinstance(self.data, (dict, list))
    
    @property
    def has_differences(self) -> bool:
        """True if differences were found (exit code 1)."""
        return self.return_code == 1
    
    @property
    def is_error(self) -> bool:
        """True if error occurred (exit code >= 2)."""
        return self.return_code >= 2
    
    def __str__(self) -> str:
        return self.raw_output

def _find_diffx_binary() -> str:
    """Find the diffx binary bundled with package."""
    import shutil
    
    # First try to find in PATH (most common case for installed packages)
    binary_path = shutil.which("diffx")
    if binary_path:
        return binary_path
    
    # Check if bundled with package
    package_dir = Path(__file__).parent.parent.parent
    bundled_binary = package_dir / "diffx"
    
    if bundled_binary.exists() and bundled_binary.is_file():
        return str(bundled_binary)
    
    # Error if binary not found
    raise DiffxError(
        f"diffx binary not found in PATH or at {bundled_binary}. This might indicate a packaging issue. "
        "Please report this at: https://github.com/kako-jun/diffx/issues"
    )

def diff(
    input1: str,
    input2: str,
    options: Optional[Union[DiffOptions, Dict[str, Any]]] = None,
    **kwargs
) -> DiffResult:
    """
    Compare two files using diffx_python.
    
    Args:
        input1: Path to first input file
        input2: Path to second input file  
        options: DiffOptions object or dict of options
        **kwargs: Additional options as keyword arguments
        
    Returns:
        DiffResult object containing comparison results
    """
    # Handle different option formats
    if options is None:
        options = DiffOptions(**kwargs)
    elif isinstance(options, dict):
        combined_options = {**options, **kwargs}
        options = DiffOptions(**combined_options)
    elif kwargs:
        # Merge kwargs into existing DiffOptions
        option_dict = {
            field.name: getattr(options, field.name) 
            for field in options.__dataclass_fields__.values()
        }
        combined_options = {**option_dict, **kwargs}
        options = DiffOptions(**combined_options)
    
    try:
        binary_path = _find_diffx_binary()
        cmd = [binary_path] + options.to_args() + [input1, input2]
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=False
        )
        
        # diffx exit codes: 0=no differences, 1=differences found, 2+=error
        if result.returncode >= 2 and result.stderr:
            raise DiffxError(f"diffx failed: {result.stderr}")
        
        format_type = options.output.value if options.output else "cli"
        return DiffResult(result.stdout, format_type, result.returncode)
        
    except FileNotFoundError:
        raise DiffxError("diffx binary not found")
    except Exception as e:
        raise DiffxError(f"Diff failed: {e}")

def diff_string(
    content1: str,
    content2: str,
    format: Format,
    options: Optional[Union[DiffOptions, Dict[str, Any]]] = None,
    **kwargs
) -> DiffResult:
    """
    Compare two strings using diffx via temporary files.
    
    Args:
        content1: First string content
        content2: Second string content
        format: File format to use for parsing
        options: DiffOptions object or dict of options
        **kwargs: Additional options as keyword arguments
        
    Returns:
        DiffResult object containing comparison results
    """
    import tempfile
    import os
    
    # Create temporary files
    with tempfile.NamedTemporaryFile(mode='w', suffix=f'.{format.value}', delete=False) as f1:
        f1.write(content1)
        temp1 = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix=f'.{format.value}', delete=False) as f2:
        f2.write(content2)
        temp2 = f2.name
    
    try:
        # Set format in options if not already set
        if options is None:
            options = DiffOptions(format=format, **kwargs)
        elif isinstance(options, dict):
            combined_options = {"format": format, **options, **kwargs}
            options = DiffOptions(**combined_options)
        else:
            if options.format is None:
                options.format = format
        
        return diff(temp1, temp2, options)
    finally:
        # Clean up temporary files
        try:
            os.unlink(temp1)
            os.unlink(temp2)
        except OSError:
            pass

def main():
    """CLI entry point for the diffx command."""
    try:
        binary_path = _find_diffx_binary()
        # Forward all arguments to the binary
        result = subprocess.run([binary_path] + sys.argv[1:])
        sys.exit(result.returncode)
    except DiffxError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

# Export main API
__all__ = [
    "diff",
    "diff_string",
    "DiffOptions",
    "DiffResult", 
    "Format",
    "OutputFormat",
    "DiffxError",
    "__version__",
    "main",
]