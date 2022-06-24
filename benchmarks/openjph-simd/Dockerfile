FROM emscripten/emsdk:3.1.14

WORKDIR /usr/src
RUN git clone https://github.com/aous72/OpenJPH.git

WORKDIR /usr/src/OpenJPH
RUN git checkout e4bfbf8fa683e43037225b90468c306c3733656a
COPY benchmark.cpp .
COPY sightglass.h .
COPY sample_jph.c .

# Building static library
WORKDIR /usr/src/OpenJPH/build
RUN emcmake cmake .. -DEMSCRIPTEN=1 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS="-s DISABLE_EXCEPTION_CATCHING=1"
RUN emmake make openjphsimd

WORKDIR /usr/src/OpenJPH
RUN emcc \
  -s INITIAL_MEMORY=134217728 -s ERROR_ON_UNDEFINED_SYMBOLS=0 \
  -s STANDALONE_WASM=1 -s "EXPORTED_FUNCTIONS=['_main']" \
  -O3 -Isrc/core/common -Lbuild -lopenjphsimd \
  -o /benchmark.wasm \
  benchmark.cpp

# We output the Wasm file to /benchmark.wasm, where the client expects it.
