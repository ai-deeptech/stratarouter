# Integrations

StrataRouter ships with 9 framework integrations out of the box.

---

## LangChain

```python
from stratarouter.integrations.langchain import StrataRouterChain
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

routes = [
    Route(name="billing", utterances=["invoice", "payment", "refund"]),
    Route(name="support", utterances=["help", "broken", "error"]),
]

chain = StrataRouterChain.from_routes(
    routes=routes,
    encoder=HuggingFaceEncoder(),
    destination_chains={
        "billing": billing_chain,
        "support": support_chain,
    },
)

result = chain.invoke({"input": "I need my April invoice"})
```

See runnable example → [`integrations/langchain_example.py`](../integrations/langchain_example.py)

---

## LangGraph

```python
from stratarouter.integrations.langgraph import create_routing_graph
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

routes = [
    Route(name="billing", utterances=["invoice", "payment"]),
    Route(name="support", utterances=["help", "error"]),
]

graph = create_routing_graph(
    routes=routes,
    encoder=HuggingFaceEncoder(),
    node_handlers={
        "billing": handle_billing,
        "support": handle_support,
    },
)

result = graph.invoke({"messages": [HumanMessage(content="I need a refund")]})
```

See runnable example → [`integrations/langgraph_example.py`](../integrations/langgraph_example.py)

---

## CrewAI

```python
from stratarouter.integrations.crewai import RoutedAgent
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

agent = RoutedAgent(
    routes=[
        Route(name="research", utterances=["find", "search", "look up"]),
        Route(name="code",     utterances=["write", "debug", "implement"]),
    ],
    encoder=HuggingFaceEncoder(),
)
```

See runnable example → [`integrations/crewai_example.py`](../integrations/crewai_example.py)

---

## AutoGen

```python
from stratarouter.integrations.autogen import StrataRouterGroupChat
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

group_chat = StrataRouterGroupChat(
    agents=[billing_agent, support_agent, sales_agent],
    routes=[
        Route(name="billing_agent", utterances=["invoice", "payment"]),
        Route(name="support_agent", utterances=["help", "error"]),
        Route(name="sales_agent",   utterances=["pricing", "demo"]),
    ],
    encoder=HuggingFaceEncoder(),
)
```

See runnable example → [`integrations/autogen_example.py`](../integrations/autogen_example.py)

---

## OpenAI Assistants

```python
from stratarouter.integrations.openai_assistants import StrataRouterAssistant
from stratarouter import Route
from stratarouter.encoders import OpenAIEncoder

assistant = StrataRouterAssistant(
    routes=[
        Route(name="billing", utterances=["invoice", "payment"]),
        Route(name="support", utterances=["help", "broken"]),
    ],
    encoder=OpenAIEncoder(),
    assistant_ids={
        "billing": "asst_abc123",
        "support": "asst_def456",
    },
)
```

---

## Vertex AI (Google Agent)

```python
from stratarouter.integrations.google_agent import StrataRouterVertexAI
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

router = StrataRouterVertexAI(
    routes=[
        Route(name="billing", utterances=["invoice", "payment"]),
        Route(name="support", utterances=["help", "error"]),
    ],
    encoder=HuggingFaceEncoder(),
    project_id="my-gcp-project",
    location="us-central1",
)
```

---

## Generic Integration

Use `GenericRouter` to integrate with any framework:

```python
from stratarouter.integrations.generic import GenericRouter
from stratarouter import Route
from stratarouter.encoders import HuggingFaceEncoder

router = GenericRouter(
    routes=[...],
    encoder=HuggingFaceEncoder(),
)

route_name = router.route("I need a refund")  # returns str | None
```

---

## Integration Comparison

| Integration | Package | Notes |
|---|---|---|
| LangChain | `langchain-core` | `StrataRouterChain` wraps LCEL |
| LangGraph | `langgraph` | Routing as a graph node |
| CrewAI | `crewai` | Agent role routing |
| AutoGen | `autogen` | GroupChat speaker selection |
| OpenAI Assistants | `openai` | Assistant ID routing |
| Vertex AI | `google-cloud-aiplatform` | GCP agent routing |
| Generic | (none) | Bring-your-own framework |

---

## Links

- **Full docs**: https://docs.stratarouter.com
- **Support**: support@stratarouter.com
- **GitHub**: https://github.com/ai-deeptech/stratarouter
