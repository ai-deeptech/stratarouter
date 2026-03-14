# Migration Guide

## v0.2.0 → v0.2.1 (recommended upgrade)

No breaking API changes. The public `Route(name, utterances)` API from v0.1
is fully restored and is now the canonical interface.

If you were using the short-lived `Route(id=..., examples=[...])` schema
from the v0.2.0 release, update as follows:

```python
# v0.2.0 (internal schema, now retired)
Route(id="billing", examples=["invoice", "payment"])

# v0.2.1 (canonical, restored)
Route(name="billing", utterances=["invoice", "payment"])
```

`RouteLayer` is now exported directly from `stratarouter`:

```python
from stratarouter import Route, RouteLayer  # works in v0.2.1
```

---

## v0.1 → v0.2

> Migration takes ~5 minutes.

### What Changed

| v0.1 | v0.2 | Notes |
|---|---|---|
| `RouteLayer(encoder, routes)` | `RouteLayer(encoder, routes)` | Same — preferred API |
| `Route(name=..., utterances=[...])` | `Route(name=..., utterances=[...])` | Same — no change |
| `result.name` | `result.name` (RouteLayer) | Same for RouteLayer |
| `result.score` | `result.score` (RouteLayer) | Same for RouteLayer |
| — | `Router` (low-level) | New: Rust-backed, `route_id` / `confidence` |
| — | `router.build_index()` | New: explicit index build for `Router` |
| — | `result.confidence` | New: calibrated score on `Router` result |

### Recommended: stay on `RouteLayer`

If you were using `RouteLayer` in v0.1, **no changes needed**:

```python
# This still works exactly the same in v0.2.1
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

routes = [
    Route(name="billing", utterances=["invoice", "payment", "refund"]),
    Route(name="support", utterances=["help", "broken", "error"]),
]
rl = RouteLayer(encoder=HuggingFaceEncoder(), routes=routes)

result = rl("I need my invoice")
print(result.name)   # "billing"
print(result.score)  # 0.87
```

### Optional: upgrade to `Router` for Rust hybrid scoring

Use the low-level `Router` if you need BM25 keyword boosting, pattern
matching, confidence calibration, or `save()`/`load()`:

```python
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")
router.add(Route(
    name="billing",
    utterances=["invoice", "payment"],
))
router.build_index()

result = router.route("I need my April invoice")
print(result.route_id)    # "billing"   ← differs from RouteLayer
print(result.confidence)  # 0.89        ← calibrated, not raw cosine
print(result.latency_ms)  # 2.3ms

router.save("router.json")
router = Router.load("router.json")
```

### Key difference: `RouteLayer` vs `Router` results

| | `RouteLayer` | `Router` |
|---|---|---|
| Result type | `RouteChoice` | `RouteResult` |
| Route name field | `result.name` | `result.route_id` |
| Score field | `result.score` | `result.confidence` |
| No-match result | `RouteChoice(name=None)` | raises `Error::NoRoutes` |
| Requires Rust core | ❌ | ✅ |

---

## Help

- [GitHub Issues](https://github.com/ai-deeptech/stratarouter/issues)
- [support@stratarouter.com](mailto:support@stratarouter.com)
