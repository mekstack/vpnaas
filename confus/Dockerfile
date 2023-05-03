FROM rust as builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY confus confus
COPY proto proto
RUN cargo build --release --manifest-path=confus/Cargo.toml

FROM debian:bookworm-slim

COPY --from=builder /app/confus/target/release/confus /usr/local/bin/confus

ENTRYPOINT ["/usr/local/bin/confus"]
