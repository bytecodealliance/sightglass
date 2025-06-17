FROM rust:1.87
RUN rustup target add wasm32-wasip1
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
ADD assets assets
ADD setup.sh setup.sh
RUN ./setup.sh
WORKDIR /usr/src/rust-benchmark
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release --target wasm32-wasip1
RUN mkdir /benchmark
RUN cp target/wasm32-wasip1/release/benchmark.wasm /benchmark/
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
