"""
# StrataRouter Python Package

High-performance semantic routing for Python.

## Installation

```bash
pip install stratarouter
```

## Quick Start

```python
from stratarouter import Router, Route

router = Router()
router.add(Route(id="billing", keywords=["invoice", "payment"]))
router.build_index()
result = router.route("Where's my invoice?")
print(result.route_id)  # "billing"
```

## Documentation

Full documentation: https://docs.stratarouter.io

## License

MIT License - See LICENSE file for details.
"""
