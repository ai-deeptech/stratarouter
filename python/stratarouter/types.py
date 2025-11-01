"""Type definitions for StrataRouter"""

from typing import List, Dict, Optional, Any
from pydantic import BaseModel, Field, field_validator


class Route(BaseModel):
    """Route definition
    
    Examples:
        >>> route = Route(
        ...     id="billing",
        ...     description="Billing queries",
        ...     keywords=["invoice", "payment"]
        ... )
    """
    
    id: str = Field(..., description="Unique route ID")
    description: str = Field(default="", description="Human-readable description")
    examples: List[str] = Field(default_factory=list, description="Example queries")
    keywords: List[str] = Field(default_factory=list, description="Important keywords")
    patterns: List[str] = Field(default_factory=list, description="Exact patterns")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Additional data")
    threshold: Optional[float] = Field(default=None, ge=0.0, le=1.0, description="Confidence threshold")
    tags: List[str] = Field(default_factory=list, description="Tags")
    
    @field_validator('id')
    @classmethod
    def validate_id(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Route ID cannot be empty")
        return v.strip()
    
    model_config = {
        "json_schema_extra": {
            "example": {
                "id": "billing",
                "description": "Billing and payment questions",
                "examples": ["Where's my invoice?"],
                "keywords": ["invoice", "payment", "billing"],
            }
        }
    }


class RouteResult(BaseModel):
    """Routing result
    
    Attributes:
        route_id: Matched route identifier
        confidence: Confidence score (0-1)
        scores: Score breakdown
        latency_ms: Routing latency in milliseconds
        metadata: Additional result metadata
    """
    
    route_id: str
    confidence: float = Field(ge=0.0, le=1.0)
    scores: Dict[str, float]
    latency_ms: float = Field(ge=0.0)
    metadata: Dict[str, Any] = Field(default_factory=dict)
    
    model_config = {
        "json_schema_extra": {
            "example": {
                "route_id": "billing",
                "confidence": 0.89,
                "scores": {
                    "dense": 0.85,
                    "sparse": 0.65,
                    "rule": 0.0,
                    "fused": 0.80
                },
                "latency_ms": 2.3
            }
        }
    }
