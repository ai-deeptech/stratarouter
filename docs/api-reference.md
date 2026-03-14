# API Reference

Complete reference for all public StrataRouter APIs.

---

## `Route`

Defines a named route with example utterances.

```python
from stratarouter import Route

route = Route(
    name="billing",
    utterances=["invoice", "payment", "refund"],
    description="Billing and payment queries",   # optional
    threshold=0.75,                              # optional, default 0.75
    metadata={"team": "billing-squad"},          # optional, any dict
)
```

### Fields

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | `str` | ✅ | Unique route identifier |
| `utterances` | `list[str]` | ✅ | Example phrases for this route |
| `description` | `str` | ❌ | Human-readable description |
| `threshold` | `float` | ❌ | Min similarity score to match (default: 0.75) |
| `metadata` | `dict` | ❌ | Arbitrary key-value metadata |

---

## `RouteChoice`

Result returned by `RouteLayer`.

```python
result = rl("I need my invoice")

result.name        # str | None — matched route name, or None on no-match
result.score       # float — raw similarity score
result.threshold   # float — threshold used for this route
result.is_match    # bool — True if score >= threshold
bool(result)       # True if matched
```

### Fields

| Field | Type | Description |
|---|---|---|
| `name` | `str \| None` | Matched route name, or `None` |
| `score` | `float` | Cosine similarity score [0, 1] |
| `threshold` | `float` | Threshold for the matched route |
| `is_match` | `bool` | `True` if score >= threshold |

---

## `RouteLayer`

High-level router. Recommended entry point for most users.
Pure Python — no Rust core required.

```python
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

rl = RouteLayer(
    encoder=HuggingFaceEncoder(),
    routes=[
        Route(name="billing", utterances=["invoice", "payment"]),
        Route(name="support", utterances=["help", "error"]),
    ],
)

result = rl("I need a refund")         # __call__
result = rl.route("I need a refund")   # explicit method
```

### Constructor

```python
RouteLayer(
    encoder: BaseEncoder,
    routes: list[Route],
    top_k: int = 1,            # how many candidate routes to evaluate
)
```

### Methods

| Method | Returns | Description |
|---|---|---|
| `__call__(query)` | `RouteChoice` | Route a query |
| `route(query)` | `RouteChoice` | Alias for `__call__` |
| `add(route)` | `None` | Add a route after construction |
| `remove(name)` | `None` | Remove a route by name |
| `routes` | `list[Route]` | Inspect current routes |

---

## `Router`

Low-level router backed by the Rust core. Use when you need hybrid BM25 + dense
scoring, calibrated confidence, `save()`/`load()`, or maximum throughput.

```python
from stratarouter import Router, Route

router = Router(encoder="sentence-transformers/all-MiniLM-L6-v2")

router.add(Route(name="billing", utterances=["invoice", "payment"]))
router.add(Route(name="support", utterances=["help", "error"]))
router.build_index()

result = router.route("I need a refund")
```

### Constructor

```python
Router(
    encoder: str | BaseEncoder,   # model name or encoder instance
    mode: DeploymentMode = DeploymentMode.LOCAL,
)
```

### Methods

| Method | Returns | Description |
|---|---|---|
| `add(route)` | `None` | Add a route (call before `build_index()`) |
| `build_index()` | `None` | Build the routing index (required once) |
| `route(query)` | `RouteResult` | Route a query |
| `save(path)` | `None` | Serialise index to JSON — no re-indexing on load |
| `Router.load(path)` | `Router` | Deserialise index |

### `RouteResult` fields

| Field | Type | Description |
|---|---|---|
| `route_id` | `str \| None` | Matched route name |
| `confidence` | `float` | Calibrated score [0, 1] |
| `scores` | `dict` | Per-route raw scores |
| `latency_ms` | `float` | Routing latency in milliseconds |

---

## Encoders

### `HuggingFaceEncoder`

```python
from stratarouter.encoders import HuggingFaceEncoder

encoder = HuggingFaceEncoder(
    name="sentence-transformers/all-MiniLM-L6-v2",  # default
)
```

Requires: `pip install stratarouter[huggingface]`

### `OpenAIEncoder`

```python
from stratarouter.encoders import OpenAIEncoder

encoder = OpenAIEncoder(
    name="text-embedding-3-small",  # default
    api_key="sk-...",               # or set OPENAI_API_KEY env var
)
```

Requires: `pip install stratarouter[openai]`

### `CohereEncoder`

```python
from stratarouter.encoders import CohereEncoder

encoder = CohereEncoder(
    name="embed-english-v3.0",  # default
    api_key="...",              # or set COHERE_API_KEY env var
)
```

Requires: `pip install stratarouter[cohere]`

---

## `DeploymentMode`

```python
from stratarouter import DeploymentMode

DeploymentMode.LOCAL   # embedded Rust core (default)
DeploymentMode.CLOUD   # StrataRouter Enterprise API
```

---

## Support

- **Docs**: https://docs.stratarouter.com
- **Issues**: https://github.com/ai-deeptech/stratarouter/issues
- **Email**: support@stratarouter.com
