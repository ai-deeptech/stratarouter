"""
#!/bin/bash
set -e

echo "Building StrataRouter release binaries..."

rm -rf target/wheels
rm -rf target/release

echo "Building for current platform..."
cd core
maturin build --release --strip --out ../target/wheels

echo ""
echo "Build complete! Wheels in target/wheels/"
ls -lh ../target/wheels/
"""