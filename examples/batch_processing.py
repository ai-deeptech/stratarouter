"""
Example: Batch processing with StrataRouter

Demonstrates efficient batch routing for high-throughput scenarios.
"""

import time
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder


def main():
    print("StrataRouter Batch Processing Example\n")
    
    # Setup routes
    routes = [
        Route(name="billing", utterances=["invoice", "payment", "refund"]),
        Route(name="support", utterances=["help", "issue", "problem"]),
        Route(name="sales", utterances=["buy", "purchase", "pricing"]),
    ]
    
    encoder = HuggingFaceEncoder()
    router = RouteLayer(encoder=encoder, routes=routes)
    
    print(f"Initialized router with {router.num_routes} routes\n")
    
    # Generate test queries
    test_queries = [
        "I need my invoice",
        "How do I contact support?",
        "What are your prices?",
        "Refund my payment",
        "Technical issue here",
        "I want to buy something",
    ] * 20  # 120 queries total
    
    print(f"Processing {len(test_queries)} queries...\n")
    
    # Method 1: One by one (slower)
    print("Method 1: One by one")
    start = time.time()
    for query in test_queries:
        result = router(query)
    time_sequential = time.time() - start
    print(f"  Time: {time_sequential:.3f}s")
    print(f"  Throughput: {len(test_queries)/time_sequential:.0f} queries/sec\n")
    
    # Method 2: Batch processing (faster)
    print("Method 2: Batch processing")
    start = time.time()
    results = router.route_batch(test_queries)
    time_batch = time.time() - start
    print(f"  Time: {time_batch:.3f}s")
    print(f"  Throughput: {len(test_queries)/time_batch:.0f} queries/sec\n")
    
    # Show speedup
    speedup = time_sequential / time_batch
    print(f"Speedup: {speedup:.1f}x faster with batch processing!\n")
    
    # Show sample results
    print("Sample results:")
    for i in range(min(5, len(results))):
        query = test_queries[i]
        result = results[i]
        print(f"  '{query[:40]}...' → {result.name} ({result.score:.3f})")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\n❌ Error: {e}")
