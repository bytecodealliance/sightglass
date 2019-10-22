#include <sightglass.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define STR1(x) #x
#define STR(x) STR1(x)

#ifdef WORKLOAD_LOCATION
#define WASM_FILE_PREFIX STR(WORKLOAD_LOCATION)
#else
#error "WORKLOAD_LOCATION not defined"
#endif

#define MAX_WASM_FILE_NAME_SIZE 200

extern TestsConfig tests_config;

TestsConfig tests_config = { .global_setup    = NULL,
                             .global_teardown = NULL,
                             .version         = TEST_ABI_VERSION };


static void body_wrapper(const char *name, void *ctx)
{
    char wasm_file[MAX_WASM_FILE_NAME_SIZE];

    snprintf(wasm_file, sizeof(wasm_file), "%s/%s", WASM_FILE_PREFIX, name);
    system(wasm_file);
}

#define BODY(NAME) \
    void NAME##_body(void *ctx) { body_wrapper(#NAME "", ctx); }

BODY(a2mm)
BODY(a3mm)
BODY(adi)
BODY(atax)
BODY(bicg)
BODY(cholesky)  
BODY(correlation)  
BODY(covariance)  
BODY(deriche)  
BODY(doitgen)  
BODY(durbin)  
BODY(fdtd_2d)  
BODY(floyd_warshall)  
BODY(gemm)  
BODY(gemver)  
BODY(gesummv)  
BODY(gramschmidt)  
BODY(heat_3d)  
BODY(jacobi_1d)  
BODY(jacobi_2d)  
BODY(lu)  
BODY(ludcmp) 
BODY(mvt)  
BODY(nussinov)   
BODY(seidel_2d)   
BODY(symm)   
BODY(syr2k)  
BODY(syrk) 
BODY(trisolv)
BODY(trmm)

