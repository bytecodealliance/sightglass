FROM rust:1.48
ARG REPOSITORY=https://github.com/bytecodealliance/wasmtime/
ARG REVISION=main

WORKDIR /usr/src
RUN git clone ${REPOSITORY}
WORKDIR /usr/src/wasmtime
RUN git checkout ${REVISION} && git submodule update --init
RUN cargo build -p wasmtime-bench-api --release
RUN mv target/release/libwasmtime_bench_api.so /engine.so
