"""LangChain integration for StrataRouter"""

from typing import Dict, List, Any, Optional, Callable


class StrataRouterChain:
    """LangChain Chain wrapper for StrataRouter
    
    Examples:
        >>> from langchain.chains import LLMChain
        >>> router_chain = StrataRouterChain.from_routes(
        ...     chains={
        ...         "billing": billing_chain,
        ...         "support": support_chain
        ...     },
        ...     routes=[...]
        ... )
        >>> result = router_chain.run("Where's my invoice?")
    """
    
    def __init__(
        self,
        router: Any,
        chains: Dict[str, Any],
        fallback_chain: Optional[Any] = None,
        threshold: float = 0.5
    ):
        """Initialize router chain
        
        Args:
            router: StrataRouter instance
            chains: Dict mapping route IDs to LangChain chains
            fallback_chain: Optional fallback chain for low confidence
            threshold: Minimum confidence threshold
        """
        if not chains:
            raise ValueError("At least one chain must be provided")
        
        self.router = router
        self.chains = chains
        self.fallback_chain = fallback_chain
        self.threshold = threshold
    
    @classmethod
    def from_routes(
        cls,
        chains: Dict[str, Any],
        routes: List[Any],
        encoder: Optional[str] = None,
        **kwargs
    ) -> "StrataRouterChain":
        """Create chain from routes
        
        Args:
            chains: Dict of route_id -> chain
            routes: List of Route objects
            encoder: Encoder model name
            **kwargs: Additional args for Router
        """
        from ..router import Router
        
        router = Router(encoder=encoder, **kwargs)
        for route in routes:
            router.add(route)
        router.build_index()
        
        return cls(router, chains, **kwargs)
    
    def run(self, query: str, **kwargs) -> Any:
        """Route and execute chain
        
        Args:
            query: Input query
            **kwargs: Additional arguments for chain execution
            
        Returns:
            Chain output
        """
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")
        
        result = self.router.route(query)
        
        if result.confidence < self.threshold:
            if self.fallback_chain:
                return self.fallback_chain.run(query, **kwargs)
            raise ValueError(
                f"Low confidence ({result.confidence:.2f}) and no fallback chain"
            )
        
        chain = self.chains.get(result.route_id)
        if not chain:
            if self.fallback_chain:
                return self.fallback_chain.run(query, **kwargs)
            raise KeyError(f"No chain found for route: {result.route_id}")
        
        return chain.run(query, **kwargs)
    
    def __call__(self, query: str, **kwargs) -> Any:
        """Alias for run()"""
        return self.run(query, **kwargs)


class StrataRouterRetriever:
    """LangChain Retriever wrapper for StrataRouter"""
    
    def __init__(
        self,
        router: Any,
        retrievers: Dict[str, Any],
        fallback_retriever: Optional[Any] = None
    ):
        """Initialize router retriever
        
        Args:
            router: StrataRouter instance
            retrievers: Dict mapping route IDs to retrievers
            fallback_retriever: Optional fallback retriever
        """
        if not retrievers:
            raise ValueError("At least one retriever must be provided")
        
        self.router = router
        self.retrievers = retrievers
        self.fallback_retriever = fallback_retriever
    
    def get_relevant_documents(self, query: str) -> List[Any]:
        """Retrieve documents using routed retriever"""
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")
        
        result = self.router.route(query)
        
        retriever = self.retrievers.get(result.route_id)
        if not retriever:
            if self.fallback_retriever:
                retriever = self.fallback_retriever
            else:
                raise KeyError(f"No retriever for route: {result.route_id}")
        
        return retriever.get_relevant_documents(query)
    
    async def aget_relevant_documents(self, query: str) -> List[Any]:
        """Async version of get_relevant_documents"""
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")
        
        result = self.router.route(query)
        
        retriever = self.retrievers.get(result.route_id)
        if not retriever:
            if self.fallback_retriever:
                retriever = self.fallback_retriever
            else:
                raise KeyError(f"No retriever for route: {result.route_id}")
        
        if hasattr(retriever, 'aget_relevant_documents'):
            return await retriever.aget_relevant_documents(query)
        return retriever.get_relevant_documents(query)


class StrataRouterRunnable:
    """LangChain LCEL Runnable wrapper"""
    
    def __init__(self, router: Any, handlers: Dict[str, Callable]):
        """Initialize runnable
        
        Args:
            router: StrataRouter instance
            handlers: Dict of route_id -> callable handler
        """
        if not handlers:
            raise ValueError("At least one handler must be provided")
        
        self.router = router
        self.handlers = handlers
    
    def invoke(self, input: Dict[str, Any], config: Optional[Dict] = None) -> Any:
        """Invoke routing and handler"""
        query = input.get("query") or input.get("input")
        if not query:
            raise ValueError("Input must contain 'query' or 'input' key")
        
        result = self.router.route(query)
        handler = self.handlers.get(result.route_id)
        
        if not handler:
            raise KeyError(f"No handler for route: {result.route_id}")
        
        input["route_result"] = result
        return handler(input, config)
    
    async def ainvoke(self, input: Dict[str, Any], config: Optional[Dict] = None) -> Any:
        """Async invoke"""
        query = input.get("query") or input.get("input")
        if not query:
            raise ValueError("Input must contain 'query' or 'input' key")
        
        result = self.router.route(query)
        handler = self.handlers.get(result.route_id)
        
        if not handler:
            raise KeyError(f"No handler for route: {result.route_id}")
        
        input["route_result"] = result
        
        if hasattr(handler, 'ainvoke'):
            return await handler.ainvoke(input, config)
        return handler(input, config)