<div align="center">

<img src="docs/assets/logo.png" alt="StrataRouter" width="120" />

# StrataRouter

### AI Execution Control Plane

**Production-grade semantic routing for AI systems.**
Fast Rust core. Hybrid scoring. 9 framework integrations.

[![PyPI](https://img.shields.io/pypi/v/stratarouter)](https://pypi.org/project/stratarouter/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.txt)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://python.org)
[![Rust 1.70+](https://img.shields.io/badge/core-Rust%201.70+-orange.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/ai-deeptech/stratarouter/actions/workflows/ci.yml/badge.svg)](https://github.com/ai-deeptech/stratarouter/actions/workflows/ci.yml)

</div>

---

## What is StrataRouter?

StrataRouter is an **AI Execution Control Plane** — infrastructure-level routing for
production AI systems.

While LangChain and LlamaIndex provide prompt-level routing helpers,
StrataRouter provides **deterministic execution, cost-aware model selection,
governance, and compliance** at the system level.

```
Your Query
    ↓
┌──────────────────────────────────────────────────┐
│  StrataRouter Core   (this repo, MIT)            │
│  Intent detection · Semantic matching            │
│  Hybrid scoring · Confidence calibration         │
└─────────────────────┬────────────────────────────┘
                      ↓
┌──────────────────────────────────────────────────┐
│  StrataRouter Runtime   (Apache 2.0)             │
│  TCFP execution · Semantic cache                 │
│  Batch dedup · REST API · Prometheus             │
└─────────────────────┬────────────────────────────┘
                      ↓
┌──────────────────────────────────────────────────┐
│  StrataRouter Enterprise   (commercial)          │
│  Consensus · Audit log · Policy · Multi-tenant   │
└──────────────────────────────────────────────────┘
```

---

## ⚡ Benchmarks

### vs semantic-router

| Metric | StrataRouter | semantic-router | Delta |
|--------|:---:|:---:|:---:|
| P99 Latency | **8.7 ms** | 178 ms | **~20× faster** |
| Memory (1K routes) | **64 MB** | 2.1 GB | **~33× less** |
| Throughput | **18K req/s** | 450 req/s | **~40× higher** |
| Accuracy | **95.4%** | 84.7% | **+12.7%** |

> Benchmarks run on Ubuntu 22.04, AMD EPYC 7B13, Python 3.11, sentence-transformers/all-MiniLM-L6-v2.
> See [`integrations/benchmarks/`](integrations/benchmarks/) for methodology and reproduction scripts.

### vs LangChain Router

| | StrataRouter | LangChain Router |
|---|:---:|:---:|
| Routing engine | Rust (hybrid scoring) | Python |
| Confidence calibration | Piecewise-linear normalisation | None |
| Sparse scoring | BM25 keyword matching | None |
| Self-hostable server | ✅ (Runtime) | ❌ |
| Workflow execution | ✅ (TCFP) | ❌ |

---

## 📦 Installation

```bash
pip install stratarouter
pip install stratarouter[huggingface]   # local embeddings, no API key
pip install stratarouter[openai]        # OpenAI embeddings
pip install stratarouter[cohere]        # Cohere embeddings
pip install stratarouter[all]           # everything
```

---

## 🚀 Quick Start

### High-level API — `RouteLayer` (recommended)

```python
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

# Define routes
routes = [
    Route(
        name="billing",
        utterances=["invoice", "payment", "refund", "charge"],
        threshold=0.75,
    ),
    Route(
        name="support",
        utterances=["help", "broken", "error", "can't login"],
        threshold=0.75,
    ),
]

encoder = HuggingFaceEncoder()
rl = RouteLayer(encoder=encoder, routes=routes)

result = rl("I need my April invoice")
print(result.name)       # "billing"
print(result.score)      # 0.87
print(bool(result))      # True — score >= threshold
```

### Low-level API — `Router` (Rust core, advanced use)

```python
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    name="billing",
    utterances=["Where's my invoice?", "I need a refund"],
))
router.add(Route(
    name="support",
    utterances=["App is crashing", "Can't login"],
))

router.build_index()

result = router.route("I need my April invoice")
print(result.route_id)    # "billing"
print(result.confidence)  # 0.89 — calibrated score
print(result.latency_ms)  # 2.3ms

# Save and reload — no re-indexing needed
router.save("my_router.json")
router = Router.load("my_router.json")
```

---

## 🎨 Framework Integrations (9)

```python
from stratarouter.integrations.langchain         import StrataRouterChain
from stratarouter.integrations.langgraph         import create_routing_graph
from stratarouter.integrations.crewai            import RoutedAgent
from stratarouter.integrations.autogen           import StrataRouterGroupChat
from stratarouter.integrations.openai_assistants import StrataRouterAssistant
from stratarouter.integrations.google_agent      import StrataRouterVertexAI
```

Runnable integration examples → [`integrations/`](integrations/)

---

## 🏗️ Architecture

```
stratarouter/
├── core/
│   ├── src/
│   │   ├── router.rs              ← Main router, hybrid scoring pipeline
│   │   ├── types.rs               ← Route, RouteResult, RouteScores
│   │   ├── cache.rs               ← LRU embedding cache
│   │   ├── ffi.rs                 ← PyO3 Python bindings (PyRouter, PyRoute)
│   │   ├── algorithms/
│   │   │   ├── hybrid_scoring.rs  ← dense(0.6427) + BM25(0.2891) + rule(0.0682)
│   │   │   ├── calibration.rs     ← piecewise-linear score normalisation per route
│   │   │   └── vector_ops.rs      ← cosine similarity (scalar; SIMD planned)
│   │   └── index/
│   │       └── hnsw.rs            ← Linear scan O(N); graph-based HNSW planned
│   ├── tests/                     ← Rust integration tests
│   └── benches/                   ← Criterion benchmarks
├── python/
│   └── stratarouter/
│       ├── layer.py               ← RouteLayer: high-level API (pure Python)
│       ├── router.py              ← Router: low-level API (Rust core)
│       ├── route.py               ← Route + RouteChoice (public data classes)
│       ├── types.py               ← RouteConfig, RouteResult (internal)
│       ├── encoders/              ← HuggingFace, OpenAI, Cohere
│       ├── integrations/          ← 9 framework adapters
│       └── cloud/
│           └── client.py          ← Thread-safe httpx CloudClient
├── tests/                         ← pytest suite
├── examples/                      ← quickstart, advanced_routing, langchain demo
└── integrations/                  ← integration examples + benchmarks
```

Python ↔ Rust via [PyO3/Maturin](https://maturin.rs/) zero-copy bindings.

---

## 🗺️ Roadmap

| Version | Feature | Status |
|---------|---------|--------|
| v0.1 | Prototype — RouteLayer API | ✅ Nov 2025 |
| v0.2 | Production Rust engine, 9 integrations | ✅ **Mar 2026** |
| v0.3 | Runtime — TCFP, REST API, cache | ✅ Available |
| v0.4 | Cost optimizer — model selection, budgets | 🔄 In Progress |
| v0.5 | Enterprise governance | ✅ Available (private) |
| v0.6 | JS / Go SDKs | 📋 Q3 2026 |
| v1.0 | StrataRouter Cloud | 📋 Q4 2026 |

Full roadmap → [ROADMAP.md](ROADMAP.md)

---

## 📚 Documentation

| | |
|---|---|
| [Getting Started](docs/getting-started.md) | Install + first router in 5 min |
| [API Reference](docs/api-reference.md) | RouteLayer, Router, Route, RouteChoice |
| [Integrations](docs/integrations.md) | All 9 framework guides |
| [Architecture](docs/architecture.md) | Rust internals, scoring, calibration |
| [Deployment](docs/deployment.md) | Docker, K8s, server |
| [Changelog](CHANGELOG.md) | Release history |
| [Roadmap](ROADMAP.md) | What's coming |

---

## 🏗️ Development

```bash
# Clone repository
git clone https://github.com/ai-deeptech/stratarouter.git
cd stratarouter

# Install development dependencies
pip install -e "python/.[dev]"

# Build Rust core
cd core && cargo build --release && cd ..

# Install Python package (with Rust core)
cd python && maturin develop --release && cd ..

# Run all tests
make test
```

---

## 🏢 Platform

| | Core (MIT) | Runtime (Apache 2.0) | Enterprise |
|---|:---:|:---:|:---:|
| Semantic routing library | ✅ | ✅ | ✅ |
| 9 framework integrations | ✅ | ✅ | ✅ |
| PyPI package | ✅ | — | — |
| TCFP workflow execution | — | ✅ | ✅ |
| REST API + Prometheus | — | ✅ | ✅ |
| Semantic cache + batch dedup | — | ✅ | ✅ |
| Multi-agent consensus | — | — | ✅ |
| Immutable audit log (SOC2/HIPAA) | — | — | ✅ |
| Policy engine (RBAC/ABAC) | — | — | ✅ |
| Multi-tenant isolation | — | — | ✅ |

→ **[Runtime](https://github.com/ai-deeptech/stratarouter-runtime)**  
→ **Enterprise:** [hello@stratarouter.dev](mailto:hello@stratarouter.dev)  
→ **Docs:** [docs.stratarouter.com](https://docs.stratarouter.com)

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). PRs welcome for:
- New encoders (Mistral, Gemini, Ollama)
- Additional framework integrations
- Benchmark improvements
- Documentation and examples

---

## 📝 License

MIT — [LICENSE.txt](LICENSE.txt)  
Built with [PyO3](https://pyo3.rs/) · [Sentence Transformers](https://sbert.net/)  
Made with ⚡ by [StrataRouter Contributors](https://github.com/ai-deeptech/stratarouter/graphs/contributors)
