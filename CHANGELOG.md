# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

---

## [0.2.0] — 2026-03-07  ← Platform Relaunch

4 months of production engineering after the v0.1 prototype.
StrataRouter is now an **AI Execution Control Plane**, not just a routing library.

### Breaking Changes
- `RouteLayer(encoder, routes)` → `Router(encoder)` + `.add()` + `.build_index()`
- `Route(name=..., utterances=[...])` → `Route(id=..., examples=[...])`
- `result.name` → `result.route_id`
- `result.score` → `result.confidence`  (isotonic-calibrated, not raw cosine)

See [MIGRATION.md](MIGRATION.md) for upgrade guide.

### Added

**Rust Core** (`core/src/`)
- `algorithms/hybrid_scoring.rs` — dense(0.6427) + BM25(0.2891) + rule(0.0682) weights
- `algorithms/calibration.rs` — per-route isotonic regression confidence calibration
- `algorithms/simd_ops.rs` — AVX2 SIMD cosine similarity (10× faster on x86_64)
- `index/hnsw.rs` — O(log N) HNSW approximate nearest-neighbour index
- `ffi.rs` — PyO3 Python bindings (PyRouter, PyRoute)
- 13 Rust integration test files, Criterion benchmarks

**Python SDK** (`python/stratarouter/`)
- `Router` class: `add()`, `build_index()`, `route()`, `save()`, `load()`
- `Route` (Pydantic): `id`, `description`, `examples`, `keywords`, `patterns`, `threshold`, `metadata`, `tags`
- `RouteResult`: `route_id`, `confidence`, `scores` (semantic/keyword/pattern), `latency_ms`
- `DeploymentMode`: `LOCAL` (embedded Rust) | `CLOUD` (Enterprise API)
- 9 framework integrations: LangChain, LangGraph, CrewAI, AutoGen, OpenAI Assistants, Vertex AI, Generic
- `cloud/client.py` — thread-safe httpx CloudClient

**Tests** (`tests/`)
- `test_router.py`, `test_types.py`, `test_integrations.py`
- `test_cloud.py`, `test_encoders.py`, `test_utils.py`, `conftest.py`

### Performance vs v0.1
- P99 latency: **8.7 ms** (was 40+ ms)
- Memory: **64 MB** for 1K routes (was 2.1 GB)
- Throughput: **18K req/s** (was ~450)
- Accuracy: **95.4%** (was ~84%)

---

## [0.1.0] — 2025-11-01  ← Initial Prototype

### Added
- `RouteLayer` + `Route` + `utterances` API
- HuggingFace + OpenAI encoder support
- LangChain integration, FastAPI demo server
- Dockerfile + docker-compose, MIT license

---

[0.2.0]: https://github.com/ai-deeptech/stratarouter/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ai-deeptech/stratarouter/releases/tag/v0.1.0
