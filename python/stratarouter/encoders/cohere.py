"""
Cohere encoder implementation
"""

from typing import List, Optional
import numpy as np

try:
    import cohere
except ImportError:
    raise ImportError(
        "Cohere not installed. Install with: pip install stratarouter[cohere]"
    )

from stratarouter.encoders.base import BaseEncoder


class CohereEncoder(BaseEncoder):
    """
    Cohere embedding encoder.
    
    Uses Cohere's embedding API to generate embeddings.
    
    Args:
        model: Model name (default: "embed-english-v3.0")
        api_key: Cohere API key (reads from COHERE_API_KEY env var if not provided)
        input_type: Input type for embeddings (default: "search_query")
    
    Example:
        >>> encoder = CohereEncoder()
        >>> embeddings = encoder(["hello", "world"])
        >>> print(embeddings.shape)  # (2, 1024)
    """
    
    def __init__(
        self,
        model: str = "embed-english-v3.0",
        api_key: Optional[str] = None,
        input_type: str = "search_query",
    ):
        self.model = model
        self.input_type = input_type
        self.client = cohere.Client(api_key=api_key)
        self._dim = self._get_dimension()
    
    def __call__(self, texts: List[str]) -> np.ndarray:
        """Encode texts using Cohere API"""
        if not texts:
            return np.array([])
        
        try:
            response = self.client.embed(
                texts=texts,
                model=self.model,
                input_type=self.input_type
            )
            
            return np.array(response.embeddings, dtype=np.float32)
        except Exception as e:
            raise RuntimeError(f"Cohere API error: {e}") from e
    
    @property
    def dim(self) -> int:
        """Return embedding dimension"""
        return self._dim
    
    def _get_dimension(self) -> int:
        """Get embedding dimension by encoding a test string"""
        try:
            test_embedding = self(["test"])[0]
            return len(test_embedding)
        except Exception:
            # Default dimensions for known models
            if "embed-english-v3.0" in self.model:
                return 1024
            elif "embed-multilingual-v3.0" in self.model:
                return 1024
            return 1024  # Default fallback
    
    def __repr__(self) -> str:
        return f"CohereEncoder(model='{self.model}', dim={self.dim})"
