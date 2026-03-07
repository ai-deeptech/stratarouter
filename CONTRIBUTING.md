"""
# Contributing to StrataRouter Lite

Thank you for your interest in contributing! 🎉

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stratarouter.git
   cd stratarouter
   ```
3. Create a branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Requirements
- Python 3.8+
- Rust 1.70+
- cargo (Rust package manager)

### Setup Steps

```bash
# Run setup script
./scripts/setup_dev.sh

# Or manually:
pip install -e ".[dev]"
cd core
cargo build --release
cargo test
```

## Code Style

### Python
- Follow PEP 8
- Use Black: `black python/`
- Use Ruff: `ruff check python/`
- Type hints required
- Max line length: 100

### Rust
- Follow Rust style guide
- Use rustfmt: `cargo fmt`
- Use clippy: `cargo clippy -- -D warnings`
- Document public APIs

## Testing

```bash
# All tests
./scripts/test_all.sh

# Python only
pytest tests/ -v --cov=stratarouter

# Rust only
cd core && cargo test --release
```

## Pull Request Process

1. **Update documentation** for any changes
2. **Add tests** for new features
3. **Ensure all tests pass**
4. **Update CHANGELOG.md**
5. **Submit PR** with clear description

### PR Title Format
- `feat: Add new feature`
- `fix: Fix bug description`
- `docs: Update documentation`
- `test: Add tests`
- `refactor: Refactor code`

## Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests added and passing
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
- [ ] CHANGELOG.md updated
- [ ] Commit messages are clear

## Questions?

- GitHub Issues: Bug reports and feature requests
- Email: hello@inteleion.ai

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for making StrataRouter better! 🚀
"""
