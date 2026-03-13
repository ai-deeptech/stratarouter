"""Tests for Route and RouteChoice (stratarouter.route)."""

import pytest

from stratarouter import Route, RouteChoice


# ── Route validation ──────────────────────────────────────────────────────────

def test_route_creation():
    route = Route(name="test", utterances=["hello", "hi", "hey"], threshold=0.75)
    assert route.name == "test"
    assert len(route.utterances) == 3
    assert route.threshold == 0.75


def test_route_empty_name_raises():
    with pytest.raises(ValueError, match="cannot be empty"):
        Route(name="", utterances=["test"])


def test_route_whitespace_name_raises():
    with pytest.raises(ValueError, match="cannot be empty"):
        Route(name="   ", utterances=["test"])


def test_route_no_utterances_raises():
    with pytest.raises(ValueError, match="at least one utterance"):
        Route(name="test", utterances=[])


def test_route_all_blank_utterances_raises():
    with pytest.raises(ValueError, match="at least one non-empty utterance"):
        Route(name="test", utterances=["  ", ""])


def test_route_threshold_too_high_raises():
    with pytest.raises(ValueError):
        Route(name="test", utterances=["hi"], threshold=1.5)


def test_route_threshold_negative_raises():
    with pytest.raises(ValueError):
        Route(name="test", utterances=["hi"], threshold=-0.1)


def test_route_utterances_are_stripped():
    route = Route(name="test", utterances=["  hello  ", "world", "", "   "])
    assert route.utterances == ["hello", "world"]


def test_route_with_metadata():
    route = Route(
        name="test",
        utterances=["hello"],
        metadata={"category": "general", "priority": 1},
    )
    assert route.metadata["category"] == "general"
    assert route.metadata["priority"] == 1


def test_route_with_description():
    route = Route(
        name="billing",
        utterances=["invoice", "payment"],
        description="Questions about billing",
    )
    assert route.description == "Questions about billing"


# ── RouteChoice ───────────────────────────────────────────────────────────────

def test_route_choice_match():
    choice = RouteChoice(name="test", score=0.85, threshold=0.75)
    assert choice.name == "test"
    assert choice.score == 0.85
    assert choice.is_match is True
    assert bool(choice) is True


def test_route_choice_no_name():
    choice = RouteChoice(name=None, score=0.65, threshold=0.75)
    assert choice.name is None
    assert choice.is_match is False
    assert bool(choice) is False


def test_route_choice_score_below_threshold():
    choice = RouteChoice(name="test", score=0.70, threshold=0.75)
    assert choice.is_match is False
    assert bool(choice) is False


def test_route_choice_score_equal_threshold():
    choice = RouteChoice(name="test", score=0.75, threshold=0.75)
    assert choice.is_match is True


def test_route_choice_repr_contains_key_fields():
    choice = RouteChoice(name="billing", score=0.8765, threshold=0.75)
    s = repr(choice)
    assert "billing" in s
    assert "0.8765" in s
    assert "0.75" in s
