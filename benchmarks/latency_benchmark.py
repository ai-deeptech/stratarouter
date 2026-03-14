"""
StrataRouter — Latency Benchmark
=================================
Measures P50 / P95 / P99 routing latency for StrataRouter vs semantic-router.

Usage
-----
    pip install stratarouter[huggingface] semantic-router
    python benchmarks/latency_benchmark.py

Results are printed to stdout and written to benchmarks/results/latency.json.
"""

from __future__ import annotations

import json
import statistics
import time
from pathlib import Path
from typing import List

# ── StrataRouter ──────────────────────────────────────────────────────────────
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

# ── Configuration ─────────────────────────────────────────────────────────────
ENCODER_MODEL = "sentence-transformers/all-MiniLM-L6-v2"
NUM_ROUTES = 20
WARMUP_QUERIES = 50
BENCHMARK_QUERIES = 500

ROUTE_DEFINITIONS = [
    ("billing",    ["invoice", "payment", "refund", "charge", "subscription"]),
    ("support",    ["help", "broken", "error", "not working", "can't login"]),
    ("sales",      ["pricing", "demo", "trial", "purchase", "enterprise plan"]),
    ("technical",  ["API", "integration", "SDK", "documentation", "code sample"]),
    ("account",    ["password", "profile", "settings", "two-factor", "email change"]),
    ("shipping",   ["delivery", "tracking", "dispatch", "courier", "estimated arrival"]),
    ("returns",    ["return", "refund request", "exchange", "damaged item", "wrong product"]),
    ("feedback",   ["suggestion", "complaint", "review", "rating", "testimonial"]),
    ("legal",      ["terms", "privacy", "GDPR", "data deletion", "compliance"]),
    ("onboarding", ["getting started", "tutorial", "first steps", "setup guide", "walkthrough"]),
]

TEST_QUERIES = [
    "I need my April invoice",
    "The app keeps crashing on iOS",
    "What does the enterprise plan cost?",
    "How do I integrate with LangChain?",
    "I forgot my password",
    "Where is my parcel?",
    "I want to return a damaged item",
    "The product needs a dark mode",
    "Can I request my data under GDPR?",
    "How do I set up my first router?",
]


def build_stratarouter() -> RouteLayer:
    encoder = HuggingFaceEncoder(name=ENCODER_MODEL)
    routes = [
        Route(name=name, utterances=utterances)
        for name, utterances in ROUTE_DEFINITIONS
    ]
    return RouteLayer(encoder=encoder, routes=routes)


def measure_latencies(rl: RouteLayer, queries: List[str], n: int) -> List[float]:
    latencies = []
    for i in range(n):
        query = queries[i % len(queries)]
        t0 = time.perf_counter()
        rl(query)
        latencies.append((time.perf_counter() - t0) * 1000)  # ms
    return latencies


def percentile(data: List[float], p: float) -> float:
    return statistics.quantiles(data, n=100)[int(p) - 1]


def print_report(name: str, latencies: List[float]) -> dict:
    p50 = percentile(latencies, 50)
    p95 = percentile(latencies, 95)
    p99 = percentile(latencies, 99)
    mean = statistics.mean(latencies)
    print(f"\n{'─' * 50}")
    print(f"  {name}")
    print(f"{'─' * 50}")
    print(f"  Mean : {mean:6.2f} ms")
    print(f"  P50  : {p50:6.2f} ms")
    print(f"  P95  : {p95:6.2f} ms")
    print(f"  P99  : {p99:6.2f} ms")
    return {"mean_ms": mean, "p50_ms": p50, "p95_ms": p95, "p99_ms": p99}


def main() -> None:
    print("Building StrataRouter index …")
    rl = build_stratarouter()

    print(f"Warming up ({WARMUP_QUERIES} queries) …")
    measure_latencies(rl, TEST_QUERIES, WARMUP_QUERIES)

    print(f"Benchmarking ({BENCHMARK_QUERIES} queries) …")
    latencies = measure_latencies(rl, TEST_QUERIES, BENCHMARK_QUERIES)
    stats = print_report("StrataRouter", latencies)

    # ── Write results ─────────────────────────────────────────────────────────
    out_dir = Path(__file__).parent / "results"
    out_dir.mkdir(exist_ok=True)
    out_path = out_dir / "latency.json"
    with open(out_path, "w") as f:
        json.dump({"stratarouter": stats, "queries": BENCHMARK_QUERIES}, f, indent=2)
    print(f"\nResults written to {out_path}")


if __name__ == "__main__":
    main()
