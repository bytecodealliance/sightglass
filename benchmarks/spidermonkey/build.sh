#!/bin/sh
#
# NOTE: this build script is expected to run within the
# Docker container specified by the Dockerfile in this
# directory.

set -x -e

SRCDIR=/usr/src
SMDIR=${SRCDIR}/spidermonkey-wasi-embedding
mkdir -p /benchmark

build_js_benchmark() {
    name="$1"

    cd ${SRCDIR}

    # Convert all JS files to C header files that just contain
    # the hex encoded contents.
    for js_file in "js/${name}"/*.js; do
        if [ -f "$js_file" ]; then
            basename=$(basename "$js_file")
            cp "$js_file" . && xxd -i "$basename" >"${basename}.h" && rm "$basename"
        fi
    done

    # Generate a simple header that includes all the JS headers
    cat >"js_files.h" <<EOF
// Generated header that includes all JS files for ${name} benchmark
#pragma once

EOF

    # Include each generated header
    for js_file in "js/${name}"/*.js; do
        if [ -f "$js_file" ]; then
            basename=$(basename "$js_file")
            echo "#include \"${basename}.h\"" >>"js_files.h"
        fi
    done

    # Compile the benchmark
    /opt/wasi-sdk/bin/clang++ -lwasi-emulated-getpid -O3 -std=c++17 \
        -DINPUT_FILE="\"spidermonkey-${name}.input\"" \
        -o "/benchmark/spidermonkey-${name}.wasm" runtime.cpp \
        -I${SMDIR}/release/include/ ${SMDIR}/release/lib/*.o ${SMDIR}/release/lib/*.a

    # Strip the binary
    /opt/wasi-sdk/bin/strip "/benchmark/spidermonkey-${name}.wasm"

    # Clean up generated files
    rm -f -- *.js.h js_files.h
}

# Build all benchmarks
build_js_benchmark "markdown"
build_js_benchmark "json"
build_js_benchmark "regex"
