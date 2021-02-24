#!/bin/bash

set -e
PROJECT_DIR=$(dirname "$0" | xargs dirname)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
export RUST_LOG=debug

$SIGHTGLASS build-engine wasmtime
for BENCH_FILE in $(find $PROJECT_DIR/benchmarks-next -name benchmark.wasm); do
    BENCH_DIR=$(dirname $BENCH_FILE)
    BENCH_NAME=$(basename $BENCH_DIR)

    # Run the Wasm benchmark.
    $SIGHTGLASS in-process-benchmark --engine wasmtime --num-iterations 3 --working-dir $BENCH_DIR $BENCH_FILE
done
