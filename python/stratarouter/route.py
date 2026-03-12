"""Public Route and RouteChoice classes.

These are the primary data structures for the StrataRouter public API.
The schema is intentionally compatible with ``semantic-router`` to make
migration straightforward.

Example
-------
>>> from stratarouter import Route, RouteChoice
>>>
>>> route = Route(
...     name="billing",
...     utterances=["invoice", "payment issue", "refund"],
...     threshold=0.75,
... )
"""

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field, field_validator


class Route(BaseModel):
    """A named semantic route with example utterances.

    Parameters
    ----------
    name:
        Unique identifier for the route (e.g. ``"billing"``).
    utterances:
        Example texts that should match this route.  At least one is required.
    description:
        Optional free-text description (used as a fallback when
        ``utterances`` is unavailable for encoding).
    metadata:
        Arbitrary key/value pairs carried through to :class:`RouteChoice`.
    threshold:
        Minimum cosine-similarity score required for a match.
        Defaults to ``0.82``.

    Example
    -------
    >>> route = Route(
    ...     name="support",
    ...     utterances=["help", "I have a problem", "something is broken"],
    ...     threshold=0.75,
    ... )
    """

    name: str = Field(..., description="Unique route name")
    utterances: List[str] = Field(
        default_factory=list, description="Example utterances for this route"
    )
    description: Optional[str] = Field(None, description="Optional route description")
    metadata: Dict[str, Any] = Field(
        default_factory=dict, description="Arbitrary metadata passed to RouteChoice"
    )
    threshold: float = Field(0.82, ge=0.0, le=1.0, description="Minimum match threshold")

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

    def __repr__(self) -> str:
        return (
            f"Route(name={self.name!r}, "
            f"utterances={len(self.utterances)}, "
            f"threshold={self.threshold})"
        )


class RouteChoice(BaseModel):
    """Result of a routing operation.

    Compatible with ``semantic-router``'s ``RouteChoice`` for easy migration.

    Parameters
    ----------
    name:
        Name of the matched route, or ``None`` if no route met the threshold.
    score:
        Best cosine-similarity score found (``0.0`` if no routes exist).
    threshold:
        The threshold that was applied when deciding whether to match.
    metadata:
        Metadata from the matched route (empty dict when ``name is None``).

    Example
    -------
    >>> choice = RouteChoice(name="billing", score=0.87, threshold=0.75)
    >>> if choice:
    ...     print(f"Routed to: {choice.name}")
    """

    name: Optional[str] = Field(None, description="Matched route name, or None")
    score: float = Field(0.0, description="Best similarity score")
    threshold: float = Field(0.82, description="Threshold applied during routing")
    metadata: Dict[str, Any] = Field(
        default_factory=dict, description="Metadata from the matched route"
    )

    @property
    def is_match(self) -> bool:
        """``True`` when a route was matched *and* its score meets the threshold."""
        return self.name is not None and self.score >= self.threshold

    def __bool__(self) -> bool:
        """Allow ``if choice:`` idiom."""
        return self.is_match

    def __repr__(self) -> str:
        return (
            f"RouteChoice(name={self.name!r}, "
            f"score={self.score:.4f}, "
            f"threshold={self.threshold})"
        )
