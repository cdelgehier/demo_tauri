"""Entry point for the PyInstaller bundle — runs the FastAPI app via uvicorn."""

import uvicorn

from main import app  # noqa: F401 — force PyInstaller to bundle main.py

if __name__ == "__main__":
    uvicorn.run("main:app", host="127.0.0.1", port=8000, log_level="info")
