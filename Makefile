.PHONY: help install build test lint fmt clean dev

help:
	@echo "StrataRouter Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install       Install in development mode (Rust core + Python SDK)"
	@echo "  make dev           Alias for install"
	@echo ""
	@echo "Build:"
	@echo "  make build         Build Rust core (release) + Python wheel"
	@echo ""
	@echo "Quality:"
	@echo "  make test          Run all tests (Rust + Python)"
	@echo "  make lint          Run all linters (clippy + ruff + mypy)"
	@echo "  make fmt           Auto-format all code (cargo fmt + black)"
	@echo ""
	@echo "Housekeeping:"
	@echo "  make clean         Remove build artifacts"
	@echo ""

# ── Setup ─────────────────────────────────────────────────────────────────────

install:
	@echo "→ Installing maturin..."
	pip install maturin
	@echo "→ Building Rust core and installing Python package..."
	cd python && maturin develop --release
	@echo "→ Installing dev dependencies..."
	cd python && pip install -e ".[dev]"
	@echo "✓ Installation complete"

dev: install

# ── Build ─────────────────────────────────────────────────────────────────────

build:
	@echo "→ Building Rust core (release)..."
	cd core && cargo build --release
	@echo "→ Building Python wheel..."
	cd python && maturin build --release
	@echo "✓ Build complete"

# ── Quality ───────────────────────────────────────────────────────────────────

test:
	@echo "→ Running Rust tests..."
	cd core && cargo test --release
	@echo "→ Running Python tests (python/tests/)..."
	cd python && pytest tests/ -v --tb=short
	@echo "→ Running root-level tests (tests/)..."
	pytest tests/ -v --tb=short
	@echo "✓ All tests passed"

lint:
	@echo "→ cargo clippy..."
	cd core && cargo clippy -- -D warnings
	@echo "→ ruff..."
	cd python && ruff check stratarouter/
	@echo "→ mypy..."
	cd python && mypy stratarouter/ --ignore-missing-imports || true
	@echo "✓ Lint complete"

fmt:
	@echo "→ cargo fmt..."
	cd core && cargo fmt
	@echo "→ black..."
	cd python && black stratarouter/ tests/
	@echo "✓ Format complete"

# ── Housekeeping ──────────────────────────────────────────────────────────────

clean:
	@echo "→ Cleaning build artifacts..."
	rm -rf core/target/
	rm -rf python/build/ python/dist/ python/*.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -name "*.so" -o -name "*.pyd" -o -name "*.dylib" | xargs rm -f 2>/dev/null || true
	@echo "✓ Clean complete"
