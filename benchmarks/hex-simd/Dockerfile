FROM rust:1.68

RUN rustup target add wasm32-wasi
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release --target wasm32-wasi
RUN cp target/wasm32-wasi/release/hex-benchmark.wasm /benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.
