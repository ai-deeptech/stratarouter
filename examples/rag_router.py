"""
RAG Router — route queries to the right RAG pipeline by topic.

Run:
    python examples/rag_router.py
"""
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    id="legal_rag",
    description="Legal, compliance, and regulatory questions",
    examples=["GDPR requirements", "contract terms", "compliance policy"],
    keywords=["legal", "law", "contract", "compliance", "gdpr", "regulation"]
))
router.add(Route(
    id="technical_rag",
    description="Technical documentation and API questions",
    examples=["How do I use the API?", "SDK documentation", "Integration guide"],
    keywords=["api", "docs", "technical", "sdk", "integration", "code"]
))
router.add(Route(
    id="product_rag",
    description="Product features, pricing, and getting started",
    examples=["How does feature X work?", "What does the Pro plan include?"],
    keywords=["feature", "product", "pricing", "plan", "getting started"]
))

router.build_index()

test_queries = [
    "What are the GDPR data retention requirements?",
    "How do I authenticate with the REST API?",
    "What features does the Enterprise plan include?",
]

print("RAG Router — selecting retrieval pipeline by intent\n")
for q in test_queries:
    r = router.route(q)
    print(f"  [{r.route_id:15s}]  {r.confidence:.0%}  '{q}'")
    print(f"                    scores: semantic={r.scores['semantic']:.2f}  "
          f"keyword={r.scores['keyword']:.2f}")
