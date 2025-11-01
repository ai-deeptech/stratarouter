"""
#!/bin/bash
set -e

echo "Publishing StrataRouter to PyPI..."

if [ ! -d "target/wheels" ] || [ -z "$(ls -A target/wheels/*.whl 2>/dev/null)" ]; then
    echo "❌ No wheels found. Run ./scripts/build_release.sh first"
    exit 1
fi

echo "Verifying symbol stripping..."
./scripts/verify_symbols.sh

if ! command -v twine &> /dev/null; then
    echo "Installing twine..."
    pip install twine
fi

echo ""
echo "About to publish these wheels:"
ls -lh target/wheels/*.whl
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted"
    exit 1
fi

echo "Publishing to PyPI..."
twine upload target/wheels/*.whl

echo ""
echo "✅ Published successfully!"
"""
