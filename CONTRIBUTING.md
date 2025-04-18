# Contributing to Bitcoin RPC Code Generator

We love your input! We want to make contributing to Bitcoin RPC Code Generator as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## We Develop with GitHub

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

## We Use [Github Flow](https://guides.github.com/introduction/flow/index.html)

Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests to the `tests/` directory.
3. If you've changed APIs or the generation process, update the documentation (like this file or `README.md`).
4. Ensure the test suite passes using `cargo test`.
5. Make sure your code adheres to the standard Rust style (`cargo fmt`) and passes linter checks (`cargo clippy`).
6. Issue that pull request!

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/yourusername/bitcoin-rpc-codegen/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/yourusername/bitcoin-rpc-codegen/issues/new); it's that easy! **Please replace `yourusername` with the actual GitHub organization or username if different.**

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Use a Consistent Coding Style

- We follow standard Rust formatting conventions. Run `cargo fmt` to format your code.
- We use Clippy for linting. Run `cargo clippy -- -D warnings` to check for issues.

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## Development Setup

1. **Install Rust**: Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

2. **Clone the repository**:

   ```bash
   # Replace 'yourusername' with the correct GitHub username/organization
   git clone https://github.com/yourusername/bitcoin-rpc-codegen.git
   cd bitcoin-rpc-codegen
   ```

3. **Build the project**: The core code generation happens during the build.

   ```bash
   cargo build
   ```

   This will parse `resources/api.json` and generate Rust code.

4. **Run the tests**:

   ```bash
   cargo test
   ```

## Project Structure

- `resources/`: Contains the source `api.json` file defining the Bitcoin RPC methods.
- `src/`: Source code for the generator library and binary.
  - `parser/`: Logic for parsing `resources/api.json`.
  - `generator/`: Code generation logic, invoked by `build.rs`.
  - `lib.rs`: Shared library code.
  - `main.rs`: Entry point for the binary executable (may have limited use).
  - `schema.json`: JSON schema file for validating `api.json`.
- `tests/`: Integration and unit tests. Run with `cargo test`.
- `examples/`: Example usage.
- `build.rs`: Build script that orchestrates the code generation process.
- `Cargo.toml`: Project manifest defining dependencies and metadata.
- `target/`: Build artifacts and generated code (usually ignored by git).

## Guidelines for Pull Requests

1. **Keep it focused**: Each pull request should address a single issue or feature.
2. **Write tests**: Include tests in the `tests/` directory for any new functionality or bug fixes.
3. **Update documentation**: Update `README.md`, `CONTRIBUTING.md`, and code comments as needed.
4. **Follow the code style**: Run `cargo fmt` and `cargo clippy`.
5. **Meaningful commits**: Use conventional commit messages (e.g., `feat(parser): add support for new type`). Try to keep commits logical, squashing is not strictly required if the history is clean.

## Questions and Discussions

If you have questions or want to discuss ideas, please open an issue on GitHub.

Thank you for contributing to the Bitcoin RPC Code Generator!
