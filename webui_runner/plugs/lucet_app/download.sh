#!/bin/bash

source ../../config.inc

>&2 echo "lucet_app download"
mkdir -p build; cd build
if [ ! -d lucet ]; then 
    git clone https://github.com/fastly/lucet.git lucet
fi
cd lucet
git clean -fd
git submodule foreach --recursive git clean -fd
git reset --hard
git submodule init && git submodule update
git submodule foreach --recursive git submodule init
git submodule foreach --recursive git submodule update

>&2 echo "lucet_app build"
WASI_SYSROOT=${WASI_SYSROOT} CLANG_ROOT=${CLANG_ROOT} CLANG=${CLANG} cargo build --release

>&2 echo "lucet_app builtins build"
cd ./lucet-builtins
make
