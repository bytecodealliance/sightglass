#!/usr/bin/env bash

# Build a benchmark using the `Dockerfile` in its benchmark directory. This
# script expects the benchmark directory to contain a `Dockerfile` that creates
# a `/benchmark` directory containing one or more `*.wasm` files that meet the
# Sightglass requirements (see `README.md`).
#
# Usage: ./build.sh <path to benchmark directory>

set -e

BENCHMARK_DIR=$1
if [[ ! -d $BENCHMARK_DIR ]]; then
    echo "Unknown benchmark directory; usage: ./build.sh <path to benchmark directory>"
    exit 1
fi

BENCHMARK_NAME=$(readlink -f $BENCHMARK_DIR | xargs basename)
IMAGE_NAME=sightglass-benchmark-$BENCHMARK_NAME
TMP_BENCHMARK=$(mktemp -d /tmp/sightglass-benchmark-XXXXXX)
>&2 echo "Building $BENCHMARK_DIR"

# Helpful logging function.
print_header() {
    >&2 echo
    >&2 echo ===== $@ =====
}

# To allow the use of symlinks in the benchmark directories (docker ignores them), we `tar` up the
# directory and `--dereference` (i.e., follow) all symlinks provided.
print_header "Create build context"
TMP_TAR=$(mktemp /tmp/sightglass-benchmark-dir-XXXXXX.tar)
(set -x; cd $BENCHMARK_DIR && tar --create --file $TMP_TAR --dereference --verbose .)

# Build the benchmark image and extract the generated `benchmark.wasm` file from its container.
print_header "Build benchmarks"
(set -x; docker build --tag $IMAGE_NAME - < $TMP_TAR)
CONTAINER_ID=$(set -x; docker create $IMAGE_NAME)
(set -x; docker cp $CONTAINER_ID:/benchmark/. $TMP_BENCHMARK)

# Verify benchmark is a valid Sightglass benchmark.
print_header "Verify benchmark"
# From https://stackoverflow.com/a/246128:
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
SIGHTGLASS_CARGO_TOML=$(dirname $SCRIPT_DIR)/Cargo.toml
for WASM in $TMP_BENCHMARK/*.wasm; do
    (set -x; cargo run --manifest-path $SIGHTGLASS_CARGO_TOML --quiet -- validate $WASM)
    (set -x; mv $WASM $BENCHMARK_DIR/)
done;

# Clean up.
print_header "Clean up"
(set -x; rm $TMP_TAR)
(set -x; rm -rf $TMP_BENCHMARK)
(set -x; docker rm $CONTAINER_ID)
# We do not remove the image and intermediate images here (e.g., `docker rmi $IMAGE_NAME`) to speed
# up `build-all.sh`; use `clean.sh` instead.
