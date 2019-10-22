#!/bin/bash

ORIG_CWD="$(pwd)/"
SCRIPT_LOC="$(realpath $(dirname ${BASH_SOURCE:-$0}))"
LUCET_APP_ROOT="${SCRIPT_LOC}/.."
SIGHTGLASS_ROOT="${SCRIPT_LOC}/../../.."
source ${SIGHTGLASS_ROOT}/config.inc

WASM_ENTRY=-DWASM_ENTRY
WASI_NO_SUPPORT="-DNO_WASI_SUPPORT"
WASI_CFLAGS="--sysroot=${WASI_SYSROOT} --target=wasm32-unknown-wasi"
 

LUCETC="${SIGHTGLASS_ROOT}/plugs/lucet_app/build/lucet/target/release/lucetc"
LUCETC_FLAGS=-"-opt-level 0 --min-reserved-size 4294967296"
LUCET_LIBBUILTINS="${SIGHTGLASS_ROOT}/plugs/lucet_app/build/lucet/lucet-builtins/build/libbuiltins.so"
LUCET_BINDINGS="${SIGHTGLASS_ROOT}/plugs/lucet_app/build/lucet/lucet-wasi/bindings.json"


#Prepare shootout
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

polyfiles2=(
./datamining/correlation/correlation.c
)


#Build Polybench
mkdir -p lucet_app_compile/

# Compile utilities/polybench.c
echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -I utilities -c  utilities/polybench.c -o lucet_app_compile/polybench.wasm.o
${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -I utilities -c  utilities/polybench.c -o lucet_app_compile/polybench.wasm.o


# Compile the rest
for cfile in ${polyfiles[@]}; do
		echo "*****"
    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -I utilities $PRINT_TIME -c $cfile -o $(basename -s .c "$cfile").wasm.o
    ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} -I utilities $PRINT_TIME -c $cfile -o $(basename -s .c "$cfile").wasm.o
		echo "++++++"
    echo ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o lucet_app_compile/polybench.wasm.o -o $(basename -s .c "$cfile").wasm  -nostartfiles -Wl,--no-entry -Wl,--export-all -Wl,--gc-sections 
          ${WASM_CC} ${WASM_ENTRY} ${WASI_CFLAGS} ${WASI_NO_SUPPORT} ${COMMON_CFLAGS} $(basename -s .c "$cfile").wasm.o lucet_app_compile/polybench.wasm.o -o $(basename -s .c "$cfile").wasm -nostartfiles -Wl,--no-entry -Wl,--export-all -Wl,--gc-sections
		echo "------"
    echo ${LUCETC} ${LUCETC_FLAGS} --emit=obj  \
		--bindings=${LUCET_BINDINGS} \
		--builtins=${LUCET_LIBBUILTINS} $(basename -s .c "$cfile").wasm -o $(basename -s .c "$cfile").wasm
    ${LUCETC} ${LUCETC_FLAGS} --emit=obj  \
		--bindings=${LUCET_BINDINGS} \
		--builtins=${LUCET_LIBBUILTINS} $(basename -s .c "$cfile").wasm -o $(basename -s .c "$cfile").wasm
		echo "======"
    echo ${CC} -rdynamic -shared \
		-L $(dirname ${LUCET_LIBBUILTINS}) \
		-Wl,-rpath $(dirname ${LUCET_LIBBUILTINS}) \
		$(basename -s .c "$cfile").wasm -lbuiltins -o $(basename -s .c "$cfile").wasm.so

        ${CC} -rdynamic -shared \
		-L $(dirname ${LUCET_LIBBUILTINS}) \
		-Wl,-rpath $(dirname ${LUCET_LIBBUILTINS}) \
		$(basename -s .c "$cfile").wasm -lbuiltins -o $(basename -s .c "$cfile").wasm.so
done



# Build implementation.so
mkdir -p ${SCRIPT_LOC}/bin; cd ${SCRIPT_LOC}/bin
>&2 echo ${CC} ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${LUCET_APP_ROOT}/build/lucet/target/release/lucet-wasi -shared -o implementation.so ../wrapper.c 
${CC} -fPIC ${COMMON_CFLAGS} -DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark -DVM_LOCATION=${LUCET_APP_ROOT}/build/lucet/target/release/lucet-wasi -shared -o implementation.so ../wrapper.c 

cd ${ORIG_CWD}