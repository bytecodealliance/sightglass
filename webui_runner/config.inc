#! /bin/sh

# Variables used by docker scripts
HOST_BASE_PREFIX=${HOST_BASE_PREFIX:-$(cd "$(dirname ${BASH_SOURCE:-$0})" && pwd)}
SG_DOCKER_IMAGE_NAME="sg_runner_image"
SG_DOCKER_CONTAINER_NAME="sg_runner"

# TODO: PORTS are currently hardcoded in the docker image
# Any change here must be consistent with the docker image
SG_HOSTNAME=localhost
SG_RESULTS_PORT=8000

# Variables used by plugs directory scripts
ROOT_LOC=${ROOT_LOC:-$(cd "$(dirname ${BASH_SOURCE:-$0})" && pwd)}

WASI_SYSROOT="/opt/wasi-sdk/share/wasi-sysroot/"
CLANG_ROOT="/usr/lib/clang/10/lib/"
CLANG="/usr/bin/clang-10"
LLVM_AR="/usr/bin/llvm-ar-10"
LLVM_NM="/usr/bin/llvm-nm-10"

WASM_CC="/opt/wasi-sdk/bin/clang-10"
CC="/usr/bin/clang-10"
COMMON_CFLAGS="--std=c99 -Ofast -Wall -W -I${ROOT_LOC}/sightglass/include"
