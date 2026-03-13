"""Internal type definitions used by the Router and Rust FFI layer.

Public users should use ``stratarouter.Route`` (from ``route.py``) and
``stratarouter.RouteChoice``.  The types here are implementation details.
"""

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field, field_validator


class RouteConfig(BaseModel):
    """Internal route schema used by the Rust core and Router.

    .. note::
       This is an internal type.  The public-facing route class is
       :class:`stratarouter.Route` (``route.py``), which uses ``name``
       and ``utterances`` fields compatible with the semantic-router API.

    ``Route`` from ``route.py`` is mapped to ``RouteConfig`` in
    :meth:`Router.add` using:  ``id = route.name``,
    ``examples = route.utterances``.
    """

    id: str = Field(..., description="Unique route identifier")
    description: str = Field(default="", description="Human-readable description")
    examples: List[str] = Field(default_factory=list, description="Example queries")
    keywords: List[str] = Field(default_factory=list, description="Important keywords")
    patterns: List[str] = Field(default_factory=list, description="Exact match patterns")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Additional metadata")
    threshold: Optional[float] = Field(
        default=None, ge=0.0, le=1.0, description="Per-route confidence threshold"
    )
    tags: List[str] = Field(default_factory=list, description="Organisational tags")

    @field_validator("id")
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
    """Result returned by the low-level :class:`Router`.

    Attributes
    ----------
    route_id:
        ID of the matched route.
    confidence:
        Calibrated confidence score in ``[0, 1]``.
    scores:
        Score breakdown (``semantic``, ``keyword``, ``pattern``,
        ``total``, ``confidence``).
    latency_ms:
        End-to-end routing latency in milliseconds.
    metadata:
        Metadata copied from the matched route.
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
                    "semantic": 0.85,
                    "keyword": 0.65,
                    "pattern": 0.0,
                    "total": 0.80,
                    "confidence": 0.89,
                },
                "latency_ms": 2.3,
            }
        }
    }
