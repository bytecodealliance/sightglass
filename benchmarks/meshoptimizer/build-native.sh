#!/usr/bin/env bash

# Build all of the shootout benchmarks as native shared libraries (Linux-only).
#
# Usage: ./build-native.sh

(set -x; rm -r -f meshoptimizer);
(set -x; git clone --recurse-submodules https://github.com/zeux/meshoptimizer.git);
(set -x; cd meshoptimizer; git reset --hard f734fd572aed5bf76e84d9ed62ca6f4f6c47d84e; cd -);
(set -x; cp sightglass.h sightglass.native.patch meshoptimizer/);
(set -x; cd meshoptimizer; patch  -Np1 -i ./sightglass.native.patch; cd -);
(set -x; cd meshoptimizer; CXX=/usr/bin/clang++ make codecbench-simd.so ; cd -);

