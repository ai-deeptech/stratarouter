#!/bin/bash
set -e
echo "🧪 Running StrataRouter Test Suite"
echo "==================================="
echo ""
echo "=== Rust Tests ==="
cd ../core
cargo test --release
cargo clippy -- -D warnings
cargo fmt -- --check
cd ..
echo ""
echo "=== Python Tests ==="
pytest tests/ -v --cov=stratarouter
echo ""
echo "=== Code Quality ==="
black --check python/
ruff check python/
echo ""
echo "==================================="
echo "✅ All tests passed!"
