#!/bin/bash

ORIG_CWD="$(pwd)"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../../"
source ${SIGHTGLASS_ROOT}/config.inc

# Copy polybench
PRINT_TIME="-DPOLYBENCH_TIME"
PRINT_TIME=""
mkdir -p ${SCRIPT_LOC}/benchmark; cd ${SCRIPT_LOC}/benchmark
cp -r ${SIGHTGLASS_ROOT}/benchmarks/polybench/* .

# Polybench list of files
polyfiles=(
./datamining/correlation/correlation.c
./datamining/covariance/covariance.c
./linear-algebra/kernels/2mm/a2mm.c
./linear-algebra/kernels/3mm/a3mm.c
./linear-algebra/kernels/atax/atax.c
./linear-algebra/kernels/bicg/bicg.c
./linear-algebra/kernels/doitgen/doitgen.c
./linear-algebra/kernels/mvt/mvt.c
./linear-algebra/blas/gemm/gemm.c
./linear-algebra/blas/gemver/gemver.c
./linear-algebra/blas/gesummv/gesummv.c
./linear-algebra/blas/symm/symm.c
./linear-algebra/blas/syr2k/syr2k.c
./linear-algebra/blas/syrk/syrk.c
./linear-algebra/blas/trmm/trmm.c
./linear-algebra/solvers/cholesky/cholesky.c
./linear-algebra/solvers/durbin/durbin.c
./linear-algebra/solvers/gramschmidt/gramschmidt.c
./linear-algebra/solvers/lu/lu.c
./linear-algebra/solvers/ludcmp/ludcmp.c
./linear-algebra/solvers/trisolv/trisolv.c
./medley/deriche/deriche.c
./medley/floyd-warshall/floyd_warshall.c
./medley/nussinov/nussinov.c
./stencils/adi/adi.c
./stencils/fdtd-2d/fdtd_2d.c
./stencils/heat-3d/heat_3d.c
./stencils/jacobi-1d/jacobi_1d.c
./stencils/jacobi-2d/jacobi_2d.c
./stencils/seidel-2d/seidel_2d.c
)

# Build polybench
mkdir -p base_native_compile/
for cfile in ${polyfiles[@]}; do 
    echo ${CC} ${COMMON_CFLAGS} -I utilities -I linear-algebra/kernels/atax utilities/polybench.c $cfile $PRINT_TIME -o  base_native_compile/$(basename -s .c "$cfile")
    ${CC} ${COMMON_CFLAGS} -I utilities -I linear-algebra/kernels/atax utilities/polybench.c $cfile $PRINT_TIME -o  base_native_compile/$(basename -s .c "$cfile")
done

#Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin

echo ${CC} ${COMMON_CFLAGS} "-DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark/base_native_compile" -shared -o implementation.so ../wrapper.c
${CC} -fPIC ${COMMON_CFLAGS} "-DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark/base_native_compile" -shared -o implementation.so ../wrapper.c

cd ${ORIG_CWD}