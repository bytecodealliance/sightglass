#!/usr/bin/env bash

# Run a single native benchmark that is already built.
#
# Usage: ./run-native.sh <path-to-benchmark-folder/benchmark.so>

set -e

BENCHMARKS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SIGHTGLASS_BASE=$(dirname $BENCHMARKS_DIR)
SIGHTGLASS="cargo run --release --bin sightglass-cli --"
ENGINE=$SIGHTGLASS_BASE/engines/native/libengine.so
BENCHMARK_NATIVE_SO=$1

if [[ -z $BENCHMARK_NATIVE_SO ]]; then
    echo "Usage: run-native.sh <path-to-benchmark-folder/benchmark.so>"
    exit 1
fi

# If an engine is not available, build it.
if [[ ! -f $ENGINE ]]; then
    cd $SIGHTGLASS_BASE/engines/native/libengine/
    cargo build --release
    cp target/release/libnative_bench_api.so ../libengine.so
    cd - > /dev/null
fi

# Because of some hard-coding in the native engine (TODO:
# https://github.com/bytecodealliance/sightglass/issues/259), we need to set up
# the temporary directory with the hard-coded paths.
BENCHMARK_NATIVE_SO="$(realpath $BENCHMARK_NATIVE_SO)"
MD5SUM=$(md5sum $BENCHMARK_NATIVE_SO | awk '{ print $1 }')
TMP_BENCHMARK_DIR=/tmp/sightglass-benchmark-native-$MD5SUM
mkdir -p $TMP_BENCHMARK_DIR
(set -x; ln -fs $BENCHMARK_NATIVE_SO $TMP_BENCHMARK_DIR/benchmark.so)
BENCHMARK_DIR=$(dirname $BENCHMARK_NATIVE_SO)
for FILE in $(find $BENCHMARK_DIR -name "*.input"); do
    (set -x; ln -fs $FILE $TMP_BENCHMARK_DIR/)
done

# Run a benchmark with the native library.
cd $SIGHTGLASS_BASE
ENGINE_DIR=$(dirname $ENGINE)
(set -x; LD_LIBRARY_PATH=$ENGINE_DIR $SIGHTGLASS benchmark --engine $ENGINE \
    --processes 1 --iterations-per-process 3 \
    --working-dir $TMP_BENCHMARK_DIR -- $TMP_BENCHMARK_DIR/benchmark.so)
cd - > /dev/null
