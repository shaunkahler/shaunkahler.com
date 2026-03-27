FROM rust:1.82-slim-bookworm

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://github.com/bytecodealliance/wasm-pack/releases/download/v0.12.1/wasm-pack-v0.12.1-x86_64-unknown-linux-musl.tar.gz -L | tar xzf - -C /usr/local/bin && \
    chmod +x /usr/local/bin/wasm-pack

WORKDIR /app
