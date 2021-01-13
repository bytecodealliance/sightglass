#!/bin/bash

set -e
PROJECT_DIR=$(dirname "$0" | xargs dirname | xargs realpath)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
export RUST_LOG=debug

for DOCKERFILE in $(find $PROJECT_DIR/benchmarks-next -name Dockerfile); do
    BENCH_DIR=$(dirname $DOCKERFILE)
    BENCH_NAME=$(basename $BENCH_DIR)

    # Build the Wasm benchmark.
    $SIGHTGLASS build-benchmark $DOCKERFILE -d $BENCH_DIR/benchmark.wasm --emit-wat
done
