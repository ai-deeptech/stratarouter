"""StrataRouter — high-performance semantic routing.

Public API
----------
Route
    Define a named route with example utterances.
RouteChoice
    Result of a routing operation (name, score, threshold).
RouteLayer
    High-level router compatible with the semantic-router API.
    Recommended entry point for most users.
Router
    Lower-level router; useful when you need fine-grained control over
    the Rust core or cloud deployment mode.
"""

from .__version__ import __version__
from .layer import RouteLayer
from .route import Route, RouteChoice
from .router import Router, DeploymentMode

__all__ = [
    "__version__",
    # Data classes
    "Route",
    "RouteChoice",
    # High-level API
    "RouteLayer",
    # Low-level / advanced API
    "Router",
    "DeploymentMode",
]
