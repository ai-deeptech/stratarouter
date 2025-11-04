cd /home/opc/backup/stratarouter/stratarouter

# Run quickstart example
python3.11 examples/quickstart.py
```

**Expected Output:**
```
StrataRouter Quick Start

1. Defining routes...
   Created 3 routes

2. Loading encoder...
   Loaded HuggingFaceEncoder(model='all-MiniLM-L6-v2', dim=384)

3. Creating router...
   RouteLayer(routes=3, encoder=HuggingFaceEncoder)

4. Routing queries...

   Query: 'I need my invoice from last month'
   → Route: billing
   → Score: 0.876
   → Match: ✓

   ...
