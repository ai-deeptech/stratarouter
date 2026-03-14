"""
StrataRouter — Accuracy Benchmark
===================================
Measures routing accuracy (correct route classification) on a labelled test set.

Usage
-----
    pip install stratarouter[huggingface]
    python benchmarks/accuracy_benchmark.py

Results are written to benchmarks/results/accuracy.json.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import List, Tuple

from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

ENCODER_MODEL = "sentence-transformers/all-MiniLM-L6-v2"

# (query, expected_route)
LABELLED_TEST_SET: List[Tuple[str, str]] = [
    # billing
    ("I need my April invoice",               "billing"),
    ("Can I get a refund?",                   "billing"),
    ("My card was charged twice",             "billing"),
    ("How do I update my payment method?",    "billing"),
    ("Cancel my subscription",               "billing"),
    # support
    ("The app keeps crashing on iOS",         "support"),
    ("I can't log into my account",           "support"),
    ("Getting a 500 error on checkout",       "support"),
    ("Nothing loads after I click submit",    "support"),
    ("Error: invalid token",                  "support"),
    # sales
    ("What does the enterprise plan cost?",   "sales"),
    ("Can I get a free trial?",               "sales"),
    ("I want to book a demo",                 "sales"),
    ("Do you offer annual discounts?",        "sales"),
    ("How many seats does the Pro plan have?","sales"),
    # technical
    ("How do I integrate with LangChain?",    "technical"),
    ("Show me a Python code example",         "technical"),
    ("Where is the API reference?",           "technical"),
    ("What SDK versions do you support?",     "technical"),
    ("How do I authenticate API calls?",      "technical"),
    # account
    ("I forgot my password",                  "account"),
    ("How do I enable two-factor auth?",      "account"),
    ("Change my email address",               "account"),
    ("Delete my account",                     "account"),
    ("Update my profile picture",             "account"),
]

ROUTE_DEFINITIONS = [
    ("billing",   ["invoice", "payment", "refund", "charge", "subscription"]),
    ("support",   ["help", "broken", "error", "not working", "can't login"]),
    ("sales",     ["pricing", "demo", "trial", "purchase", "enterprise plan"]),
    ("technical", ["API", "integration", "SDK", "documentation", "code sample"]),
    ("account",   ["password", "profile", "settings", "two-factor", "email change"]),
]


def build_router() -> RouteLayer:
    encoder = HuggingFaceEncoder(name=ENCODER_MODEL)
    routes = [Route(name=n, utterances=u) for n, u in ROUTE_DEFINITIONS]
    return RouteLayer(encoder=encoder, routes=routes)


def evaluate(rl: RouteLayer) -> dict:
    correct = 0
    errors = []
    for query, expected in LABELLED_TEST_SET:
        result = rl(query)
        predicted = result.name if result else None
        if predicted == expected:
            correct += 1
        else:
            errors.append({"query": query, "expected": expected, "predicted": predicted})

    total = len(LABELLED_TEST_SET)
    accuracy = correct / total
    return {
        "total": total,
        "correct": correct,
        "accuracy": round(accuracy, 4),
        "accuracy_pct": f"{accuracy * 100:.1f}%",
        "errors": errors,
    }


def main() -> None:
    print("Building router …")
    rl = build_router()

    print(f"Evaluating on {len(LABELLED_TEST_SET)} labelled queries …")
    results = evaluate(rl)

    print(f"\n{'─' * 50}")
    print(f"  Accuracy : {results['accuracy_pct']}")
    print(f"  Correct  : {results['correct']} / {results['total']}")
    if results["errors"]:
        print(f"\n  Misclassified ({len(results['errors'])}):")
        for e in results["errors"]:
            print(f"    [{e['expected']:10s}] ← '{e['query']}' → got '{e['predicted']}'")

    out_dir = Path(__file__).parent / "results"
    out_dir.mkdir(exist_ok=True)
    out_path = out_dir / "accuracy.json"
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nResults written to {out_path}")


if __name__ == "__main__":
    main()
