#!/bin/bash
set -e

echo "======================================"
echo "Building StrataRouter"
echo "======================================"
echo ""

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    echo -e "${YELLOW}➜ $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

info "Cleaning previous builds..."
rm -rf target/
rm -rf core/target/
rm -rf python/build/
rm -rf python/dist/
rm -rf python/*.egg-info
find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
success "Clean complete"
echo ""

info "Building Rust core..."
cd ../core
cargo build --release
success "Rust core built"
cd ..
echo ""

info "Running Rust tests..."
cd ../core
cargo test --release
success "Rust tests passed"
cd ..
echo ""

info "Building Python package..."
cd ../python
maturin build --release
success "Python package built"
cd ..
echo ""

echo "======================================"
echo "✨ Build complete!"
echo ""
echo "Built packages:"
ls -lh target/wheels/ 2>/dev/null || ls -lh python/target/wheels/ 2>/dev/null || echo "No wheels found"
echo ""
echo "To install locally:"
echo "  pip install target/wheels/stratarouter-*.whl"
echo ""
echo "======================================"
