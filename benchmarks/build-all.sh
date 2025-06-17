#!/usr/bin/env bash

# Either build all of the benchmarks or a random subset of them.
#
# Usage: ./build-all.sh <number of benchmarks>
# - <number of benchmarks>, an optional number of benchmarks to build; if
#   provided, this script will randomize the list of benchmarks and pick a
#   subset of them to build

set -e
# From https://stackoverflow.com/a/246128:
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
BENCHMARKS_DIR=$(realpath $SCRIPT_DIR)
BUILD_SCRIPT=$BENCHMARKS_DIR/build.sh
DOCKERFILES=$(find $BENCHMARKS_DIR -name Dockerfile)

# Some benchmarks have known rebuild failures but are still useful to retain.
# This "skip list" prevents them from being considered for rebuilding.
SKIPLIST=("spidermonkey")
for SKIP in $SKIPLIST; do
    DOCKERFILES=$(echo $DOCKERFILES | sed -e "s+$BENCHMARKS_DIR/$SKIP/Dockerfile++")
done

# If a numeric parameter `N` is provided to the script (e.g., `./build-all.sh
# 5`), randomly select `N` benchmarks to rebuild; otherwise, rebuild all
# benchmarks.
re='^[0-9]+$'
if [[ $1 =~ $re ]]; then
    DOCKERFILES=$(echo "$DOCKERFILES" | shuf -n $1)
fi

for DOCKERFILE in $DOCKERFILES; do
    BENCHMARK_DIR=$(dirname $DOCKERFILE)
    $BUILD_SCRIPT $BENCHMARK_DIR
done
