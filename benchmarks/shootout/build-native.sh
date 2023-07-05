#!/usr/bin/env bash

# Build all of the shootout benchmarks as native shared libraries (Linux-only).
#
# Usage: ./build-native.sh

ENGINE_DIR=${ENGINE_LIBRARY_DIR:-"../../engines/native"}
SRC_DIR=${SRC_DIR:-"src"}
BENCHMARKS=${BENCHMARKS:-$(find $SRC_DIR -name '*.c')}
CFLAGS=${CFLAGS:-"-O3 -fPIC -I$SRC_DIR -Wno-attributes"}
LDFLAGS=${LDFLAGS:-"-shared -L$ENGINE_DIR -lengine"}

for BENCHMARK in $BENCHMARKS; do
    NAME=$(basename $BENCHMARK .c);
    cc -Dmain=native_entry $CFLAGS $LDFLAGS $BENCHMARK -o shootout-$NAME.so
done
