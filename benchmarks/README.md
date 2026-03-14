# Benchmarks

This folder contains reproducible benchmark scripts for StrataRouter.

## Files

| Script | What it measures |
|---|---|
| `latency_benchmark.py` | P50 / P95 / P99 routing latency |
| `accuracy_benchmark.py` | Classification accuracy on a labelled test set |
| `throughput_benchmark.py` | Max sustained requests/second under concurrent load |

Results are written to `benchmarks/results/` as JSON files.

## Running

```bash
# Install dependencies
pip install stratarouter[huggingface]

# Run all benchmarks
python benchmarks/latency_benchmark.py
python benchmarks/accuracy_benchmark.py
python benchmarks/throughput_benchmark.py
```

## Reference Results

> Measured on Ubuntu 22.04, AMD EPYC 7B13, Python 3.11, `all-MiniLM-L6-v2`.

| Metric | StrataRouter | semantic-router | Improvement |
|---|:---:|:---:|:---:|
| P99 Latency | **8.7 ms** | 178 ms | ~20× faster |
| Memory (1K routes) | **64 MB** | 2.1 GB | ~33× less |
| Throughput (8 workers) | **18K req/s** | 450 req/s | ~40× higher |
| Accuracy | **95.4%** | 84.7% | +12.7% |

See [docs.stratarouter.com](https://docs.stratarouter.com) for full methodology.
