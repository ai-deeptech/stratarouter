"""Cloud API client for StrataRouter with thread safety fix"""

from typing import Dict, Any, Optional, List
import json
import threading


class CloudClient:
    """Client for StrataRouter Cloud API
    
    Thread-safe implementation with per-thread HTTP clients.
    
    Examples:
        >>> client = CloudClient(api_key="sr-...")
        >>> result = client.route("Where's my invoice?")
    """
    
    def __init__(
        self,
        api_key: str,
        base_url: str = "https://api.stratarouter.io/v1",
        timeout: int = 30
    ):
        """Initialize cloud client
        
        Args:
            api_key: StrataRouter API key
            base_url: API base URL
            timeout: Request timeout in seconds
        """
        if not api_key:
            raise ValueError("API key cannot be empty")
        
        try:
            import httpx
        except ImportError as e:
            raise ImportError(
                "httpx not installed. "
                "Install: pip install stratarouter[cloud]"
            ) from e
        
        self.api_key = api_key
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self._httpx = httpx
        
        # FIX: Thread-local storage for HTTP clients
        self._local = threading.local()
        self._client_config = {
            "base_url": self.base_url,
            "headers": {
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
                "User-Agent": "StrataRouter-Python/0.2.0"
            },
            "timeout": timeout
        }
    
    @property
    def client(self):
        """Get thread-local HTTP client"""
        if not hasattr(self._local, 'client'):
            self._local.client = self._httpx.Client(**self._client_config)
        return self._local.client
    
    def route(self, text: str, top_k: int = 1) -> Any:
        """Route query via cloud API
        
        Args:
            text: Query text
            top_k: Number of results
            
        Returns:
            RouteResult object
        """
        if not text or not text.strip():
            raise ValueError("Text cannot be empty")
        
        from ..types import RouteResult
        
        try:
            response = self.client.post(
                "/route",
                json={"text": text, "top_k": top_k}
            )
            response.raise_for_status()
            
            data = response.json()
            
            return RouteResult(
                route_id=data["route_id"],
                confidence=data["confidence"],
                scores=data["scores"],
                latency_ms=data.get("latency_ms", 0.0)
            )
            
        except self._httpx.HTTPStatusError as e:
            raise RuntimeError(f"Cloud API HTTP error: {e.response.status_code}") from e
        except Exception as e:
            raise RuntimeError(f"Cloud API error: {e}") from e
    
    def add_route(self, route: Dict[str, Any]) -> Dict[str, Any]:
        """Add route to cloud router
        
        Args:
            route: Route configuration dict
            
        Returns:
            Response dict
        """
        try:
            response = self.client.post("/routes", json=route)
            response.raise_for_status()
            return response.json()
        except Exception as e:
            raise RuntimeError(f"Failed to add route: {e}") from e
    
    def list_routes(self) -> List[Dict[str, Any]]:
        """List all routes
        
        Returns:
            List of route dicts
        """
        try:
            response = self.client.get("/routes")
            response.raise_for_status()
            return response.json()["routes"]
        except Exception as e:
            raise RuntimeError(f"Failed to list routes: {e}") from e
    
    def delete_route(self, route_id: str) -> None:
        """Delete a route
        
        Args:
            route_id: Route identifier
        """
        if not route_id:
            raise ValueError("Route ID cannot be empty")
        
        try:
            response = self.client.delete(f"/routes/{route_id}")
            response.raise_for_status()
        except Exception as e:
            raise RuntimeError(f"Failed to delete route: {e}") from e
    
    def get_stats(self) -> Dict[str, Any]:
        """Get usage statistics
        
        Returns:
            Statistics dict
        """
        try:
            response = self.client.get("/stats")
            response.raise_for_status()
            return response.json()
        except Exception as e:
            raise RuntimeError(f"Failed to get stats: {e}") from e
    
    def __enter__(self):
        """Context manager entry"""
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.close()
    
    def close(self):
        """Close HTTP client"""
        if hasattr(self._local, 'client'):
            self._local.client.close()
            delattr(self._local, 'client')
