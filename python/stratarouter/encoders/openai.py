"""OpenAI encoder implementation"""

from typing import Union, List, Optional
import numpy as np
from .base import BaseEncoder


class OpenAIEncoder(BaseEncoder):
    """OpenAI embeddings encoder
    
    Uses OpenAI's embedding API for encoding.
    
    Examples:
        >>> encoder = OpenAIEncoder(api_key="sk-...")
        >>> embedding = encoder.encode("Hello world")
        >>> embedding.shape
        (1536,)
    """
    
    def __init__(
        self,
        model: str = "text-embedding-3-small",
        api_key: Optional[str] = None,
        organization: Optional[str] = None,
        **kwargs
    ):
        """Initialize OpenAI encoder
        
        Args:
            model: Model name (text-embedding-3-small, text-embedding-3-large, etc.)
            api_key: OpenAI API key (or set OPENAI_API_KEY env var)
            organization: OpenAI organization ID
            **kwargs: Additional arguments for OpenAI client
        """
        try:
            from openai import OpenAI
        except ImportError as e:
            raise ImportError(
                "OpenAI package not installed. "
                "Install: pip install stratarouter[openai]"
            ) from e
        
        self.model = model
        self.client = OpenAI(
            api_key=api_key,
            organization=organization,
            **kwargs
        )
        
        # Dimension mapping
        self._dimension = {
            "text-embedding-3-small": 1536,
            "text-embedding-3-large": 3072,
            "text-embedding-ada-002": 1536,
        }.get(model, 1536)
    
    def encode(self, text: Union[str, List[str]]) -> np.ndarray:
        """Encode text using OpenAI API
        
        Args:
            text: Single text or list of texts
            
        Returns:
            numpy array of embeddings
        """
        if not text:
            raise ValueError("Text cannot be empty")
        
        if isinstance(text, str):
            text = [text]
            single = True
        else:
            single = False
        
        try:
            response = self.client.embeddings.create(
                model=self.model,
                input=text
            )
            
            embeddings = np.array([item.embedding for item in response.data])
            
            if single:
                return embeddings[0]
            return embeddings
            
        except Exception as e:
            raise RuntimeError(f"OpenAI API error: {e}") from e
    
    @property
    def dimension(self) -> int:
        """Get embedding dimension"""
        return self._dimension
