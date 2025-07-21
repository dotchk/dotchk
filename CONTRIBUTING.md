# Contributing to dotchk

Thank you for your interest in contributing to dotchk! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/dotchk/dotchk.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/dotchk/dotchk.git
cd dotchk

# Build the project
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- example.com
```

## Code Style

- Follow Rust standard style guidelines
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update documentation for API changes

## Testing

- Write unit tests for new functions
- Add integration tests for new CLI commands
- Ensure all tests pass: `cargo test`
- Test performance impact with benchmarks: `cargo bench`
- Test install scripts on your platform if modified

## Pull Request Guidelines

1. **Keep PRs focused**: One feature or fix per PR
2. **Write clear commit messages**: Describe what and why
3. **Update documentation**: Include README updates if needed
4. **Add tests**: New features need test coverage
5. **Pass CI checks**: All tests, formatting, and linting must pass

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Provide clear reproduction steps for bugs
- Include system information (OS, Rust version)
- Search existing issues before creating new ones

## Feature Requests

We welcome feature requests! Please:
- Explain the use case
- Describe the expected behavior
- Consider implementation complexity

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Assume good intentions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.