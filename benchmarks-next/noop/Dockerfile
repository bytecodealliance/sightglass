FROM emscripten/emsdk:2.0.8

WORKDIR /
COPY benchmark.c .
COPY sightglass.h .
RUN emcc benchmark.c -O3 -g -DNDEBUG -I. -o benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.
