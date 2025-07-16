"""
Backward compatibility layer for existing diffx_python users
"""

import os
import subprocess
import sys
import platform
from pathlib import Path


def run_diffx(args):
    """
    Run diffx command with given arguments (backward compatibility)
    
    This function maintains compatibility with the original diffx_python API.
    
    Args:
        args: List of command line arguments for diffx
        
    Returns:
        subprocess.CompletedProcess object with stdout, stderr, and returncode
        
    Examples:
        >>> result = run_diffx(["file1.json", "file2.json"])
        >>> print(result.stdout)
    """
    # Determine the path to the diffx binary
    package_dir = Path(__file__).parent.parent.parent
    binary_name = "diffx.exe" if platform.system() == "Windows" else "diffx"
    diffx_binary_path = package_dir / "bin" / binary_name

    # Fall back to system PATH if local binary doesn't exist
    if not diffx_binary_path.exists():
        diffx_binary_path = "diffx"

    command = [str(diffx_binary_path)] + args
    
    try:
        result = subprocess.run(command, capture_output=True, text=True, check=False)
        
        if result.returncode != 0 and result.stderr:
            print(f"Error running diffx: {result.stderr}", file=sys.stderr)
        
        return result
    except FileNotFoundError:
        # Create a mock result object for consistency
        class MockResult:
            def __init__(self):
                self.stdout = ""
                self.stderr = "diffx binary not found. Please ensure the package is installed correctly."
                self.returncode = -1
        
        result = MockResult()
        print(f"Error: {result.stderr}", file=sys.stderr)
        return result