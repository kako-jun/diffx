#!/usr/bin/env python3
"""Binary installer for diffx."""

import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile
from pathlib import Path

DIFFX_VERSION = "0.3.0"


def get_platform_info():
    """Get platform-specific download information."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    if system == "windows":
        return "diffx-windows-x86_64.zip", "diffx.exe"
    elif system == "darwin":
        if machine in ["arm64", "aarch64"]:
            return "diffx-macos-aarch64.tar.gz", "diffx"
        else:
            return "diffx-macos-x86_64.tar.gz", "diffx"
    elif system == "linux":
        return "diffx-linux-x86_64.tar.gz", "diffx"
    else:
        raise RuntimeError(f"Unsupported platform: {system}-{machine}")


def download_file(url: str, dest_path: Path) -> None:
    """Download a file from URL to destination."""
    print(f"Downloading diffx binary from {url}...")
    
    try:
        with urllib.request.urlopen(url) as response:
            with open(dest_path, 'wb') as dest_file:
                shutil.copyfileobj(response, dest_file)
    except Exception as e:
        raise RuntimeError(f"Failed to download {url}: {e}")


def extract_archive(archive_path: Path, extract_dir: Path) -> None:
    """Extract archive file."""
    print("Extracting binary...")
    
    if archive_path.suffix == '.zip':
        with zipfile.ZipFile(archive_path, 'r') as zip_file:
            zip_file.extractall(extract_dir)
    elif archive_path.name.endswith('.tar.gz'):
        with tarfile.open(archive_path, 'r:gz') as tar_file:
            tar_file.extractall(extract_dir)
    else:
        raise RuntimeError(f"Unsupported archive format: {archive_path}")


def main():
    """Main function to download and install diffx binary."""
    try:
        # Get the package directory (where this script will be installed)
        if hasattr(sys, '_MEIPASS'):
            # If running from PyInstaller bundle
            package_dir = Path(sys._MEIPASS).parent
        else:
            # Normal installation
            package_dir = Path(__file__).parent.parent.parent
        
        bin_dir = package_dir / "bin"
        
        # Get platform info
        archive_name, binary_name = get_platform_info()
        binary_path = bin_dir / binary_name
        
        # Skip download if binary already exists
        if binary_path.exists():
            print("diffx binary already exists, skipping download.")
            return 0
        
        # Create bin directory
        bin_dir.mkdir(exist_ok=True)
        
        # Download URL
        download_url = f"https://github.com/kako-jun/diffx/releases/download/v{DIFFX_VERSION}/{archive_name}"
        
        # Download to temporary file
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            archive_path = temp_path / archive_name
            
            download_file(download_url, archive_path)
            extract_archive(archive_path, bin_dir)
        
        # Make binary executable on Unix systems
        if platform.system() != "Windows":
            binary_path.chmod(0o755)
        
        print(f"SUCCESS: diffx binary installed successfully at {binary_path}")
        return 0
        
    except Exception as error:
        print(f"ERROR: Failed to download diffx binary: {error}")
        print("You may need to install diffx manually from: https://github.com/kako-jun/diffx/releases")
        # Don't fail the installation, just warn
        return 0


if __name__ == "__main__":
    sys.exit(main())