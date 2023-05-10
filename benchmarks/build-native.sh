#!/usr/bin/env bash

# Build a single native benchmark

set -e

BENCHMARKS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SIGHTGLASS_BASE=$(dirname $BENCHMARKS_DIR)
ENGINE=$SIGHTGLASS_BASE/engines/native/libengine.so

BENCHMARK_DIR="";

for ARG in "$@"; do
    if [[ -d $ARG ]]; then
        BENCHMARK_DIR=$ARG;
        break
    fi
done

print_header() {
    >&2 echo
    >&2 echo ===== $@ =====
}


if [[ $BENCHMARK_DIR == "" ]]; then
    echo "Unknown benchmark directory; usage: ./build-native.sh [--host] <path to benchmark directory>"
    exit 1
fi


# Build engine if not availble since it needs to be present for linking
if [[ ! -f $ENGINE ]]; then
    cd $SIGHTGLASS_BASE/engines/native/libengine/
    cargo build --release
    cp target/release/libnative_bench_api.so ../libengine.so
    cd - > /dev/null
fi

# Build directly on host
FLAG='--host'
if [[ $* == *$FLAG* ]]; then

print_header "Build Native Benchmark Using Host"
(set -x; cd $BENCHMARK_DIR && cargo run --release)


else

BENCHMARK_NAME=$(readlink -f $BENCHMARK_DIR | xargs basename)
IMAGE_NAME=sightglass-benchmark-$BENCHMARK_NAME-native

# To allow the use of symlinks in the benchmark directories (docker ignores them), we `tar` up the
# directory and `--dereference` (i.e., follow) all symlinks provided.
print_header "Create build context"
TMP_TAR=$(mktemp /tmp/sightglass-benchmark-dir-XXXXXX.tar)
(set -x; cd $BENCHMARK_DIR && ln -f -s ../../engines/native/libengine.so ./libengine.so && tar --create --file $TMP_TAR --dereference --verbose . && rm libengine.so)

# Build the benchmark and extract the build results from the container
print_header "Build Native Benchmark Using Host"
(set -x; docker build -f Dockerfile.native --tag $IMAGE_NAME - < $TMP_TAR)
CONTAINER_ID=$(set -x; docker create $IMAGE_NAME)
(set -x; mkdir -p $BENCHMARK_DIR/target; docker cp $CONTAINER_ID:/benchmark/target/benchmark.so $BENCHMARK_DIR/target/benchmark.so;)

# Copy host files to container and build inside a container

fi

