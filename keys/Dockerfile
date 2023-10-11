FROM rust:1-slim-buster as builder

RUN apt-get update && \
    apt-get install -y build-essential pkg-config libprotobuf-dev libssl-dev protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY keys keys
COPY proto proto
RUN cargo build --release --manifest-path=keys/Cargo.toml

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/keys/target/release/keys /usr/local/bin/keys

ENTRYPOINT ["/usr/local/bin/keys"]
