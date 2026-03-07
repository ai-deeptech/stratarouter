# StrataRouter Roadmap

> Updated quarterly. Last updated: March 2026.

## Vision: AI Execution Control Plane
What Kubernetes is for containers — for AI systems.

---

## ✅ Released

### v0.1 — Prototype (November 2025)
- [x] `RouteLayer` + `Route` + `utterances` API
- [x] HuggingFace + OpenAI encoders
- [x] LangChain integration, FastAPI demo server
- [x] MIT license, Dockerfile

### v0.2 — Production Core (March 2026) ← current
- [x] Rust engine: HNSW, AVX2 SIMD, BM25, isotonic calibration
- [x] New API: `Router` + `Route` + `build_index()`
- [x] 9 framework integrations
- [x] `Router.save()` / `Router.load()`
- [x] Full pytest suite (7 files), 13 Rust test files
- [x] 20× faster vs semantic-router

### v0.3 — Runtime (Available now)
- [x] TCFP execution engine, Axum REST API
- [x] Semantic cache (LRU + TTL + similarity — 85% hit rate)
- [x] Batch deduplication (3–5× throughput)
- [x] Prometheus metrics, Docker/K8s, PostgreSQL/SQLite
- [x] Core→Runtime bridge with circuit breakers
- → [stratarouter-runtime](https://github.com/ai-deeptech/stratarouter-runtime)

### v0.4 — Enterprise Governance (Available — private)
- [x] Multi-agent consensus (quorum, Byzantine fault tolerant)
- [x] Immutable audit log (SHA-256 chain, SOC2/HIPAA/ISO 27001)
- [x] Policy engine (RBAC + ABAC, cost limits, time constraints)
- [x] Multi-tenant isolation, per-tenant quotas
- [x] Idempotency manager (prevents $50K+ duplicate transactions)
- [x] 55 tests, 95.8% coverage
- → [support@inteleion.com](mailto:support@inteleion.com)

---

## 🔄 In Progress

### v0.5 — Cost Optimization (Q2 2026)
- [ ] Model cost-performance optimizer per route
- [ ] Entropy-aware reasoning depth selection
- [ ] Per-route budget enforcement
- [ ] Cost analytics dashboard

### v0.6 — SDKs (Q3 2026)
- [ ] JavaScript / TypeScript SDK
- [ ] Go SDK

---

## 📋 Planned

### v1.0 — StrataRouter Cloud (Q4 2026)
- [ ] Fully managed cloud API
- [ ] Web dashboard + usage analytics
- [ ] Usage-based billing
- [ ] Multi-region deployment

---

## Community Requests

Open an issue with label `roadmap` to vote on features.

- [ ] Cohere + Mistral encoder support (#14)
- [ ] Ambiguity detection (#13)
- [ ] Entropy-based routing depth (#12)
- [ ] Multi-model cost-aware router (#15)
- [ ] JavaScript/TypeScript SDK (#16)
