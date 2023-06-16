# This Dockerfile builds all of libsodium's benchmarks with Zig.
FROM ubuntu:22.04
RUN apt update && apt install -y wget xz-utils unzip git

# Download and extract the latest stable version of Zig and retrieve the
# `stable` branch of libsodium that can be built by it.
WORKDIR /
RUN wget https://ziglang.org/download/0.10.1/zig-linux-x86_64-0.10.1.tar.xz
RUN tar --extract --verbose --file=zig-linux-x86_64-0.10.1.tar.xz
ENV PATH="${PATH}:/zig-linux-x86_64-0.10.1"
RUN echo "zig version = $(zig version)"
RUN wget https://github.com/jedisct1/libsodium/archive/stable.zip
RUN unzip stable.zip
RUN mv /libsodium-stable /libsodium

# Build the benchmarks. Note how we patch the self-timing code with calls to the
# Sightglass `bench_start()` and `bench_end()` functions. Also note that each
# benchmark contains internal iterations; see `benchmarks_iterations` in
# `build.zig` and `ITERATIONS` in `cmptest.h`.
WORKDIR /libsodium
COPY sightglass.h src/libsodium/include/
COPY add-bench-calls.diff .
RUN git apply add-bench-calls.diff
RUN zig build -Dtarget=wasm32-wasi \
    -Drelease-fast -Dcpu=generic+bulk_memory+simd128 \
    -Denable_benchmarks=true -Diterations=1

# Place all of the WebAssembly benchmarks in `/benchmark`. Some benchmarks lose
# the `bench_end` import for some reason and are removed (TODO).
WORKDIR /benchmark
RUN cp /libsodium/zig-out/bin/*.wasm .
RUN for f in *.wasm; do mv "$f" "libsodium-$f"; done
RUN rm libsodium-sodium_utils2.wasm libsodium-sodium_utils3.wasm
