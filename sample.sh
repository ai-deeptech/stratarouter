#!/bin/bash
set -e

cd /home/opc/backup/stratarouter/stratarouter
python3.11 -m venv .

echo "Installing StrataRouter..."

# Install dependencies
#pip install -q numpy pydantic sentence-transformers

# Get site-packages location
SITE_PACKAGES=$(python -c "import site; print(site.getsitepackages()[0])")
echo "Site packages: $SITE_PACKAGES"

# Copy Python package
echo "Copying Python package..."
rm -rf "$SITE_PACKAGES/stratarouter"
cp -r python/stratarouter "$SITE_PACKAGES/"

# Create package metadata
cat > "$SITE_PACKAGES/stratarouter-0.1.0.dist-info/METADATA" << 'EOF'
Metadata-Version: 2.1
Name: stratarouter
Version: 0.1.0
Summary: High-performance semantic routing
Home-page: https://github.com/stratarouter/stratarouter
Author: StrataRouter Team
Author-email: team@stratarouter.dev
License: MIT
Classifier: Programming Language :: Python :: 3
Requires-Python: >=3.8
Description-Content-Type: text/markdown

StrataRouter - High-performance semantic routing
EOF

mkdir -p "$SITE_PACKAGES/stratarouter-0.1.0.dist-info"
cat > "$SITE_PACKAGES/stratarouter-0.1.0.dist-info/METADATA" << 'EOF'
Metadata-Version: 2.1
Name: stratarouter
Version: 0.1.0
EOF

echo ""
echo "Testing installation..."
python << 'PYTEST'
try:
    import stratarouter
    print(f"✓ stratarouter imported")
    
    from stratarouter import Route, RouteLayer
    print("✓ Classes available")
    
    from stratarouter.encoders import HuggingFaceEncoder
    print("✓ Encoders available")
    
    print("\n✨ Installation successful!")
except Exception as e:
    print(f"✗ Error: {e}")
    import traceback
    traceback.print_exc()
    exit(1)
PYTEST

echo ""
echo "======================================"
echo "Ready to use StrataRouter!"
echo ""
echo "Try: python examples/quickstart.py"
echo "======================================"
