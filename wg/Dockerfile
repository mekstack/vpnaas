FROM rust as builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY wg wg
COPY proto proto
RUN cargo build --release --manifest-path=wg/Cargo.toml

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y wireguard-tools iproute2

COPY --from=builder /app/wg/target/release/wgrpc /usr/local/bin/wgrpc
COPY --from=builder /app/wg/entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
