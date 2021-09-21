FROM ubuntu:18.04 AS builder
WORKDIR /
RUN apt update && apt install -y wget git build-essential

# Download and extract wasi-sdk.
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-12/wasi-sdk-12.0-linux.tar.gz
RUN tar xvf wasi-sdk-12.0-linux.tar.gz

WORKDIR /usr/src
RUN git clone https://github.com/jlb6740/meshoptimizer.git \
    && cd meshoptimizer \
    && git checkout sightglass \
    && git pull

# Second, compile the benchmark to Wasm.

WORKDIR /usr/src/meshoptimizer
COPY sightglass.h .

# Meshoptimizer is expected to be compiled with emscripten but that build
# does not run properly in wasmtime.
ENV WASM_CC=/wasi-sdk-12.0/bin/clang++
ENV WASI_SYSROOT=/wasi-sdk-12.0/share/wasi-sysroot/
RUN make codecbench.wasm && mv codecbench.wasm /benchmark.wasm
