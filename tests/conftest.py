"""Pytest configuration and shared fixtures for the root test suite.

These tests target the compiled Rust extension (`stratarouter._core`).
The extension must be built before running:

    cd python && maturin develop --release && cd ..
    pytest tests/
"""

import random

import pytest

# ---------------------------------------------------------------------------
# Import the compiled Rust extension.
# The canonical module path after maturin build is stratarouter._core.
# ---------------------------------------------------------------------------
try:
    from stratarouter._core import PyRoute, PyRouter  # type: ignore[import]
except ImportError as exc:  # pragma: no cover
    raise ImportError(
        "Compiled Rust core not found. "
        "Build it first with: cd python && maturin develop --release"
    ) from exc


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

@pytest.fixture()
def sample_routes():
    """Three pre-configured PyRoute fixtures covering distinct intent classes."""
    billing = PyRoute("billing")
    billing.description = "Billing and payment questions"
    billing.examples = ["Where's my invoice?", "I need a refund"]
    billing.keywords = ["invoice", "billing", "payment", "refund"]

    support = PyRoute("support")
    support.description = "Technical support questions"
    support.examples = ["App is crashing", "Can't login"]
    support.keywords = ["crash", "bug", "error", "login"]

    sales = PyRoute("sales")
    sales.description = "Sales inquiries"
    sales.examples = ["I want to buy", "Pricing information"]
    sales.keywords = ["buy", "purchase", "pricing", "upgrade"]

    return [billing, support, sales]


@pytest.fixture()
def sample_embeddings():
    """Three deterministic 384-dim float embeddings (one per sample route)."""
    rng = random.Random(42)
    return [
        [rng.gauss(0, 1) for _ in range(384)],
        [rng.gauss(0, 1) for _ in range(384)],
        [rng.gauss(0, 1) for _ in range(384)],
    ]


@pytest.fixture()
def router_with_routes(sample_routes, sample_embeddings):
    """A fully initialised PyRouter with 3 routes and a built index."""
    router = PyRouter(dimension=384, threshold=0.3)
    for route in sample_routes:
        router.add_route(route)
    router.build_index(sample_embeddings)
    return router


@pytest.fixture()
def query_embedding():
    """A deterministic 384-dim query embedding."""
    rng = random.Random(123)
    return [rng.gauss(0, 1) for _ in range(384)]
