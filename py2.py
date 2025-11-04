#!/bin/bash
# Automatic Python setup for StrataRouter

set -e

echo "Setting up Python for StrataRouter..."

# Try to find Python 3.8+
for version in 3.11 3.10 3.9 3.8; do
    if command -v python$version &> /dev/null; then
        PYTHON=python$version
        echo "✓ Found $PYTHON"
        break
    fi
done

if [ -z "$PYTHON" ]; then
    echo "❌ No Python 3.8+ found"
    echo ""
    echo "Installing Python 3.11..."
    sudo yum install -y python3.11 python3.11-devel
    PYTHON=python3.11
fi

# Set environment
export PYO3_PYTHON=$PYTHON
echo "export PYO3_PYTHON=$PYTHON" >> ~/.bashrc

# Install pip packages
$PYTHON -m pip install --user --upgrade pip
$PYTHON -m pip install --user maturin numpy pydantic sentence-transformers

echo ""
echo "✓ Setup complete!"
echo "  Python: $PYTHON ($($PYTHON --version))"
echo ""
echo "Run: source ~/.bashrc"
echo "Then: sh scripts/build.sh"
