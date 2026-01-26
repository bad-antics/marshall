# Contributing to Marshall Browser

Thank you for your interest in contributing!

## Ways to Contribute

### Bug Reports
1. Check existing issues first
2. Use the bug report template
3. Include system info, steps to reproduce, and error logs

### Feature Requests
1. Check if already requested
2. Describe the feature and use case
3. Consider privacy implications

### Code Contributions
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests if applicable
5. Submit a pull request

## Development Setup

```bash
# Clone
git clone https://github.com/bad-antics/marshall
cd marshall

# Install dependencies (Debian/Ubuntu)
sudo apt install libgtk-4-dev libwebkitgtk-6.0-dev libadwaita-1-dev

# Build
cargo build

# Run
cargo run
```

## Code Style
- Follow Rust conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` for linting

## Architecture
- `src/main.rs` - Application entry
- `src/browser/` - Browser functionality
- `src/assistant/` - AI assistant
- `src/ui/` - UI components

## Pull Request Process
1. Update documentation if needed
2. Ensure `cargo test` passes
3. Run `cargo clippy` without warnings
4. Request review
