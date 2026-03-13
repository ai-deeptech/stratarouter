"""Encoder implementations for StrataRouter.

All encoders inherit from :class:`BaseEncoder` and implement:

* ``encode(text: str | list[str]) -> np.ndarray``
* ``dimension: int``  (property)

Optional encoders are imported lazily so that missing extras don't break
the base package import.
"""

from .base import BaseEncoder

__all__: list = ["BaseEncoder"]

try:
    from .huggingface import HuggingFaceEncoder  # noqa: F401
    __all__.append("HuggingFaceEncoder")
except ImportError:
    pass

try:
    from .openai import OpenAIEncoder  # noqa: F401
    __all__.append("OpenAIEncoder")
except ImportError:
    pass

try:
    from .cohere import CohereEncoder  # noqa: F401
    __all__.append("CohereEncoder")
except ImportError:
    pass
