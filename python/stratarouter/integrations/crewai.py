"""CrewAI integration for StrataRouter"""

from typing import Any, List, Optional


class RoutedAgent:
    """CrewAI Agent with routing capabilities"""

    def __init__(
        self,
        role: str,
        goal: str,
        backstory: str,
        routes: List[Any],
        router: Optional[Any] = None,
        **kwargs
    ):
        """Initialize routed agent"""
        try:
            from crewai import Agent
        except ImportError as e:
            raise ImportError(
                "CrewAI not installed. "
                "Install: pip install stratarouter[crewai]"
            ) from e

        if not routes:
            raise ValueError("At least one route must be provided")

        self.role = role
        self.routes = routes
        self.router = router

        # Create CrewAI agent
        self.agent = Agent(
            role=role,
            goal=goal,
            backstory=backstory,
            **kwargs
        )

    def can_handle(self, query: str) -> tuple[bool, float]:
        """Check if agent can handle query"""
        if not query or not query.strip():
            return False, 0.0

        if not self.router:
            # Simple keyword matching
            query_lower = query.lower()
            for route in self.routes:
                if any(kw.lower() in query_lower for kw in route.keywords):
                    return True, 0.8
            return False, 0.0

        result = self.router.route(query)
        can_handle = result.route_id in [r.id for r in self.routes]
        return can_handle, result.confidence


class StrataRouterCrew:
    """CrewAI Crew with intelligent routing"""

    def __init__(
        self,
        agents: List[Any],
        tasks: List[Any],
        router: Optional[Any] = None,
        **kwargs
    ):
        """Initialize routed crew"""
        try:
            from crewai import Crew
        except ImportError as e:
            raise ImportError(
                "CrewAI not installed. "
                "Install: pip install stratarouter[crewai]"
            ) from e

        if not agents:
            raise ValueError("At least one agent must be provided")

        self.agents = agents
        self.tasks = tasks
        self.router = router

        # Create CrewAI crew
        self.crew = Crew(
            agents=[a.agent if isinstance(a, RoutedAgent) else a for a in agents],
            tasks=tasks,
            **kwargs
        )

    def kickoff(self, query: str) -> Any:
        """Start crew with routing"""
        if not query or not query.strip():
            raise ValueError("Query cannot be empty")

        if self.router:
            self.router.route(query)

            # Find best agent for this route
            for agent in self.agents:
                if isinstance(agent, RoutedAgent):
                    can_handle, _ = agent.can_handle(query)
                    if can_handle:
                        # Prioritize this agent's tasks
                        prioritized_tasks = [
                            t for t in self.tasks
                            if getattr(t, 'agent', None) == agent.agent
                        ] + [
                            t for t in self.tasks
                            if getattr(t, 'agent', None) != agent.agent
                        ]
                        self.crew.tasks = prioritized_tasks
                        break

        return self.crew.kickoff()
