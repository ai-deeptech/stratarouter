# Migration Guide: v0.1 → v0.2

> **Breaking change**: `RouteLayer` + `utterances` → `Router` + `examples`.
> Migration takes ~5 minutes.

## What Changed

| v0.1 | v0.2 | Reason |
|---|---|---|
| `RouteLayer(encoder, routes)` | `Router(encoder)` | Decouple init from routes |
| `Route(name=..., utterances=[...])` | `Route(id=..., examples=[...])` | Clearer naming |
| Auto-indexed on construction | Explicit `build_index()` | Control when indexing happens |
| `result.name` | `result.route_id` | Consistent across SDK |
| `result.score` | `result.confidence` | Signals isotonic calibration |

## Migration

```python
# ── BEFORE (v0.1) ─────────────────────────────────────────────────────────────
from stratarouter import RouteLayer, Route
from stratarouter.encoders import HuggingFaceEncoder

router = RouteLayer(
    encoder=HuggingFaceEncoder(),
    routes=[
        Route(name="billing", utterances=["invoice", "payment", "refund"]),
        Route(name="support", utterances=["help", "broken", "error"]),
    ]
)
result = router("I need my invoice")
print(f"Route: {result.name}, Score: {result.score:.2f}")

# ── AFTER (v0.2) ──────────────────────────────────────────────────────────────
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")
router.add(Route(
    id="billing",                         # was: name
    description="Billing questions",
    examples=["invoice", "payment"],      # was: utterances
    keywords=["invoice", "bill", "charge"]  # NEW: BM25 boost
))
router.add(Route(
    id="support",
    description="Technical support",
    examples=["help", "broken", "error"],
    keywords=["help", "bug", "issue"]
))
router.build_index()                      # NEW: explicit call

result = router.route("I need my invoice")
print(f"Route: {result.route_id}")        # was: result.name
print(f"Confidence: {result.confidence:.2f}")  # was: result.score
```

## New v0.2 Capabilities

```python
# Per-route confidence floor
Route(..., threshold=0.75)

# Exact pattern match (highest priority, overrides semantic)
Route(..., patterns=["cancel my subscription", "request a refund"])

# Metadata for downstream use
Route(..., metadata={"team": "billing", "priority": "high"})

# Score breakdown
result.scores.semantic   # dense HNSW score
result.scores.keyword    # BM25 sparse score
result.scores.pattern    # pattern match score

# Save and reload — no re-indexing
router.save("router.json")
router = Router.load("router.json")

# Cloud mode (Enterprise)
router = Router(mode="cloud", api_key="sr-...")
```

## Help

- [GitHub Issues](https://github.com/ai-deeptech/stratarouter/issues)
- [support@stratarouter.com](mailto:support@stratarouter.com)
