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
RUN apt update && apt install -y git patch

# Set common env vars.
ENV CC=/wasi-sdk/bin/clang
ENV CXX=/wasi-sdk/bin/clang++
ENV LD=/wasi-sdk/bin/lld
ENV CFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
ENV CXXFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
# Prepend wasi-sdk's bin while keeping the system PATH so `git`/`patch` resolve.
ENV PATH=/wasi-sdk/bin:$PATH

WORKDIR /usr/src
RUN git clone https://github.com/BLAKE3-team/BLAKE3.git

WORKDIR /usr/src/BLAKE3
RUN git checkout 8aa5145039b972ba30e98e788752d37d14568824
# BLAKE3's dispatcher only selects its SSE2 kernels when IS_X86 is defined (it is
# not on wasm). This patch forces the SSE2 path on wasm and exposes the SSE2
# function declarations there.
COPY blake3-wasm-sse2.patch .
RUN patch -p1 < blake3-wasm-sse2.patch

WORKDIR /usr/src/BLAKE3/c
COPY benchmark.c .
COPY sightglass.h .
COPY wasm_sse_compat.h .
# wasi-sdk's clang cannot compile x86 SSE intrinsics for wasm. `wasm_sse_compat.h`
# reimplements the ones blake3_sse2.c uses on top of `<wasm_simd128.h>`; we expose
# it under the x86 intrinsic-header name so blake3_sse2.c's `#include <immintrin.h>`
# resolves to it (the shim dir is first on the include path).
RUN mkdir -p /shim && cp wasm_sse_compat.h /shim/ && \
    for h in immintrin emmintrin tmmintrin xmmintrin; do \
      echo '#include "wasm_sse_compat.h"' > /shim/$h.h; \
    done

RUN mkdir /benchmark
# Build BLAKE3 with its hand-written SSE2 implementation (blake3_sse2.c), lowered
# to Wasm SIMD by the compat header above. `-DBLAKE3_NO_SSE41/AVX2/AVX512` keep the
# (patched) dispatcher and headers limited to the SSE2 tier. The benchmark reads
# its workload from disk into the heap under a plain WASI runtime (no JS);
# wasi-libc's `malloc` grows linear memory natively.
RUN $CC $CFLAGS -O3 -g -msimd128 -I/shim \
    -DBLAKE3_NO_SSE41 -DBLAKE3_NO_AVX2 -DBLAKE3_NO_AVX512 \
    -Wl,-z,stack-size=131072 \
    -o /benchmark/benchmark.wasm \
    blake3.c blake3_dispatch.c blake3_portable.c blake3_sse2.c benchmark.c
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
