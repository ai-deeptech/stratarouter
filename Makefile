.PHONY: help install build test clean dev server

help:
	@echo "StrataRouter Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install       - Install in development mode"
	@echo "  make build         - Build release version"
	@echo ""
	@echo "Development:"
	@echo "  make dev           - Setup dev environment"
	@echo "  make test          - Run all tests"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "Server:"
	@echo "  make server        - Start FastAPI server"
	@echo ""

install:
	@echo "Installing StrataRouter..."
	pip install maturin
	cd python && maturin develop --release
	@echo "✓ Installation complete"

build:
	@echo "Building StrataRouter..."
	cd core && cargo build --release
	cd python && maturin build --release
	@echo "✓ Build complete"

test:
	@echo "Running tests..."
	cd core && cargo test
	cd python && pytest tests/ -v
	@echo "✓ Tests complete"

clean:
	@echo "Cleaning build artifacts..."
	rm -rf target/
	rm -rf core/target/
	rm -rf python/build/
	rm -rf python/dist/
	rm -rf python/*.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	@echo "✓ Clean complete"

dev: install
	@echo "Development environment ready!"

server:
	@echo "Starting FastAPI server..."
	cd server && uvicorn main:app --reload --host 0.0.0.0 --port 8000
