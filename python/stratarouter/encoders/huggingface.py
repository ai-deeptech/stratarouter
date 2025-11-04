"""
HuggingFace encoder implementation
"""

from typing import List, Optional
import numpy as np

try:
    from sentence_transformers import SentenceTransformer
except ImportError:
    raise ImportError(
        "sentence-transformers not installed. Install with: pip install stratarouter[huggingface]"
    )

from stratarouter.encoders.base import BaseEncoder


class HuggingFaceEncoder(BaseEncoder):
    """
    HuggingFace SentenceTransformer encoder.
    
    Uses local sentence-transformers models for fast, offline embedding generation.
    
    Args:
        model: Model name or path (default: "all-MiniLM-L6-v2")
        device: Device to run on ("cpu" or "cuda", default: auto-detect)
    
    Example:
        >>> encoder = HuggingFaceEncoder()
        >>> embeddings = encoder(["hello", "world"])
        >>> print(embeddings.shape)  # (2, 384)
    """
    
    def __init__(
        self,
        model: str = "all-MiniLM-L6-v2",
        device: Optional[str] = None,
    ):
        self

"""
HuggingFace encoder implementation
"""

from typing import List, Optional
import numpy as np

try:
    from sentence_transformers import SentenceTransformer
except ImportError:
    raise ImportError(
        "sentence-transformers not installed. Install with: pip install stratarouter[huggingface]"
    )

from stratarouter.encoders.base import BaseEncoder


class HuggingFaceEncoder(BaseEncoder):
    """
    HuggingFace SentenceTransformer encoder.
    
    Uses local sentence-transformers models for fast, offline embedding generation.
    
    Args:
        model: Model name or path (default: "all-MiniLM-L6-v2")
        device: Device to run on ("cpu" or "cuda", default: auto-detect)
    
    Example:
        >>> encoder = HuggingFaceEncoder()
        >>> embeddings = encoder(["hello", "world"])
        >>> print(embeddings.shape)  # (2, 384)
    """
    
    def __init__(
        self,
        model: str = "all-MiniLM-L6-v2",
        device: Optional[str] = None,
    ):
        self.model_name = model
        self.model = SentenceTransformer(model, device=device)
        self._dim = self.model.get_sentence_embedding_dimension()
    
    def __call__(self, texts: List[str]) -> np.ndarray:
        """Encode texts using SentenceTransformer"""
        if not texts:
            return np.array([])
        
        try:
            embeddings = self.model.encode(
                texts,
                show_progress_bar=False,
                convert_to_numpy=True
            )
            
            return embeddings.astype(np.float32)
        except Exception as e:
            raise RuntimeError(f"HuggingFace encoding error: {e}") from e
    
    @property
    def dim(self) -> int:
        """Return embedding dimension"""
        return self._dim
    
    def __repr__(self) -> str:
        return f"HuggingFaceEncoder(model='{self.model_name}', dim={self.dim})"
