"""Tests for Route type"""
import pytest
from stratarouter_core.stratarouter_core import PyRoute as Route

def test_route_creation():
    """Test creating a route"""
    route = Route("test-route")
    assert route.id == "test-route"
    assert route.description == ""
    assert route.examples == []
    assert route.keywords == []

def test_route_empty_id():
    """Test creating route with empty ID"""
    with pytest.raises(RuntimeError, match="Route ID cannot be empty"):
        Route("")

def test_route_set_properties():
    """Test setting route properties"""
    route = Route("test")
    route.description = "Test description"
    route.examples = ["example 1", "example 2"]
    route.keywords = ["key1", "key2"]
    
    assert route.description == "Test description"
    assert len(route.examples) == 2
    assert len(route.keywords) == 2

def test_route_repr():
    """Test route string representation"""
    route = Route("test-route")
    route.keywords = ["key1", "key2"]
    
    repr_str = repr(route)
    assert "test-route" in repr_str
    assert "2" in repr_str
