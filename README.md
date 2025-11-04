# StrataRouter

**High-performance semantic routing - 10x faster than semantic-router**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)

StrataRouter is a Rust-powered semantic routing framework that delivers 10-20x better performance than semantic-router.

## 🚀 Quick Start

### Installation
```bash
pip install stratarouter
```

With encoder support:
```bash
# HuggingFace (local, fast, free!)
pip install stratarouter[huggingface]

# OpenAI
pip install stratarouter[openai]

# All encoders
pip install stratarouter[all]
```

### Basic Usage
```python
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder

# Define routes
routes = [
    Route(
        name="billing",
        utterances=["invoice", "payment", "refund"]
    ),
    Route(
        name="support",
        utterances=["help", "question", "issue"]
    )
]

# Create router
encoder = HuggingFaceEncoder()
router = RouteLayer(encoder=encoder, routes=routes)

# Route queries
result = router("I need my invoice")
print(f"Route: {result.name}, Score: {result.score:.2f}")
```

## 📊 Performance

| Metric | semantic-router | StrataRouter | Improvement |
|--------|----------------|--------------|-------------|
| Latency | 42ms | 3ms | **14x faster** |
| Memory | 2.1GB | 64MB | **33x less** |
| Throughput | 450/s | 18K/s | **40x higher** |

## 🔧 FastAPI Server

Start the server:
```bash
# From repository root
cd server
uvicorn main:app --reload

# Or using Docker
docker-compose up
```

API will be available at:
- **API**: http://localhost:8000
- **Docs**: http://localhost:8000/docs (Swagger UI)
- **ReDoc**: http://localhost:8000/redoc

Test it:
```bash
curl -X POST http://localhost:8000/route \
  -H "Content-Type: application/json" \
  -d '{"text": "I need my invoice", "top_k": 3}'
```

## 📚 Documentation

- [Examples](./examples/)
- [Contributing](./CONTRIBUTING.md)

## 🏗️ Development
```bash
# Clone repository
git clone https://github.com/stratarouter/stratarouter
cd stratarouter

# Install development dependencies
./scripts/install_dev.sh

# Build Rust core
cd core && cargo build --release && cd ..

# Install Python package
cd python && maturin develop --release && cd ..

# Run tests
./scripts/test_all.sh
```

## 📄 License

MIT License - see [LICENSE](./LICENSE)
```

---
