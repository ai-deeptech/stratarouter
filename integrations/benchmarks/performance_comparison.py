"""Performance benchmark for StrataRouter"""

import time
import numpy as np
from stratarouter import Router, Route


def benchmark_stratarouter(queries, num_runs=100):
    """Benchmark StrataRouter"""
    print("Benchmarking StrataRouter...")
    
    router = Router()
    router.add(Route(id="test1", keywords=["invoice"]))
    router.add(Route(id="test2", keywords=["crash"]))
    router.build_index()
    
    latencies = []
    for _ in range(num_runs):
        for query in queries:
            start = time.perf_counter()
            router.route(query)
            latency = (time.perf_counter() - start) * 1000
            latencies.append(latency)
    
    return latencies


def main():
    print("StrataRouter Performance Benchmark\n")
    
    queries = [
        "Where's my invoice?",
        "App is crashing",
        "I need support",
        "Payment failed"
    ]
    
    print(f"Running {len(queries)} queries, 100 iterations each\n")
    
    latencies = benchmark_stratarouter(queries)
    
    p50 = np.percentile(latencies, 50)
    p95 = np.percentile(latencies, 95)
    p99 = np.percentile(latencies, 99)
    mean = np.mean(latencies)
    
    print("Results:")
    print(f"  Mean latency: {mean:.2f}ms")
    print(f"  P50 latency: {p50:.2f}ms")
    print(f"  P95 latency: {p95:.2f}ms")
    print(f"  P99 latency: {p99:.2f}ms")
    print(f"  Throughput: {1000/mean:.0f} req/sec")


if __name__ == "__main__":
    main()