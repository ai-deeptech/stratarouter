# StrataRouter Roadmap

> Updated quarterly. Last updated: March 2026.

## Vision: AI Execution Control Plane

What Kubernetes is for containers — StrataRouter is for AI systems.
Deterministic routing, cost governance, and compliance at the infrastructure level.

---

## ✅ Released

### v0.1 — Prototype (November 2025)
- [x] `RouteLayer` + `Route` + `utterances` API
- [x] HuggingFace + OpenAI encoders
- [x] LangChain integration, FastAPI demo server
- [x] MIT license, Dockerfile

### v0.2 — Production Core (March 2026) ← current
- [x] Rust engine: hybrid scoring (dense + BM25 + rule), piecewise-linear calibration
- [x] New API: `Router` + `Route` + `build_index()`
- [x] 9 framework integrations (LangChain, LangGraph, CrewAI, AutoGen, OpenAI Assistants, Vertex AI)
- [x] `Router.save()` / `Router.load()` — zero re-indexing on reload
- [x] Full pytest suite (7 files), 13 Rust test files
- [x] ~20× faster vs semantic-router on P99 latency

### v0.3 — Runtime (Available now)
- [x] TCFP execution engine, Axum REST API
- [x] Semantic cache (LRU + TTL + similarity — 85% hit rate)
- [x] Batch deduplication (3–5× throughput gain)
- [x] Prometheus metrics, Docker/K8s, PostgreSQL/SQLite
- [x] Core → Runtime bridge with circuit breakers
- → [stratarouter-runtime](https://github.com/ai-deeptech/stratarouter-runtime)

### v0.4 — Enterprise Governance (Available — contact us)
- [x] Multi-agent consensus (quorum, Byzantine fault tolerant)
- [x] Immutable audit log (SHA-256 chain, SOC 2 / HIPAA / ISO 27001)
- [x] Policy engine (RBAC + ABAC, cost limits, time constraints)
- [x] Multi-tenant isolation, per-tenant quotas
- [x] Idempotency manager (prevents duplicate high-value transactions)
- [x] 55 tests, 95.8% coverage
- → **Contact**: support@stratarouter.com

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
- → https://stratarouter.com

---

## Community Requests

Open an issue with label `roadmap` to vote on features.

- [ ] Cohere + Mistral encoder support (#14)
- [ ] Ambiguity detection (#13)
- [ ] Entropy-based routing depth (#12)
- [ ] Multi-model cost-aware router (#15)
- [ ] JavaScript / TypeScript SDK (#16)

---

## Links

- **Docs**: https://docs.stratarouter.com
- **Website**: https://stratarouter.com
- **Support**: support@stratarouter.com
- **GitHub**: https://github.com/ai-deeptech/stratarouter
