# Contributing to Reverse-Proxy-Reveal

First off, thanks for taking the time to contribute! 🎉

The following is a set of guidelines for contributing to Reverse-Proxy-Reveal.

## Code of Conduct

This project and everyone participating in it is governed by respect and professionalism. Please be kind and courteous to others.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed and what you expected**
- **Include system information** (OS, Rust version, etc.)
- **If possible, include relevant command output**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List any similar features in other tools**

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure your code follows Rust best practices (`cargo fmt` and `cargo clippy`)
4. Update the README.md with details of changes if needed
5. The PR will be merged once you have approval

## Development Setup

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone your fork
git clone https://github.com/YOUR_USERNAME/Reverse-Proxy-Reveal.git
cd Reverse-Proxy-Reveal
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Testing Your Changes

```bash
# Run with sample data
cargo run -- example.com --ranges 1.1.1.0/24 --verbose

# Run specific features
cargo run -- example.com --method GET --content-match "nginx"
```

## Style Guide

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and small

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests after the first line

Examples:
```
Add support for IPv6 scanning

This commit adds functionality to scan IPv6 addresses in addition to IPv4.
Closes #123
```

## Project Structure

```
Reverse-Proxy-Reveal/
├── src/
│   └── main.rs          # Main application code
├── target/              # Build output (ignored by git)
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Locked dependency versions
├── README.md            # Project documentation
├── LICENSE              # MIT License
├── .gitignore           # Git ignore rules
└── ips.txt.example      # Example IP ranges file
```

## Performance Considerations

When contributing code that affects performance:

1. **Benchmark your changes** - Use `cargo bench` if applicable
2. **Profile memory usage** - Ensure no memory leaks
3. **Test with large datasets** - Verify performance with /16 or /12 ranges
4. **Consider async/await overhead** - Tokio best practices
5. **Document performance characteristics** - Note Big-O complexity

## Security Considerations

- Never commit sensitive data (API keys, passwords, etc.)
- Validate all user inputs
- Use safe Rust practices (avoid `unsafe` unless absolutely necessary)
- Document security implications of changes
- Follow responsible disclosure for security issues

## Documentation

- Update README.md for user-facing changes
- Add inline comments for complex logic
- Update command-line help text if adding options
- Create examples for new features

## Questions?

Feel free to open an issue with the question label!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

