"""Base encoder interface"""

from abc import ABC, abstractmethod
from typing import Union, List
import numpy as np


class BaseEncoder(ABC):
    """Base encoder abstract class
    
    All encoder implementations must inherit from this class
    and implement the required methods.
    """
    
    @abstractmethod
    def encode(self, text: Union[str, List[str]]) -> np.ndarray:
        """Encode text to embeddings
        
        Args:
            text: Single text string or list of strings
            
        Returns:
            numpy array of embeddings
            - For single text: shape (dimension,)
            - For list of texts: shape (n_texts, dimension)
        """
        pass
    
    @property
    @abstractmethod
    def dimension(self) -> int:
        """Get embedding dimension
        
        Returns:
            Integer dimension of embeddings
        """
        pass
