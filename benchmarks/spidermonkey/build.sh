#!/bin/sh

set -x -e

SM=/usr/src/spidermonkey-wasi-embedding

xxd -i js/marked.min.js > marked_js.h
xxd -i js/main.js > main_js.h

mkdir -p /benchmark
/opt/wasi-sdk/bin/clang++ -lwasi-emulated-getpid -O3 -std=c++17 -o /benchmark/benchmark.wasm runtime.cpp -I$SM/release/include/ $SM/release/lib/*.o $SM/release/lib/*.a

/opt/wasi-sdk/bin/strip /benchmark/benchmark.wasm
