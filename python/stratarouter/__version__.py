"""Version information for StrataRouter.

The single source of truth is ``pyproject.toml``.  At runtime the version is
read from the installed package metadata so it is always in sync, even when
the package is installed in editable mode.
"""

from importlib.metadata import PackageNotFoundError, version

try:
    __version__: str = version("stratarouter")
except PackageNotFoundError:
    # Package is not installed (e.g. running directly from the source tree).
    __version__ = "0.0.0.dev0"

__version_info__ = tuple(
    int(x) for x in __version__.split(".")[:3] if x.isdigit()
)
