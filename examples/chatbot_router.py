"""
Chatbot Router — route user messages to specialized agents by intent.

Run:
    python examples/chatbot_router.py
"""
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    id="billing",
    description="Billing, invoices, and payment questions",
    examples=["Where's my invoice?", "I was charged twice", "Need a refund"],
    keywords=["invoice", "bill", "charge", "refund", "payment"]
))
router.add(Route(
    id="support",
    description="Technical support and troubleshooting",
    examples=["App is crashing", "Can't login", "Getting an error"],
    keywords=["crash", "bug", "error", "broken", "login"]
))
router.add(Route(
    id="sales",
    description="Pricing, plans, and upgrades",
    examples=["What's the pricing?", "I want to upgrade", "Enterprise plan"],
    keywords=["price", "cost", "plan", "upgrade", "enterprise"]
))
router.add(Route(
    id="general",
    description="General assistance",
    examples=["Hello", "Help", "Thanks", "How does this work?"]
))

router.build_index()

test_messages = [
    "I need my April invoice",
    "The app keeps crashing on iOS",
    "What does the enterprise plan include?",
    "Hi, how can I get started?",
]

print("Chatbot Router — routing user messages\n")
for msg in test_messages:
    r = router.route(msg)
    print(f"  [{r.route_id:10s}]  {r.confidence:.0%}  '{msg}'")
