"""
FastAPI server for StrataRouter

High-performance API with automatic documentation.
"""

from fastapi import FastAPI, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from contextlib import asynccontextmanager
import time
from typing import List, Optional

from server.models import (
    RouteRequest,
    RouteResponse,
    RouteMatchResponse,
    HealthResponse,
    AddRouteRequest,
)
from server.config import get_settings
from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder


# Global router instance
router_instance = None
settings = get_settings()


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Startup and shutdown events"""
    global router_instance
    
    # Startup
    print("🚀 Starting StrataRouter server...")
    
    # Initialize encoder
    print(f"📦 Loading encoder: {settings.encoder_model}")
    encoder = HuggingFaceEncoder(model=settings.encoder_model)
    
    # Initialize router
    router_instance = RouteLayer(
        encoder=encoder,
        top_k=settings.top_k,
        cache_size=settings.cache_size
    )
    
    # Add default routes if configured
    if settings.default_routes:
        print(f"📝 Adding {len(settings.default_routes)} default routes...")
        for route_data in settings.default_routes:
            route = Route(**route_data)
            router_instance.add(route)
    
    print(f"✅ Server ready with {router_instance.num_routes} routes")
    
    yield
    
    # Shutdown
    print("👋 Shutting down StrataRouter server...")


# Create FastAPI app
app = FastAPI(
    title="StrataRouter API",
    description="High-performance semantic routing - 10x faster than semantic-router",
    version="0.1.0",
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/", response_model=HealthResponse)
async def root():
    """Root endpoint"""
    return HealthResponse(
        status="healthy",
        version="0.1.0",
        routes_count=router_instance.num_routes if router_instance else 0
    )


@app.get("/health", response_model=HealthResponse)
async def health():
    """Health check endpoint"""
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    return HealthResponse(
        status="healthy",
        version="0.1.0",
        routes_count=router_instance.num_routes
    )


@app.post("/route", response_model=RouteResponse)
async def route_request(request: RouteRequest):
    """
    Route a semantic query to matching routes.
    
    Returns top_k matches sorted by score (highest first).
    
    Example:
```json
        {
          "text": "I need my invoice",
          "top_k": 3,
          "threshold": 0.75
        }
```
    """
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    start_time = time.time()
    
    try:
        # Route the query
        result = router_instance(request.text, threshold=request.threshold)
        
        # Calculate timing
        timing_ms = (time.time() - start_time) * 1000
        
        # Convert to response format
        matches = []
        if result.is_match:
            matches.append(
                RouteMatchResponse(
                    name=result.name,
                    score=result.score,
                    threshold=result.threshold,
                    metadata=result.metadata
                )
            )
        
        return RouteResponse(
            matches=matches,
            timing_ms=round(timing_ms, 2)
        )
        
    except ValueError as e:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(e)
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Routing error: {str(e)}"
        )


@app.post("/route/batch", response_model=List[RouteResponse])
async def route_batch(texts: List[str], threshold: Optional[float] = None):
    """
    Route multiple queries in batch.
    
    Example:
```json
        ["I need my invoice", "How do I cancel?", "Support question"]
```
    """
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    if not texts:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Empty text list"
        )
    
    start_time = time.time()
    
    try:
        results = router_instance.route_batch(texts, threshold=threshold)
        timing_ms = (time.time() - start_time) * 1000
        
        responses = []
        for result in results:
            matches = []
            if result.is_match:
                matches.append(
                    RouteMatchResponse(
                        name=result.name,
                        score=result.score,
                        threshold=result.threshold,
                        metadata=result.metadata
                    )
                )
            
            responses.append(
                RouteResponse(
                    matches=matches,
                    timing_ms=round(timing_ms / len(texts), 2)
                )
            )
        
        return responses
        
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Batch routing error: {str(e)}"
        )


@app.get("/routes", response_model=List[str])
async def list_routes():
    """List all route names"""
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    return router_instance.list_route_names()


@app.post("/routes", status_code=status.HTTP_201_CREATED)
async def add_route(request: AddRouteRequest):
    """
    Add a new route dynamically.
    
    Example:
```json
        {
          "name": "billing",
          "utterances": ["invoice", "payment", "refund"],
          "threshold": 0.75
        }
```
    """
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    try:
        route = Route(
            name=request.name,
            utterances=request.utterances,
            threshold=request.threshold,
            description=request.description,
            metadata=request.metadata
        )
        
        router_instance.add(route)
        
        return JSONResponse(
            status_code=status.HTTP_201_CREATED,
            content={
                "message": f"Route '{request.name}' added successfully",
                "routes_count": router_instance.num_routes
            }
        )
        
    except ValueError as e:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(e)
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Error adding route: {str(e)}"
        )


@app.delete("/routes/{route_name}", status_code=status.HTTP_200_OK)
async def delete_route(route_name: str):
    """Delete a route by name"""
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    try:
        router_instance.remove(route_name)
        
        return JSONResponse(
            content={
                "message": f"Route '{route_name}' deleted successfully",
                "routes_count": router_instance.num_routes
            }
        )
        
    except ValueError as e:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=str(e)
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Error deleting route: {str(e)}"
        )


@app.post("/cache/clear", status_code=status.HTTP_200_OK)
async def clear_cache():
    """Clear the embedding cache"""
    if router_instance is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Router not initialized"
        )
    
    router_instance.clear_cache()
    
    return JSONResponse(
        content={"message": "Cache cleared successfully"}
    )


if __name__ == "__main__":
    import uvicorn
    
    uvicorn.run(
        "main:app",
        host=settings.host,
        port=settings.port,
        reload=settings.reload,
        log_level=settings.log_level.lower()
    )
