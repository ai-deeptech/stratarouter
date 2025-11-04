FROM rust:1.75-slim as rust-builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    python3-dev \
    python3-pip \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install maturin
RUN pip3 install maturin

# Copy Rust core
COPY core/ ./core/

# Build Rust core
WORKDIR /app/core
RUN cargo build --release

# Python stage
FROM python:3.11-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy built Rust library
COPY --from=rust-builder /app/core/target/release/*.so /usr/local/lib/

# Copy Python code
COPY python/ ./python/
COPY server/ ./server/

# Install Python dependencies
RUN pip install --no-cache-dir \
    fastapi==0.104.1 \
    uvicorn[standard]==0.24.0 \
    pydantic==2.5.0 \
    pydantic-settings==2.1.0 \
    sentence-transformers==2.2.0 \
    numpy>=1.21.0

# Install stratarouter
WORKDIR /app/python
RUN pip install -e .

# Switch to server directory
WORKDIR /app/server

# Expose port
EXPOSE 8000

# Run server
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
