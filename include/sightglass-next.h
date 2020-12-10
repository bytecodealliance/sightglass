#ifndef sightglass_h
#define sightglass_h 1

/**
 * Call this function to indicate that recording should start. This call should be placed
 * immediately prior to the code to measure with sightglass-recorder. The attributes allow compilers
 * to generate the correct Wasm imports. 
 */
__attribute__((import_module("bench")))
__attribute__((import_name("start"))) void
bench_start();

/**
 * Call this function to indicate that recording should end. This call should be placed immediately
 * after the code to measure with sightglass-recorder. The attributes allow compilers to generate
 * the correct Wasm imports. 
 */
__attribute__((import_module("bench")))
__attribute__((import_name("end"))) void
bench_end();

#endif
