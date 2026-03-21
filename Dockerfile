FROM rust:1.85-bookworm AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests for ALL crates in the workspace so Cargo can resolve deps
COPY Cargo.toml Cargo.lock ./
# If you have workspace members (e.g. tron/, tron-macros/), copy their Cargo.tomls too
COPY tron/Cargo.toml ./tron/Cargo.toml
COPY tron-macros/Cargo.toml ./tron-macros/Cargo.toml

# Dummy source files for every workspace member so `cargo build` can cache deps
RUN mkdir -p tron/src tron-macros/src && \
    echo 'fn main(){}' > tron/src/main.rs && \
    echo '' > tron-macros/src/lib.rs

# Pre-build dependencies only (this layer is cached unless Cargo.lock changes)
RUN cargo build --release --package tron 2>/dev/null || true

# Wipe dummy fingerprints so Cargo rebuilds your real code
RUN rm -rf tron/src tron-macros/src \
           target/release/.fingerprint/tron-* \
           target/release/.fingerprint/tron-macros-*

# Copy real source
COPY . .

# Build for real — cache the registry to speed up dep re-downloads,
# but do NOT cache /app/target (that's what caused the bug)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --package tron

# ---- Runtime stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tron /app/tron
WORKDIR /data
ENTRYPOINT ["/app/tron"]
