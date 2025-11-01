"""OpenAI Assistants integration for StrataRouter"""

from typing import Dict, Optional, List, Any


class StrataRouterAssistant:
    """OpenAI Assistants API with routing"""
    
    def __init__(
        self,
        assistants: Dict[str, str],
        routes: List[Any],
        api_key: Optional[str] = None,
        router: Optional[Any] = None,
        **kwargs
    ):
        """Initialize routed assistants"""
        try:
            from openai import OpenAI
        except ImportError as e:
            raise ImportError(
                "OpenAI package not installed. "
                "Install: pip install stratarouter[openai]"
            ) from e
        
        if not assistants:
            raise ValueError("At least one assistant must be provided")
        
        self.client = OpenAI(api_key=api_key)
        self.assistants = assistants
        
        if router is None:
            from ..router import Router
            router = Router(**kwargs)
            for route in routes:
                router.add(route)
            router.build_index()
        
        self.router = router
        self.threads: Dict[str, str] = {}
    
    def chat(
        self,
        message: str,
        user_id: str = "default",
        create_thread: bool = True,
        **kwargs
    ) -> str:
        """Send message with routing"""
        if not message or not message.strip():
            raise ValueError("Message cannot be empty")
        
        # Route to appropriate assistant
        result = self.router.route(message)
        assistant_id = self.assistants.get(result.route_id)
        
        if not assistant_id:
            raise ValueError(f"No assistant found for route: {result.route_id}")
        
        # Get or create thread
        thread_id = self.threads.get(user_id)
        if not thread_id and create_thread:
            thread = self.client.beta.threads.create()
            thread_id = thread.id
            self.threads[user_id] = thread_id
        elif not thread_id:
            raise ValueError(f"No thread found for user: {user_id}")
        
        # Send message
        self.client.beta.threads.messages.create(
            thread_id=thread_id,
            role="user",
            content=message
        )
        
        # Run assistant
        run = self.client.beta.threads.runs.create(
            thread_id=thread_id,
            assistant_id=assistant_id,
            **kwargs
        )
        
        # Wait for completion
        while run.status in ["queued", "in_progress"]:
            run = self.client.beta.threads.runs.retrieve(
                thread_id=thread_id,
                run_id=run.id
            )
        
        if run.status != "completed":
            raise RuntimeError(f"Run failed with status: {run.status}")
        
        # Get response
        messages = self.client.beta.threads.messages.list(
            thread_id=thread_id,
            limit=1
        )
        
        if messages.data:
            return messages.data[0].content[0].text.value
        
        return ""
    
    def create_thread(self, user_id: str) -> str:
        """Create a new thread for user"""
        thread = self.client.beta.threads.create()
        self.threads[user_id] = thread.id
        return thread.id
    
    def delete_thread(self, user_id: str) -> None:
        """Delete user's thread"""
        thread_id = self.threads.get(user_id)
        if thread_id:
            self.client.beta.threads.delete(thread_id)
            del self.threads[user_id]
