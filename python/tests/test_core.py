"""
Tests for Rust core functionality
"""

import pytest
from stratarouter.core import (
    Router,
    RustRoute,
    RouteMatch,
    cosine_similarity,
    cosine_similarity_batch
)


def test_cosine_similarity_identical():
    """Test cosine similarity of identical vectors"""
    a = [1.0, 2.0, 3.0]
    b = [1.0, 2.0, 3.0]
    
    sim = cosine_similarity(a, b)
    assert abs(sim - 1.0) < 1e-6


def test_cosine_similarity_orthogonal():
    """Test cosine similarity of orthogonal vectors"""
    a = [1.0, 0.0, 0.0]
    b = [0.0, 1.0, 0.0]
    
    sim = cosine_similarity(a, b)
    assert abs(sim - 0.0) < 1e-6


def test_cosine_similarity_opposite():
    """Test cosine similarity of opposite vectors"""
    a = [1.0, 0.0]
    b = [-1.0, 0.0]
    
    sim = cosine_similarity(a, b)
    assert abs(sim - (-1.0)) < 1e-6


def test_cosine_similarity_dimension_mismatch():
    """Test cosine similarity with mismatched dimensions"""
    a = [1.0, 2.0]
    b = [1.0, 2.0, 3.0]
    
    with pytest.raises(Exception):
        cosine_similarity(a, b)


def test_cosine_similarity_batch():
    """Test batch cosine similarity"""
    query = [1.0, 0.0, 0.0]
    embeddings = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0]
    ]
    
    sims = cosine_similarity_batch(query, embeddings)
    assert len(sims) == 3
    assert abs(sims[0] - 1.0) < 1e-6
    assert abs(sims[1] - 0.0) < 1e-6
    assert abs(sims[2] - (-1.0)) < 1e-6


def test_rust_route_creation():
    """Test creating a Rust route"""
    route = RustRoute(
        name="test",
        embeddings=[[1.0, 0.0, 0.0]],
        threshold=0.8
    )
    
    assert route.name == "test"
    assert route.num_examples == 1
    assert route.embedding_dim == 3
    assert route.threshold == 0.8


def test_rust_route_validation():
    """Test Rust route validation"""
    with pytest.raises(Exception):
        RustRoute(name="", embeddings=[[1.0]], threshold=0.8)
    
    with pytest.raises(Exception):
        RustRoute(name="test", embeddings=[], threshold=0.8)
    
    with pytest.raises(Exception):
        RustRoute(name="test", embeddings=[[1.0]], threshold=1.5)


def test_router_creation():
    """Test creating a router"""
    router = Router(top_k=5, cache_size=1000)
    assert router.num_routes == 0
    assert router.top_k == 5


def test_router_add_route():
    """Test adding routes to router"""
    router = Router()
    route = RustRoute(
        name="test",
        embeddings=[[1.0, 0.0, 0.0]],
        threshold=0.8
    )
    
    router.add(route)
    assert router.num_routes == 1
    assert "test" in router.list_routes()


def test_router_duplicate_route():
    """Test adding duplicate route"""
    router = Router()
    route1 = RustRoute(name="test", embeddings=[[1.0]], threshold=0.8)
    route2 = RustRoute(name="test", embeddings=[[2.0]], threshold=0.8)
    
    router.add(route1)
    with pytest.raises(Exception):
        router.add(route2)


def test_router_remove_route():
    """Test removing routes"""
    router = Router()
    route = RustRoute(name="test", embeddings=[[1.0]], threshold=0.8)
    
    router.add(route)
    assert router.num_routes == 1
    
    router.remove("test")
    assert router.num_routes == 0


def test_router_routing():
    """Test routing queries"""
    router = Router(top_k=2)
    
    route1 = RustRoute(
        name="route1",
        embeddings=[[1.0, 0.0, 0.0]],
        threshold=0.7
    )
    route2 = RustRoute(
        name="route2",
        embeddings=[[0.0, 1.0, 0.0]],
        threshold=0.7
    )
    
    router.add(route1)
    router.add(route2)
    
    matches = router.route([1.0, 0.0, 0.0])
    assert len(matches) <= 2
    assert matches[0].name == "route1"
    assert matches[0].score > 0.9


def test_router_clear():
    """Test clearing router"""
    router = Router()
    route = RustRoute(name="test", embeddings=[[1.0]], threshold=0.8)
    
    router.add(route)
    assert router.num_routes == 1
    
    router.clear()
    assert router.num_routes == 0


def test_route_match():
    """Test RouteMatch properties"""
    match = RouteMatch(name="test", score=0.85, threshold=0.75)
    
    assert match.name == "test"
    assert match.score == 0.85
    assert match.threshold == 0.75
    assert match.is_match
    
    match2 = RouteMatch(name="test", score=0.70, threshold=0.75)
    assert not match2.is_match
