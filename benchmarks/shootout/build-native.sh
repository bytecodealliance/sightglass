#!/usr/bin/env bash

# Build all of the shootout benchmarks as native shared libraries (Linux-only).
#
# Usage: ./build-native.sh

ENGINE_DIR=${ENGINE_DIR:-"../../engines/native"}
SRC_DIR=${SRC_DIR:-"src"}
BENCHMARKS=${BENCHMARKS:-$(find $SRC_DIR -name '*.c')}
CFLAGS=${CFLAGS:-"-O3 -fPIC -I$SRC_DIR -Wno-attributes"}
LDFLAGS=${LDFLAGS:-"-shared -L$ENGINE_DIR"}

for BENCHMARK in $BENCHMARKS; do
    NAME=$(basename $BENCHMARK .c);
    (set -x; cc -Dmain=native_entry $CFLAGS $LDFLAGS -o shootout-$NAME.so $BENCHMARK -lengine)
done
