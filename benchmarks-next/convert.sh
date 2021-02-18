#!/bin/bash

# Helper script for converting an existing C benchmark (e.g. a shootout benchmark) into a directory
# in benchmarks-next. The conversion is skewed towards shootout benchmarks and some massaging of the
# C may be necessary (removing the old context structures, changing `[benchmark]_body` to `main`,
# adding `printf` statements, etc.). Use:
#
# ./convert.sh path/to/c/file.c [optionally, a directory name]

set -e
BENCHMARK_C=$(realpath $1)
BENCHMARK_NAME=${2:-$(basename ${BENCHMARK_C} ".${BENCHMARK_C##*.}")}
BENCHMARKS_DIR=$(cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd)
SIGHTGLASS="cargo +nightly run --bin sightglass-cli --"
export RUST_LOG=debug

set -x
mkdir -p ${BENCHMARKS_DIR}/${BENCHMARK_NAME}
pushd ${BENCHMARKS_DIR}/${BENCHMARK_NAME}
ln -sf ../Dockerfile.wasi-sdk Dockerfile
ln -sf ../../include/sightglass-next.h sightglass.h
cp -n ${BENCHMARK_C} benchmark.c
$SIGHTGLASS build-benchmark ./Dockerfile -d ./benchmark.wasm
popd
