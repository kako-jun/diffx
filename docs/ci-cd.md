# CI/CD Integration Examples

This section provides examples of how to integrate `diffx` into your Continuous Integration/Continuous Deployment (CI/CD) pipelines.

## GitHub Actions

YouYou can use `diffx` in your GitHub Actions workflows to automatically check for structured differences in configuration files, data, or other structured assets. This can be particularly useful for ensuring that changes adhere to expected patterns or for flagging unexpected modifications.

Here's a basic example of a GitHub Actions workflow that uses `diffx` to compare two JSON files. This workflow could be triggered on a pull request to ensure that configuration changes are reviewed for their structural impact.

```yaml
name: Check Config Differences

on:
  pull_request:
    branches:
      - main
    paths:
      - 'config/*.json'

jobs:
  diff_check:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        toolchain: stable

    - name: Install diffx
      run: cargo install --path .

    - name: Create dummy config files (for demonstration)
      run: |
        mkdir -p config
        echo '{"version": "1.0", "features": ["a", "b"]}' > config/base.json
        echo '{"version": "1.1", "features": ["a", "c"]}' > config/new.json

    - name: Run diffx check
      id: diff
      continue-on-error: true
      run: diffx config/base.json config/new.json --output cli

    - name: Report differences
      if: steps.diff.outcome == 'failure'
      run: |
        echo "## :x: Structural Differences Found"
        echo "```"
        echo "${{ steps.diff.outputs.stdout }}"
        echo "${{ steps.diff.outputs.stderr }}"
        echo "```"
        exit 1
      shell: bash

    - name: No differences found
      if: steps.diff.outcome == 'success'
      run: echo "## :white_check_mark: No structural differences found."
      shell: bash
```

### Explanation:

- **`on: pull_request`**: This workflow triggers whenever a pull request is opened or updated targeting the `main` branch, specifically if files in the `config/` directory with a `.json` extension are changed.
- **`actions/checkout@v4`**: Checks out your repository code.
- **`dtolnay/rust-toolchain@stable`**: Installs the Rust toolchain required to build `diffx`.
- **`cargo install --path .`**: Installs `diffx` from the current repository path.
- **`Create dummy config files`**: (For demonstration purposes) Creates two sample JSON files to compare. In a real scenario, these would be your actual configuration files.
- **`Run diffx check`**: Executes the `diffx` command. 
  - `config/base.json config/new.json`: The two files being compared.
  - `--output cli`: Specifies the CLI output format.
  - `continue-on-error: true`: Allows the workflow to continue even if `diffx` exits with a non-zero status (which it does when differences are found).
- **`Report differences`**: This step runs only if the `diff` step failed (meaning differences were found). It outputs a formatted message to the GitHub Actions summary, making it easy to see the differences directly in the pull request UI.
- **`No differences found`**: This step runs if `diffx` found no differences.

This example can be adapted for different file formats (YAML, TOML) and for various triggering events in your CI/CD pipeline.