#!/usr/bin/env bash

# Build regex benchmark as native shared libraries (Linux-only).
#
# Usage: ./build-native.sh

(set -x;)
(rm -rf rust-benchmark-native);
(cp -r rust-benchmark rust-benchmark-native/);
(cp sightglass.native.patch rust-benchmark-native/);
(cd rust-benchmark-native; patch -Np1 -i ./sightglass.native.patch; cd -);
(cd rust-benchmark-native; cargo build --release; cp target/release/libbenchmark.so ../benchmark.so; cd -);
(set +x;)
