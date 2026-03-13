"""Utility helper functions"""

from typing import List, Dict, Any, Optional
import json
from pathlib import Path


def load_routes_from_json(path: str) -> List[Dict[str, Any]]:
    """Load routes from JSON file
    
    Args:
        path: Path to JSON file
        
    Returns:
        List of route dicts
        
    Examples:
        >>> routes = load_routes_from_json("routes.json")
    """
    path_obj = Path(path)
    if not path_obj.exists():
        raise FileNotFoundError(f"Routes file not found: {path}")
    
    try:
        with open(path_obj) as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON in routes file: {e}") from e
    
    if isinstance(data, list):
        return data
    elif isinstance(data, dict) and "routes" in data:
        return data["routes"]
    else:
        raise ValueError(
            "Invalid routes file format. Expected list or dict with 'routes' key"
        )


def save_routes_to_json(routes: List[Dict[str, Any]], path: str) -> None:
    """Save routes to JSON file
    
    Args:
        routes: List of route dicts
        path: Output path
    """
    if not routes:
        raise ValueError("Cannot save empty routes list")
    
    path_obj = Path(path)
    path_obj.parent.mkdir(parents=True, exist_ok=True)
    
    try:
        with open(path_obj, "w") as f:
            json.dump({"routes": routes}, f, indent=2)
    except Exception as e:
        raise RuntimeError(f"Failed to save routes: {e}") from e


def validate_route_dict(route: Dict[str, Any]) -> bool:
    """Validate route dictionary
    
    Args:
        route: Route dict
        
    Returns:
        True if valid
        
    Raises:
        ValueError: If invalid
    """
    if not isinstance(route, dict):
        raise ValueError("Route must be a dictionary")
    
    required = ["id"]
    for field in required:
        if field not in route:
            raise ValueError(f"Missing required field: {field}")
    
    if not route["id"]:
        raise ValueError("Route ID cannot be empty")
    
    if not route.get("examples") and not route.get("description"):
        raise ValueError("Route must have examples or description")
    
    return True


def merge_routes(routes1: List[Dict], routes2: List[Dict]) -> List[Dict]:
    """Merge two lists of routes, avoiding duplicates
    
    Args:
        routes1: First list of routes
        routes2: Second list of routes
        
    Returns:
        Merged list (routes2 overwrites routes1 on ID collision)
    """
    if not isinstance(routes1, list) or not isinstance(routes2, list):
        raise TypeError("Both arguments must be lists")
    
    merged = {r["id"]: r for r in routes1}
    for route in routes2:
        merged[route["id"]] = route
    return list(merged.values())


def calculate_route_similarity(route1: Dict, route2: Dict) -> float:
    """Calculate similarity between two routes based on keywords
    
    Args:
        route1: First route
        route2: Second route
        
    Returns:
        Similarity score (0-1)
    """
    if not isinstance(route1, dict) or not isinstance(route2, dict):
        raise TypeError("Both arguments must be dicts")
    
    # Keyword-based Jaccard similarity
    kw1 = set(route1.get("keywords", []))
    kw2 = set(route2.get("keywords", []))
    
    if not kw1 or not kw2:
        return 0.0
    
    intersection = len(kw1 & kw2)
    union = len(kw1 | kw2)
    
    return intersection / union if union > 0 else 0.0


def suggest_route_improvements(route: Dict) -> List[str]:
    """Suggest improvements for a route
    
    Args:
        route: Route dict
        
    Returns:
        List of suggestion strings
    """
    if not isinstance(route, dict):
        raise TypeError("Route must be a dictionary")
    
    suggestions = []
    
    if not route.get("description"):
        suggestions.append("Add a clear description")
    
    if not route.get("examples"):
        suggestions.append("Add example queries")
    
    if not route.get("keywords"):
        suggestions.append("Add relevant keywords")
    
    examples = route.get("examples", [])
    if len(examples) < 3:
        suggestions.append(f"Add more examples (currently {len(examples)}, recommend 3+)")
    
    keywords = route.get("keywords", [])
    if len(keywords) < 3:
        suggestions.append(f"Add more keywords (currently {len(keywords)}, recommend 3+)")
    
    if not suggestions:
        suggestions.append("Route looks good!")
    
    return suggestions

