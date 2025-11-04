"""
Encoder implementations for different embedding providers
"""

from stratarouter.encoders.base import BaseEncoder

__all__ = ["BaseEncoder"]

# Optional imports
try:
    from stratarouter.encoders.openai import OpenAIEncoder
    __all__.append("OpenAIEncoder")
except ImportError:
    pass

try:
    from stratarouter.encoders.cohere import CohereEncoder
    __all__.append("CohereEncoder")
except ImportError:
    pass

try:
    from stratarouter.encoders.huggingface import HuggingFaceEncoder
    __all__.append("HuggingFaceEncoder")
except ImportError:
    pass
