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
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/yourusername/bitcoin-rpc-codegen/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/yourusername/bitcoin-rpc-codegen/issues/new); it's that easy!

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

- Use 4 spaces for indentation rather than tabs
- You can try running `npm run lint` for style unification

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## Development Setup

1. **Install Rust**: Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

2. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/bitcoin-rpc-codegen.git
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

5. **Run the code generator**:
   ```bash
   cargo run --bin bitcoin-rpc-generator
   ```

## Project Structure

- `src/` - Source code for the generator
  - `main.rs` - Entry point and orchestration
  - `generator/` - Code generation logic
  - `parser/` - API documentation parsing
- `generated/` - Output directory for generated code
  - `client/` - Generated client code
  - `types/` - Generated type definitions
- `examples/` - Example code demonstrating how to use the generated code
- `scripts/` - Utility scripts for development and CI/CD

## Guidelines for Pull Requests

1. **Keep it focused**: Each pull request should address a single issue or feature.
2. **Write tests**: Include tests for any new functionality.
3. **Update documentation**: Update the README.md and other documentation as needed.
4. **Follow the code style**: Use the same code style as the rest of the project.
5. **Squash commits**: Before submitting a pull request, squash your commits into a single commit.

## Questions and Discussions

If you have questions or want to discuss ideas, please open an issue on GitHub.

Thank you for contributing to the Bitcoin RPC Code Generator!
