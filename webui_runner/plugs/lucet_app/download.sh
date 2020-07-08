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
git pull

>&2 echo "lucet_app build"
make install
