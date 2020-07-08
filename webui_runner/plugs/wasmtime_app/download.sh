#!/bin/bash

>&2 echo "wasmtime_app download"
mkdir -p build; cd build
if [ ! -d wasmtime ]; then
    git clone https://github.com/CraneStation/wasmtime.git wasmtime
fi
cd wasmtime
git clean -fd
git submodule foreach --recursive git clean -fd
git reset --hard
git submodule init && git submodule update
git submodule foreach --recursive git submodule init
git submodule foreach --recursive git submodule update
git pull

>&2 echo "wasmtime_app build"
cargo build --release
cd ../../
