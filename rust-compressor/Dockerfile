# Dockerfile for Rust project
FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Final runtime image (alpine is a minimal Linux distribution)

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-compressor /usr/local/bin/rust-compressor

ENTRYPOINT ["/usr/local/bin/rust-compressor"]
