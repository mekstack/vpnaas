FROM rust:1-alpine as builder

RUN apk add --no-cache musl-dev build-base protobuf-dev

WORKDIR /app
COPY wg wg
COPY proto proto
RUN cargo build --release --manifest-path=wg/Cargo.toml

FROM alpine

RUN apk add --no-cache wireguard-tools iproute2

COPY --from=builder /app/wg/target/release/wgrpc /usr/local/bin/wgrpc
COPY --from=builder /app/wg/entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
