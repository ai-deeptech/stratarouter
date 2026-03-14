"""Tests for the PyRouter Rust extension (stratarouter._core.PyRouter)."""

import pytest

from stratarouter._core import PyRoute, PyRouter  # type: ignore[import]


# ── Initialisation ────────────────────────────────────────────────────────────

def test_router_initialization():
    router = PyRouter(dimension=384, threshold=0.5)
    assert router.route_count() == 0
    assert not router.is_index_built()


def test_router_invalid_dimension():
    with pytest.raises(RuntimeError, match="Dimension must be positive"):
        PyRouter(dimension=0, threshold=0.5)


def test_router_invalid_threshold():
    with pytest.raises(RuntimeError, match="Threshold must be between"):
        PyRouter(dimension=384, threshold=1.5)


# ── Route management ──────────────────────────────────────────────────────────

def test_add_route():
    router = PyRouter(dimension=384, threshold=0.5)
    route = PyRoute("test-route")
    route.description = "Test route"
    route.examples = ["test query"]
    route.keywords = ["test"]
    router.add_route(route)
    assert router.route_count() == 1


def test_add_route_no_examples_or_description():
    router = PyRouter(dimension=384, threshold=0.5)
    route = PyRoute("test-route")
    with pytest.raises(RuntimeError, match="must have examples or description"):
        router.add_route(route)


# ── Index building ────────────────────────────────────────────────────────────

def test_build_index(sample_routes, sample_embeddings):
    router = PyRouter(dimension=384, threshold=0.5)
    for route in sample_routes:
        router.add_route(route)
    router.build_index(sample_embeddings)
    assert router.is_index_built()


def test_build_index_dimension_mismatch(sample_routes):
    router = PyRouter(dimension=384, threshold=0.5)
    for route in sample_routes:
        router.add_route(route)
    wrong_embeddings = [[1.0] * 256 for _ in range(3)]
    with pytest.raises(RuntimeError, match="[Dd]imension"):
        router.build_index(wrong_embeddings)


# ── Routing ───────────────────────────────────────────────────────────────────

def test_route_query(router_with_routes, query_embedding):
    """Verify that route() returns the expected dict schema."""
    result = router_with_routes.route("Where is my invoice?", query_embedding)

    # Top-level keys
    assert "route_id" in result
    assert "confidence" in result
    assert "scores" in result
    assert "latency_ms" in result          # ← correct field name (not latency_us)

    # Types
    assert isinstance(result["route_id"], str)
    assert 0.0 <= result["confidence"] <= 1.0

    # Score sub-keys — must match ffi.rs dict construction
    scores = result["scores"]
    for key in ("semantic", "keyword", "pattern", "total", "confidence"):
        assert key in scores, f"Expected scores['{key}'] but got keys: {list(scores)}"


def test_route_without_index():
    router = PyRouter(dimension=384, threshold=0.5)
    route = PyRoute("test")
    route.description = "test"
    router.add_route(route)
    with pytest.raises(RuntimeError, match="[Ii]ndex"):
        router.route("test query", [0.5] * 384)


def test_route_empty_text(router_with_routes):
    with pytest.raises(RuntimeError, match="[Ee]mpty"):
        router_with_routes.route("", [0.5] * 384)


def test_route_empty_embedding(router_with_routes):
    with pytest.raises(RuntimeError, match="[Ee]mpty"):
        router_with_routes.route("test query", [])
