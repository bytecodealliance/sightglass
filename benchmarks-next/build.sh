#!/bin/bash

set -e
PROJECT_DIR=$(dirname "$0" | xargs dirname | xargs realpath)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
export RUST_LOG=debug

for DOCKERFILE in $(find $PROJECT_DIR/benchmarks-next -name Dockerfile); do
    BENCH_DIR=$(dirname $DOCKERFILE)
    BENCH_NAME=$(basename $BENCH_DIR)

    # Copy the sightglass header into the bench directory (TODO remove; this is a hack to avoid
    # duplicating the header everywhere since Docker will not recognize linked context files).
    cp $PROJECT_DIR/include/sightglass-next.h $BENCH_DIR/sightglass.h

    # Build the Wasm benchmark.
    $SIGHTGLASS build $DOCKERFILE -d $BENCH_DIR/benchmark.wasm --emit-wat
done
