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
RUN apt update && apt install -y git cmake make patch

# Set common env vars.
ENV CC=/wasi-sdk/bin/clang
ENV CXX=/wasi-sdk/bin/clang++
ENV LD=/wasi-sdk/bin/lld
ENV CFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
ENV CXXFLAGS=--sysroot=/wasi-sdk/share/wasi-sysroot
# Prepend wasi-sdk's bin while keeping the system PATH so `git`, `cmake`, `make`
# and `patch` still resolve.
ENV PATH=/wasi-sdk/bin:$PATH

WORKDIR /usr/src
RUN git clone https://github.com/kpu/intgemm.git

WORKDIR /usr/src/intgemm
RUN git checkout be3053515a8a04d19c6959a370eaf8b5a6eab686
COPY benchmark.cpp .
COPY sightglass.h .
COPY patch-intgemm.diff .
COPY wasm_sse_compat.h .
RUN patch -p 1 < patch-intgemm.diff

# wasi-sdk's clang cannot compile x86 SSE intrinsics for wasm. `wasm_sse_compat.h`
# reimplements the `_mm_*` intrinsics intgemm uses on top of `<wasm_simd128.h>`.
# We expose it under the x86 intrinsic-header names in a shim directory placed
# first on the include path, so intgemm's `#include <emmintrin.h>` (etc.) resolve
# to the compat header instead of clang's x86-only headers (which #error on wasm).
RUN mkdir -p /shim && cp wasm_sse_compat.h /shim/ && \
    for h in emmintrin tmmintrin xmmintrin smmintrin pmmintrin nmmintrin immintrin; do \
      echo '#include "wasm_sse_compat.h"' > /shim/$h.h; \
    done

# Flags shared by the library and the final link: emit Wasm SIMD (`-msimd128`),
# resolve x86 intrinsic headers to the compat shim, and define `WASM` (activates
# intgemm's patched SSSE3-only CPU selection — there is no runtime CPUID on Wasm).
# intgemm's AVX2/AVX512 feature-detection fails under wasi-sdk, so those kernels
# compile out. `-Wno-unknown-warning-option` keeps intgemm's `-Werror` build from
# choking on its Emscripten-only `-Wno-error=pthreads-mem-growth` flag.
# `-fno-exceptions` matches the other C++ benchmarks (gcc-loops, hashset) and
# avoids pulling in the C++ exception runtime, which wasi-sdk does not link by
# default; intgemm's only `throw` is already guarded behind `__EXCEPTIONS`.
ENV WASM_FLAGS="-O3 -msimd128 -fno-exceptions -I/shim -DWASM -Wno-unknown-warning-option"

# Build the static library.
RUN mkdir build
WORKDIR /usr/src/intgemm/build
RUN cmake .. -DCMAKE_TOOLCHAIN_FILE=/wasi-sdk/share/cmake/wasi-sdk.cmake \
    -DCOMPILE_WASM=1 -DCMAKE_CXX_FLAGS="$WASM_FLAGS"
RUN make intgemm

WORKDIR /usr/src/intgemm
RUN mkdir /benchmark
RUN $CXX $CXXFLAGS $WASM_FLAGS -Ibuild \
    -Wl,-z,stack-size=2097152 \
    -o /benchmark/benchmark.wasm \
    benchmark.cpp build/libintgemm.a
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
