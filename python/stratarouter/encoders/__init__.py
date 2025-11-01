"""Encoder implementations for StrataRouter"""

from .base import BaseEncoder

__all__ = ["BaseEncoder"]

# Conditional imports for optional dependencies
try:
    from .huggingface import HuggingFaceEncoder
    __all__.append("HuggingFaceEncoder")
except ImportError:
    pass

try:
    from .openai import OpenAIEncoder
    __all__.append("OpenAIEncoder")
except ImportError:
    pass
