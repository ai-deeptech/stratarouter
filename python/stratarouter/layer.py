"""RouteLayer — high-level routing interface.

:class:`RouteLayer` is the recommended entry point for most users.  It is
intentionally compatible with ``semantic-router``'s ``RouteLayer`` class so
that existing code can be migrated with minimal changes.

Unlike the lower-level :class:`~stratarouter.Router`, ``RouteLayer`` works
entirely in pure Python and does **not** require the compiled Rust extension.
This makes it easy to unit-test with a ``MockEncoder``.

Example
-------
>>> from stratarouter import Route, RouteLayer
>>> from stratarouter.encoders.huggingface import HuggingFaceEncoder
>>>
>>> encoder = HuggingFaceEncoder()
>>> routes = [
...     Route(name="billing",  utterances=["invoice", "payment"]),
...     Route(name="support",  utterances=["help", "broken"]),
... ]
>>> rl = RouteLayer(encoder=encoder, routes=routes)
>>> choice = rl("Where is my invoice?")
>>> print(choice.name)   # "billing"
"""

from __future__ import annotations

from typing import Dict, List, Optional

import numpy as np

from .encoders.base import BaseEncoder
from .route import Route, RouteChoice

__all__ = ["RouteLayer"]


def _cosine_similarity(a: np.ndarray, b: np.ndarray) -> float:
    """Return cosine similarity between two 1-D float arrays."""
    a = np.asarray(a, dtype=np.float32).ravel()
    b = np.asarray(b, dtype=np.float32).ravel()
    norm_a = float(np.linalg.norm(a))
    norm_b = float(np.linalg.norm(b))
    if norm_a == 0.0 or norm_b == 0.0:
        return 0.0
    return float(np.dot(a, b) / (norm_a * norm_b))


class RouteLayer:
    """High-level semantic router.

    Parameters
    ----------
    encoder:
        Any :class:`~stratarouter.encoders.base.BaseEncoder` implementation
        (HuggingFace, OpenAI, Cohere, or a custom subclass).
    routes:
        Optional list of :class:`~stratarouter.Route` objects to register
        at construction time.
    threshold:
        Global default confidence threshold.  When supplied, it overrides
        each route's own threshold.  Route-level thresholds still apply
        unless this argument is set.

    Example
    -------
    >>> rl = RouteLayer(encoder=my_encoder, routes=[route1, route2])
    >>> choice = rl("Where is my invoice?")
    >>> if choice:
    ...     print(f"Matched: {choice.name} (score={choice.score:.2f})")
    """

    def __init__(
        self,
        encoder: BaseEncoder,
        routes: Optional[List[Route]] = None,
        threshold: Optional[float] = None,
    ) -> None:
        if not hasattr(encoder, "encode"):
            raise TypeError("encoder must implement encode(text) -> np.ndarray")
        if not hasattr(encoder, "dimension"):
            raise TypeError("encoder must expose a 'dimension' property")

        self.encoder = encoder
        self._global_threshold = threshold
        self._routes: Dict[str, Route] = {}
        self._embeddings: Dict[str, List[np.ndarray]] = {}  # name → per-utterance embeddings

        for route in routes or []:
            self.add(route)

    # ── Routing ───────────────────────────────────────────────────────────────

    def __call__(
        self,
        text: str,
        threshold: Optional[float] = None,
    ) -> RouteChoice:
        """Route ``text`` and return a :class:`~stratarouter.RouteChoice`.

        Parameters
        ----------
        text:
            Query string to route.
        threshold:
            Optional per-call threshold override.  Overrides both the
            global threshold and the matched route's own threshold.
        """
        if not text or not text.strip():
            raise ValueError("Empty query text — provide a non-empty string.")

        query_emb = self._encode_one(text)

        best_name: Optional[str] = None
        best_score: float = -2.0  # below any valid cosine value
        best_threshold: float = self._global_threshold or 0.82

        for name, route in self._routes.items():
            route_threshold = threshold or self._global_threshold or route.threshold

            for emb in self._embeddings[name]:
                score = _cosine_similarity(query_emb, emb)
                if score > best_score:
                    best_score = score
                    best_threshold = route_threshold
                    if score >= route_threshold:
                        best_name = name

        return RouteChoice(
            name=best_name,
            score=max(best_score, 0.0),
            threshold=best_threshold,
            metadata=self._routes[best_name].metadata if best_name else {},
        )

    def route_batch(
        self,
        texts: List[str],
        threshold: Optional[float] = None,
    ) -> List[RouteChoice]:
        """Route a list of queries, returning one :class:`RouteChoice` per item."""
        return [self(text, threshold=threshold) for text in texts]

    # ── Route management ──────────────────────────────────────────────────────

    def add(self, route: Route) -> None:
        """Register a route.

        Raises
        ------
        ValueError
            If a route with the same name is already registered.
        """
        if route.name in self._routes:
            raise ValueError(
                f"Route '{route.name}' already exists. "
                "Call remove(name) before re-adding."
            )
        embs = [self._encode_one(u) for u in route.utterances]
        self._routes[route.name] = route
        self._embeddings[route.name] = embs

    def remove(self, name: str) -> None:
        """Unregister the route with the given name (no-op if absent)."""
        self._routes.pop(name, None)
        self._embeddings.pop(name, None)

    def clear(self) -> None:
        """Remove all registered routes."""
        self._routes.clear()
        self._embeddings.clear()

    # ── Introspection ─────────────────────────────────────────────────────────

    @property
    def num_routes(self) -> int:
        """Number of currently registered routes."""
        return len(self._routes)

    def list_route_names(self) -> List[str]:
        """Return the names of all registered routes."""
        return list(self._routes.keys())

    def get_route(self, name: str) -> Optional[Route]:
        """Return the :class:`Route` with the given name, or ``None``."""
        return self._routes.get(name)

    # ── Internal helpers ──────────────────────────────────────────────────────

    def _encode_one(self, text: str) -> np.ndarray:
        """Encode a single string to a 1-D float32 array."""
        emb = self.encoder.encode(text)
        arr = np.asarray(emb, dtype=np.float32)
        if arr.ndim > 1:
            arr = arr[0]
        return arr

    def __repr__(self) -> str:
        return (
            f"RouteLayer(encoder={self.encoder.__class__.__name__}, "
            f"routes={self.num_routes})"
        )
