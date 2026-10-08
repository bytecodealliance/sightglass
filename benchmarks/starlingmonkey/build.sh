#!/bin/sh
#
# Runs inside the Docker image from this directory's Dockerfile: turns each
# `js/<name>/` workload into a pre-initialized `starlingmonkey-<name>.wasm`.

set -e -x

SRC=/usr/src/benchmark
COMPONENTIZE=/usr/src/build/starling-raw.wasm/componentize.sh
mkdir -p /benchmark

for name in json markdown regex; do
    script="${SRC}/starlingmonkey-${name}.js"
    # One classic script, like the `spidermonkey` benchmarks evaluate: libraries
    # first, then `main.js`, then any warm-up, then hand `main` to the
    # `bench.cpp` builtin.
    : >"$script"
    for f in "${SRC}/js/${name}"/*.js; do
        [ "$(basename "$f")" = main.js ] || { cat "$f"; echo; } >>"$script"
    done
    cat "${SRC}/js/${name}/main.js" >>"$script"
    [ ! -f "${SRC}/warmup/${name}.js" ] || cat "${SRC}/warmup/${name}.js" >>"$script"
    printf '\nregisterBenchmark("./starlingmonkey-%s.input", main);\n' "$name" >>"$script"

    # Wizer evaluates the script and snapshots the heap.
    "$COMPONENTIZE" --legacy-script "$script" -o "/benchmark/starlingmonkey-${name}.wasm"
done
