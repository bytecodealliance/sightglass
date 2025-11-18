# This two-phase Dockerfile allows us to avoid re-downloading APT packages and wasi-sdk with every
# build.

# First, retrieve wasi-sdk:

FROM ubuntu:24.04 AS builder
WORKDIR /
RUN apt update && apt install -y wget

# Download and extract wasi-sdk.
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-28/wasi-sdk-28.0-x86_64-linux.tar.gz
RUN tar xvf wasi-sdk-28.0-x86_64-linux.tar.gz

# Second, compile the benchmark to Wasm.

FROM ubuntu:24.04
WORKDIR /
COPY --from=builder /wasi-sdk-28.0-x86_64-linux /wasi-sdk/

# Set common env vars.
ENV CC=/wasi-sdk/bin/clang
ENV CXX=/wasi-sdk/bin/clang++
ENV LD=/wasi-sdk/bin/lld
ENV CFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
ENV CXXFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
ENV PATH /wasi-sdk

# Compile `benchmark.c` to `./benchmark.wasm`.
COPY quicksort.c .
COPY sightglass.h .
WORKDIR /benchmark
RUN $CC $CFLAGS ../quicksort.c -O3 -g -I.. -o benchmark.wasm
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
