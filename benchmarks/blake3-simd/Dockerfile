FROM emscripten/emsdk:2.0.8

WORKDIR /usr/src
RUN git clone https://github.com/BLAKE3-team/BLAKE3.git

WORKDIR /usr/src/BLAKE3/c
COPY benchmark.c .
COPY sightglass.h .
RUN mkdir /benchmark
# I am not sure that all of the parameters passed below are needed; this is what I received. It
# would be preferable if the blake3 would tell us what SIMD parameters they plan to use for their
# NPM package (https://www.npmjs.com/package/blake3), which currently uses only scalar instructions.
RUN emcc -O3 -s STANDALONE_WASM=1 \
    -s INITIAL_MEMORY=1048576 -s MAXIMUM_MEMORY=16777216 \
    -s ALLOW_MEMORY_GROWTH=1 -s TOTAL_STACK=131072 \
    -s "EXPORTED_FUNCTIONS=['_main']" \
    -msimd128 -msse2 -msse3 -msse4.1 -msse4.2 \
    -DBLAKE3_NO_SSE41 -DBLAKE3_NO_AVX2 -DBLAKE3_NO_AVX512 \
    -o /benchmark/benchmark.wasm \
    blake3.c blake3_sse2.c blake3_dispatch.c \
    blake3_portable.c benchmark.c
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
