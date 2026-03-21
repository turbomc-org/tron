FROM rust:1.85-bookworm AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY tron/Cargo.toml ./tron/Cargo.toml
COPY tron-macros/Cargo.toml ./tron-macros/Cargo.toml

RUN mkdir -p tron/src tron-macros/src && \
    echo 'fn main(){}' > tron/src/main.rs && \
    echo '' > tron-macros/src/lib.rs

RUN cargo build --release --package tron 2>/dev/null || true

RUN rm -rf tron/src tron-macros/src \
           target/release/.fingerprint/tron-* \
           target/release/.fingerprint/tron-macros-*

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --package tron

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tron /app/tron

WORKDIR /data

ENTRYPOINT ["/app/tron"]
