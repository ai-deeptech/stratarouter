"""StrataRouter - High-performance semantic routing"""

try:
    # Try importing from the Rust extension module
    from stratarouter_core.stratarouter_core import PyRouter, PyRoute
    
    # Re-export with cleaner names
    Router = PyRouter
    Route = PyRoute
    
except ImportError:
    # Fallback for development
    from stratarouter_core import PyRouter, PyRoute
    Router = PyRouter
    Route = PyRoute

__version__ = "0.2.0"
__all__ = ["Router", "Route"]
