#!/bin/bash
set -e

echo "======================================"
echo "Building StrataRouter (2-step process)"
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

cd /home/opc/backup/stratarouter/stratarouter
source .venv/bin/activate

# Step 1: Build Rust core
info "Step 1: Building Rust core..."
maturin develop --release --manifest-path core/Cargo.toml
success "Rust core built and installed"
echo ""

# Step 2: Install Python package
info "Step 2: Installing Python package..."
cd python
pip install -e .
cd ..
success "Python package installed"
echo ""

# Verify
info "Verifying installation..."
python << 'EOF'
try:
    import stratarouter
    print(f"✓ stratarouter v{stratarouter.__version__}")
    
    from stratarouter import Route, RouteLayer, Router
    print("✓ All classes imported successfully")
    
    from stratarouter.core import cosine_similarity
    print("✓ Rust core accessible")
    
    print("\n✨ Installation successful!")
except Exception as e:
    print(f"✗ Error: {e}")
    exit(1)
EOF

echo ""
echo "======================================"
echo "Ready to use!"
echo ""
echo "Try: python examples/quickstart.py"
echo "======================================"
