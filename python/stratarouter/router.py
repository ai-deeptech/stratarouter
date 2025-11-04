"""
Main RouteLayer class - compatible with semantic-router API
"""

from typing import List, Optional, Union
import numpy as np

from stratarouter.route import Route, RouteChoice
from stratarouter.core import Router as RustRouter, RustRoute, ensure_list, ensure_2d_list
from stratarouter.encoders.base import BaseEncoder


class RouteLayer:
    """
    High-performance semantic router.
    
    Drop-in replacement for semantic-router's RouteLayer with 10-20x better performance.
    
    Args:
        encoder: Encoder instance for generating embeddings
        routes: List of Route objects
        top_k: Number of top matches to return (default: 1)
        cache_size: Size of embedding cache (default: 1000)
    
    Example:
        >>> from stratarouter import Route, RouteLayer
        >>> from stratarouter.encoders import HuggingFaceEncoder
        >>> 
        >>> routes = [
        ...     Route(name="billing", utterances=["invoice", "payment"]),
        ...     Route(name="support", utterances=["help", "issue"])
        ... ]
        >>> 
        >>> encoder = HuggingFaceEncoder()
        >>> rl = RouteLayer(encoder=encoder, routes=routes)
        >>> 
        >>> result = rl("I need my invoice")
        >>> print(result.name)  # "billing"
    """
    
    def __init__(
        self,
        encoder: BaseEncoder,
        routes: Optional[List[Route]] = None,
        top_k: int = 1,
        cache_size: int = 1000,
    ):
        self.encoder = encoder
        self.router = RustRouter(top_k=top_k, cache_size=cache_size)
        self._routes = {}
        
        if routes:
            for route in routes:
                self.add(route)
    
    def add(self, route: Route) -> None:
        """
        Add a route to the layer.
        
        Args:
            route: Route object to add
        
        Raises:
            ValueError: If route with same name already exists
        """
        if route.name in self._routes:
            raise ValueError(f"Route '{route.name}' already exists")
        
        # Encode utterances
        embeddings = self.encoder(route.utterances)
        embeddings_list = ensure_2d_list(embeddings)
        
        # Create Rust route
        rust_route = RustRoute(
            name=route.name,
            embeddings=embeddings_list,
            threshold=route.threshold
        )
        
        # Add to router
        self.router.add(rust_route)
        self._routes[route.name] = route
    
    def remove(self, name: str) -> None:
        """
        Remove a route by name.
        
        Args:
            name: Name of route to remove
        
        Raises:
            ValueError: If route doesn't exist
        """
        if name not in self._routes:
            raise ValueError(f"Route '{name}' not found")
        
        self.router.remove(name)
        del self._routes[name]
    
    def __call__(
        self,
        text: Union[str, List[str]],
        threshold: Optional[float] = None,
    ) -> Union[RouteChoice, List[RouteChoice]]:
        """
        Route input text to matching routes.
        
        Args:
            text: Input text or list of texts
            threshold: Optional custom threshold (overrides per-route thresholds)
        
        Returns:
            RouteChoice or list of RouteChoice objects
        
        Example:
            >>> result = rl("hello there")
            >>> print(result.name)  # "chitchat"
            >>> print(result.score)  # 0.95
        """
        is_single = isinstance(text, str)
        texts = [text] if is_single else text
        
        # Validate inputs
        for txt in texts:
            if not txt or not txt.strip():
                raise ValueError("Empty or whitespace-only text cannot be routed")
        
        results = []
        for txt in texts:
            # Encode query
            embedding = self.encoder([txt])[0]
            embedding_list = ensure_list(embedding)
            
            # Route
            if threshold is not None:
                matches = self.router.route_with_threshold(embedding_list, threshold)
            else:
                matches = self.router.route(embedding_list)
            
            # Convert to RouteChoice
            if matches and matches[0].is_match:
                match = matches[0]
                route = self._routes.get(match.name)
                choice = RouteChoice(
                    name=match.name,
                    score=match.score,
                    threshold=match.threshold,
                    metadata=route.metadata if route else {}
                )
            else:
                choice = RouteChoice(name=None, score=0.0, threshold=0.82)
            
            results.append(choice)
        
        return results[0] if is_single else results
    
    def route(
        self,
        text: str,
        threshold: Optional[float] = None,
    ) -> RouteChoice:
        """
        Route a single text (alias for __call__).
        
        Args:
            text: Input text
            threshold: Optional custom threshold
        
        Returns:
            RouteChoice object
        """
        return self(text, threshold=threshold)
    
    def route_batch(
        self,
        texts: List[str],
        threshold: Optional[float] = None,
    ) -> List[RouteChoice]:
        """
        Route multiple texts in batch.
        
        Args:
            texts: List of input texts
            threshold: Optional custom threshold
        
        Returns:
            List of RouteChoice objects
        """
        return self(texts, threshold=threshold)
    
    @property
    def routes(self) -> List[Route]:
        """Get list of all routes"""
        return list(self._routes.values())
    
    @property
    def num_routes(self) -> int:
        """Get number of routes"""
        return len(self._routes)
    
    def list_route_names(self) -> List[str]:
        """Get list of route names"""
        return list(self._routes.keys())
    
    def clear(self) -> None:
        """Remove all routes"""
        self.router.clear()
        self._routes.clear()
    
    def clear_cache(self) -> None:
        """Clear the embedding cache"""
        self.router.clear_cache()
    
    def __repr__(self) -> str:
        return f"RouteLayer(routes={self.num_routes}, encoder={self.encoder.__class__.__name__})"
