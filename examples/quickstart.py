"""
StrataRouter Quick Start Example

This example shows the basics of using StrataRouter.
"""

from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder


def main():
    print("StrataRouter Quick Start\n")
    
    # 1. Define your routes
    print("1. Defining routes...")
    routes = [
        Route(
            name="billing",
            utterances=[
                "I need my invoice",
                "Where is my receipt?",
                "Payment question",
                "Refund request",
                "Billing issue"
            ],
            threshold=0.75
        ),
        Route(
            name="support",
            utterances=[
                "I need help",
                "Technical issue",
                "How do I use this?",
                "Support question",
                "Something is broken"
            ],
            threshold=0.75
        ),
        Route(
            name="chitchat",
            utterances=[
                "Hello",
                "Hi there",
                "How are you?",
                "Good morning",
                "Nice to meet you"
            ],
            threshold=0.75
        )
    ]
    print(f"   Created {len(routes)} routes\n")
    
    # 2. Create encoder (local, fast, free!)
    print("2. Loading encoder...")
    encoder = HuggingFaceEncoder(model="all-MiniLM-L6-v2")
    print(f"   Loaded {encoder}\n")
    
    # 3. Create router
    print("3. Creating router...")
    rl = RouteLayer(encoder=encoder, routes=routes)
    print(f"   {rl}\n")
    
    # 4. Route some queries
    print("4. Routing queries...\n")
    
    test_queries = [
        "I need my invoice from last month",
        "Hey, how's it going?",
        "My app is not working",
        "Good morning!",
        "Where can I find my receipt?"
    ]
    
    for query in test_queries:
        result = rl(query)
        
        if result.is_match:
            print(f"   Query: '{query}'")
            print(f"   → Route: {result.name}")
            print(f"   → Score: {result.score:.3f}")
            print(f"   → Match: ✓\n")
        else:
            print(f"   Query: '{query}'")
            print(f"   → No match found\n")
    
    # 5. Batch processing
    print("5. Batch processing...\n")
    
    batch_queries = [
        "I want a refund",
        "Hello there!",
        "Help me fix this bug"
    ]
    
    results = rl.route_batch(batch_queries)
    
    for query, result in zip(batch_queries, results):
        print(f"   '{query}' → {result.name} ({result.score:.3f})")
    
    print("\n✨ Done! StrataRouter is 10x faster than semantic-router!")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\n❌ Error: {e}")
        print("\nMake sure you have installed stratarouter with:")
        print("  pip install stratarouter[huggingface]")
