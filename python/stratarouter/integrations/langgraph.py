"""LangGraph integration for StrataRouter"""

from typing import Any, Callable, Dict, Optional


class StrataRouterNode:
    """LangGraph node for routing"""

    def __init__(self, router: Any, route_key: str = "route_id"):
        """Initialize router node

        Args:
            router: StrataRouter instance
            route_key: Key to store route_id in state
        """
        if not router:
            raise ValueError("Router cannot be None")

        self.router = router
        self.route_key = route_key

    def __call__(self, state: Dict[str, Any]) -> Dict[str, Any]:
        """Execute routing"""
        query = state.get("query") or state.get("input") or state.get("messages", [""])[-1]

        if isinstance(query, dict):
            query = query.get("content", "")
        elif isinstance(query, list):
            query = query[-1] if query else ""

        if not query:
            raise ValueError("No query found in state")

        result = self.router.route(str(query))

        # Add route info to state
        state[self.route_key] = result.route_id
        state["route_confidence"] = result.confidence
        state["route_scores"] = result.scores

        return state


def create_routing_graph(
    router: Any,
    handlers: Dict[str, Callable],
    state_class: Any,
    fallback_handler: Optional[Callable] = None
) -> Any:
    """Create a complete LangGraph with routing"""
    try:
        from langgraph.graph import END, StateGraph
    except ImportError as e:
        raise ImportError(
            "LangGraph not installed. "
            "Install: pip install stratarouter[langgraph]"
        ) from e

    if not handlers:
        raise ValueError("At least one handler must be provided")

    # Create graph
    graph = StateGraph(state_class)

    # Add router node
    router_node = StrataRouterNode(router)
    graph.add_node("router", router_node)

    # Add handler nodes
    for route_id, handler in handlers.items():
        graph.add_node(route_id, handler)

    # Add fallback if provided
    if fallback_handler:
        graph.add_node("fallback", fallback_handler)

    # Set entry point
    graph.set_entry_point("router")

    # Add conditional edges
    def route_decision(state: Dict[str, Any]) -> str:
        route_id = state.get("route_id")
        confidence = state.get("route_confidence", 1.0)

        if confidence < 0.5 and fallback_handler:
            return "fallback"

        if route_id in handlers:
            return route_id

        if fallback_handler:
            return "fallback"

        return END

    graph.add_conditional_edges("router", route_decision)

    # Connect handlers to END
    for route_id in handlers:
        graph.add_edge(route_id, END)

    if fallback_handler:
        graph.add_edge("fallback", END)

    return graph.compile()
