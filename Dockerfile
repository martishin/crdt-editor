# Stage 1: Build the Rust library and prepare Python environment
FROM rust:1.72 as builder

WORKDIR /app

# Install Miniforge based on the detected architecture
RUN apt-get update && \
    apt-get install -y wget && \
    ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
        wget https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh -O Miniforge3.sh; \
    elif [ "$ARCH" = "aarch64" ]; then \
        wget https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-aarch64.sh -O Miniforge3.sh; \
    else \
        echo "Unsupported architecture"; exit 1; \
    fi && \
    bash Miniforge3.sh -b -p /opt/conda && \
    rm Miniforge3.sh && \
    apt-get clean

# Set up the Conda environment
COPY ./environment.yml /app/environment.yml
ENV PATH="/opt/conda/bin:$PATH"
RUN /opt/conda/bin/conda env create -f /app/environment.yml

# Install additional build dependencies for Rust and Python
RUN /opt/conda/bin/conda run -n rust-crdt-lww pip install setuptools-rust maturin

# Copy source code
COPY ./crdt /app/crdt
COPY ./server /app/server
COPY ./setup.py /app/setup.py

# Build and install the crdt-lww library
RUN /opt/conda/bin/conda run -n rust-crdt-lww pip install .

# Stage 2: Set up the final environment and run the server
FROM python:3.11-slim

WORKDIR /app
COPY --from=builder /opt/conda /opt/conda
ENV PATH="/opt/conda/bin:$PATH"

# Copy the server directory and compiled library
COPY --from=builder /app/server /app/server

# Set environment variables and default command to start the server
ENV DATA_FILE=/app/server/data.json
CMD ["/opt/conda/envs/rust-crdt-lww/bin/python", "-m", "uvicorn", "server.server:app", "--host", "0.0.0.0", "--port", "8000"]
