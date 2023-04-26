FROM rust as builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path=keys/Cargo.toml

FROM debian:bookworm-slim

WORKDIR /app/keys
COPY --from=builder /app/keys/target/release/keys /usr/local/bin/keys

ENTRYPOINT ["/usr/local/bin/keys"]
