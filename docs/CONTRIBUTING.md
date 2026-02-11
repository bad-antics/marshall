# Contributing to Marshall

## Development Setup

```bash
# Clone and build
git clone https://github.com/bad-antics/marshall
cd marshall
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Write tests for new features
- Document public APIs with `///` doc comments

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Write code and tests
4. Run `cargo fmt && cargo clippy && cargo test`
5. Submit PR with clear description

## Areas to Contribute

- 🐛 Bug fixes
- 🔒 Privacy feature improvements
- 🎨 UI/UX enhancements
- 📖 Documentation
- 🌍 Translations
- 🧪 Test coverage
