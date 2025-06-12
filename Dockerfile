FROM rust:latest AS builder
WORKDIR /workspace
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /workspace/target/release/image-resizer /usr/local/bin/image-resizer
ENTRYPOINT ["/usr/local/bin/image-resizer"]
