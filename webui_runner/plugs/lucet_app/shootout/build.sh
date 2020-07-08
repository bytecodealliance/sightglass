#!/bin/bash

ORIG_CWD="$(pwd)/"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
LUCET_APP_ROOT="${SCRIPT_LOC}/.."
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../.."
source ${SIGHTGLASS_ROOT}/config.inc

WASM_ENTRY=-DWASM_ENTRY
WASI_NO_SUPPORT="-DNO_WASI_SUPPORT"
WASI_CFLAGS="--sysroot=${WASI_SYSROOT}"

LUCET="/opt/lucet/bin/lucet-wasi"
LUCETC="/opt/lucet/bin/lucetc-wasi"
LUCETC_FLAGS=-"-opt-level 0 --min-reserved-size 4294967296"
LUCET_LIBBUILTINS="${SIGHTGLASS_ROOT}/plugs/lucet_app/build/lucet/lucet-builtins/build/libbuiltins.so"
LUCET_BINDINGS="${SIGHTGLASS_ROOT}/plugs/lucet_app/build/lucet/lucet-wasi/bindings.json"


#Prepare shootout
mkdir -p ${SCRIPT_LOC}/benchmark; cd ${SCRIPT_LOC}/benchmark
cp -r ${SIGHTGLASS_ROOT}/benchmarks/shootout/* .


#Build shootout
for cfile in ./*.c; do
    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -c $cfile -o $(basename -s .c "$cfile").wasm.o
    ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -c $cfile -o $(basename -s .c "$cfile").wasm.o

    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o -o $(basename -s .c "$cfile").wasm  -nostartfiles
    ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o -o $(basename -s .c "$cfile").wasm -nostartfiles

	echo ${LUCETC} $(basename -s .c "$cfile").wasm -o $(basename -s .c "$cfile").wasm.so
	${LUCETC} $(basename -s .c "$cfile").wasm -o $(basename -s .c "$cfile").wasm.so
done

#Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin
echo ${CC} ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${LUCET} -shared -o implementation.so ../wrapper.c
${CC} -fPIC ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${LUCET} -shared -o implementation.so ../wrapper.c

cd ${ORIG_CWD}