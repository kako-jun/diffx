import os
import platform
import subprocess
import sys
from setuptools import setup, Command
from setuptools.command.install import install

class DownloadDiffxCommand(Command):
    description = "Download diffx binary from GitHub Releases"
    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        # Determine OS and architecture
        system = platform.system()
        machine = platform.machine()

        if system == "Linux":
            target = "x86_64-unknown-linux-gnu"
            archive_ext = "tar.gz"
        elif system == "Darwin":
            target = "x86_64-apple-darwin" # Assuming x86_64 for now
            archive_ext = "tar.gz"
        elif system == "Windows":
            target = "x86_64-pc-windows-msvc"
            archive_ext = "zip"
        else:
            raise RuntimeError(f"Unsupported operating system: {system}")

        # Replace with your actual GitHub repo and latest version
        # You might want to fetch the latest version dynamically
        DIFFX_VERSION = "v0.1.0" # Placeholder: Update this to the actual latest release version
        REPO = "kako-jun/diffx" # Your GitHub repository

        download_url = f"https://github.com/{REPO}/releases/download/{DIFFX_VERSION}/diffx-{target}.{archive_ext}"
        
        # Define where to store the binary within the package
        install_dir = os.path.join(self.install_dir, 'diffx_python', 'bin')
        os.makedirs(install_dir, exist_ok=True)
        
        archive_path = os.path.join(install_dir, f"diffx.{archive_ext}")
        
        print(f"Downloading diffx from: {download_url}")
        subprocess.run(["curl", "-L", download_url, "-o", archive_path], check=True)

        if archive_ext == "tar.gz":
            subprocess.run(["tar", "-xzf", archive_path, "-C", install_dir], check=True)
        elif archive_ext == "zip":
            subprocess.run(["unzip", archive_path, "-d", install_dir], check=True)
        
        # Clean up the archive
        os.remove(archive_path)
        
        print(f"diffx binary downloaded and extracted to {install_dir}")

class CustomInstallCommand(install):
    def run(self):
        self.run_command('download_diffx')
        install.run(self)

setup(
    name="diffx-python",
    version="0.1.0",
    author="kako-jun",
    description="A Python wrapper for the diffx CLI tool.",
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    url="https://github.com/kako-jun/diffx",
    packages=["diffx_python"],
    package_data={
        "diffx_python": ["bin/*"], # Include the downloaded binary
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License", # Assuming MIT License
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.6',
    cmdclass={
        'install': CustomInstallCommand,
        'download_diffx': DownloadDiffxCommand,
    },
)
