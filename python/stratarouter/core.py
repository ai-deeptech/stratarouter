"""
Bridge to Rust core functionality
"""

from typing import List
import numpy as np

try:
    from stratarouter._core import (
        Router as _RustRouter,
        Route as _RustRoute,
        RouteMatch as _RustRouteMatch,
        cosine_similarity,
        cosine_similarity_batch,
    )
except ImportError as e:
    raise ImportError(
        "Could not import Rust core. This usually means:\n"
        "1. The package wasn't built with maturin\n"
        "2. Run: pip install stratarouter\n"
        "3. Or build from source: cd python && maturin develop --release\n"
        f"\nOriginal error: {e}"
    ) from e

# Re-export Rust types
Router = _RustRouter
RustRoute = _RustRoute
RouteMatch = _RustRouteMatch

__all__ = [
    "Router",
    "RustRoute",
    "RouteMatch",
    "cosine_similarity",
    "cosine_similarity_batch",
]


def ensure_list(arr) -> List[float]:
    """Convert numpy array or list to Python list"""
    if isinstance(arr, np.ndarray):
        return arr.tolist()
    return list(arr)


def ensure_2d_list(arr) -> List[List[float]]:
    """Convert 2D numpy array or list to Python list of lists"""
    if isinstance(arr, np.ndarray):
        return arr.tolist()
    return [ensure_list(row) for row in arr]
