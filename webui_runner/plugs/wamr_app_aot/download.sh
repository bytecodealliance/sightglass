#!/bin/bash

source ../../config.inc

>&2 echo "wamr_app_aot download"
mkdir -p build; cd build
if [ ! -d wamr ]; then
    git clone https://github.com/bytecodealliance/wasm-micro-runtime.git wamr
fi
cd wamr
git clean -fd
git submodule foreach --recursive git clean -fd
git reset --hard
git submodule init && git submodule update
git submodule foreach --recursive git submodule init
git submodule foreach --recursive git submodule update
git pull

>&2 echo "wamr_app_aot build"
cd wamr-compiler
./build_llvm.sh
mkdir -p build && cd build
cmake ..
make
ln -s ./wamrc /usr/bin/wamrc
cd ../..

>&2 echo "wamr_app_aot core build"
cd product-mini/platforms/linux/
mkdir -p build && cd build
cmake ..
make
cd ../..
