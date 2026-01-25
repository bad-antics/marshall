# Contributing to Marshall

Thank you for your interest in contributing to Marshall! This document provides guidelines and instructions.

## Getting Started

### Prerequisites

- Rust 1.75+ toolchain
- GTK4 development libraries
- libadwaita development libraries

### Build Setup

```bash
# Clone the repository
git clone https://github.com/bad-antics/marshall.git
cd marshall

# Install dependencies (Fedora/RHEL)
sudo dnf install gtk4-devel libadwaita-devel

# Install dependencies (Debian/Ubuntu)
sudo apt install libgtk-4-dev libadwaita-1-dev

# Build
cargo build --release

# Run
cargo run
```

## Development Workflow

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Address all clippy warnings: `cargo clippy`
- Document public APIs with doc comments
- Keep functions focused and testable

### Commit Messages

Use conventional commits format:

```
type(scope): brief description

Optional longer description.

Refs: #issue_number
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Pull Requests

1. Fork and create a feature branch
2. Make changes with clear commits
3. Update documentation as needed
4. Ensure CI passes
5. Request review

## Architecture

### Crate Structure

```
marshall/
├── src/
│   ├── main.rs          # Application entry
│   ├── app.rs           # GTK application setup
│   ├── window.rs        # Main window
│   ├── widgets/         # Custom UI components
│   ├── services/        # Business logic
│   └── models/          # Data structures
├── assets/              # Icons, images, stylesheets
└── resources/           # GResource bundles
```

### Key Components

- **Application**: GTK4 + libadwaita setup
- **Window Management**: Multi-window support
- **State Management**: Reactive data flow
- **File Operations**: Async I/O patterns

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Documentation

```bash
# Generate docs
cargo doc --open

# Check doc coverage
cargo doc --document-private-items
```

## Feature Requests

Open a GitHub issue with:
- Clear use case description
- Expected behavior
- Optional: Implementation suggestions

## Bug Reports

Include:
- Marshall version
- OS and desktop environment
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers
- Focus on the work, not the person
- Assume good intentions

## License

Contributions are licensed under MIT. By contributing, you agree to these terms.

## Questions?

- GitHub Discussions for questions
- Discord for real-time chat: [discord.gg/killers](https://discord.gg/killers)
