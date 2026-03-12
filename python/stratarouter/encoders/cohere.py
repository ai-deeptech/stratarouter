"""Cohere embedding encoder."""

from __future__ import annotations

from typing import List, Optional, Union

import numpy as np

from .base import BaseEncoder

__all__ = ["CohereEncoder"]


class CohereEncoder(BaseEncoder):
    """Cohere embedding encoder.

    Uses the Cohere Python SDK to generate embeddings via the Cohere API.

    Parameters
    ----------
    model:
        Cohere embedding model name (default: ``"embed-english-v3.0"``).
    api_key:
        Cohere API key.  Falls back to the ``COHERE_API_KEY`` environment
        variable if not supplied.
    input_type:
        Cohere ``input_type`` parameter.  Use ``"search_query"`` for queries
        and ``"search_document"`` for documents.

    Example
    -------
    >>> encoder = CohereEncoder(api_key="...")
    >>> emb = encoder.encode("Hello world")
    >>> emb.shape
    (1024,)
    """

    # Default dimensions for well-known Cohere models
    _KNOWN_DIMS = {
        "embed-english-v3.0": 1024,
        "embed-multilingual-v3.0": 1024,
        "embed-english-light-v3.0": 384,
        "embed-multilingual-light-v3.0": 384,
        "embed-english-v2.0": 4096,
        "embed-english-light-v2.0": 1024,
        "embed-multilingual-v2.0": 768,
    }

    def __init__(
        self,
        model: str = "embed-english-v3.0",
        api_key: Optional[str] = None,
        input_type: str = "search_query",
    ) -> None:
        try:
            import cohere  # noqa: PLC0415 — lazy import for optional dependency
        except ImportError as exc:
            raise ImportError(
                "Cohere package not installed.  "
                "Install with: pip install stratarouter[cohere]"
            ) from exc

        self.model = model
        self.input_type = input_type
        # cohere ≥ 5 uses ClientV2; fall back to Client for older versions.
        client_cls = getattr(cohere, "ClientV2", None) or cohere.Client
        self._client = client_cls(api_key=api_key)
        self._dimension: int = self._KNOWN_DIMS.get(model, 1024)

    # ── BaseEncoder interface ─────────────────────────────────────────────────

    def encode(self, text: Union[str, List[str]]) -> np.ndarray:
        """Encode one or more texts using the Cohere embedding API.

        Parameters
        ----------
        text:
            A single string or a list of strings.

        Returns
        -------
        np.ndarray
            Shape ``(dimension,)`` for a single string or
            ``(n, dimension)`` for a list.
        """
        if not text:
            raise ValueError("text must be a non-empty string or list")

        single = isinstance(text, str)
        texts: List[str] = [text] if single else list(text)

        try:
            response = self._client.embed(
                texts=texts,
                model=self.model,
                input_type=self.input_type,
            )
            embeddings = np.array(response.embeddings, dtype=np.float32)
        except Exception as exc:
            raise RuntimeError(f"Cohere API error: {exc}") from exc

        return embeddings[0] if single else embeddings

    @property
    def dimension(self) -> int:
        """Embedding dimension for the selected model."""
        return self._dimension

    def __repr__(self) -> str:
        return f"CohereEncoder(model={self.model!r}, dimension={self.dimension})"
