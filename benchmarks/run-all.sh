#!/usr/bin/env bash

# Run all of the benchmarks found in the `benchmarks` directory.
#
# Usage: ./run-all.sh

set -e
PROJECT_DIR=$(dirname "$0" | xargs dirname)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
ENGINE=$PROJECT_DIR/engines/wasmtime/libengine.so
export RUST_LOG=debug

# If an engine is not available, build it.
if [[ ! -f $ENGINE ]]; then
    pushd $PROJECT_DIR/engines/wasmtime
    rustc build.rs
    ./build
    popd
fi

# Benchmark each Wasm file.
DENY_LIST=("shootout-ackermann")
for BENCH_FILE in $(find $PROJECT_DIR/benchmarks -name benchmark.wasm); do
    BENCH_DIR=$(dirname $BENCH_FILE)
    BENCH_NAME=$(basename $BENCH_DIR)
    for DENY_FILE in $DENY_LIST; do
        if [[ $BENCH_NAME == $DENY_FILE ]]; then
            echo ""
            echo ""
            echo "********************************************************************"
            echo "Skipping $BENCH_NAME since it is included in the DENY_LIST:"
            echo "($DENY_LIST)"
            echo "********************************************************************"
            echo ""
            echo ""
            continue 2
        fi
    done
    $SIGHTGLASS benchmark --engine $ENGINE --processes 1 --iterations-per-process 3 --working-dir $BENCH_DIR $BENCH_FILE
done
