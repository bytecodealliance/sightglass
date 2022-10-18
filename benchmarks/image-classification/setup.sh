#!/bin/bash

# This will download a copy of OpenVINO and extract it locally. Once this script is run, you can use this command to run it:
# cargo run -- benchmark benchmarks/wasi-nn/benchmark.wasm --engine engines/wasmtime/libengine.so

WASI_NN_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
FILENAME=l_openvino_toolkit_ubuntu20_2022.2.0.7713.af16ea1d79a_x86_64
[ ! -d ${WASI_NN_DIR}/openvino ] && wget -nc -q --no-check-certificate https://storage.openvinotoolkit.org/repositories/openvino/packages/2022.2/linux/${FILENAME}.tgz -O ${WASI_NN_DIR}/${FILENAME}.tgz
[ ! -d ${WASI_NN_DIR}/openvino ] && tar -C ${WASI_NN_DIR} -zxf ${WASI_NN_DIR}/${FILENAME}.tgz  && mv ${WASI_NN_DIR}/${FILENAME} ${WASI_NN_DIR}/openvino || echo "OpenVINO is already there, skipping..."
source ${WASI_NN_DIR}/openvino/setupvars.sh
