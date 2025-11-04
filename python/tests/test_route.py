"""
Tests for Route and RouteChoice
"""

import pytest
from stratarouter import Route, RouteChoice


def test_route_creation():
    """Test creating a valid route"""
    route = Route(
        name="test",
        utterances=["hello", "hi", "hey"],
        threshold=0.75
    )
    
    assert route.name == "test"
    assert len(route.utterances) == 3
    assert route.threshold == 0.75


def test_route_validation_empty_name():
    """Test route with empty name raises error"""
    with pytest.raises(ValueError, match="cannot be empty"):
        Route(name="", utterances=["test"])


def test_route_validation_no_utterances():
    """Test route without utterances raises error"""
    with pytest.raises(ValueError, match="at least one utterance"):
        Route(name="test", utterances=[])


def test_route_validation_invalid_threshold():
    """Test route with invalid threshold raises error"""
    with pytest.raises(ValueError, match="between 0.0 and 1.0"):
        Route(name="test", utterances=["test"], threshold=1.5)
    
    with pytest.raises(ValueError, match="between 0.0 and 1.0"):
        Route(name="test", utterances=["test"], threshold=-0.1)


def test_route_with_metadata():
    """Test route with metadata"""
    route = Route(
        name="test",
        utterances=["test"],
        metadata={"category": "general", "priority": 1}
    )
    
    assert route.metadata["category"] == "general"
    assert route.metadata["priority"] == 1


def test_route_with_description():
    """Test route with description"""
    route = Route(
        name="billing",
        utterances=["invoice", "payment"],
        description="Questions about billing and payments"
    )
    
    assert route.description == "Questions about billing and payments"


def test_route_choice_match():
    """Test RouteChoice with match"""
    choice = RouteChoice(
        name="test",
        score=0.85,
        threshold=0.75
    )
    
    assert choice.name == "test"
    assert choice.score == 0.85
    assert choice.is_match
    assert bool(choice)


def test_route_choice_no_match():
    """Test RouteChoice without match"""
    choice = RouteChoice(
        name=None,
        score=0.65,
        threshold=0.75
    )
    
    assert choice.name is None
    assert not choice.is_match
    assert not bool(choice)


def test_route_choice_below_threshold():
    """Test RouteChoice below threshold"""
    choice = RouteChoice(
        name="test",
        score=0.70,
        threshold=0.75
    )
    
    assert not choice.is_match
    assert not bool(choice)


def test_route_choice_repr():
    """Test RouteChoice string representation"""
    choice = RouteChoice(name="test", score=0.8765, threshold=0.75)
    repr_str = repr(choice)
    
    assert "test" in repr_str
    assert "0.8765" in repr_str
    assert "0.75" in repr_str


def test_route_utterances_cleaned():
    """Test that utterances are cleaned"""
    route = Route("""
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
        name="test",
        utterances=["  hello  ", "world", "", "  "]
    )
    
    assert len(route.utterances) == 2
    assert route.utterances == ["hello", "world"]
