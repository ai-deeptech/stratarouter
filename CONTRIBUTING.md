# Contributing to StrataRouter

Thank you for your interest in contributing! 🎉

StrataRouter is an AI Execution Control Plane backed by a Rust core.
Contributions are welcome across the entire stack: Rust engine, Python SDK,
framework integrations, benchmarks, and documentation.

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork:
   ```bash
   git clone https://github.com/ai-deeptech/stratarouter.git
   cd stratarouter
   ```
3. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

---

## Development Setup

### Requirements
- Python 3.8+
- Rust 1.70+ (`rustup update stable`)
- `cargo` (comes with Rust)
- `maturin` (`pip install maturin`)

### Setup Steps

```bash
# Install Python dev dependencies
pip install -e "python/.[dev]"

# Build the Rust core and install Python bindings
cd python && maturin develop --release && cd ..

# Verify everything works
make test
```

---

## Code Style

### Python
- Follow **PEP 8**
- Format with **Black**: `black python/`
- Lint with **Ruff**: `ruff check python/`
- Type hints required on all public APIs
- Max line length: 100

### Rust
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Format: `cargo fmt` (required before every commit)
- Lint: `cargo clippy -- -D warnings` (all warnings are errors)
- Document all public APIs with `///` doc comments

---

## Testing

```bash
# Run everything
make test

# Python tests only
cd python && pytest tests/ -v --cov=stratarouter

# Rust tests only
cd core && cargo test --release

# Benchmarks (compile-check only in CI)
cd core && cargo bench --no-run
```

---

## Pull Request Process

1. **Write / update tests** for your changes (Rust: `cargo test`, Python: `pytest`)
2. **Update documentation** — docstrings, README, or docs/ as appropriate
3. **Update `CHANGELOG.md`** under `[Unreleased]`
4. **Ensure CI is green** — all checks must pass before review
5. **Open a PR** with a clear description and reference to any related issue

### PR Title Format (Conventional Commits)

| Prefix | When to use |
|---|---|
| `feat:` | New feature or capability |
| `fix:` | Bug fix |
| `perf:` | Performance improvement |
| `docs:` | Documentation only |
| `test:` | Tests only |
| `refactor:` | Code restructuring, no behaviour change |
| `ci:` | CI/CD workflow changes |
| `chore:` | Build system, dependencies |

Examples:
- `feat: Add Ollama encoder`
- `fix: Cohere encoder lazy import`
- `perf: SIMD cosine similarity for AVX2`
- `docs: Update quick-start example`

---

## Code Review Checklist

- [ ] `cargo fmt` and `cargo clippy -- -D warnings` pass
- [ ] `black python/` and `ruff check python/` pass
- [ ] `mypy stratarouter/ --ignore-missing-imports` has no new errors
- [ ] All tests pass (`cargo test --release`, `pytest tests/ -v`)
- [ ] New tests added for new behaviour
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] Public API changes reflected in docstrings and docs/

---

## Good First Issues

Look for [`good first issue`](https://github.com/ai-deeptech/stratarouter/issues?q=label%3A%22good+first+issue%22)
labels — these are well-scoped tasks ideal for new contributors.

Contributions we especially welcome:
- New encoders: Mistral, Gemini, Ollama
- Framework integrations (DSPy, Haystack, Semantic Kernel)
- Benchmark improvements and methodology documentation
- Documentation improvements and tutorials

---

## Questions?

- **GitHub Issues** — bug reports and feature requests
- **GitHub Discussions** — design questions, ideas, general help
- **Email** — support@stratarouter.com
- **Docs** — https://docs.stratarouter.com

---

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](LICENSE.txt).

Thank you for making StrataRouter better! 🚀
