#!/bin/bash
# ============================================================================
# STRATAROUTER - WHAT TO DO NEXT
# Complete guide after successful setup
# ============================================================================

# ============================================================================
# STEP 1: Verify Installation
# ============================================================================

echo "Step 1: Verify StrataRouter is installed"
echo "========================================="

# Check if StrataRouter is importable
python -c "import stratarouter; print(f'✓ StrataRouter v{stratarouter.__version__} installed')"

# Check if Rust core is working
python -c "from stratarouter._core import PyRouter; print('✓ Rust core working')"

echo ""

# ============================================================================
# STEP 2: Run Quick Test
# ============================================================================

echo "Step 2: Run a quick test"
echo "========================"

# Create a simple test script
cat > test_quick.py << 'EOF'
from stratarouter import Router, Route

# Create router
router = Router()

# Add a test route
route = Route(
    id="test",
    description="Test route",
    examples=["hello world"],
    keywords=["test", "hello"]
)

router.add(route)
print("✓ Route added successfully")

# Build index would require an encoder
# For now, just verify the route was added
print(f"✓ Router has {len(router.routes)} route(s)")
print("✓ Quick test passed!")
EOF

python test_quick.py
rm test_quick.py

echo ""

# ============================================================================
# STEP 3: Choose Your Next Action
# ============================================================================

echo "Step 3: Choose what to do next"
echo "==============================="
echo ""
echo "Option A: Run the Quickstart Example"
echo "--------------------------------------"
echo "  cd examples"
echo "  python quickstart.py"
echo ""
echo "Option B: Run the Test Suite"
echo "-----------------------------"
echo "  ./scripts/test_all.sh"
echo ""
echo "Option C: Try Advanced Example"
echo "------------------------------"
echo "  cd examples"
echo "  python advanced_routing.py"
echo ""
echo "Option D: Build for Production"
echo "-------------------------------"
echo "  ./scripts/build_release.sh"
echo ""
echo "Option E: Start Developing"
echo "--------------------------"
echo "  # Edit code in core/ or python/"
echo "  # Then rebuild with:"
echo "  cd core && maturin develop --release"
echo ""

# ============================================================================
# RECOMMENDED: Run Examples
# ============================================================================

echo ""
echo "RECOMMENDED NEXT STEPS:"
echo "======================="
echo ""
echo "1. Install encoder (required for routing):"
echo "   pip install sentence-transformers torch"
echo ""
echo "2. Run quickstart example:"
echo "   python examples/quickstart.py"
echo ""
echo "3. Run tests to verify everything:"
echo "   pytest tests/ -v"
echo ""
echo "4. Try integration with your framework:"
echo "   - LangChain: examples/integrations/langchain_example.py"
echo "   - CrewAI: examples/integrations/crewai_example.py"
echo "   - etc."
echo ""

# ============================================================================
# DEVELOPMENT WORKFLOW
# ============================================================================

echo ""
echo "DEVELOPMENT WORKFLOW:"
echo "====================="
echo ""
echo "When making changes:"
echo ""
echo "1. Edit Rust code (core/src/*.rs)"
echo "   cd core"
echo "   cargo fmt              # Format code"
echo "   cargo clippy           # Lint code"
echo "   cargo test             # Run tests"
echo "   maturin develop        # Rebuild Python bindings"
echo ""
echo "2. Edit Python code (python/stratarouter/*.py)"
echo "   black python/          # Format code"
echo "   ruff check python/     # Lint code"
echo "   pytest tests/          # Run tests"
echo ""
echo "3. Run complete test suite:"
echo "   ./scripts/test_all.sh"
echo ""

# ============================================================================
# QUICK COMMANDS REFERENCE
# ============================================================================

echo ""
echo "QUICK COMMANDS:"
echo "==============="
echo ""
echo "# Activate virtual environment"
echo "source venv/bin/activate"
echo ""
echo "# Run single Python test file"
echo "pytest tests/test_router.py -v"
echo ""
echo "# Run specific test"
echo "pytest tests/test_router.py::test_routing -v"
echo ""
echo "# Rebuild Rust core quickly"
echo "cd core && maturin develop --release && cd .."
echo ""
echo "# Check test coverage"
echo "pytest tests/ --cov=stratarouter --cov-report=html"
echo ""
echo "# Benchmark performance"
echo "cd core && cargo bench"
echo ""

# ============================================================================
# TROUBLESHOOTING
# ============================================================================

echo ""
echo "TROUBLESHOOTING:"
echo "================"
echo ""
echo "If you get 'encoder not found' error:"
echo "  pip install sentence-transformers torch"
echo ""
echo "If import fails:"
echo "  pip install --force-reinstall target/wheels/*.whl"
echo ""
echo "If Rust changes don't apply:"
echo "  cd core && cargo clean && maturin develop --release"
echo ""
echo "If tests fail:"
echo "  Check Python version: python --version (needs 3.8+)"
echo "  Check Rust version: rustc --version (needs 1.70+)"
echo ""

# ============================================================================
# CREATE A SIMPLE WORKING EXAMPLE
# ============================================================================

echo ""
echo "CREATING A SIMPLE EXAMPLE FOR YOU:"
echo "==================================="
echo ""

# Create a working example
cat > my_first_router.py << 'EOF'
"""
My First StrataRouter Example
This is a minimal working example to get you started.
"""

from stratarouter import Router, Route

def main():
    print("Creating your first router...\n")
    
    # Step 1: Create router
    # Note: This requires sentence-transformers
    # Install with: pip install sentence-transformers torch
    try:
        router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")
        print("✓ Router created")
    except ImportError:
        print("⚠ Encoder not installed. Install with:")
        print("  pip install sentence-transformers torch")
        return
    
    # Step 2: Define routes
    routes = [
        Route(
            id="greeting",
            description="Greetings and hellos",
            examples=["hello", "hi there", "good morning"],
            keywords=["hello", "hi", "hey", "greetings"]
        ),
        Route(
            id="farewell",
            description="Goodbyes",
            examples=["goodbye", "see you later", "bye"],
            keywords=["goodbye", "bye", "farewell", "later"]
        ),
    ]
    
    # Step 3: Add routes
    for route in routes:
        router.add(route)
    print(f"✓ Added {len(routes)} routes")
    
    # Step 4: Build index
    router.build_index()
    print("✓ Index built")
    
    # Step 5: Test routing
    print("\nTesting routing:\n")
    
    test_queries = [
        "Hello there!",
        "Goodbye friend",
        "Hey what's up",
        "See you tomorrow"
    ]
    
    for query in test_queries:
        result = router.route(query)
        print(f"Query: '{query}'")
        print(f"  → Route: {result.route_id}")
        print(f"  → Confidence: {result.confidence:.2f}")
        print()
    
    print("✓ Routing complete!")

if __name__ == "__main__":
    main()
EOF

echo "Created: my_first_router.py"
echo ""
echo "Run it with:"
echo "  python my_first_router.py"
echo ""

# ============================================================================
# FINAL SUMMARY
# ============================================================================

echo ""
echo "============================================"
echo "✓ StrataRouter is ready to use!"
echo "============================================"
echo ""
echo "You can now:"
echo "  1. Run: python my_first_router.py"
echo "  2. Explore: examples/quickstart.py"
echo "  3. Test: pytest tests/ -v"
echo "  4. Develop: Edit code and rebuild"
echo ""
echo "Documentation:"
echo "  - docs/getting-started.md"
echo "  - docs/api-reference.md"
echo ""
echo "Happy routing! 🚀"
echo "============================================"
