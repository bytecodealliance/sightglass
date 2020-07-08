#!/bin/bash
ORIG_CWD="$(pwd)/"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
WASMTIME_APP_ROOT="${SCRIPT_LOC}/.."
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../.."
source ${SIGHTGLASS_ROOT}/config.inc

WASM_ENTRY=-DWASM_ENTRY
WASI_NO_SUPPORT="-DNO_WASI_SUPPORT"
WASI_CFLAGS="--sysroot=${WASI_SYSROOT}"

#Prepare shootout
mkdir -p ${SCRIPT_LOC}/benchmark; cd ${SCRIPT_LOC}/benchmark
cp ${SIGHTGLASS_ROOT}/benchmarks/shootout/* .

#Build shootout
for cfile in ./*.c; do
    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -c $cfile -o $(basename -s .c "$cfile").wasm.o
    ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -c $cfile -o $(basename -s .c "$cfile").wasm.o
    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o -o $(basename -s .c "$cfile").wasm -nostartfiles
    ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o -o $(basename -s .c "$cfile").wasm -nostartfiles
done

#Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin
echo ${WASMTIME_APP_ROOT}
>&2 echo ${CC} ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${WASMTIME_APP_ROOT}/build/wasmtime/target/release/wasmtime -shared -o implementation.so ../wrapper.c  -fPIC
${CC} -fPIC ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${WASMTIME_APP_ROOT}/build/wasmtime/target/release/wasmtime -shared -o implementation.so ../wrapper.c  -fPIC

cd ${ORIG_CWD}
