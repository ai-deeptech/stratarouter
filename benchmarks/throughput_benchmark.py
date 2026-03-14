"""
StrataRouter — Throughput Benchmark
=====================================
Measures maximum sustained routing throughput (requests/second) using
concurrent threads — simulates production load.

Usage
-----
    pip install stratarouter[huggingface]
    python benchmarks/throughput_benchmark.py

Results are written to benchmarks/results/throughput.json.
"""

from __future__ import annotations

import json
import threading
import time
from pathlib import Path
from typing import List

from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

ENCODER_MODEL = "sentence-transformers/all-MiniLM-L6-v2"
CONCURRENCY_LEVELS = [1, 2, 4, 8]
DURATION_SECS = 5  # duration per concurrency level
WARMUP_SECS = 2

ROUTE_DEFINITIONS = [
    ("billing",   ["invoice", "payment", "refund", "charge", "subscription"]),
    ("support",   ["help", "broken", "error", "not working", "can't login"]),
    ("sales",     ["pricing", "demo", "trial", "purchase", "enterprise plan"]),
    ("technical", ["API", "integration", "SDK", "documentation", "code sample"]),
    ("account",   ["password", "profile", "settings", "two-factor", "email change"]),
]

QUERIES = [
    "I need my April invoice",
    "The app keeps crashing",
    "What does enterprise cost?",
    "How do I call the API?",
    "I forgot my password",
]


def build_router() -> RouteLayer:
    encoder = HuggingFaceEncoder(name=ENCODER_MODEL)
    routes = [Route(name=n, utterances=u) for n, u in ROUTE_DEFINITIONS]
    return RouteLayer(encoder=encoder, routes=routes)


def worker(rl: RouteLayer, counter: list, stop_event: threading.Event, idx: int) -> None:
    count = 0
    while not stop_event.is_set():
        rl(QUERIES[count % len(QUERIES)])
        count += 1
    counter[idx] = count


def benchmark_concurrency(rl: RouteLayer, workers: int) -> dict:
    # Warmup
    stop = threading.Event()
    counts: List[int] = [0] * workers
    threads = [threading.Thread(target=worker, args=(rl, counts, stop, i)) for i in range(workers)]
    for t in threads:
        t.start()
    time.sleep(WARMUP_SECS)
    stop.set()
    for t in threads:
        t.join()

    # Actual measurement
    stop = threading.Event()
    counts = [0] * workers
    threads = [threading.Thread(target=worker, args=(rl, counts, stop, i)) for i in range(workers)]
    t_start = time.perf_counter()
    for t in threads:
        t.start()
    time.sleep(DURATION_SECS)
    stop.set()
    for t in threads:
        t.join()
    elapsed = time.perf_counter() - t_start

    total_requests = sum(counts)
    rps = total_requests / elapsed
    return {
        "workers": workers,
        "total_requests": total_requests,
        "elapsed_secs": round(elapsed, 2),
        "rps": round(rps, 1),
    }


def main() -> None:
    print("Building router …")
    rl = build_router()

    results = []
    print(f"\n{'Workers':>8}  {'RPS':>10}  {'Total Reqs':>12}")
    print("─" * 36)
    for n in CONCURRENCY_LEVELS:
        r = benchmark_concurrency(rl, n)
        results.append(r)
        print(f"{r['workers']:>8}  {r['rps']:>10,.0f}  {r['total_requests']:>12,}")

    out_dir = Path(__file__).parent / "results"
    out_dir.mkdir(exist_ok=True)
    out_path = out_dir / "throughput.json"
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nResults written to {out_path}")


if __name__ == "__main__":
    main()
