<div align="center">

# StrataRouter

### AI Execution Control Plane

**Production-grade semantic routing for AI systems.**
20× faster than semantic-router. Rust core. 9 framework integrations.

[![PyPI](https://img.shields.io/pypi/v/stratarouter)](https://pypi.org/project/stratarouter/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.txt)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://python.org)
[![Rust 1.70+](https://img.shields.io/badge/core-Rust%201.70+-orange.svg)](https://www.rust-lang.org/)

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
| P99 Latency | **8.7 ms** | 178 ms | **20× faster** |
| Memory (1K routes) | **64 MB** | 2.1 GB | **33× less** |
| Throughput | **18K req/s** | 450 req/s | **40× higher** |
| Accuracy | **95.4%** | 84.7% | **+12.7%** |

### vs LangChain Router

| | StrataRouter | LangChain Router |
|---|:---:|:---:|
| Routing engine | Rust (HNSW + AVX2 SIMD) | Python |
| Confidence calibration | Isotonic regression | None |
| Sparse scoring | BM25 | None |
| Self-hostable server | ✅ (Runtime) | ❌ |
| Workflow execution | ✅ (TCFP) | ❌ |

---

## 📦 Installation

```bash
pip install stratarouter
pip install stratarouter[huggingface]   # local embeddings, no API key
pip install stratarouter[openai]        # OpenAI embeddings
pip install stratarouter[all]           # everything
```

---

## 🚀 Quick Start

```python
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(
    id="billing",
    description="Billing and payment questions",
    examples=["Where's my invoice?", "I need a refund"],
    keywords=["invoice", "billing", "payment", "refund"]
))
router.add(Route(
    id="support",
    description="Technical support",
    examples=["App is crashing", "Can't login"],
    keywords=["crash", "bug", "error", "login"]
))

router.build_index()

result = router.route("I need my April invoice")
print(result.route_id)    # "billing"
print(result.confidence)  # 0.89  — isotonic-calibrated
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
│   │   ├── router.rs          ← Main router, hybrid scoring
│   │   ├── types.rs           ← Route, RouteResult, RouteScores
│   │   ├── ffi.rs             ← PyO3 Python bindings (PyRouter, PyRoute)
│   │   ├── algorithms/
│   │   │   ├── hybrid_scoring.rs  ← dense(0.6427) + BM25(0.2891) + rule(0.0682)
│   │   │   ├── calibration.rs     ← isotonic regression per route
│   │   │   └── simd_ops.rs        ← AVX2 SIMD cosine similarity
│   │   └── index/
│   │       └── hnsw.rs            ← O(log N) approximate nearest neighbour
│   ├── tests/                 ← 13 Rust integration test files
│   └── benches/               ← Criterion benchmarks
├── python/
│   └── stratarouter/
│       ├── router.py          ← Python Router: LOCAL | CLOUD mode
│       ├── types.py           ← Route (Pydantic), RouteResult, RouteScores
│       ├── encoders/          ← HuggingFace, OpenAI
│       ├── integrations/      ← 9 framework adapters
│       └── cloud/
│           └── client.py      ← Thread-safe httpx CloudClient
├── tests/                     ← pytest suite (7 files)
├── docs/                      ← API reference, guides, architecture
├── examples/                  ← quickstart, advanced_routing, langchain demo
└── integrations/              ← integration examples + benchmarks
```

Python ↔ Rust via [PyO3/Maturin](https://maturin.rs/) zero-copy bindings.
AVX2 SIMD path on x86_64, safe scalar fallback on ARM.

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
| [API Reference](docs/api-reference.md) | Router, Route, RouteResult |
| [Integrations](docs/integrations.md) | All 9 framework guides |
| [Architecture](docs/architecture.md) | Rust internals, HNSW, SIMD |
| [Deployment](docs/deployment.md) | Docker, K8s, server |
| [Migration v0.1 → v0.2](MIGRATION.md) | Breaking API changes |
| [Roadmap](ROADMAP.md) | What's coming |

---

## 🔬 Examples

```
examples/
├── quickstart.py          ← 5-minute working example
├── advanced_routing.py    ← thresholds, patterns, metadata
├── langchain_rag_demo.py  ← LangChain RAG pipeline routing

integrations/
├── langchain_example.py
├── crewai_example.py
├── autogen_example.py
├── langgraph_example.py
└── benchmarks/
    └── performance_comparison.py
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
| Idempotency + dedup | — | — | ✅ |

→ **[Runtime](https://github.com/ai-deeptech/stratarouter-runtime)**
→ **Enterprise:** [support@inteleion.com](mailto:support@inteleion.com) · [inteleion.com](https://inteleion.com)
→ **Docs:** [docs.stratarouter.com](https://docs.stratarouter.com)

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). PRs welcome for:
- New encoders (Cohere, Mistral, Gemini, Ollama)
- Additional framework integrations
- Benchmark improvements
- Documentation and examples

---

## 📝 License

MIT — [LICENSE.txt](LICENSE.txt)
Built with [PyO3](https://pyo3.rs/) · [Sentence Transformers](https://sbert.net/)
Made with ⚡ by [InteleionAI](https://inteleion.com) · [stratarouter.com](https://stratarouter.com)
