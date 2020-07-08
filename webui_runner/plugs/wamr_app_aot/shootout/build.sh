#!/bin/bash

ORIG_CWD="$(pwd)/"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../.."
source ${SIGHTGLASS_ROOT}/config.inc

WASM_ENTRY=-DWASM_ENTRY
WASI_NO_SUPPORT="-DNO_WASI_SUPPORT"
WASI_CFLAGS="--sysroot=${WASI_SYSROOT}"

WAMRC="${SIGHTGLASS_ROOT}/plugs/wamr_app_aot/build/wamr/wamr-compiler/build/wamrc"
IWASM="${SIGHTGLASS_ROOT}/plugs/wamr_app_aot/build/wamr/product-mini/platforms/linux/build/iwasm"

#Prepare shootout
mkdir -p ${SCRIPT_LOC}/benchmark; cd ${SCRIPT_LOC}/benchmark
cp -r ${SIGHTGLASS_ROOT}/benchmarks/shootout/* .
ln -s ${WAMRC} wamrc

#Build shootout
for cfile in ./*.c; do
    echo ${WASM_CC} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $cfile -o $(basename -s .c "$cfile").wasm -Wl,--no_entry
    ${WASM_CC}  ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $cfile -o $(basename -s .c "$cfile").wasm -Wl,--no-entry

	echo ${WAMRC} -o $(basename -s .c "$cfile").wasm.aot $(basename -s .c "$cfile").wasm
	${WAMRC} -o $(basename -s .c "$cfile").wasm.aot $(basename -s .c "$cfile").wasm
done

#Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin
echo ${CC} ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${IWASM} -shared -o implementation.so ../wrapper.c
${CC} -fPIC ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${IWASM} -shared -o implementation.so ../wrapper.c

cd ${ORIG_CWD}