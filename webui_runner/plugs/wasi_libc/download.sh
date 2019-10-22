#!/bin/bash

echo "download wasi_libc start"
mkdir -p build; cd build
if [ ! -d wasi_libc ]; then 
    git clone https://github.com/CraneStation/wasi-libc.git wasi_libc
fi
cd wasi_libc
git clean -fd
git submodule foreach --recursive git clean -fd
git reset --hard
git submodule init && git submodule update
git submodule foreach --recursive git submodule init
git submodule foreach --recursive git submodule update
git checkout master

PATH_TO_CLANG="/usr/bin/clang-8"
PATH_TO_LLVM_AR="/usr/bin/llvm-ar-8"
PATH_TO_LLVM_NM="/usr/bin/llvm-nm-8"

make WASM_CC=${PATH_TO_CLANG} WASM_AR=${PATH_TO_LLVM_AR} WASM_NM=${PATH_TO_LLVM_NM}
echo "download wasi_libc done"