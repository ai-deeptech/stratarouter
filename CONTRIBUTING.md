# Contributing to StrataRouter

Thank you for your interest in contributing! 🎉

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/ai-deeptech/stratarouter.git
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
- Use `cargo fmt` before committing
- Use `cargo clippy -- -D warnings` and fix all warnings
- Document all public APIs with `///` doc comments

## Testing

```bash
# All tests
make test

# Python only
cd python && pytest tests/ -v --cov=stratarouter

# Rust only
cd core && cargo test --release
```

## Pull Request Process

1. **Update documentation** for any changes
2. **Add tests** for new features (Rust: `cargo test`, Python: `pytest`)
3. **Ensure all tests pass** — CI must be green before review
4. **Update CHANGELOG.md** under `[Unreleased]`
5. **Submit PR** with a clear description referencing any related issues

### PR Title Format (Conventional Commits)
- `feat: Add Ollama encoder`
- `fix: Cohere encoder lazy import`
- `docs: Update quick-start example`
- `test: Add RouteLayer batch routing tests`
- `refactor: Rename IsotonicCalibrator → ScoreNormalizer`

## Code Review Checklist

- [ ] Code follows style guidelines (`cargo fmt`, `black`, `ruff`)
- [ ] Tests added and passing (`cargo test`, `pytest`)
- [ ] Documentation updated (doc comments + README if API changes)
- [ ] No breaking changes, or clearly documented in CHANGELOG.md
- [ ] Commit messages follow Conventional Commits format

## Good First Issues

Look for issues tagged [`good first issue`](https://github.com/ai-deeptech/stratarouter/issues?q=label%3A%22good+first+issue%22) — these are well-scoped tasks ideal for new contributors.

Contributions we especially welcome:
- New encoders: Cohere, Mistral, Gemini, Ollama
- Framework integrations
- Benchmark improvements
- Documentation and examples

## Questions?

- GitHub Issues: bug reports and feature requests
- Email: hello@stratarouter.dev

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for making StrataRouter better! 🚀
