"""
diffx: Python wrapper for the diffx CLI tool

This package provides a Python interface to the diffx CLI tool for semantic
diffing of structured data formats like JSON, YAML, TOML, XML, INI, and CSV.

The diffx binary is embedded in the wheel for offline installation.
"""

from .diffx import (
    diff,
    diff_string,
    is_diffx_available,
    DiffOptions,
    DiffResult,
    DiffError,
    Format,
    OutputFormat,
)

# For backward compatibility with existing diffx_python users
from .compat import run_diffx

__version__ = "0.5.1"
__all__ = [
    "diff",
    "diff_string", 
    "is_diffx_available",
    "DiffOptions",
    "DiffResult", 
    "DiffError",
    "Format",
    "OutputFormat",
    "run_diffx",  # Backward compatibility
]