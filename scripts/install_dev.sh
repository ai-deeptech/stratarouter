#!/bin/bash
set -e

echo "======================================"
echo "Installing StrataRouter (Development)"
echo "======================================"
echo ""

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

info() {
    echo -e "${YELLOW}➜ $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check Rust
if ! command -v cargo &> /dev/null; then
    error "Rust is not installed"
    echo ""
    echo "Install Rust from: https://rustup.rs/"
    echo "Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
success "Rust is installed"

# Check Python
if ! command -v python3 &> /dev/null; then
    error "Python 3 is not installed"
    exit 1
fi
PYTHON_VERSION=$(python3 --version)
success "Python is installed: $PYTHON_VERSION"
echo ""

# Install maturin
info "Installing maturin..."
pip install maturin
success "Maturin installed"
echo ""

# Install development dependencies
info "Installing development dependencies..."
pip install pytest pytest-cov black ruff mypy
success "Dev dependencies installed"
echo ""

# Install encoder dependencies
info "Installing encoder dependencies..."
pip install sentence-transformers numpy pydantic
success "Encoder dependencies installed"
echo ""

# Build in development mode
info "Building in development mode..."
cd python
maturin develop --release
cd ..
success "Build complete"
echo ""

# Verify installation
info "Verifying installation..."
python3 -c "import stratarouter; print(f'StrataRouter version: {stratarouter.__version__}')"
success "Installation verified"
echo ""

echo "======================================"
echo "✨ Development setup complete!"
echo ""
echo "You can now:"
echo "  - Edit code and run: cd python && maturin develop"
echo "  - Run tests: ./scripts/test_all.sh"
echo "  - Run examples: python examples/quickstart.py"
echo "======================================"
