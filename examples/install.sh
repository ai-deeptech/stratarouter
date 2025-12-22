#!/bin/bash
# ============================================================================
# FIX: Install StrataRouter Python Package
# ============================================================================

echo "Installing StrataRouter..."
echo ""

# ============================================================================
# OPTION 1: Install from wheel (if already built)
# ============================================================================

echo "Option 1: Checking for pre-built wheel..."
if [ -d "target/wheels" ] && [ -n "$(ls -A target/wheels/*.whl 2>/dev/null)" ]; then
    echo "✓ Found wheel file"
    pip install target/wheels/*.whl --force-reinstall
    echo "✓ Installed from wheel"
else
    echo "⚠ No wheel found, need to build"
fi

echo ""

# ============================================================================
# OPTION 2: Build and install with maturin (RECOMMENDED)
# ============================================================================

echo "Option 2: Building and installing with maturin..."
echo ""

# Make sure you're in the project root
if [ -d "../core" ]; then
    echo "✓ Found core directory"
    
    # Navigate to core
    cd ../core
    
    # Build with maturin
    echo "Building wheel..."
    maturin build --release --strip --out ../target/wheels
    
    # Go back
    cd ..
    
    # Install
    echo "Installing wheel..."
    pip install target/wheels/*.whl --force-reinstall
    
    echo "✓ Installation complete"
else
    echo "✗ core directory not found"
    echo "Make sure you're in the project root directory"
    exit 1
fi

echo ""

# ============================================================================
# VERIFY INSTALLATION
# ============================================================================

echo "Verifying installation..."
echo ""

# Test import
python3 << 'EOF'
import sys

try:
    import stratarouter
    print(f"✓ StrataRouter v{stratarouter.__version__} imported successfully")
    
    # Test Rust core
    from stratarouter._core import PyRouter
    print("✓ Rust core imported successfully")
    
    # Test Router class
    from stratarouter import Router, Route
    print("✓ Router and Route classes available")
    
    print("\n✓ Installation verified!")
    
except ImportError as e:
    print(f"✗ Import failed: {e}")
    sys.exit(1)
EOF

echo ""

# ============================================================================
# QUICK COMMANDS FOR YOU
# ============================================================================

echo "============================================"
echo "Quick Install Commands"
echo "============================================"
echo ""
echo "# If you're in /home/opc/SR_router/StrataRouter:"
echo ""
echo "1. Go to project root:"
echo "   cd /home/opc/SR_router/StrataRouter"
echo ""
echo "2. Build and install:"
echo "   cd core"
echo "   maturin build --release --strip --out ../target/wheels"
echo "   cd .."
echo "   pip install target/wheels/*.whl --force-reinstall"
echo ""
echo "3. Verify:"
echo "   python -c 'import stratarouter; print(stratarouter.__version__)'"
echo ""
echo "4. Run example:"
echo "   cd examples"
echo "   python quickstart.py"
echo ""

# ============================================================================
# ALTERNATIVE: Development install (editable)
# ============================================================================

echo ""
echo "Alternative: Development Install (editable)"
echo "==========================================="
echo ""
echo "For active development, use:"
echo ""
echo "   cd core"
echo "   maturin develop --release"
echo ""
echo "This installs in editable mode, so changes"
echo "are reflected without reinstalling."
echo ""
