"""
OpenAI encoder implementation
"""

from typing import List, Optional
import numpy as np

try:
    from openai import OpenAI
except ImportError:
    raise ImportError(
        "OpenAI not installed. Install with: pip install stratarouter[openai]"
    )

from stratarouter.encoders.base import BaseEncoder


class OpenAIEncoder(BaseEncoder):
    """
    OpenAI embedding encoder.
    
    Uses OpenAI's embedding API to generate embeddings.
    
    Args:
        model: Model name (default: "text-embedding-3-small")
        api_key: OpenAI API key (reads from OPENAI_API_KEY env var if not provided)
    
    Example:
        >>> encoder = OpenAIEncoder()
        >>> embeddings = encoder(["hello", "world"])
        >>> print(embeddings.shape)  # (2, 1536)
    """
    
    def __init__(
        self,
        model: str = "text-embedding-3-small",
        api_key: Optional[str] = None,
    ):
        self.model = model
        self.client = OpenAI(api_key=api_key)
        self._dim = self._get_dimension()
    
    def __call__(self, texts: List[str]) -> np.ndarray:
        """Encode texts using OpenAI API"""
        if not texts:
            return np.array([])
        
        try:
            response = self.client.embeddings.create(
                model=self.model,
                input=texts
            )
            
            embeddings = [item.embedding for item in response.data]
            return np.array(embeddings, dtype=np.float32)
        except Exception as e:
            raise RuntimeError(f"OpenAI API error: {e}") from e
    
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
            if "text-embedding-3-small" in self.model:
                return 1536
            elif "text-embedding-3-large" in self.model:
                return 3072
            elif "text-embedding-ada-002" in self.model:
                return 1536
            return 1536  # Default fallback
    
    def __repr__(self) -> str:
        return f"OpenAIEncoder(model='{self.model}', dim={self.dim})"
