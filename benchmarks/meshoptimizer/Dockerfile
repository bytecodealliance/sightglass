FROM ubuntu:18.04 AS builder
WORKDIR /
RUN apt update && apt install -y wget git build-essential

# Download and extract wasi-sdk.
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-12/wasi-sdk-12.0-linux.tar.gz
RUN tar xvf wasi-sdk-12.0-linux.tar.gz

WORKDIR /usr/src
RUN git clone https://github.com/zeux/meshoptimizer.git \
    && cd meshoptimizer \
    && git reset --hard f734fd572aed5bf76e84d9ed62ca6f4f6c47d84e

# Second, compile the benchmark to Wasm.

WORKDIR /usr/src/meshoptimizer
COPY sightglass.h .
COPY sightglass.patch .
RUN patch -Np1 < ./sightglass.patch

# Meshoptimizer is expected to be compiled with emscripten but that build
# does not run properly in wasmtime.
ENV WASI_SDK=/wasi-sdk-12.0
RUN make codecbench-simd.wasm
RUN mkdir /benchmark
RUN mv codecbench-simd.wasm /benchmark/benchmark.wasm
