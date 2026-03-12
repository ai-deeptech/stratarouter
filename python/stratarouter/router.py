"""Low-level Router with local and cloud deployment modes.

For most use-cases prefer :class:`~stratarouter.RouteLayer`, which is
simpler and does not require the compiled Rust extension.

Use :class:`Router` when you need:
* Access to the Rust hybrid-scoring core (dense + BM25 + pattern).
* Cloud-deployment mode with a hosted inference endpoint.
* Persistence (``save`` / ``load``).
"""

from __future__ import annotations

import json
import threading
import warnings
from enum import Enum
from pathlib import Path
from typing import Any, Dict, Optional, Union

from .route import Route, RouteChoice
from .types import RouteConfig, RouteResult
from .encoders.base import BaseEncoder

__all__ = ["Router", "DeploymentMode"]


class DeploymentMode(str, Enum):
    """Deployment backend for :class:`Router`."""

    LOCAL = "local"
    CLOUD = "cloud"


class Router:
    """Semantic router with Rust-accelerated hybrid scoring.

    Parameters
    ----------
    encoder:
        Encoder instance or model-name string (HuggingFace model).
        Only required in ``LOCAL`` mode.
    mode:
        ``"local"`` (default) or ``"cloud"``.
    api_key:
        Required in ``CLOUD`` mode.
    dimension:
        Embedding dimension. Overridden by encoder's dimension if they
        differ (a warning is emitted).
    threshold:
        Default confidence threshold.

    Example
    -------
    >>> from stratarouter import Route
    >>> from stratarouter.router import Router
    >>>
    >>> router = Router()
    >>> router.add(Route(name="billing", utterances=["invoice", "payment"]))
    >>> router.build_index()
    >>> choice = router.route("Where's my invoice?")
    """

    def __init__(
        self,
        encoder: Optional[Union[str, BaseEncoder]] = None,
        mode: Union[DeploymentMode, str] = DeploymentMode.LOCAL,
        api_key: Optional[str] = None,
        dimension: int = 384,
        threshold: float = 0.5,
        **kwargs: Any,
    ) -> None:
        if dimension <= 0:
            raise ValueError("dimension must be positive")
        if not 0.0 <= threshold <= 1.0:
            raise ValueError("threshold must be between 0 and 1")

        self.mode = DeploymentMode(mode)
        self.dimension = dimension
        self.threshold = threshold
        self.routes: Dict[str, Route] = {}
        self._index_built = False
        self._build_lock = threading.Lock()

        if self.mode == DeploymentMode.LOCAL:
            self._init_local(encoder)
        elif self.mode == DeploymentMode.CLOUD:
            self._init_cloud(api_key)

    # ── Initialisation helpers ────────────────────────────────────────────────

    def _init_local(self, encoder: Optional[Union[str, BaseEncoder]]) -> None:
        if encoder is None:
            encoder = "sentence-transformers/all-MiniLM-L6-v2"

        if isinstance(encoder, str):
            encoder = self._load_encoder(encoder)

        if not hasattr(encoder, "encode"):
            raise TypeError("Encoder must implement encode(text)")
        if not hasattr(encoder, "dimension"):
            raise TypeError("Encoder must expose a 'dimension' property")

        if encoder.dimension != self.dimension:
            warnings.warn(
                f"Using encoder dimension ({encoder.dimension}) "
                f"instead of specified dimension ({self.dimension})."
            )
            self.dimension = encoder.dimension

        self.encoder = encoder

        try:
            from ._core import PyRouter  # type: ignore[import]
            self._core = PyRouter(dimension=self.dimension, threshold=self.threshold)
        except ImportError as exc:
            raise ImportError(
                "Compiled Rust core not found.  "
                "Reinstall with: pip install --force-reinstall stratarouter"
            ) from exc

    def _init_cloud(self, api_key: Optional[str]) -> None:
        if not api_key:
            raise ValueError(
                "api_key is required for cloud mode.  "
                "Get one at: https://stratarouter.dev"
            )
        self.api_key = api_key
        try:
            from .cloud.client import CloudClient  # type: ignore[import]
            self.cloud_client = CloudClient(api_key=api_key)
        except ImportError as exc:
            raise ImportError(
                "Cloud dependencies not installed.  "
                "Install with: pip install stratarouter[cloud]"
            ) from exc

    @staticmethod
    def _load_encoder(model_name: str) -> BaseEncoder:
        try:
            from .encoders.huggingface import HuggingFaceEncoder
            return HuggingFaceEncoder(model_name)
        except ImportError as exc:
            raise ImportError(
                "HuggingFace encoder not available.  "
                "Install with: pip install stratarouter[huggingface]"
            ) from exc

    # ── Route management ──────────────────────────────────────────────────────

    def add(self, route: Route) -> None:
        """Register a :class:`~stratarouter.Route`.

        Raises
        ------
        ValueError
            If the route has no name or no utterances.
        """
        if not route.name:
            raise ValueError("Route name cannot be empty")
        if not route.utterances:
            raise ValueError(f"Route '{route.name}' must have at least one utterance")

        if route.name in self.routes:
            warnings.warn(
                f"Route '{route.name}' already exists and will be overwritten.  "
                "Call build_index() to rebuild the index."
            )

        self.routes[route.name] = route
        self._index_built = False

    # ── Index ─────────────────────────────────────────────────────────────────

    def build_index(self) -> None:
        """Encode all routes and build the search index.

        Must be called (or will be called automatically on first ``route()``)
        after all routes have been added.

        Raises
        ------
        ValueError
            If no routes have been added.
        RuntimeError
            If encoding or index construction fails.
        """
        with self._build_lock:
            self._index_built = False

            if self.mode == DeploymentMode.CLOUD:
                warnings.warn("build_index() is not required in cloud mode.")
                return

            if not self.routes:
                raise ValueError("No routes added.  Call router.add(route) first.")

            embeddings = []
            for route in self.routes.values():
                text = route.utterances[0] if route.utterances else (route.description or "")
                if not text:
                    raise ValueError(f"Route '{route.name}' has no text to encode")
                try:
                    emb = self.encoder.encode(text)
                    import numpy as np  # lazy import — numpy is a hard dep
                    emb = np.asarray(emb, dtype=np.float32)
                    if emb.ndim > 1:
                        emb = emb[0]
                    embeddings.append(emb.tolist())
                except Exception as exc:
                    raise RuntimeError(
                        f"Failed to encode route '{route.name}': {exc}"
                    ) from exc

            try:
                self._core.build_index(embeddings)
                self._index_built = True
            except Exception as exc:
                raise RuntimeError(f"Failed to build index: {exc}") from exc

    # ── Routing ───────────────────────────────────────────────────────────────

    def route(self, text: str) -> RouteResult:
        """Route ``text`` and return a :class:`~stratarouter.types.RouteResult`.

        Automatically calls :meth:`build_index` if the index is stale.

        Raises
        ------
        ValueError
            If ``text`` is empty.
        RuntimeError
            If encoding or routing fails.
        """
        if not text or not text.strip():
            raise ValueError("Query text cannot be empty")

        if self.mode == DeploymentMode.CLOUD:
            return self._route_cloud(text)

        if not self._index_built:
            self.build_index()

        try:
            import numpy as np
            emb = self.encoder.encode(text)
            emb = np.asarray(emb, dtype=np.float32)
            if emb.ndim > 1:
                emb = emb[0]
            if len(emb) != self.dimension:
                raise ValueError(
                    f"Encoder dimension mismatch: expected {self.dimension}, "
                    f"got {len(emb)}.  Check encoder configuration."
                )
        except ValueError:
            raise
        except Exception as exc:
            raise RuntimeError(f"Failed to encode query: {exc}") from exc

        try:
            raw = self._core.route(text, emb.tolist())
        except Exception as exc:
            raise RuntimeError(f"Routing failed: {exc}") from exc

        return RouteResult(
            route_id=raw["route_id"],
            confidence=raw["confidence"],
            scores=raw["scores"],
            latency_ms=raw.get("latency_ms", 0.0),
        )

    def _route_cloud(self, text: str) -> RouteResult:
        try:
            return self.cloud_client.route(text)
        except Exception as exc:
            raise RuntimeError(f"Cloud routing failed: {exc}") from exc

    # ── Persistence ───────────────────────────────────────────────────────────

    def save(self, path: str) -> None:
        """Serialise the router configuration to a JSON file.

        Parameters
        ----------
        path:
            Destination file path (parent directories are created if needed).
        """
        from .__version__ import __version__

        path_obj = Path(path)
        path_obj.parent.mkdir(parents=True, exist_ok=True)

        encoder_config = None
        if hasattr(self, "encoder"):
            encoder_config = {
                "type": self.encoder.__class__.__name__,
                "model_name": getattr(self.encoder, "model_name", None),
                "dimension": self.encoder.dimension,
            }

        state = {
            "version": __version__,
            "mode": self.mode.value,
            "dimension": self.dimension,
            "threshold": self.threshold,
            "encoder_config": encoder_config,
            # Serialise using route.name + utterances (public schema)
            "routes": [
                {
                    "name": r.name,
                    "utterances": r.utterances,
                    "description": r.description,
                    "metadata": r.metadata,
                    "threshold": r.threshold,
                }
                for r in self.routes.values()
            ],
        }

        try:
            with open(path_obj, "w", encoding="utf-8") as fh:
                json.dump(state, fh, indent=2)
        except OSError as exc:
            raise RuntimeError(f"Failed to save router: {exc}") from exc

    @classmethod
    def load(cls, path: str, **kwargs: Any) -> "Router":
        """Load a router previously saved with :meth:`save`.

        Parameters
        ----------
        path:
            Path to the JSON file created by :meth:`save`.
        **kwargs:
            Keyword arguments forwarded to :class:`Router.__init__`.

        Raises
        ------
        FileNotFoundError
            If ``path`` does not exist.
        ValueError
            If the file is not valid JSON or is missing required keys.
        """
        path_obj = Path(path)
        if not path_obj.exists():
            raise FileNotFoundError(f"Router file not found: {path}")

        try:
            with open(path_obj, encoding="utf-8") as fh:
                state = json.load(fh)
        except json.JSONDecodeError as exc:
            raise ValueError(f"Invalid router file: {exc}") from exc

        for key in ("version", "dimension", "threshold", "routes"):
            if key not in state:
                raise ValueError(f"Router file is missing required key: '{key}'")

        if "encoder_config" in state and "encoder" not in kwargs:
            enc_type = state["encoder_config"].get("type", "unknown")
            warnings.warn(
                f"Router was saved with encoder '{enc_type}'.  "
                "Pass encoder=<instance> to load() for best results."
            )

        router = cls(
            mode=kwargs.pop("mode", state.get("mode", "local")),
            dimension=state["dimension"],
            threshold=state["threshold"],
            **kwargs,
        )

        for route_data in state["routes"]:
            try:
                router.add(Route(**route_data))
            except Exception as exc:
                warnings.warn(f"Skipping malformed route during load: {exc}")

        if router.mode == DeploymentMode.LOCAL:
            try:
                router.build_index()
            except Exception as exc:
                warnings.warn(f"Could not build index during load: {exc}")

        return router
