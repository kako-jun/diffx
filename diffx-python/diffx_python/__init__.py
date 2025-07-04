import os
import subprocess
import sys

def run_diffx(args):
    # Determine the path to the diffx binary
    script_dir = os.path.dirname(__file__)
    diffx_binary_path = os.path.join(script_dir, "bin", "diffx")

    # For Windows, add .exe extension
    if sys.platform == "win32":
        diffx_binary_path += ".exe"

    if not os.path.exists(diffx_binary_path):
        raise FileNotFoundError(f"diffx binary not found at {diffx_binary_path}. Please ensure the package is installed correctly.")

    command = [diffx_binary_path] + args
    
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    
    if result.returncode != 0:
        print(f"Error running diffx: {result.stderr}", file=sys.stderr)
    
    return result

# Example usage (for testing within the package)
if __name__ == "__main__":
    # Create dummy files for testing
    with open("file1.json", "w") as f:
        f.write("{\"key\": \"value1\"}")
    with open("file2.json", "w") as f:
        f.write("{\"key\": \"value2\"}")

    print("Running diffx...")
    result = run_diffx(["file1.json", "file2.json"])
    print("Stdout:", result.stdout)
    print("Stderr:", result.stderr)
    print("Return Code:", result.returncode)

    os.remove("file1.json")
    os.remove("file2.json")

