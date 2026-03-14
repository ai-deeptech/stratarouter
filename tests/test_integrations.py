"""Integration smoke tests.

These tests verify that the integration adapters are importable and expose
the expected public symbols. They do NOT make live API calls.

Full end-to-end integration tests are in integrations/ and require the
respective framework packages to be installed.
"""

import importlib

import pytest

INTEGRATION_MODULES = [
    ("stratarouter.integrations.langchain",          "StrataRouterChain"),
    ("stratarouter.integrations.langgraph",          "create_routing_graph"),
    ("stratarouter.integrations.crewai",             "RoutedAgent"),
    ("stratarouter.integrations.autogen",            "StrataRouterGroupChat"),
    ("stratarouter.integrations.openai_assistants",  "StrataRouterAssistant"),
    ("stratarouter.integrations.google_agent",       "StrataRouterVertexAI"),
    ("stratarouter.integrations.generic",            "GenericRouter"),
]


@pytest.mark.parametrize("module_path,symbol", INTEGRATION_MODULES)
def test_integration_module_exposes_symbol(module_path, symbol):
    """Each integration module must be importable and expose its primary class."""
    try:
        mod = importlib.import_module(module_path)
        assert hasattr(mod, symbol), (
            f"{module_path} does not expose '{symbol}'. "
            "Check integrations/__init__.py or the module file."
        )
    except ImportError as exc:
        pytest.skip(f"Optional dependency not installed: {exc}")
