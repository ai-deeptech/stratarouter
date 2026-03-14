"""Cloud client smoke tests.

Full cloud tests require a live API key and are excluded from the default
test run. Marked with `@pytest.mark.integration` — run with:

    pytest tests/test_cloud.py -m integration --api-key=<key>
"""

import pytest


def test_cloud_module_importable():
    """The cloud sub-package must be importable without an API key."""
    from stratarouter.cloud import client  # noqa: F401  — import side-effect test
    assert hasattr(client, "CloudClient")


def test_deployment_mode_cloud_value():
    """DeploymentMode.CLOUD must equal the string 'cloud'."""
    from stratarouter import DeploymentMode
    assert DeploymentMode.CLOUD.value == "cloud"


def test_router_cloud_mode_requires_api_key():
    """Router(mode='cloud') must raise ValueError when no api_key is given."""
    from stratarouter import Router
    with pytest.raises(ValueError, match="api_key"):
        Router(mode="cloud")
