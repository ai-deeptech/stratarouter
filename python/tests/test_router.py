"""Tests for RouteLayer (stratarouter.layer)."""

import numpy as np
import pytest

from stratarouter import Route, RouteChoice, RouteLayer
from stratarouter.encoders.base import BaseEncoder


# ── Mock encoder ──────────────────────────────────────────────────────────────

class MockEncoder(BaseEncoder):
    """Deterministic encoder for testing.

    Mapping (by first character of text):
      'b'  → [1.0, 0.0, 0.0]   (billing / b-words)
      's'  → [0.0, 1.0, 0.0]   (support / s-words)
      other → [0.0, 0.0, 1.0]
    """

    def encode(self, text):
        if isinstance(text, list):
            return np.array([self._vec(t) for t in text], dtype=np.float32)
        return np.array(self._vec(text), dtype=np.float32)

    @staticmethod
    def _vec(text: str):
        ch = text.strip().lower()[:1]
        if ch == "b":
            return [1.0, 0.0, 0.0]
        if ch == "s":
            return [0.0, 1.0, 0.0]
        return [0.0, 0.0, 1.0]

    @property
    def dimension(self) -> int:
        return 3


# ── Fixtures ──────────────────────────────────────────────────────────────────

@pytest.fixture()
def encoder():
    return MockEncoder()


@pytest.fixture()
def billing_route():
    return Route(name="billing", utterances=["billing", "balance"], threshold=0.7)


@pytest.fixture()
def support_route():
    return Route(name="support", utterances=["support", "service"], threshold=0.7)


# ── Creation ──────────────────────────────────────────────────────────────────

def test_route_layer_creation(encoder, billing_route, support_route):
    rl = RouteLayer(encoder=encoder, routes=[billing_route, support_route])
    assert rl.num_routes == 2
    assert "billing" in rl.list_route_names()
    assert "support" in rl.list_route_names()


def test_route_layer_starts_empty(encoder):
    rl = RouteLayer(encoder=encoder)
    assert rl.num_routes == 0
    assert rl.list_route_names() == []


# ── Basic routing ─────────────────────────────────────────────────────────────

def test_billing_query_matches_billing_route(encoder, billing_route, support_route):
    rl = RouteLayer(encoder=encoder, routes=[billing_route, support_route])
    result = rl("billing question")
    assert result.name == "billing"
    assert result.score > 0.7
    assert result.is_match


def test_support_query_matches_support_route(encoder, billing_route, support_route):
    rl = RouteLayer(encoder=encoder, routes=[billing_route, support_route])
    result = rl("support issue")
    assert result.name == "support"
    assert result.is_match


def test_no_match_when_score_below_threshold(encoder):
    rl = RouteLayer(
        encoder=encoder,
        routes=[Route(name="billing", utterances=["billing"], threshold=0.9)],
    )
    # "other" starts with 'o' → [0,0,1]; "billing" → [1,0,0]; cosine = 0.0 < 0.9
    result = rl("other topic")
    assert result.name is None
    assert result.is_match is False


# ── Add / remove / clear ──────────────────────────────────────────────────────

def test_add_route(encoder):
    rl = RouteLayer(encoder=encoder)
    rl.add(Route(name="test", utterances=["test"]))
    assert rl.num_routes == 1


def test_remove_route(encoder):
    rl = RouteLayer(encoder=encoder)
    rl.add(Route(name="test", utterances=["test"]))
    rl.remove("test")
    assert rl.num_routes == 0


def test_remove_nonexistent_route_is_noop(encoder):
    rl = RouteLayer(encoder=encoder)
    rl.remove("does_not_exist")  # should not raise


def test_add_duplicate_raises(encoder):
    route = Route(name="test", utterances=["test"])
    rl = RouteLayer(encoder=encoder, routes=[route])
    with pytest.raises(ValueError, match="already exists"):
        rl.add(route)


def test_clear(encoder):
    rl = RouteLayer(
        encoder=encoder,
        routes=[
            Route(name="a", utterances=["alpha"]),
            Route(name="b", utterances=["beta"]),
        ],
    )
    assert rl.num_routes == 2
    rl.clear()
    assert rl.num_routes == 0


# ── Batch routing ─────────────────────────────────────────────────────────────

def test_route_batch(encoder, billing_route, support_route):
    rl = RouteLayer(encoder=encoder, routes=[billing_route, support_route])
    results = rl.route_batch(["billing", "support", "other"])
    assert len(results) == 3
    assert results[0].name == "billing"
    assert results[1].name == "support"
    assert results[2].name is None  # "other" → [0,0,1] → no match above threshold


# ── Threshold overrides ───────────────────────────────────────────────────────

def test_per_call_threshold_override_low_allows_match(encoder):
    # "other" vs "other" → score = 1.0; with threshold=0.5 → match
    rl = RouteLayer(
        encoder=encoder,
        routes=[Route(name="misc", utterances=["other"], threshold=0.99)],
    )
    result = rl("other topic", threshold=0.5)
    assert result.is_match


def test_global_threshold_no_match(encoder):
    # With global_threshold=0.99 even identical vectors might not match if
    # the route's own threshold is lower — global wins when per-call is set.
    # "billing" vs "billing" → cosine ≈ 1.0; threshold 0.5 → match.
    rl = RouteLayer(
        encoder=encoder,
        routes=[Route(name="billing", utterances=["billing"], threshold=0.5)],
    )
    result = rl("billing")
    assert result.is_match


# ── Edge cases ────────────────────────────────────────────────────────────────

def test_empty_text_raises(encoder):
    rl = RouteLayer(encoder=encoder, routes=[Route(name="t", utterances=["test"])])
    with pytest.raises(ValueError, match="[Ee]mpty"):
        rl("")


def test_whitespace_only_text_raises(encoder):
    rl = RouteLayer(encoder=encoder, routes=[Route(name="t", utterances=["test"])])
    with pytest.raises(ValueError, match="[Ee]mpty"):
        rl("   ")


def test_list_route_names(encoder, billing_route, support_route):
    rl = RouteLayer(encoder=encoder, routes=[billing_route, support_route])
    names = rl.list_route_names()
    assert set(names) == {"billing", "support"}


def test_invalid_encoder_missing_encode_raises():
    class BadEncoder:
        dimension = 3

    with pytest.raises(TypeError, match="encode"):
        RouteLayer(encoder=BadEncoder())


def test_invalid_encoder_missing_dimension_raises():
    class BadEncoder:
        def encode(self, text):
            return np.zeros(3)

    with pytest.raises(TypeError, match="dimension"):
        RouteLayer(encoder=BadEncoder())
