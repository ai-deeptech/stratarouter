"""Generic adapter for any framework"""

from typing import Dict, Any, Callable, Optional
from ..types import Route


class GenericRouterAdapter:
    """Generic routing adapter for any framework
    
    Provides a simple interface for routing to arbitrary handlers.
    
    Examples:
        >>> def handle_billing(query, context):
        ...     return f"Billing: {query}"
        >>> 
        >>> adapter = GenericRouterAdapter(
        ...     handlers={"billing": handle_billing},
        ...     router=router
        ... )
        >>> result = adapter.route("Where's my invoice?")
    """
    
    def __init__(
        self,
        handlers: Dict[str, Callable],
        router: Optional[Any] = None,
        routes: Optional[list] = None,
        fallback_handler: Optional[Callable] = None,
        threshold: float = 0.5,
        **router_kwargs
    ):
        """Initialize generic adapter
        
        Args:
            handlers: Dict mapping route_id to handler functions
            router: Optional existing router
            routes: Optional list of routes (if router not provided)
            fallback_handler: Optional fallback handler for low confidence
            threshold: Minimum confidence threshold
            **router_kwargs: Arguments for Router initialization
        """
        if not handlers:
            raise ValueError("At least one handler must be provided")
        
        if router is None:
            if not routes:
                raise ValueError("Must provide either router or routes")
            
            from ..router import Router
            router = Router(**router_kwargs)
            for route in routes:
                router.add(route)
            router.build_index()
        
        self.router = router
        self.handlers = handlers
        self.fallback_handler = fallback_handler
        self.threshold = threshold
    
    def route(
        self,
        query: str,
        context: Optional[Dict[str, Any]] = None,
        **kwargs
    ) -> Any:
        """Route query and execute handler
        
        Args:
            query: Input query
            context: Optional context dict passed to handler
            **kwargs: Additional arguments for handler
            
        Returns:
            Handler output
        """
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")
        
        result = self.router.route(query)
        
        # Check confidence
        if result.confidence < self.threshold:
            if self.fallback_handler:
                context = context or {}
                context["route_result"] = result
                return self.fallback_handler(query, context, **kwargs)
            raise ValueError(
                f"Low confidence ({result.confidence:.2f}) and no fallback handler"
            )
        
        # Get handler
        handler = self.handlers.get(result.route_id)
        if not handler:
            if self.fallback_handler:
                context = context or {}
                context["route_result"] = result
                return self.fallback_handler(query, context, **kwargs)
            raise KeyError(f"No handler found for route: {result.route_id}")
        
        # Execute handler
        context = context or {}
        context["route_result"] = result
        
        try:
            return handler(query, context, **kwargs)
        except Exception as e:
            raise RuntimeError(f"Handler execution failed: {e}") from e
    
    async def route_async(
        self,
        query: str,
        context: Optional[Dict[str, Any]] = None,
        **kwargs
    ) -> Any:
        """Async version of route
        
        Args:
            query: Input query
            context: Optional context
            **kwargs: Additional args
            
        Returns:
            Handler output
        """
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")
        
        result = self.router.route(query)
        
        if result.confidence < self.threshold:
            if self.fallback_handler:
                context = context or {}
                context["route_result"] = result
                
                if hasattr(self.fallback_handler, '__call__'):
                    return self.fallback_handler(query, context, **kwargs)
            raise ValueError(
                f"Low confidence ({result.confidence:.2f}) and no fallback handler"
            )
        
        handler = self.handlers.get(result.route_id)
        if not handler:
            if self.fallback_handler:
                context = context or {}
                context["route_result"] = result
                return self.fallback_handler(query, context, **kwargs)
            raise KeyError(f"No handler found for route: {result.route_id}")
        
        context = context or {}
        context["route_result"] = result
        
        # Try to call async if available
        if hasattr(handler, '__call__'):
            return handler(query, context, **kwargs)
        return await handler(query, context, **kwargs)
    
    def add_handler(self, route_id: str, handler: Callable) -> None:
        """Add a new handler
        
        Args:
            route_id: Route identifier
            handler: Handler function
        """
        if not route_id:
            raise ValueError("Route ID cannot be empty")
        if not callable(handler):
            raise TypeError("Handler must be callable")
        
        self.handlers[route_id] = handler
    
    def remove_handler(self, route_id: str) -> None:
        """Remove a handler
        
        Args:
            route_id: Route identifier
        """
        self.handlers.pop(route_id, None)