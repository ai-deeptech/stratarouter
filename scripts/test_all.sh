#!/bin/bash
set -e

echo "======================================"
echo "Running All Tests for StrataRouter"
echo "======================================"
echo ""

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

error() {
    echo -e "${RED}✗ $1${NC}"
}

# 1. Rust tests
echo "1. Running Rust tests..."
cd core
if cargo test --all-features; then
    success "Rust tests passed"
else
    error "Rust tests failed"
    exit 1
fi
cd ..
echo ""

# 2. Rust formatting
echo "2. Checking Rust formatting..."
cd core
if cargo fmt -- --check; then
    success "Rust formatting OK"
else
    error "Rust formatting failed"
    echo "Run: cd core && cargo fmt"
    exit 1
fi
cd ..
echo ""

# 3. Rust clippy
echo "3. Running Rust clippy..."
cd core
if cargo clippy -- -D warnings; then
    success "Rust clippy passed"
else
    error "Rust clippy found issues"
    exit 1
fi
cd ..
echo ""

# 4. Build Rust core
echo "4. Building Rust core..."
cd python
if maturin develop --release; then
    success "Build successful"
else
    error "Build failed"
    exit 1
fi
cd ..
echo ""

# 5. Python tests
echo "5. Running Python tests..."
cd python
if pytest tests/ -v --cov=stratarouter --cov-report=term-missing; then
    success "Python tests passed"
else
    error "Python tests failed"
    exit 1
fi
cd ..
echo ""

# 6. Python formatting
echo "6. Checking Python formatting..."
cd python
if black --check stratarouter/; then
    success "Python formatting OK"
else
    error "Python formatting failed"
    echo "Run: cd python && black stratarouter/"
    exit 1
fi
cd ..
echo ""

# 7. Python linting
echo "7. Running Python linting..."
cd python
if ruff check stratarouter/; then
    success "Python linting passed"
else
    error "Python linting found issues"
    echo "Run: cd python && ruff check --fix stratarouter/"
    exit 1
fi
cd ..
echo ""

echo "======================================"
echo "✨ All tests passed successfully!"
echo "======================================"
