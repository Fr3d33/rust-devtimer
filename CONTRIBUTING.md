# Contributing to DevTimer

Thank you for considering contributing to DevTimer!

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (OS, Rust version)

### Suggesting Features

Feature suggestions are welcome! Please open an issue and describe:
- The feature you'd like to see
- Why it would be useful
- Any implementation ideas you have

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and ensure code compiles (`cargo build` and `cargo test`)
5. Format your code (`cargo fmt`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Style

- Follow Rust conventions and idioms
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Add comments for complex logic
- Write clear commit messages

### Testing

Before submitting a PR:
```bash
# Build the project
cargo build

# Run tests (if available)
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt
```

## Code of Conduct

- Be respectful and considerate
- Welcome newcomers and help them learn
- Accept constructive criticism gracefully
- Focus on what's best for the project

## Questions?

Feel free to open an issue for any questions about contributing!
