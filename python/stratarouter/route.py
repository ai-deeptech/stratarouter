"""
Route and RouteChoice classes - compatible with semantic-router API
"""

from typing import List, Optional, Dict, Any
from pydantic import BaseModel, Field, field_validator


class Route(BaseModel):
    """
    A semantic route definition.
    
    Compatible with semantic-router's Route class for easy migration.
    
    Attributes:
        name: Unique identifier for the route
        utterances: Example texts that should match this route
        description: Optional description
        metadata: Optional metadata dictionary
        threshold: Similarity threshold (0.0 to 1.0), default 0.82
    
    Example:
        >>> route = Route(
        ...     name="billing",
        ...     utterances=["invoice", "payment issue", "refund"],
        ...     threshold=0.75
        ... )
    """
    
    name: str = Field(..., description="Unique route name")
    utterances: List[str] = Field(default_factory=list, description="Example utterances")
    description: Optional[str] = Field(None, description="Route description")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Additional metadata")
    threshold: float = Field(0.82, description="Similarity threshold")
    
    @field_validator("name")
    @classmethod
    def validate_name(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Route name cannot be empty")
        return v.strip()
    
    @field_validator("utterances")
    @classmethod
    def validate_utterances(cls, v: List[str]) -> List[str]:
        if not v:
            raise ValueError("Route must have at least one utterance")
        cleaned = [u.strip() for u in v if u and u.strip()]
        if not cleaned:
            raise ValueError("Route must have at least one non-empty utterance")
        return cleaned
    
    @field_validator("threshold")
    @classmethod
    def validate_threshold(cls, v: float) -> float:
        if not 0.0 <= v <= 1.0:
            raise ValueError(f"Threshold must be between 0.0 and 1.0, got {v}")
        return v
    
    def __repr__(self) -> str:
        return (
            f"Route(name='{self.name}', "
            f"utterances={len(self.utterances)}, "
            f"threshold={self.threshold})"
        )


class RouteChoice(BaseModel):
    """
    Result of a routing operation.
    
    Compatible with semantic-router's RouteChoice for easy migration.
    
    Attributes:
        name: Name of the matched route (None if no match)
        score: Similarity score
        threshold: Threshold used for matching
        metadata: Route metadata
    
    Example:
        >>> choice = RouteChoice(name="billing", score=0.87, threshold=0.75)
        >>> if choice.name:
        ...     print(f"Matched route: {choice.name}")
    """
    
    name: Optional[str] = Field(None, description="Matched route name")
    score: float = Field(0.0, description="Similarity score")
    threshold: float = Field(0.82, description="Threshold used")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Route metadata")
    
    @property
    def is_match(self) -> bool:
        """Check if this is a valid match"""
        return self.name is not None and self.score >= self.threshold
    
    def __repr__(self) -> str:
        return (
            f"RouteChoice(name='{self.name}', "
            f"score={self.score:.4f}, "
            f"threshold={self.threshold})"
        )
    
    def __bool__(self) -> bool:
        """Allow using RouteChoice in boolean context"""
        return self.is_match
