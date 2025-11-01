"""Framework integrations for StrataRouter"""

__all__ = []

# Conditional imports
try:
    from .langchain import StrataRouterChain, StrataRouterRetriever, StrataRouterRunnable
    __all__.extend(["StrataRouterChain", "StrataRouterRetriever", "StrataRouterRunnable"])
except ImportError:
    pass

try:
    from .langgraph import StrataRouterNode, create_routing_graph
    __all__.extend(["StrataRouterNode", "create_routing_graph"])
except ImportError:
    pass

try:
    from .crewai import StrataRouterCrew, RoutedAgent
    __all__.extend(["StrataRouterCrew", "RoutedAgent"])
except ImportError:
    pass

try:
    from .autogen import StrataRouterGroupChat
    __all__.extend(["StrataRouterGroupChat"])
except ImportError:
    pass

try:
    from .openai_assistants import StrataRouterAssistant
    __all__.extend(["StrataRouterAssistant"])
except ImportError:
    pass

try:
    from .google_agent import StrataRouterVertexAI
    __all__.extend(["StrataRouterVertexAI"])
except ImportError:
    pass

from .generic import GenericRouterAdapter
__all__.append("GenericRouterAdapter")
