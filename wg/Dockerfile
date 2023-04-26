FROM rust as builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path=wg/Cargo.toml

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y wireguard-tools iproute2

WORKDIR /app/wg
COPY --from=builder /app/wg/target/release/wg /usr/local/bin/wg
COPY --from=builder /app/wg/entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]