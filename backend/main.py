from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

app = FastAPI(title="Sum API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "tauri://localhost",
        "http://tauri.localhost",
        "https://tauri.localhost",
    ],
    allow_methods=["POST"],
    allow_headers=["Content-Type"],
)


class SumRequest(BaseModel):
    a: int
    b: int


class SumResponse(BaseModel):
    result: int


@app.post("/sum", response_model=SumResponse)
def compute_sum(req: SumRequest) -> SumResponse:
    return SumResponse(result=req.a + req.b)
