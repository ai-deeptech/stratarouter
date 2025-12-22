"""Quickstart example for StrataRouter"""

from stratarouter_core.stratarouter_core import PyRouter, PyRoute
import numpy as np

def main():
    # Create router with default config
    router = PyRouter(dimension=384, threshold=0.3)
    
    # Add routes
    billing_route = PyRoute("billing")
    billing_route.description = "Billing and payment queries"
    billing_route.examples = [
        "I need my invoice",
        "Where is my bill?",
        "Payment issue"
    ]
    billing_route.keywords = ["invoice", "bill", "payment", "charge"]
    
    support_route = PyRoute("support")
    support_route.description = "Technical support"
    support_route.examples = [
        "App is crashing",
        "Can't login",
        "Error message"
    ]
    support_route.keywords = ["error", "crash", "bug", "issue"]
    
    # Add routes to router
    router.add_route(billing_route)
    router.add_route(support_route)
    
    # Build index with dummy embeddings (in production, use real embeddings)
    embeddings = [
        np.random.randn(384).tolist(),
        np.random.randn(384).tolist()
    ]
    router.build_index(embeddings)
    
    # Route a query
    query_embedding = np.random.randn(384).tolist()
    result = router.route("I need my invoice for last month", query_embedding)
    
    print(f"✓ Routed to: {result['route_id']}")
    print(f"✓ Confidence: {result['confidence']:.3f}")
    print(f"✓ Scores: {result['scores']}")
    print(f"✓ Latency: {result['latency_us']}μs")

if __name__ == "__main__":
    main()
