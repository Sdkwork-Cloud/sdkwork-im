FROM rust:1.88-bookworm AS builder
WORKDIR /workspace

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY services ./services

RUN cargo build --release -p local-minimal-node

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/local-minimal-node /usr/local/bin/local-minimal-node

ENV CRAW_CHAT_BIND_ADDR=0.0.0.0:18090
EXPOSE 18090

CMD ["/usr/local/bin/local-minimal-node"]
