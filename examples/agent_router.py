"""
Agent Router — assign tasks to specialized agents in a multi-agent workflow.

Run:
    python examples/agent_router.py
"""
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    id="researcher",
    description="Research, web search, information gathering",
    examples=["Search for information about", "Find me the latest", "Research the topic"],
    keywords=["search", "find", "research", "lookup", "gather"]
))
router.add(Route(
    id="coder",
    description="Writing and fixing code",
    examples=["Write a Python function", "Fix this bug", "Implement a feature"],
    keywords=["code", "function", "bug", "implement", "script", "class"]
))
router.add(Route(
    id="analyst",
    description="Data analysis and evaluation",
    examples=["Analyze this dataset", "Compare these metrics", "Evaluate performance"],
    keywords=["analyze", "metrics", "data", "compare", "evaluate"]
))
router.add(Route(
    id="writer",
    description="Writing reports, emails, documentation",
    examples=["Write a report on", "Draft an email to", "Summarize this document"],
    keywords=["write", "draft", "document", "summarize", "report"]
))

router.build_index()

test_tasks = [
    "Research the latest LLM benchmark results",
    "Write a Python function to parse JSON with error handling",
    "Analyze Q1 2026 revenue metrics",
    "Draft an email to investors about our progress",
]

print("Agent Router — assigning tasks to specialized agents\n")
for task in test_tasks:
    r = router.route(task)
    print(f"  [{r.route_id:12s}]  {r.confidence:.0%}  '{task}'")
