#!/bin/bash
set -e

echo "======================================"
echo "Building StrataRouter"
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

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Check Python version
info "Checking Python version..."
PYTHON_CMD="${PYO3_PYTHON:-python3}"

if ! command -v $PYTHON_CMD &> /dev/null; then
    error "Python not found"
    exit 1
fi

PYTHON_VERSION=$($PYTHON_CMD --version 2>&1 | awk '{print $2}')
PYTHON_MAJOR=$(echo $PYTHON_VERSION | cut -d. -f1)
PYTHON_MINOR=$(echo $PYTHON_VERSION | cut -d. -f2)

echo "  Found: Python $PYTHON_VERSION"

if [ "$PYTHON_MAJOR" -lt 3 ] || ([ "$PYTHON_MAJOR" -eq 3 ] && [ "$PYTHON_MINOR" -lt 8 ]); then
    error "Python 3.8+ required (found $PYTHON_VERSION)"
    exit 1
fi

success "Python version OK ($PYTHON_VERSION)"
echo ""

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
cd "$PROJECT_ROOT/core"
if PYO3_PYTHON=$PYTHON_CMD cargo build --release 2>&1 | tee /tmp/cargo_build.log; then
    success "Rust core built"
else
    error "Rust build failed"
    exit 1
fi
echo ""

info "Running Rust tests..."
if cargo test --release 2>&1; then
    success "Rust tests passed"
else
    error "Rust tests failed (continuing anyway)"
fi
cd "$PROJECT_ROOT"
echo ""

info "Building Python package..."
cd "$PROJECT_ROOT/python"
if command -v maturin &> /dev/null; then
    if PYO3_PYTHON=$PYTHON_CMD maturin develop --release 2>&1; then
        success "Python package built and installed"
    else
        error "Maturin build failed"
        exit 1
    fi
else
    error "Maturin not found"
    echo ""
    echo "Install maturin: pip install maturin"
    exit 1
fi
cd "$PROJECT_ROOT"
echo ""

echo "======================================"
echo "✨ Build complete!"
echo ""
echo "Python used: $PYTHON_CMD ($PYTHON_VERSION)"
echo ""

# Test installation
info "Testing installation..."
if $PYTHON_CMD -c "import stratarouter; print(f'✓ StrataRouter {stratarouter.__version__} imported successfully')" 2>/dev/null; then
    success "Installation verified"
else
    error "Import failed"
fi
echo ""

echo "Next steps:"
echo "  1. Run tests: ./scripts/test_all.sh"
echo "  2. Try examples: $PYTHON_CMD examples/quickstart.py"
echo "  3. Start server: cd server && uvicorn main:app --reload"
echo "======================================"
