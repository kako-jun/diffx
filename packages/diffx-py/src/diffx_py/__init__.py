"""
diffx-py: Python wrapper for diffx - semantic diff for structured data

This package provides a Python interface to the powerful diffx command-line tool,
enabling semantic comparison of structured data formats including JSON, YAML, 
TOML, XML, INI, and CSV.
"""

from .diffx import (
    diff,
    diff_string,
    is_diffx_available,
    DiffError,
    DiffOptions,
    DiffResult,
    Format,
    OutputFormat,
)

__version__ = "0.2.0"
__author__ = "kako-jun"
__email__ = "kako.jun.42@gmail.com"

__all__ = [
    "diff",
    "diff_string", 
    "is_diffx_available",
    "DiffError",
    "DiffOptions",
    "DiffResult",
    "Format",
    "OutputFormat",
]