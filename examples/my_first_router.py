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
