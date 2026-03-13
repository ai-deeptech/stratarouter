"""Framework integrations for StrataRouter"""

__all__ = []

# Conditional imports
try:
    from .langchain import StrataRouterChain, StrataRouterRetriever, StrataRouterRunnable  # noqa: F401, I001
    __all__.extend(["StrataRouterChain", "StrataRouterRetriever", "StrataRouterRunnable"])
except ImportError:
    pass

try:
    from .langgraph import StrataRouterNode, create_routing_graph  # noqa: F401
    __all__.extend(["StrataRouterNode", "create_routing_graph"])
except ImportError:
    pass

try:
    from .crewai import RoutedAgent, StrataRouterCrew  # noqa: F401
    __all__.extend(["StrataRouterCrew", "RoutedAgent"])
except ImportError:
    pass

try:
    from .autogen import StrataRouterGroupChat  # noqa: F401
    __all__.extend(["StrataRouterGroupChat"])
except ImportError:
    pass

try:
    from .openai_assistants import StrataRouterAssistant  # noqa: F401
    __all__.extend(["StrataRouterAssistant"])
except ImportError:
    pass

try:
    from .google_agent import StrataRouterVertexAI  # noqa: F401
    __all__.extend(["StrataRouterVertexAI"])
except ImportError:
    pass

from .generic import GenericRouterAdapter  # noqa: F401

__all__.append("GenericRouterAdapter")
