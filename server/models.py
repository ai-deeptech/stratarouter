"""
Pydantic models for FastAPI server
"""

from typing import List, Optional, Dict, Any
from pydantic import BaseModel, Field


class RouteRequest(BaseModel):
    """Request model for routing"""
    text: str = Field(..., description="Text to route", min_length=1)
    top_k: int = Field(1, description="Number of top matches", ge=1, le=100)
    threshold: Optional[float] = Field(None, description="Custom threshold", ge=0.0, le=1.0)
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "text": "I need my invoice",
                    "top_k": 3,
                    "threshold": 0.75
                }
            ]
        }
    }


class RouteMatchResponse(BaseModel):
    """Individual route match"""
    name: str = Field(..., description="Route name")
    score: float = Field(..., description="Similarity score")
    threshold: float = Field(..., description="Threshold used")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Route metadata")


class RouteResponse(BaseModel):
    """Response model for routing"""
    matches: List[RouteMatchResponse] = Field(default_factory=list, description="Matched routes")
    timing_ms: float = Field(..., description="Processing time in milliseconds")


class HealthResponse(BaseModel):
    """Health check response"""
    status: str = Field(..., description="Service status")
    version: str = Field(..., description="API version")
    routes_count: int = Field(..., description="Number of configured routes")


class AddRouteRequest(BaseModel):
    """Request model for adding a route"""
    name: str = Field(..., description="Unique route name", min_length=1)
    utterances: List[str] = Field(..., description="Example utterances", min_length=1)
    threshold: float = Field(0.82, description="Similarity threshold", ge=0.0, le=1.0)
    description: Optional[str] = Field(None, description="Route description")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Route metadata")
    
    model_config = {
        "json_schema_extra": {
            "examples": [
                {
                    "name": "billing",
                    "utterances": ["invoice", "payment", "refund"],
                    "threshold": 0.75,
                    "description": "Billing and payment questions",
                    "metadata": {"category": "finance"}
                }
            ]
        }
    }
