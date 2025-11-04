"""
Tests for RouteLayer
"""

import pytest
import numpy as np

from stratarouter import Route, RouteLayer
from stratarouter.encoders.base import BaseEncoder


class MockEncoder(BaseEncoder):
    """Mock encoder for testing"""
    
    def __call__(self, texts):
        embeddings = []
        for text in texts:
            if text.lower().startswith("b"):
                emb = [1.0, 0.0, 0.0]
            elif text.lower().startswith("s"):
                emb = [0.0, 1.0, 0.0]
            else:
                emb = [0.0, 0.0, 1.0]
            embeddings.append(emb)
        return np.array(embeddings, dtype=np.float32)
    
    @property
    def dim(self):
        return 3


def test_route_layer_creation():
    """Test creating a RouteLayer"""
    encoder = MockEncoder()
    routes = [
        Route(name="billing", utterances=["billing", "invoice"]),
        Route(name="support", utterances=["support", "help"])
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    assert rl.num_routes == 2
    assert "billing" in rl.list_route_names()
    assert "support" in rl.list_route_names()


def test_route_layer_routing():
    """Test basic routing"""
    encoder = MockEncoder()
    routes = [
        Route(name="billing", utterances=["billing", "invoice"], threshold=0.7),
        Route(name="support", utterances=["support", "help"], threshold=0.7)
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    
    result = rl("billing question")
    assert result.name == "billing"
    assert result.score > 0.7
    assert result.is_match
    
    result2 = rl("support issue")
    assert result2.name == "support"


def test_route_layer_no_match():
    """Test when no route matches"""
    encoder = MockEncoder()
    routes = [
        Route(name="billing", utterances=["billing"], threshold=0.9)
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    result = rl("other topic")
    assert result.name is None
    assert not result.is_match


def test_route_layer_add_remove():
    """Test adding and removing routes"""
    encoder = MockEncoder()
    rl = RouteLayer(encoder=encoder)
    
    assert rl.num_routes == 0
    
    route = Route(name="test", utterances=["test"])
    rl.add(route)
    assert rl.num_routes == 1
    
    rl.remove("test")
    assert rl.num_routes == 0


def test_route_layer_duplicate():
    """Test adding duplicate route raises error"""
    encoder = MockEncoder()
    route = Route(name="test", utterances=["test"])
    
    rl = RouteLayer(encoder=encoder, routes=[route])
    
    with pytest.raises(ValueError, match="already exists"):
        rl.add(route)


def test_route_layer_batch():
    """Test batch routing"""
    encoder = MockEncoder()
    routes = [
        Route(name="billing", utterances=["billing"], threshold=0.7),
        Route(name="support", utterances=["support"], threshold=0.7)
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    
    results = rl.route_batch(["billing", "support", "other"])
    assert len(results) == 3
    assert results[0].name == "billing"
    assert results[1].name == "support"
    assert results[2].name is None


def test_route_layer_custom_threshold():
    """Test routing with custom threshold"""
    encoder = MockEncoder()
    routes = [
        Route(name="test", utterances=["test"], threshold=0.5)
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    
    result1 = rl("test")
    assert result1.is_match
    
    result2 = rl("test", threshold=0.99)
    assert not result2.is_match


def test_route_layer_clear():
    """Test clearing routes"""
    encoder = MockEncoder()
    routes = [
        Route(name="test1", utterances=["test1"]),
        Route(name="test2", utterances=["test2"])
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    assert rl.num_routes == 2
    
    rl.clear()
    assert rl.num_routes == 0


def test_route_layer_empty_text():
    """Test that empty text raises error"""
    encoder = MockEncoder()
    routes = [Route(name="test", utterances=["test"])]
    rl = RouteLayer(encoder=encoder, routes=routes)
    
    with pytest.raises(ValueError, match="Empty"):
        rl("")
    
    with pytest.raises(ValueError, match="Empty"):
        rl("   ")


def test_route_layer_list_routes():
    """Test listing route names"""
    encoder = MockEncoder()
    routes = [
        Route(name="billing", utterances=["billing"]),
        Route(name="support", utterances=["support"])
    ]
    
    rl = RouteLayer(encoder=encoder, routes=routes)
    names = rl.list_route_names()
    
    assert len(names) == 2
    assert "billing" in names
    assert "support" in names
