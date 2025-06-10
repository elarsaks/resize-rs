FROM rust:1.75 as builder
WORKDIR /workspace
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /workspace/target/release/image-resizer /usr/local/bin/image-resizer
ENTRYPOINT ["/usr/local/bin/image-resizer"]
