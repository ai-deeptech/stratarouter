"""HuggingFace encoder implementation"""

from typing import List, Union

import numpy as np

from .base import BaseEncoder


class HuggingFaceEncoder(BaseEncoder):
    """HuggingFace Sentence Transformers encoder

    Uses sentence-transformers library for encoding.

    Examples:
        >>> encoder = HuggingFaceEncoder("sentence-transformers/all-MiniLM-L6-v2")
        >>> embedding = encoder.encode("Hello world")
        >>> embedding.shape
        (384,)
    """

    def __init__(self, model_name: str = "sentence-transformers/all-MiniLM-L6-v2",
                 device: str = "cpu", **kwargs):
        """Initialize HuggingFace encoder

        Args:
            model_name: Name of the sentence-transformers model
            device: Device to run on ("cpu" or "cuda")
            **kwargs: Additional arguments for SentenceTransformer
        """
        try:
            from sentence_transformers import SentenceTransformer
        except ImportError as e:
            raise ImportError(
                "sentence-transformers not installed. "
                "Install: pip install stratarouter[huggingface]"
            ) from e

        self.model_name = model_name
        self.model = SentenceTransformer(model_name, device=device, **kwargs)
        self._dimension = self.model.get_sentence_embedding_dimension()

    def encode(self, text: Union[str, List[str]]) -> np.ndarray:
        """Encode text using sentence-transformers

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
            embeddings = self.model.encode(text, convert_to_numpy=True)

            if single:
                return embeddings[0]
            return embeddings
        except Exception as e:
            raise RuntimeError(f"Encoding failed: {e}") from e

    @property
    def dimension(self) -> int:
        """Get embedding dimension"""
        return self._dimension
