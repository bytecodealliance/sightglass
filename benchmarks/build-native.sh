#!/usr/bin/env bash

# Build a single native benchmark using

set -e

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

FLAG='--host'

if [[ $BENCHMARK_DIR == "" ]]; then
    echo "Unknown benchmark directory; usage: ./build-native.sh [--host] <path to benchmark directory>"
    exit 1
fi

# Build directly on host
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
(set -x; cd $BENCHMARK_DIR && tar --create --file $TMP_TAR --dereference --verbose .)

# Build the benchmark and extract the build results from the container
print_header "Build Native Benchmark Using Host"
(set -x; docker build -f Dockerfile.native --tag $IMAGE_NAME - < $TMP_TAR)
CONTAINER_ID=$(set -x; docker create $IMAGE_NAME)
(set -x; mkdir -p $BENCHMARK_DIR/target; docker cp $CONTAINER_ID:/benchmark/target/benchmark.so $BENCHMARK_DIR/target/benchmark.so;)

# Copy host files to container and build inside a container

fi

