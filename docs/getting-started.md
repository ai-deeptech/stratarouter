# Getting Started

Welcome to **StrataRouter** — production-grade semantic routing for AI systems.

This guide gets you from zero to a working router in under 5 minutes.

---

## Prerequisites

- Python 3.8 or later
- pip

---

## Installation

Install the base package:

```bash
pip install stratarouter
```

Install with an encoder backend (recommended for local, no-API-key use):

```bash
pip install stratarouter[huggingface]
```

Other encoder options:

```bash
pip install stratarouter[openai]    # OpenAI text-embedding-3-small
pip install stratarouter[cohere]    # Cohere embed-english-v3.0
pip install stratarouter[all]       # everything
```

Verify your installation:

```bash
python -c "import stratarouter; print(stratarouter.__version__)"
```

---

## Your First Router

### Step 1 — Define your routes

```python
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

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
    Route(
        name="sales",
        utterances=["pricing", "demo", "enterprise", "trial"],
        threshold=0.75,
    ),
]
```

### Step 2 — Create the encoder and RouteLayer

```python
encoder = HuggingFaceEncoder()          # downloads model on first run
rl = RouteLayer(encoder=encoder, routes=routes)
```

### Step 3 — Route a query

```python
result = rl("I need my April invoice")

print(result.name)     # "billing"
print(result.score)    # 0.87
print(bool(result))    # True — score >= threshold
```

### Step 4 — Handle no-match

```python
result = rl("What is the weather in London?")

if result:
    print(f"Matched: {result.name}")
else:
    print("No route matched — use fallback handler")
```

---

## Threshold Tuning

`threshold` controls the minimum confidence required to match a route.

- **Higher threshold** (e.g., 0.85) — fewer matches, higher precision
- **Lower threshold** (e.g., 0.60) — more matches, higher recall

Start at `0.75` and adjust based on your test data.

---

## Next Steps

| Topic | Link |
|---|---|
| Low-level Rust API (`Router`) | [API Reference](api-reference.md) |
| Framework integrations | [Integrations](integrations.md) |
| Rust core architecture | [Architecture](architecture.md) |
| Docker / Kubernetes deployment | [Deployment](deployment.md) |
| Full changelog | [CHANGELOG.md](../CHANGELOG.md) |

Full docs → **[docs.stratarouter.com](https://docs.stratarouter.com)**  
Support → **[support@stratarouter.com](mailto:support@stratarouter.com)**
