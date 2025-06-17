#!/usr/bin/env bash

# Check that the `all.suite` file contains all of the WebAssembly benchmarks.
# This script allows for benchmarks to be commented out within the file but
# ensures that new benchmarks are added to `all.suite`.
#
# Usage: ./check-incomplete-suite.sh

# From https://stackoverflow.com/a/246128:
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
SUITE=$SCRIPT_DIR/all.suite

for BENCHMARK in $(find $SCRIPT_DIR -name '*.wasm'); do
    BENCHMARK=${BENCHMARK#"$SCRIPT_DIR/"}
    if ! grep -q $BENCHMARK $SUITE ; then
        echo "did not find '$BENCHMARK' in $SUITE which should list all benchmarks--add a line for this benchmark?"
        exit 1
    fi
done
