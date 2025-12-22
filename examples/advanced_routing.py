"""Advanced StrataRouter Example"""

from stratarouter_core import Router as PyRouter, Route as PyRoute


def handle_billing_query(query: str, result):
    """Handle billing queries"""
    print(f"\n[BILLING] Processing: {query}")
    print(f"  Confidence: {result.confidence:.2f}")
    return "Routing to billing department..."


def handle_support_query(query: str, result):
    """Handle support queries"""
    print(f"\n[SUPPORT] Processing: {query}")
    print(f"  Confidence: {result.confidence:.2f}")
    return "Creating support ticket..."


def handle_fallback(query: str, result):
    """Handle low confidence queries"""
    print(f"\n[FALLBACK] Low confidence ({result.confidence:.2f})")
    print(f"  Query: {query}")
    return "Routing to general support..."


def main():
    print("Advanced StrataRouter Example\n")
    
    # Create router with custom settings
    router = Router(
        encoder="sentence-transformers/all-MiniLM-L6-v2",
        threshold=0.6,
        dimension=384
    )
    
    # Define routes with custom thresholds
    routes = [
        Route(
            id="billing",
            description="Billing queries",
            examples=["invoice", "payment", "refund"],
            keywords=["invoice", "billing", "payment", "refund"],
            threshold=0.7
        ),
        Route(
            id="support",
            description="Technical support",
            examples=["crash", "error", "bug"],
            keywords=["crash", "bug", "error", "broken"],
            threshold=0.65
        )
    ]
    
    for route in routes:
        router.add(route)
    
    router.build_index()
    
    # Define handlers
    handlers = {
        "billing": handle_billing_query,
        "support": handle_support_query
    }
    
    # Test with various confidence levels
    test_queries = [
        ("I need my invoice", "high confidence"),
        ("App is not working", "medium confidence"),
        ("Hello there", "low confidence - should fallback")
    ]
    
    print("Testing routing with confidence thresholds:\n")
    for query, expected in test_queries:
        result = router.route(query)
        
        print(f"Query: {query} ({expected})")
        print(f"  Route: {result.route_id}")
        print(f"  Confidence: {result.confidence:.2f}")
        
        if result.confidence >= 0.6:
            handler = handlers.get(result.route_id)
            if handler:
                handler(query, result)
        else:
            handle_fallback(query, result)
        
        print()


if __name__ == "__main__":
    main()