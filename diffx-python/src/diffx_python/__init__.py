"""
diffx - Unified API for semantic diff operations

This package provides a unified interface for comparing structured data
with a single main function: diff()
"""

from .diffx import (
    # Main unified API
    diff,
    
    # Parser functions
    parse_json,
    parse_csv, 
    parse_yaml,
    parse_toml,
    parse_ini,
    parse_xml,
    
    # Utility functions
    format_output,
    diff_files,
    diff_strings,
    
    # Exception
    DiffError,
    
    # Version
    __version__
)

__all__ = [
    # Main unified API
    "diff",
    
    # Parser functions
    "parse_json",
    "parse_csv",
    "parse_yaml", 
    "parse_toml",
    "parse_ini",
    "parse_xml",
    
    # Utility functions
    "format_output",
    "diff_files",
    "diff_strings",
    
    # Exception
    "DiffError",
    
    # Version
    "__version__"
]