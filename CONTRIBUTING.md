# Contributing to StrataRouter

Thank you for your interest in contributing to StrataRouter!

## Development Setup

1. **Clone the repository**
```bash
git clone https://github.com/stratarouter/stratarouter
cd stratarouter
```

2. **Install dependencies**
```bash
./scripts/install_dev.sh
```

3. **Run tests**
```bash
./scripts/test_all.sh
```

## Making Changes

1. Create a new branch
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes

3. Run tests
```bash
./scripts/test_all.sh
```

4. Format code
```bash
# Rust
cd core && cargo fmt

# Python
cd python && black stratarouter/
```

5. Commit and push
```bash
git commit -m "feat: your feature description"
git push origin feature/your-feature-name
```

6. Create a Pull Request

## Code Style

- **Rust**: Follow standard Rust conventions, use `cargo fmt` and `cargo clippy`
- **Python**: Follow PEP 8, use `black` for formatting, line length 100
- **Tests**: Write tests for all new features
- **Documentation**: Update README and docstrings

## Running Examples
```bash
python examples/quickstart.py
python examples/fastapi_example.py
python examples/batch_processing.py
```

## Questions?

Open an issue on GitHub or join our Discord community.
