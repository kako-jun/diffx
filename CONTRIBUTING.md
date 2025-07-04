# Contributing to diffx

We welcome contributions to `diffx`! By following these guidelines, you can help us maintain a high-quality and consistent codebase.

## How to Contribute

1.  **Fork the Repository**: Start by forking the `diffx` repository on GitHub.
2.  **Clone Your Fork**: Clone your forked repository to your local machine:

    ```bash
    git clone https://github.com/your-username/diffx.git
    cd diffx
    ```

3.  **Create a New Branch**: Create a new branch for your feature or bug fix:

    ```bash
    git checkout -b feature/your-feature-name
    # or
    git checkout -b bugfix/your-bug-fix-name
    ```

4.  **Make Your Changes**: Implement your changes, ensuring they adhere to the coding style and guidelines (see below).

5.  **Test Your Changes**: Run tests to ensure your changes haven't introduced any regressions and that new features are adequately covered.

6.  **Commit Your Changes**: Write clear, concise commit messages. Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification (e.g., `feat: add new feature`, `fix: resolve bug`).

7.  **Push to Your Fork**: Push your new branch to your GitHub fork:

    ```bash
    git push origin feature/your-feature-name
    ```

8.  **Create a Pull Request**: Open a pull request from your forked repository to the `main` branch of the `diffx` repository. Provide a clear description of your changes.

## Development Environment Setup

`diffx` is written in Rust. To get started, you'll need to install the Rust toolchain.

1.  **Install Rust**: If you don't have Rust installed, we recommend using `rustup`:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Follow the on-screen instructions. After installation, restart your terminal or run `source $HOME/.cargo/env`.

2.  **Build the Project**: Navigate to the root of the `diffx` repository and build the project:

    ```bash
    cargo build
    ```

3.  **Run Tests**: To ensure everything is set up correctly and to verify your changes, run the tests:

    ```bash
    cargo test
    ```

## Coding Style and Guidelines

*   **Rustfmt**: We use `rustfmt` for code formatting. Please ensure your code is formatted by running:

    ```bash
    cargo fmt
    ```

*   **Clippy**: We use `clippy` for linting. Please ensure your code passes `clippy` checks:

    ```bash
    cargo clippy
    ```

*   **Error Handling**: Use `anyhow` and `thiserror` for robust error handling.
*   **Testing**: All new features and bug fixes should be accompanied by appropriate unit and/or integration tests.
*   **Documentation**: Add comments to complex code sections and update relevant documentation (e.g., `README.md`, `docs/`) for any new features or significant changes.

## Reporting Bugs

If you find a bug, please open an issue on the [GitHub Issues page](https://github.com/your-org/diffx/issues). Provide a clear description of the bug, steps to reproduce it, and expected behavior.

## Feature Requests

If you have an idea for a new feature, please open an issue on the [GitHub Issues page](https://github.com/your-org/diffx/issues) to discuss it. We appreciate your suggestions!
