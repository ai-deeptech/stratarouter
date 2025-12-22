"""
Tool Router — select the right tool or API based on query intent.

Run:
    python examples/tool_router.py
"""
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    id="calculator",
    description="Mathematical calculations",
    examples=["Calculate the total", "What is 15% of 200?", "Compute the sum"],
    patterns=["calculate", "what is", "how much", "how many"],
    keywords=["calculate", "compute", "total", "sum", "percentage"]
))
router.add(Route(
    id="web_search",
    description="Web search for current information",
    examples=["Latest news on AI", "Search for Python tutorials", "Current weather"],
    keywords=["search", "news", "latest", "current", "recent", "find online"]
))
router.add(Route(
    id="database",
    description="Database queries and data retrieval",
    examples=["Get all orders from last week", "Find customer records", "Query the users"],
    keywords=["database", "query", "records", "sql", "find", "orders"]
))
router.add(Route(
    id="calendar",
    description="Calendar and scheduling",
    examples=["Schedule a meeting", "Check my availability", "Book a slot next week"],
    keywords=["calendar", "schedule", "meeting", "availability", "book", "appointment"]
))

router.build_index()

test_queries = [
    "What's the latest news on AI?",
    "Get all orders from last week",
    "What is 20% of 450?",
    "Schedule a team meeting for Friday",
]

print("Tool Router — selecting tools by intent\n")
for q in test_queries:
    r = router.route(q)
    print(f"  [{r.route_id:12s}]  {r.confidence:.0%}  '{q}'")
