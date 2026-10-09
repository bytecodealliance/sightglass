FROM rust:1.96.0@sha256:58fe97504a0e4cbba5d85599619a589923d3e779472a6fb0840d58d1c4ba99d7
RUN rustup target add wasm32-wasip2
WORKDIR /usr/src/rust-template
COPY Cargo.toml Cargo.lock ./
COPY src ./src
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
# wasm32-wasip2 links a component that imports `bench` and exports `wasi:cli/run`.
RUN cargo build --locked --release --target wasm32-wasip2
RUN mkdir /benchmark && cp target/wasm32-wasip2/release/rust-template.wasm /benchmark/
