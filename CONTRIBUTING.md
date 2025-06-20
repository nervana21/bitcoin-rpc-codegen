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
2. If you've added code that should be tested, add tests.
3. If you've changed APIs or the generation process, update the documentation.
4. Ensure the test suite passes using `cargo test`.
5. Make sure your code adheres to the standard Rust style (`cargo fmt`) and passes linter checks (`cargo clippy`).
6. Issue that pull request!

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/nervana21/bitcoin-rpc-codegen/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/nervana21/bitcoin-rpc-codegen/issues/new); it's that easy! **Please replace `yourusername` with the actual GitHub organization or username if different.**

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
   git clone https://github.com/nervana21/bitcoin-rpc-codegen.git
   cd bitcoin-rpc-codegen
   ```

3. **Build the project**:

   ```bash
   cargo build
   ```

4. **Run the tests**:

   ```bash
   cargo test
   ```

## Project Structure

The project is organized into several focused crates:

- `rpc_api/`: JSON model of RPC methods and parameters
- `parser/`: Parses `api.json` into structured form
- `schema/`: Normalizes and validates parsed data
- `codegen/`: Emits Rust modules and client implementations
- `transport/`: Async RPC transport + error handling
- `node/`: Regtest node management and test client support
- `pipeline/`: Orchestrates parsing → schema → generation

## Guidelines for Pull Requests

1. **Keep it focused**: Each pull request should address a single issue or feature.
2. **Write tests**: Include tests for any new functionality or bug fixes.
3. **Update documentation**: Update relevant documentation as needed.
4. **Follow the code style**: Run `cargo fmt` and `cargo clippy`.
5. **Meaningful commits**: Use conventional commit messages (e.g., `feat(parser): add support for new type`).

## Questions and Discussions

If you have questions or want to discuss ideas, please open an issue on GitHub.

Thank you for contributing to the Bitcoin RPC Code Generator!
