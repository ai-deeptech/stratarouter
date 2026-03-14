"""Tests for encoder implementations (root test suite).

These tests use the MockEncoder from python/tests/test_router.py's pattern
but are self-contained so the root suite can run without the full SDK install.
"""

import numpy as np
import pytest

from stratarouter.encoders.base import BaseEncoder  # type: ignore[import]


# ── Minimal mock encoder ──────────────────────────────────────────────────────

class MockEncoder(BaseEncoder):
    """Deterministic fixed-output encoder for testing."""

    def encode(self, text):
        if isinstance(text, list):
            return np.array([self._vec() for _ in text], dtype=np.float32)
        return np.array(self._vec(), dtype=np.float32)

    @staticmethod
    def _vec():
        return [0.1] * 384

    @property
    def dimension(self) -> int:
        return 384


@pytest.fixture()
def mock_encoder():
    return MockEncoder()


# ── Tests ─────────────────────────────────────────────────────────────────────

def test_mock_encoder_single(mock_encoder):
    embedding = mock_encoder.encode("test")
    assert embedding.shape == (384,)
    assert embedding.dtype == np.float32


def test_mock_encoder_consistency(mock_encoder):
    emb1 = mock_encoder.encode("test")
    emb2 = mock_encoder.encode("test")
    np.testing.assert_array_almost_equal(emb1, emb2)


def test_mock_encoder_batch(mock_encoder):
    texts = ["test1", "test2", "test3"]
    embeddings = mock_encoder.encode(texts)
    assert embeddings.shape == (3, 384)
    assert embeddings.dtype == np.float32


def test_encoder_dimension_property(mock_encoder):
    assert mock_encoder.dimension == 384


def test_base_encoder_requires_encode():
    """BaseEncoder.encode() must be implemented by subclasses."""
    with pytest.raises(TypeError):
        BaseEncoder()  # abstract — cannot instantiate directly
