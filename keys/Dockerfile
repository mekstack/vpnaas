FROM rust:1-alpine as builder

RUN apk add --no-cache musl-dev build-base protobuf-dev openssl-dev pkgconfig
ENV OPENSSL_DIR=/usr

WORKDIR /app
COPY keys keys
COPY proto proto
RUN cargo build --release --manifest-path=keys/Cargo.toml

FROM alpine

COPY --from=builder /app/keys/target/release/keys /usr/local/bin/keys

ENTRYPOINT ["/usr/local/bin/keys"]
