# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

---

## [Unreleased]

---

## [0.2.1] — 2026-03-12  ← OSS Quality Pass

Applied all fixes from the OSS code-review conducted against Google
open-source best practices and PyPI/crates.io publishing standards.

### Fixed

**Rust Core**
- Renamed `HnswIndex` → `LinearIndex` — implementation is O(N) brute-force scan,
  not HNSW; graph-based HNSW is planned (see ROADMAP.md)
- Renamed `simd_ops.rs` → `vector_ops.rs` — implementation is scalar; removed
  unused `wide` crate dependency
- Renamed `IsotonicCalibrator` → `ScoreNormalizer` — implementation is piecewise-
  linear interpolation, not fitted isotonic regression
- `LinearIndex::new/add/search` now return `Result<_>` — no more `assert!` panics
  on user-supplied dimension mismatches
- Removed `Arc<RwLock<>>` from `LinearIndex` (unnecessary; Router owns it exclusively)
- Added `///` doc comments to all public `cache.rs` methods (resolves `missing_docs`)
- Removed `overflow-checks = false` from `[profile.release]`
- Fixed `repository`/`homepage` URLs in `Cargo.toml` to `ai-deeptech/stratarouter`
- Unified author identity across `Cargo.toml` and `pyproject.toml`
- Removed `//! Proprietary` doc comment from `hybrid_scoring.rs`
- Neutralised orphaned `route.rs` and `similarity.rs` (referenced non-existent
  `crate::error::StrataError` — compile errors under `--features python`)

**Python SDK**
- `RouteLayer` added (`layer.py`) — pure-Python high-level router, no Rust core
  required, compatible with semantic-router API
- `Route` unified to `name` + `utterances` schema (public API); internal Rust
  mapping type renamed to `RouteConfig`
- `__version__` now single source of truth via `importlib.metadata`
- `CohereEncoder`: `import cohere` moved inside `__init__` (was top-level —
  crashed all users without cohere installed); implements `encode()` + `dimension`
- `router.save()` reads version from `__version__` (was hardcoded `"0.2.0"`)
- Fixed stale `from .types import Route` imports in `router.py` and `generic.py`
- Added `langchain` and `cloud` optional-dependency groups to `pyproject.toml`
- `langchain_new.py` removed (was verbatim duplicate of `langchain.py`)
- `backup_setup.py` retired from VCS

**Tests**
- `test_route.py` fully rewritten — was corrupted with embedded duplicate content
- `MockEncoder` fixed to implement `encode()` + `dimension` (satisfies `BaseEncoder` ABC)
- All tests updated to use `RouteLayer` (was importing non-existent symbol)

**Docs / CI**
- `CONTRIBUTING.md`: removed Python `"""` wrapper; updated email to `hello@stratarouter.dev`
- `README.md`: corrected architecture diagram, API examples, and benchmark caveats
- `CHANGELOG.md`: corrected internal module names to match actual implementation
- `ci.yml`: removed `continue-on-error: true` from pytest and security jobs;
  added `concurrency` cancel-in-progress; added `pip-audit` Python dep scan;
  added `mypy` type-check step

---

## [0.2.0] — 2026-03-07  ← Platform Relaunch

4 months of production engineering after the v0.1 prototype.
StrataRouter is now an **AI Execution Control Plane**, not just a routing library.

### Breaking Changes (v0.1 → v0.2)
- `RouteLayer` is now the recommended high-level API; `Router` is the low-level API
- `Route` uses `name` + `utterances` (public), `RouteConfig` used internally
- `result.route_id` returned by low-level `Router.route()`
- `result.confidence` — calibrated score (not raw cosine)

See [MIGRATION.md](MIGRATION.md) for upgrade guide.

### Added

**Rust Core** (`core/src/`)
- `algorithms/hybrid_scoring.rs` — dense(0.6427) + BM25(0.2891) + rule(0.0682) weights
- `algorithms/calibration.rs` — per-route piecewise-linear score normalisation
- `algorithms/vector_ops.rs` — scalar cosine similarity (SIMD planned)
- `index/hnsw.rs` — linear-scan nearest-neighbour index (`LinearIndex`); HNSW planned
- `ffi.rs` — PyO3 Python bindings (`PyRouter`, `PyRoute`)
- Criterion benchmarks

**Python SDK** (`python/stratarouter/`)
- `RouteLayer`: high-level pure-Python router; `Router`: low-level Rust-backed router
- `Route` (Pydantic): `name`, `utterances`, `description`, `threshold`, `metadata`
- `RouteChoice`: `name`, `score`, `threshold`, `is_match`, `__bool__`
- `RouteResult`: `route_id`, `confidence`, `scores`, `latency_ms`
- `DeploymentMode`: `LOCAL` (embedded Rust) | `CLOUD` (Enterprise API)
- 9 framework integrations: LangChain, LangGraph, CrewAI, AutoGen, OpenAI Assistants, Vertex AI, Generic
- `cloud/client.py` — thread-safe httpx CloudClient

### Performance vs v0.1
- P99 latency: **8.7 ms** (was 40+ ms)
- Memory: **64 MB** for 1K routes (was 2.1 GB)
- Throughput: **18K req/s** (was ~450)

---

## [0.1.0] — 2025-11-01  ← Initial Prototype

### Added
- `RouteLayer` + `Route(name, utterances)` API
- HuggingFace + OpenAI encoder support
- LangChain integration, FastAPI demo server
- Dockerfile + docker-compose, MIT license

---

[Unreleased]: https://github.com/ai-deeptech/stratarouter/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/ai-deeptech/stratarouter/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ai-deeptech/stratarouter/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ai-deeptech/stratarouter/releases/tag/v0.1.0
