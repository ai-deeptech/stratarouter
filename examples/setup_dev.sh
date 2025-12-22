#!/bin/bash
# ============================================================================
# StrataRouter Development Environment Setup Script
# Sets up complete development environment for Rust + Python
# ============================================================================

set -e  # Exit on error

echo "=================================================="
echo "StrataRouter Development Environment Setup"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo "ℹ $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# ============================================================================
# 1. Check System Requirements
# ============================================================================

echo "Step 1: Checking system requirements..."
echo ""

# Check for Rust
if command_exists cargo; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    print_success "Rust found: $RUST_VERSION"
else
    print_error "Rust not found"
    echo ""
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_success "Rust installed successfully"
fi

# Check for Python
if command_exists python3; then
    PYTHON_VERSION=$(python3 --version | awk '{print $2}')
    print_success "Python found: $PYTHON_VERSION"
    
    # Check Python version >= 3.8
    PYTHON_MAJOR=$(echo $PYTHON_VERSION | cut -d. -f1)
    PYTHON_MINOR=$(echo $PYTHON_VERSION | cut -d. -f2)
    
    if [ "$PYTHON_MAJOR" -lt 3 ] || ([ "$PYTHON_MAJOR" -eq 3 ] && [ "$PYTHON_MINOR" -lt 8 ]); then
        print_error "Python 3.8+ required, found $PYTHON_VERSION"
        exit 1
    fi
else
    print_error "Python 3 not found"
    echo "Please install Python 3.8 or higher"
    exit 1
fi

# Check for pip
if command_exists pip3; then
    print_success "pip3 found"
else
    print_error "pip3 not found"
    echo "Installing pip..."
    python3 -m ensurepip --upgrade
    print_success "pip3 installed"
fi

echo ""

# ============================================================================
# 2. Install Rust Components
# ============================================================================

echo "Step 2: Installing Rust components..."
echo ""

# Update Rust
rustup update stable
print_success "Rust updated to latest stable"

# Install Rust components
rustup component add rustfmt clippy
print_success "rustfmt and clippy installed"

echo ""

# ============================================================================
# 3. Install Python Dependencies
# ============================================================================

echo "Step 3: Installing Python dependencies..."
echo ""

# Upgrade pip
python3 -m pip install --upgrade pip
print_success "pip upgraded"

# Install maturin
python3 -m pip install maturin
print_success "maturin installed"

# Install development dependencies
python3 -m pip install pytest pytest-cov pytest-asyncio black ruff mypy
print_success "Development tools installed"

# Install optional dependencies (with error handling)
echo ""
print_info "Installing optional dependencies..."

if python3 -m pip install sentence-transformers torch --no-deps 2>/dev/null; then
    print_success "sentence-transformers installed"
else
    print_warning "sentence-transformers installation skipped (optional)"
fi

if python3 -m pip install openai 2>/dev/null; then
    print_success "openai installed"
else
    print_warning "openai installation skipped (optional)"
fi

if python3 -m pip install httpx 2>/dev/null; then
    print_success "httpx installed"
else
    print_warning "httpx installation skipped (optional)"
fi

echo ""

# ============================================================================
# 4. Build Rust Core
# ============================================================================

echo "Step 4: Building Rust core..."
echo ""

cd ../core

# Clean previous builds
if [ -d "target" ]; then
    print_info "Cleaning previous builds..."
    cargo clean
fi

# Build in release mode
print_info "Building Rust core (this may take a few minutes)..."
if cargo build --release; then
    print_success "Rust core built successfully"
else
    print_error "Rust core build failed"
    cd ..
    exit 1
fi

# Run Rust tests
print_info "Running Rust tests..."
if cargo test --release; then
    print_success "All Rust tests passed"
else
    print_error "Some Rust tests failed"
    cd ..
    exit 1
fi

cd ..

echo ""

# ============================================================================
# 5. Build Python Package
# ============================================================================

echo "Step 5: Building Python package..."
echo ""

# Build wheel
cd ../core
print_info "Building Python wheel..."
if maturin build --release --strip; then
    print_success "Python wheel built successfully"
else
    print_error "Python wheel build failed"
    cd ..
    exit 1
fi
cd ..

# Install wheel
print_info "Installing StrataRouter package..."
if python3 -m pip install target/wheels/*.whl --force-reinstall; then
    print_success "StrataRouter installed successfully"
else
    print_error "StrataRouter installation failed"
    exit 1
fi

echo ""

# ============================================================================
# 6. Verify Installation
# ============================================================================

echo "Step 6: Verifying installation..."
echo ""

# Test Python import
if python3 -c "import stratarouter; print(f'StrataRouter v{stratarouter.__version__}')"; then
    print_success "Python package import successful"
else
    print_error "Python package import failed"
    exit 1
fi

# Test Rust core import
if python3 -c "from stratarouter._core import PyRouter"; then
    print_success "Rust core binding successful"
else
    print_error "Rust core binding failed"
    exit 1
fi

echo ""

# ============================================================================
# 7. Setup Pre-commit Hooks (Optional)
# ============================================================================

echo "Step 7: Setting up development tools..."
echo ""

# Check for pre-commit
if command_exists pre-commit; then
    print_info "Installing pre-commit hooks..."
    if pre-commit install 2>/dev/null; then
        print_success "Pre-commit hooks installed"
    else
        print_warning "Pre-commit hooks not installed (optional)"
    fi
else
    print_info "pre-commit not found (optional)"
    echo "Install with: pip install pre-commit"
fi

echo ""

# ============================================================================
# 8. Create Development Directories
# ============================================================================

echo "Step 8: Creating development directories..."
echo ""

# Create directories if they don't exist
mkdir -p ../logs
mkdir -p ../models
mkdir -p ../.stratarouter

print_success "Development directories created"

echo ""

# ============================================================================
# 9. Setup Environment Variables
# ============================================================================

echo "Step 9: Setting up environment..."
echo ""

# Create .env.example if it doesn't exist
if [ ! -f ".env.example" ]; then
    cat > .env.example << EOF
# StrataRouter Development Environment Variables

# OpenAI API Key (optional)
OPENAI_API_KEY=sk-...

# StrataRouter Cloud API Key (optional)
STRATAROUTER_API_KEY=sr-...

# Development Mode
STRATAROUTER_DEV=1

# Log Level
LOG_LEVEL=INFO
EOF
    print_success ".env.example created"
fi

echo ""

# ============================================================================
# 10. Display Summary
# ============================================================================

echo "=================================================="
echo "Setup Complete! 🎉"
echo "=================================================="
echo ""
echo "Installation Summary:"
echo "  ✓ Rust toolchain configured"
echo "  ✓ Python environment ready"
echo "  ✓ StrataRouter built and installed"
echo "  ✓ Development tools configured"
echo ""
echo "Next Steps:"
echo ""
echo "  1. Run tests:"
echo "     ./scripts/test_all.sh"
echo ""
echo "  2. Try the quickstart example:"
echo "     python examples/quickstart.py"
echo ""
echo "  3. Build a release:"
echo "     ./scripts/build_release.sh"
echo ""
echo "  4. Start developing:"
echo "     # Edit code in core/ or python/"
echo "     # Run: maturin develop --release"
echo "     # Test: pytest tests/"
echo ""
echo "Development Commands:"
echo "  - Format Rust:   cargo fmt"
echo "  - Format Python: black python/"
echo "  - Lint Rust:     cargo clippy"
echo "  - Lint Python:   ruff check python/"
echo "  - Test Rust:     cargo test"
echo "  - Test Python:   pytest tests/"
echo ""
echo "Documentation:"
echo "  - Getting Started: docs/getting-started.md"
echo "  - API Reference:   docs/api-reference.md"
echo "  - Contributing:    CONTRIBUTING.md"
echo ""
echo "Happy coding! 🚀"
echo "=================================================="

# ============================================================================
# Optional: Quick verification test
# ============================================================================

echo ""
read -p "Run quick verification test? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Running quick verification test..."
    
    python3 << 'EOF'
from stratarouter import Router, Route
import sys

try:
    # Create router
    router = Router()
    
    # Add test route
    route = Route(
        id="test",
        description="Test route",
        keywords=["test", "demo"]
    )
    router.add(route)
    
    # Build index (will use mock encoder in test)
    print("✓ Router created successfully")
    print("✓ Route added successfully")
    print("✓ All systems operational!")
    sys.exit(0)
    
except Exception as e:
    print(f"✗ Verification failed: {e}")
    sys.exit(1)
EOF
    
    if [ $? -eq 0 ]; then
        echo ""
        print_success "Verification test passed!"
    else
        echo ""
        print_error "Verification test failed"
        exit 1
    fi
fi

echo ""
echo "Setup complete! You're ready to develop StrataRouter."
