"""Google Vertex AI integration for StrataRouter"""

from typing import Any, Dict, List, Optional


class StrataRouterVertexAI:
    """Google Vertex AI agents with routing"""

    def __init__(
        self,
        models: Dict[str, str],
        routes: List[Any],
        project_id: str,
        location: str = "us-central1",
        router: Optional[Any] = None,
        **kwargs
    ):
        """Initialize Vertex AI router"""
        try:
            import vertexai
            from vertexai.generative_models import GenerativeModel
        except ImportError as e:
            raise ImportError(
                "Google Cloud Vertex AI SDK not installed. "
                "Install: pip install google-cloud-aiplatform"
            ) from e

        if not models:
            raise ValueError("At least one model must be provided")
        if not project_id:
            raise ValueError("Project ID cannot be empty")

        vertexai.init(project=project_id, location=location)

        self.models = {
            route_id: GenerativeModel(model_name)
            for route_id, model_name in models.items()
        }

        if router is None:
            from ..router import Router
            router = Router(**kwargs)
            for route in routes:
                router.add(route)
            router.build_index()

        self.router = router

    def generate(self, prompt: str, **kwargs) -> str:
        """Generate response with routing"""
        if not prompt or not prompt.strip():
            raise ValueError("Prompt cannot be empty")

        # Route to appropriate model
        result = self.router.route(prompt)
        model = self.models.get(result.route_id)

        if not model:
            raise ValueError(f"No model found for route: {result.route_id}")

        # Generate response
        response = model.generate_content(prompt, **kwargs)
        return response.text

    async def generate_async(self, prompt: str, **kwargs) -> str:
        """Async version of generate"""
        if not prompt or not prompt.strip():
            raise ValueError("Prompt cannot be empty")

        result = self.router.route(prompt)
        model = self.models.get(result.route_id)

        if not model:
            raise ValueError(f"No model found for route: {result.route_id}")

        response = await model.generate_content_async(prompt, **kwargs)
        return response.text
