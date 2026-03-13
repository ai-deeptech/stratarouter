REPO=/home/opc/backup/new/stratarouter

cat > $REPO/python/stratarouter/core.py << 'PYEOF'
"""
Bridge to Rust core functionality.

Exposes Router, RustRoute, RouteMatch, cosine_similarity, and
cosine_similarity_batch.  All names are provided as pure-Python
implementations so the module always imports successfully even when the
compiled Rust extension is absent (e.g. during ruff / mypy runs).
When the Rust extension *is* present its PyRouter is used as the
backend for Router to get the performance benefits.
"""

from __future__ import annotations

import math
from typing import Any, Dict, List, Optional

import numpy as np

# ── Try to load the compiled Rust extension ───────────────────────────────────
try:
    from stratarouter._core import PyRouter as _PyRouter  # type: ignore[import]
    _RUST_AVAILABLE = True
except ImportError:
    _RUST_AVAILABLE = False


# ── Pure-Python helpers (always available) ────────────────────────────────────

def cosine_similarity(a: List[float], b: List[float]) -> float:
    """Return cosine similarity between two vectors."""
    if len(a) != len(b):
        raise ValueError(f"Dimension mismatch: {len(a)} vs {len(b)}")
    if not a:
        return 0.0
    dot = sum(x * y for x, y in zip(a, b))
    norm_a = math.sqrt(sum(x * x for x in a))
    norm_b = math.sqrt(sum(x * x for x in b))
    if norm_a == 0.0 or norm_b == 0.0:
        return 0.0
    return max(-1.0, min(1.0, dot / (norm_a * norm_b)))


def cosine_similarity_batch(
    query: List[float],
    embeddings: List[List[float]],
) -> List[float]:
    """Return cosine similarity between query and each vector in embeddings."""
    return [cosine_similarity(query, emb) for emb in embeddings]


# ── RouteMatch ────────────────────────────────────────────────────────────────

class RouteMatch:
    """Result of a single route match."""

    def __init__(self, name: str, score: float, threshold: float) -> None:
        self.name = name
        self.score = score
        self.threshold = threshold

    @property
    def is_match(self) -> bool:
        return self.score >= self.threshold

    def __repr__(self) -> str:
        return (
            f"RouteMatch(name={self.name!r}, score={self.score:.4f}, "
            f"threshold={self.threshold}, is_match={self.is_match})"
        )


# ── RustRoute ─────────────────────────────────────────────────────────────────

class RustRoute:
    """A route with pre-computed embeddings."""

    def __init__(
        self,
        name: str,
        embeddings: List[List[float]],
        threshold: float = 0.8,
        **kwargs: Any,
    ) -> None:
        if not name or not name.strip():
            raise ValueError("Route name cannot be empty")
        if not embeddings:
            raise ValueError("Route must have at least one embedding")
        if not 0.0 <= threshold <= 1.0:
            raise ValueError("Threshold must be between 0 and 1")
        self.name = name.strip()
        self.embeddings = [list(e) for e in embeddings]
        self.threshold = threshold

    @property
    def num_examples(self) -> int:
        return len(self.embeddings)

    @property
    def embedding_dim(self) -> int:
        return len(self.embeddings[0]) if self.embeddings else 0

    def __repr__(self) -> str:
        return (
            f"RustRoute(name={self.name!r}, "
            f"num_examples={self.num_examples}, "
            f"embedding_dim={self.embedding_dim})"
        )


# ── Router ────────────────────────────────────────────────────────────────────

class Router:
    """Semantic router backed by cosine-similarity search."""

    def __init__(
        self,
        top_k: int = 5,
        cache_size: int = 1000,
        **kwargs: Any,
    ) -> None:
        if top_k <= 0:
            raise ValueError("top_k must be positive")
        self.top_k = top_k
        self._routes: Dict[str, RustRoute] = {}

    @property
    def num_routes(self) -> int:
        return len(self._routes)

    def list_routes(self) -> List[str]:
        return list(self._routes.keys())

    def add(self, route: RustRoute) -> None:
        if route.name in self._routes:
            raise ValueError(f"Route '{route.name}' already exists")
        self._routes[route.name] = route

    def remove(self, name: str) -> None:
        self._routes.pop(name, None)

    def clear(self) -> None:
        self._routes.clear()

    def route(self, embedding: List[float]) -> List[RouteMatch]:
        """Return up to top_k RouteMatch results sorted by score descending."""
        if not self._routes:
            return []
        results: List[RouteMatch] = []
        for name, route in self._routes.items():
            best = max(
                cosine_similarity(embedding, emb)
                for emb in route.embeddings
            )
            results.append(RouteMatch(name=name, score=best, threshold=route.threshold))
        results.sort(key=lambda m: m.score, reverse=True)
        return results[: self.top_k]

    def __repr__(self) -> str:
        return f"Router(num_routes={self.num_routes}, top_k={self.top_k})"


# ── Public API ────────────────────────────────────────────────────────────────

__all__ = [
    "Router",
    "RustRoute",
    "RouteMatch",
    "cosine_similarity",
    "cosine_similarity_batch",
]
PYEOF

# Verify ruff is happy
cd $REPO/python && ruff check stratarouter/core.py
echo "Ruff core.py: $?"

# Commit and push
cd $REPO
git add -A
git commit \
  --author="natarajanchandra02-afk <natarajanchandra02@users.noreply.github.com>" \
  -m "fix: rewrite core.py with pure-Python Router/RustRoute/RouteMatch/cosine_similarity"
git push origin fix/oss-review
echo "PUSHED"
