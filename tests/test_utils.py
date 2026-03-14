"""Utility / helper tests."""

import pytest


def test_version_is_semver():
    """Package version must follow semver (x.y.z)."""
    import stratarouter
    v = stratarouter.__version__
    parts = v.split(".")
    assert len(parts) >= 2, f"Version '{v}' is not semver"
    assert all(p.isdigit() for p in parts[:3] if p), (
        f"Version '{v}' contains non-numeric segment"
    )


def test_public_api_exports():
    """All documented public symbols must be exported from the top-level package."""
    import stratarouter
    expected = ["Route", "RouteChoice", "RouteLayer", "Router", "DeploymentMode"]
    for name in expected:
        assert hasattr(stratarouter, name), (
            f"stratarouter.{name} is missing from __init__.py exports"
        )


def test_route_repr_is_informative():
    from stratarouter import Route
    r = Route(name="billing", utterances=["invoice"])
    assert "billing" in repr(r)


def test_route_choice_bool_protocol():
    from stratarouter import RouteChoice
    assert bool(RouteChoice(name="x", score=0.9, threshold=0.75)) is True
    assert bool(RouteChoice(name=None, score=0.4, threshold=0.75)) is False
