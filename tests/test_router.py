"""Tests for main Router class"""
import pytest
from stratarouter_core.stratarouter_core import PyRouter as Router, PyRoute as Route

def test_router_initialization():
    """Test router initialization"""
    router = Router(dimension=384, threshold=0.5)
    assert router.route_count() == 0
    assert not router.is_index_built()

def test_router_invalid_dimension():
    """Test router with invalid dimension"""
    with pytest.raises(RuntimeError, match="Dimension must be positive"):
        Router(dimension=0, threshold=0.5)

def test_router_invalid_threshold():
    """Test router with invalid threshold"""
    with pytest.raises(RuntimeError, match="Threshold must be between"):
        Router(dimension=384, threshold=1.5)

def test_add_route():
    """Test adding routes"""
    router = Router(dimension=384, threshold=0.5)
    
    route = Route("test-route")
    route.description = "Test route"
    route.examples = ["test query"]
    route.keywords = ["test"]
    
    router.add_route(route)
    assert router.route_count() == 1

def test_add_route_no_examples_or_description():
    """Test adding route without examples or description fails"""
    router = Router(dimension=384, threshold=0.5)
    route = Route("test-route")
    
    with pytest.raises(RuntimeError, match="must have examples or description"):
        router.add_route(route)

def test_build_index(sample_routes, sample_embeddings):
    """Test building index"""
    router = Router(dimension=384, threshold=0.5)
    
    for route in sample_routes:
        router.add_route(route)
    
    router.build_index(sample_embeddings)
    assert router.is_index_built()

def test_build_index_dimension_mismatch(sample_routes):
    """Test building index with wrong dimension"""
    router = Router(dimension=384, threshold=0.5)
    
    for route in sample_routes:
        router.add_route(route)
    
    # Wrong dimension embeddings
    wrong_embeddings = [[1.0] * 256 for _ in range(3)]
    
    with pytest.raises(RuntimeError, match="Dimension mismatch"):
        router.build_index(wrong_embeddings)

def test_route_query(router_with_routes, query_embedding):
    """Test routing a query"""
    result = router_with_routes.route("Where is my invoice?", query_embedding)
    
    assert "route_id" in result
    assert "confidence" in result
    assert "scores" in result
    assert "latency_us" in result
    
    assert isinstance(result["route_id"], str)
    assert 0.0 <= result["confidence"] <= 1.0
    assert "dense" in result["scores"]
    assert "sparse" in result["scores"]
    assert "rule" in result["scores"]
    assert "fused" in result["scores"]

def test_route_without_index():
    """Test routing before building index"""
    router = Router(dimension=384, threshold=0.5)
    route = Route("test")
    route.description = "test"
    router.add_route(route)
    
    query_embedding = [0.5] * 384
    
    with pytest.raises(RuntimeError, match="Index not built"):
        router.route("test query", query_embedding)

def test_route_empty_text(router_with_routes):
    """Test routing with empty text"""
    query_embedding = [0.5] * 384
    
    with pytest.raises(RuntimeError, match="Query text cannot be empty"):
        router_with_routes.route("", query_embedding)

def test_route_empty_embedding(router_with_routes):
    """Test routing with empty embedding"""
    with pytest.raises(RuntimeError, match="Embedding cannot be empty"):
        router_with_routes.route("test query", [])
