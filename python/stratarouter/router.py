"""Main Router implementation with all bug fixes applied"""

from typing import List, Optional, Dict, Any, Union
from enum import Enum
from pathlib import Path
import time
import json
import warnings
import threading

from .types import Route, RouteResult
from .encoders.base import BaseEncoder


class DeploymentMode(str, Enum):
    """Deployment modes"""
    LOCAL = "local"
    CLOUD = "cloud"


class Router:
    """
    High-performance semantic router
    
    Examples:
        >>> router = Router(mode="local")
        >>> router.add(Route(id="billing", keywords=["invoice"]))
        >>> result = router.route("I need my invoice")
        >>> print(result.route_id)
        'billing'
    """
    
    def __init__(
        self,
        encoder: Optional[Union[str, BaseEncoder]] = None,
        mode: DeploymentMode = DeploymentMode.LOCAL,
        api_key: Optional[str] = None,
        dimension: int = 384,
        threshold: float = 0.5,
        **kwargs
    ):
        # Validation
        if dimension <= 0:
            raise ValueError("dimension must be positive")
        
        if not 0.0 <= threshold <= 1.0:
            raise ValueError("threshold must be between 0 and 1")
        
        self.mode = DeploymentMode(mode)
        self.dimension = dimension
        self.threshold = threshold
        self.routes: Dict[str, Route] = {}
        self._index_built = False
        self._build_lock = threading.Lock()  # FIX: Thread safety
        
        if self.mode == DeploymentMode.LOCAL:
            self._init_local_mode(encoder)
        elif self.mode == DeploymentMode.CLOUD:
            self._init_cloud_mode(api_key)
    
    def _init_local_mode(self, encoder: Optional[Union[str, BaseEncoder]]) -> None:
        """Initialize local mode with encoder validation"""
        if encoder is None:
            encoder = "sentence-transformers/all-MiniLM-L6-v2"
        
        if isinstance(encoder, str):
            encoder = self._load_encoder(encoder)
        
        # FIX: Validate encoder has required interface
        if not hasattr(encoder, 'encode'):
            raise TypeError("Encoder must have encode() method")
        if not hasattr(encoder, 'dimension'):
            raise TypeError("Encoder must have dimension property")
        
        self.encoder = encoder
        
        # Validate encoder dimension
        if hasattr(encoder, 'dimension') and encoder.dimension != self.dimension:
            warnings.warn(
                f"Using encoder dimension ({encoder.dimension}) "
                f"instead of specified dimension ({self.dimension})"
            )
            self.dimension = encoder.dimension
        
        # Initialize core
        try:
            from ._core import PyRouter
            self._core = PyRouter(dimension=self.dimension, threshold=self.threshold)
        except ImportError as e:
            raise ImportError(
                "Compiled core not found. Reinstall: "
                "pip install --force-reinstall stratarouter"
            ) from e
        except Exception as e:
            raise RuntimeError(f"Failed to initialize router: {e}") from e
    
    def _init_cloud_mode(self, api_key: Optional[str]) -> None:
        """Initialize cloud mode"""
        if not api_key:
            raise ValueError(
                "api_key required for cloud mode. "
                "Get one at: https://cloud.stratarouter.io"
            )
        
        self.api_key = api_key
        
        try:
            from .cloud.client import CloudClient
            self.cloud_client = CloudClient(api_key=api_key)
        except ImportError as e:
            raise ImportError(
                "Cloud dependencies not installed. "
                "Install: pip install stratarouter[cloud]"
            ) from e
    
    def _load_encoder(self, model_name: str) -> BaseEncoder:
        """Load encoder"""
        try:
            from .encoders.huggingface import HuggingFaceEncoder
            return HuggingFaceEncoder(model_name)
        except ImportError as e:
            raise ImportError(
                "HuggingFace encoder not available. "
                "Install: pip install stratarouter[huggingface]"
            ) from e
    
    def add(self, route: Route) -> None:
        """Add route"""
        if not route.id:
            raise ValueError("Route ID cannot be empty")
        
        if not route.examples and not route.description:
            raise ValueError(
                f"Route '{route.id}' must have examples or description"
            )
        
        if route.id in self.routes:
            warnings.warn(
                f"Route '{route.id}' already exists. Overwriting. "
                f"Call build_index() to update."
            )
        
        self.routes[route.id] = route
        self._index_built = False
    
    def build_index(self) -> None:
        """Build routing index with proper error handling"""
        # FIX: Thread-safe index building
        with self._build_lock:
            self._index_built = False  # FIX: Reset on failure
            
            if self.mode == DeploymentMode.CLOUD:
                warnings.warn("build_index() not needed in cloud mode")
                return
            
            if not self.routes:
                raise ValueError("No routes added. Use router.add(route) first.")
            
            embeddings = []
            for route in self.routes.values():
                text = route.examples[0] if route.examples else route.description
                if not text:
                    raise ValueError(f"Route '{route.id}' has no text to encode")
                
                try:
                    emb = self.encoder.encode(text)
                    if len(emb.shape) > 1:
                        emb = emb[0]
                    embeddings.append(emb.tolist())
                except Exception as e:
                    self._index_built = False  # FIX: Ensure flag is false
                    raise RuntimeError(f"Failed to encode route '{route.id}': {e}") from e
            
            try:
                self._core.build_index(embeddings)
                self._index_built = True
            except Exception as e:
                self._index_built = False  # FIX: Ensure flag is false
                raise RuntimeError(f"Failed to build index: {e}") from e
    
    def route(self, text: str, top_k: int = 1) -> RouteResult:
        """Route query with comprehensive error handling"""
        if not text or not text.strip():
            raise ValueError("Query text cannot be empty")
        
        start_time = time.perf_counter()
        
        if self.mode == DeploymentMode.CLOUD:
            return self._route_cloud(text)
        
        if not self._index_built:
            self.build_index()
        
        try:
            embedding = self.encoder.encode(text)
            if len(embedding.shape) > 1:
                embedding = embedding[0]
            
            # FIX: Early dimension check with clear error
            if len(embedding) != self.dimension:
                raise ValueError(
                    f"Encoder dimension mismatch: expected {self.dimension}, "
                    f"got {len(embedding)}. Check encoder configuration."
                )
        except ValueError:
            raise  # Re-raise ValueError as-is
        except Exception as e:
            raise RuntimeError(f"Failed to encode query: {e}") from e
        
        try:
            result_dict = self._core.route(text, embedding.tolist())
        except Exception as e:
            raise RuntimeError(f"Routing failed: {e}") from e
        
        latency_ms = (time.perf_counter() - start_time) * 1000
        
        return RouteResult(
            route_id=result_dict["route_id"],
            confidence=result_dict["confidence"],
            scores=result_dict["scores"],
            latency_ms=latency_ms,
        )
    
    def _route_cloud(self, text: str) -> RouteResult:
        """Route using cloud"""
        try:
            return self.cloud_client.route(text)
        except Exception as e:
            raise RuntimeError(f"Cloud routing failed: {e}") from e
    
    def save(self, path: str) -> None:
        """Save router with encoder configuration"""
        path_obj = Path(path)
        path_obj.parent.mkdir(parents=True, exist_ok=True)
        
        # FIX: Save encoder configuration
        encoder_config = None
        if hasattr(self, 'encoder'):
            encoder_config = {
                "type": self.encoder.__class__.__name__,
                "model_name": getattr(self.encoder, 'model_name', None),
                "dimension": self.encoder.dimension
            }
        
        state = {
            "version": "0.2.0",
            "mode": self.mode.value,
            "dimension": self.dimension,
            "threshold": self.threshold,
            "routes": [r.model_dump() for r in self.routes.values()],
            "encoder_config": encoder_config,  # FIX: Include encoder config
        }
        
        try:
            with open(path_obj, "w") as f:
                json.dump(state, f, indent=2)
        except Exception as e:
            raise RuntimeError(f"Failed to save: {e}") from e
    
    @classmethod
    def load(cls, path: str, **kwargs) -> "Router":
        """Load router with encoder validation"""
        path_obj = Path(path)
        
        if not path_obj.exists():
            raise FileNotFoundError(f"Router file not found: {path}")
        
        try:
            with open(path_obj) as f:
                state = json.load(f)
        except json.JSONDecodeError as e:
            raise ValueError(f"Invalid router file: {e}") from e
        
        required = ["version", "dimension", "threshold", "routes"]
        missing = [k for k in required if k not in state]
        if missing:
            raise ValueError(f"Missing required keys: {', '.join(missing)}")
        
        # FIX: Validate encoder compatibility
        if 'encoder_config' in state and 'encoder' not in kwargs:
            encoder_type = state['encoder_config'].get('type', 'unknown')
            warnings.warn(
                f"Router was saved with encoder {encoder_type}. "
                f"Pass encoder=... to load() for best results."
            )
        
        router = cls(
            mode=kwargs.get("mode", state.get("mode", "local")),
            dimension=state["dimension"],
            threshold=state["threshold"],
            **kwargs
        )
        
        for route_data in state["routes"]:
            try:
                router.add(Route(**route_data))
            except Exception as e:
                warnings.warn(f"Failed to load route: {e}")
        
        if router.mode == DeploymentMode.LOCAL:
            try:
                router.build_index()
            except Exception as e:
                warnings.warn(f"Failed to build index: {e}")
        
        return router
