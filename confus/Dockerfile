FROM rust:1-alpine as builder

RUN apk add --no-cache musl-dev build-base protobuf-dev

WORKDIR /app
COPY confus confus
COPY proto proto
RUN cargo build --release --manifest-path=confus/Cargo.toml

FROM alpine

COPY --from=builder /app/confus/target/release/confus /usr/local/bin/confus

ENTRYPOINT ["/usr/local/bin/confus"]
