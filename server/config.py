"""
Configuration for FastAPI server
"""

from typing import List, Optional, Dict, Any
from pydantic_settings import BaseSettings
from functools import lru_cache


class Settings(BaseSettings):
    """Server settings"""
    
    # Server
    host: str = "0.0.0.0"
    port: int = 8000
    reload: bool = False
    log_level: str = "INFO"
    
    # CORS
    cors_origins: List[str] = ["*"]
    
    # Router
    encoder_model: str = "all-MiniLM-L6-v2"
    top_k: int = 3
    cache_size: int = 1000
    
    # Default routes (optional)
    default_routes: List[Dict[str, Any]] = []
    
    model_config = {
        "env_file": ".env",
        "env_file_encoding": "utf-8"
    }


@lru_cache()
def get_settings() -> Settings:
    """Get cached settings instance"""
    return Settings()
```

---
