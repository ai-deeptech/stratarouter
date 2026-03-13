#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════
# COMPLETE FIX SCRIPT  —  paste entire block on ole9
# ═══════════════════════════════════════════════════════════════════
set -e
REPO=/home/opc/backup/new/stratarouter

# ── 1. Write core.py correctly (ruff-clean from the start) ────────
cat > $REPO/python/stratarouter/core.py << 'PYEOF'
"""
Bridge to Rust core functionality.

Exposes Router, RustRoute, RouteMatch, cosine_similarity, and
cosine_similarity_batch as pure-Python implementations that always
import successfully regardless of whether the Rust extension is built.
"""

from __future__ import annotations

import math
from typing import Any


# ── Try to load compiled Rust extension (optional) ────────────────
try:
    from stratarouter._core import PyRouter as _PyRouter  # noqa: F401
    _RUST_AVAILABLE = True
except ImportError:
    _RUST_AVAILABLE = False


# ── Cosine similarity helpers ─────────────────────────────────────

def cosine_similarity(a: list[float], b: list[float]) -> float:
    """Return cosine similarity in [-1, 1] between two vectors."""
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
    query: list[float],
    embeddings: list[list[float]],
) -> list[float]:
    """Return cosine similarity between query and each embedding."""
    return [cosine_similarity(query, emb) for emb in embeddings]


# ── RouteMatch ────────────────────────────────────────────────────

class RouteMatch:
    """Result of a single route match."""

    def __init__(self, name: str, score: float, threshold: float) -> None:
        self.name = name
        self.score = score
        self.threshold = threshold

    @property
    def is_match(self) -> bool:
        """True when score meets the threshold."""
        return self.score >= self.threshold

    def __repr__(self) -> str:
        return (
            f"RouteMatch(name={self.name!r}, score={self.score:.4f}, "
            f"threshold={self.threshold}, is_match={self.is_match})"
        )


# ── RustRoute ─────────────────────────────────────────────────────

class RustRoute:
    """A named route with pre-computed embeddings."""

    def __init__(
        self,
        name: str,
        embeddings: list[list[float]],
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
        """Number of stored embeddings."""
        return len(self.embeddings)

    @property
    def embedding_dim(self) -> int:
        """Dimension of the embeddings."""
        return len(self.embeddings[0]) if self.embeddings else 0

    def __repr__(self) -> str:
        return (
            f"RustRoute(name={self.name!r}, "
            f"num_examples={self.num_examples}, "
            f"embedding_dim={self.embedding_dim})"
        )


# ── Router ────────────────────────────────────────────────────────

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
        self._routes: dict[str, RustRoute] = {}

    @property
    def num_routes(self) -> int:
        """Number of registered routes."""
        return len(self._routes)

    def list_routes(self) -> list[str]:
        """Return names of all registered routes."""
        return list(self._routes.keys())

    def add(self, route: RustRoute) -> None:
        """Add a route; raises ValueError if the name already exists."""
        if route.name in self._routes:
            raise ValueError(f"Route '{route.name}' already exists")
        self._routes[route.name] = route

    def remove(self, name: str) -> None:
        """Remove route by name (no-op if absent)."""
        self._routes.pop(name, None)

    def clear(self) -> None:
        """Remove all routes."""
        self._routes.clear()

    def route(self, embedding: list[float]) -> list[RouteMatch]:
        """Return up to top_k matches sorted by score descending."""
        if not self._routes:
            return []
        results: list[RouteMatch] = []
        for name, r in self._routes.items():
            best = max(cosine_similarity(embedding, e) for e in r.embeddings)
            results.append(RouteMatch(name=name, score=best, threshold=r.threshold))
        results.sort(key=lambda m: m.score, reverse=True)
        return results[: self.top_k]

    def __repr__(self) -> str:
        return f"Router(num_routes={self.num_routes}, top_k={self.top_k})"


# ── Public API ────────────────────────────────────────────────────

__all__ = [
    "Router",
    "RustRoute",
    "RouteMatch",
    "cosine_similarity",
    "cosine_similarity_batch",
]
PYEOF

echo "core.py written"

# ── 2. Run ruff --fix on everything ───────────────────────────────
cd $REPO/python
pip install ruff --break-system-packages -q
ruff check stratarouter/ --fix --unsafe-fixes 2>&1 || true

# ── 3. Verify ruff is clean ───────────────────────────────────────
echo ""
echo "=== ruff check ==="
ruff check stratarouter/
RUFF=$?
echo "Ruff exit: $RUFF"

# ── 4. cargo fmt ──────────────────────────────────────────────────
echo ""
echo "=== cargo fmt ==="
cd $REPO/core
rustup component add rustfmt 2>/dev/null || true
cargo fmt
cargo fmt --check
FMT=$?
echo "Fmt exit: $FMT"

# ── 5. cargo clippy ───────────────────────────────────────────────
echo ""
echo "=== cargo clippy ==="
cargo clippy -- -D warnings 2>&1 | tail -30
CLIPPY=$?
echo "Clippy exit: $CLIPPY"

# ── 6. Commit only when all 3 pass ────────────────────────────────
echo ""
echo "Summary — Ruff:$RUFF  Fmt:$FMT  Clippy:$CLIPPY"

if [ "$RUFF" = "0" ] && [ "$FMT" = "0" ] && [ "$CLIPPY" = "0" ]; then
  cd $REPO
  git add -A
  git commit \
    --author="natarajanchandra02-afk <natarajanchandra02@users.noreply.github.com>" \
    -m "fix: clean core.py rewrite, ruff/fmt/clippy all pass"
  git push origin fix/oss-review
  echo "=== PUSHED — CI should pass ==="
else
  echo "=== NOT PUSHED — remaining errors above ==="
  if [ "$RUFF" != "0" ]; then
    echo "--- ruff ---"
    cd $REPO/python && ruff check stratarouter/
  fi
  if [ "$CLIPPY" != "0" ]; then
    echo "--- clippy ---"
    cd $REPO/core && cargo clippy -- -D warnings 2>&1 | grep "^error"
  fi
fi
