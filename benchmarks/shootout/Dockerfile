# This two-phase Dockerfile allows us to avoid re-downloading APT packages and
# wasi-sdk with every build.
ARG WASI_SDK_VERSION=20

# First, retrieve wasi-sdk:
FROM ubuntu:22.04 AS builder
ARG WASI_SDK_VERSION
WORKDIR /
RUN apt update && apt install -y wget
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-$WASI_SDK_VERSION/wasi-sdk-$WASI_SDK_VERSION.0-linux.tar.gz
RUN tar xvf wasi-sdk-${WASI_SDK_VERSION}.0-linux.tar.gz

# Second, compile the benchmark to Wasm.
FROM ubuntu:22.04
ARG WASI_SDK_VERSION
COPY --from=builder /wasi-sdk-${WASI_SDK_VERSION}.0 /wasi-sdk-${WASI_SDK_VERSION}.0/

# Set common env vars.
ENV CC=/wasi-sdk-${WASI_SDK_VERSION}.0/bin/clang
ENV CXX=/wasi-sdk-${WASI_SDK_VERSION}.0/bin/clang++
ENV LD=/wasi-sdk-${WASI_SDK_VERSION}.0/bin/lld
ENV CFLAGS=--sysroot=/wasi-sdk-${WASI_SDK_VERSION}.0/share/wasi-sysroot
ENV CXXFLAGS=--sysroot=/wasi-sdk-${WASI_SDK_VERSION}.0/share/wasi-sysroot
ENV PATH=$PATH:/wasi-sdk-${WASI_SDK_VERSION}.0

# Copy in the `src` directory.
ENV SRC=/usr/src/shootout
WORKDIR $SRC
COPY src .

# Compile each of the benchmarks into the `/benchmark` directory.
WORKDIR /benchmark
RUN for BENCH in $(find $SRC -name '*.c'); do \
    WASM=shootout-$(basename $BENCH .c).wasm; \
    echo "Compiling $WASM"; \
    $CC $CFLAGS $BENCH -O3 -g -I$SRC -o $WASM; \
    done
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
