"""
Example FastAPI integration with StrataRouter
"""

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Optional

from stratarouter import Route, RouteLayer
from stratarouter.encoders import HuggingFaceEncoder


# Create FastAPI app
app = FastAPI(title="StrataRouter Example")

# Initialize router globally
encoder = HuggingFaceEncoder()
routes = [
    Route(name="billing", utterances=["invoice", "payment", "refund"]),
    Route(name="support", utterances=["help", "issue", "problem"]),
]
router = RouteLayer(encoder=encoder, routes=routes)


class QueryRequest(BaseModel):
    text: str
    threshold: Optional[float] = None


class QueryResponse(BaseModel):
    route: Optional[str]
    score: float
    is_match: bool


@app.get("/")
async def root():
    return {
        "message": "StrataRouter FastAPI Example",
        "routes": router.list_route_names()
    }


@app.post("/route", response_model=QueryResponse)
async def route_query(request: QueryRequest):
    """Route a query to matching routes"""
    try:
        result = router(request.text, threshold=request.threshold)
        
        return QueryResponse(
            route=result.name,
            score=result.score,
            is_match=result.is_match
        )
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))


@app.get("/routes")
async def list_routes():
    """List all available routes"""
    return {"routes": router.list_route_names()}


if __name__ == "__main__":
    import uvicorn
    
    print("Starting FastAPI server with StrataRouter...")
    print("Visit http://localhost:8000/docs for API documentation")
    
    uvicorn.run(app, host="0.0.0.0", port=8000)
