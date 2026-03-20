FROM rust:1.85-bookworm AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src target/release/.fingerprint/tron-*

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --package tron

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tron /app/tron
WORKDIR /data
ENTRYPOINT ["/app/tron"]
