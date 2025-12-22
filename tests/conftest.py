"""Pytest configuration and fixtures"""
import pytest
from stratarouter_core.stratarouter_core import PyRouter as Router, PyRoute as Route
import random

@pytest.fixture
def sample_routes():
    """Sample routes for testing"""
    billing = Route("billing")
    billing.description = "Billing and payment questions"
    billing.examples = ["Where's my invoice?", "I need a refund"]
    billing.keywords = ["invoice", "billing", "payment", "refund"]
    
    support = Route("support")
    support.description = "Technical support questions"
    support.examples = ["App is crashing", "Can't login"]
    support.keywords = ["crash", "bug", "error", "login"]
    
    sales = Route("sales")
    sales.description = "Sales inquiries"
    sales.examples = ["I want to buy", "Pricing information"]
    sales.keywords = ["buy", "purchase", "pricing", "upgrade"]
    
    return [billing, support, sales]

@pytest.fixture
def sample_embeddings():
    """Sample embeddings for testing"""
    random.seed(42)
    return [
        [random.gauss(0, 1) for _ in range(384)],
        [random.gauss(0, 1) for _ in range(384)],
        [random.gauss(0, 1) for _ in range(384)]
    ]

@pytest.fixture
def router_with_routes(sample_routes, sample_embeddings):
    """Router with routes already added and index built"""
    router = Router(dimension=384, threshold=0.3)
    
    for route in sample_routes:
        router.add_route(route)
    
    router.build_index(sample_embeddings)
    return router

@pytest.fixture
def query_embedding():
    """Sample query embedding"""
    random.seed(123)
    return [random.gauss(0, 1) for _ in range(384)]
