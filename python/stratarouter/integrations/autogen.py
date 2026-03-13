"""AutoGen integration for StrataRouter"""

from typing import Any, List, Optional


class StrataRouterGroupChat:
    """AutoGen GroupChat with intelligent routing"""

    def __init__(
        self,
        agents: List[Any],
        routes: Optional[List[Any]] = None,
        router: Optional[Any] = None,
        max_round: int = 10,
        **kwargs
    ):
        """Initialize routed group chat"""
        try:
            from autogen import GroupChat, GroupChatManager
        except ImportError as e:
            raise ImportError(
                "AutoGen not installed. "
                "Install: pip install stratarouter[autogen]"
            ) from e

        if not agents:
            raise ValueError("At least one agent must be provided")

        self.agents = agents
        self.router = router
        self.routes = routes or []

        # Create AutoGen GroupChat
        self.group_chat = GroupChat(
            agents=agents,
            messages=[],
            max_round=max_round,
            speaker_selection_method=self._select_speaker,
            **kwargs
        )

        self.manager = GroupChatManager(
            groupchat=self.group_chat,
            **kwargs
        )

    def _select_speaker(self, last_speaker: Any, groupchat: Any) -> Any:
        """Select next speaker using routing"""
        if not groupchat.messages:
            return self.agents[0]

        last_message = groupchat.messages[-1]["content"]

        if self.router and self.routes:
            try:
                result = self.router.route(last_message)

                # Find agent for this route
                for i, route in enumerate(self.routes):
                    if route.id == result.route_id and i < len(self.agents):
                        return self.agents[i]
            except Exception:
                pass  # Fall through to round-robin

        # Default: round-robin
        current_idx = self.agents.index(last_speaker)
        next_idx = (current_idx + 1) % len(self.agents)
        return self.agents[next_idx]

    def initiate_chat(self, message: str, **kwargs) -> Any:
        """Start group chat"""
        if not message or not message.strip():
            raise ValueError("Message cannot be empty")

        user_proxy = self.agents[0]
        return user_proxy.initiate_chat(
            self.manager,
            message=message,
            **kwargs
        )
