FROM emscripten/emsdk:2.0.8

WORKDIR /usr/src
RUN git clone https://github.com/kpu/intgemm.git

WORKDIR /usr/src/intgemm
RUN git checkout be3053515a8a04d19c6959a370eaf8b5a6eab686
COPY benchmark.cpp .
COPY sightglass.h .

COPY patch-intgemm.diff .
RUN patch -p 1 < patch-intgemm.diff

# Building static library
RUN mkdir build
WORKDIR /usr/src/intgemm/build
RUN emcmake cmake .. -DCOMPILE_WASM=1 -DCMAKE_CXX_FLAGS="-msimd128 -mssse3 -O3"
RUN emmake make intgemm

WORKDIR /usr/src/intgemm
RUN mkdir /benchmark
RUN emcc -O3 -s STANDALONE_WASM=1 \
    -s INITIAL_MEMORY=33554432 -s MAXIMUM_MEMORY=33554432 \
    -s ALLOW_MEMORY_GROWTH=0 -s TOTAL_STACK=2097152 \
    -msimd128 -mssse3 -O3 -Ibuild \
    -s "EXPORTED_FUNCTIONS=['_main']" \
    -o /benchmark/benchmark.wasm \
    benchmark.cpp build/libintgemm.a
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
