"""
Base encoder interface
"""

from abc import ABC, abstractmethod
from typing import List
import numpy as np


class BaseEncoder(ABC):
    """
    Abstract base class for all encoders.
    
    All encoder implementations must inherit from this class and implement
    the __call__ method.
    """
    
    @abstractmethod
    def __call__(self, texts: List[str]) -> np.ndarray:
        """
        Encode a list of texts into embeddings.
        
        Args:
            texts: List of texts to encode
        
        Returns:
            Numpy array of shape (len(texts), embedding_dim)
        """
        pass
    
    @property
    @abstractmethod
    def dim(self) -> int:
        """Return the embedding dimension"""
        pass
