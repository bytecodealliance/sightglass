#!/bin/bash

ORIG_CWD="$(pwd)"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../../"
source ${SIGHTGLASS_ROOT}/config.inc

#Copy shootout
mkdir -p ${SCRIPT_LOC}/benchmark; cd ${SCRIPT_LOC}/benchmark
cp -r ${SIGHTGLASS_ROOT}/benchmarks/shootout/* .

#Patch shootout

#Build shootout
for cfile in ./*.c; do 
    echo ${CC} ${COMMON_CFLAGS} -DSKIP_ENTRY_AND_LEND $cfile -o  $(basename -s .c "$cfile")
    ${CC} ${COMMON_CFLAGS} -DSKIP_ENTRY_AND_LEND $cfile -o  $(basename -s .c "$cfile")
done

#Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin

echo ${CC} ${COMMON_CFLAGS} "-DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark" -shared -o implementation.so ../wrapper.c
${CC} -fPIC ${COMMON_CFLAGS} "-DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark" -shared -o implementation.so ../wrapper.c

cd ${ORIG_CWD}