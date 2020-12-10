FROM emscripten/emsdk:2.0.8

WORKDIR /
COPY benchmark.c .
RUN emcc benchmark.c -O3 -g -DNDEBUG -s TOTAL_MEMORY=268435456 -o benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.
