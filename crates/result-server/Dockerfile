# Build the web service using Rust tools.
FROM rust:1.48 AS builder
COPY . /usr/src/sightglass-result-server
WORKDIR /usr/src/sightglass-result-server
RUN cargo build --release

# Since the Rust image can be quite large, we run the actual server inside a smaller image.
FROM ubuntu:latest
WORKDIR /opt/sightglass
COPY --from=builder /usr/src/sightglass-result-server/target/release/sightglass-result-server .
CMD ["./sightglass-result-server"]
