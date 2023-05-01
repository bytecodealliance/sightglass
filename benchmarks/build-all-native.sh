#!/usr/bin/env bash

# Either build all of the benchmark's native target or a random subset of them.
#
# Usage: ./build-all-native.sh <number of benchmarks>
# - <number of benchmarks>, an optional number of benchmarks to build; if provided, this script will
#   randomize the list of benchmarks and pick a subset of them to build

set -e
#PROJECT_DIR=$(dirname "$0" | xargs dirname | xargs realpath)
# Assumes this script is located at the base of the benchmark directory
BENCHMARKS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo $BENCHMARKS_DIR
#BENCHMARKS_DIR=$SCRIPT_DIR/benchmarks
BUILD_SCRIPT=$BENCHMARKS_DIR/build-native.sh
DOCKERFILES=$(find $BENCHMARKS_DIR -name Dockerfile.native)

# If a numeric parameter `N` is provided to the script (e.g., `./build-all-native.sh 5`), randomly select
# `N` benchmarks to rebuild; otherwise, rebuild all benchmarks.
re='^[0-9]+$'
if [[ $1 =~ $re ]]; then
    DOCKERFILES=$(echo "$DOCKERFILES" | shuf -n $1)
fi

for DOCKERFILE in $DOCKERFILES; do
    BENCHMARK_DIR=$(dirname $DOCKERFILE)
    $BUILD_SCRIPT $BENCHMARK_DIR
done
