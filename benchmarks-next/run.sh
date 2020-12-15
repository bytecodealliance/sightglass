#!/bin/bash

set -e
PROJECT_DIR=$(dirname "$0" | xargs dirname)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
ENGINE=$(realpath $PROJECT_DIR/../wasmtime/target/debug/libwasmtime_bench_api.so)
export RUST_LOG=debug

for BENCH_FILE in $(find $PROJECT_DIR/benchmarks-next -name benchmark.wasm); do
    BENCH_DIR=$(dirname $BENCH_FILE)
    BENCH_NAME=$(basename $BENCH_DIR)

    # Run the Wasm benchmark.
    $SIGHTGLASS benchmark $BENCH_FILE --engine $ENGINE --num-iterations 3
done
