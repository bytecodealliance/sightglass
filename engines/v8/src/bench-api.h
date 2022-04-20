// Describe the C API that this engine must conform to for use within
// Sightglass.
#ifndef __BENCH_API_H
#define __BENCH_API_H

#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef char* wasm_bench_timer_t;
typedef void (*wasm_bench_callback_t)(wasm_bench_timer_t);

/**
 * @brief This struct must match what is passed by sightglass (TODO cite).
 */
typedef struct wasm_bench_config_t {
  char* working_dir_ptr;
  size_t working_dir_len;
  char* stdout_path_ptr;
  size_t stdout_path_len;
  char* stderr_path_ptr;
  size_t stderr_path_len;
  char* stdin_path_ptr;
  size_t stdin_path_len;
  wasm_bench_timer_t compilation_timer;
  wasm_bench_callback_t compilation_start;
  wasm_bench_callback_t compilation_end;
  wasm_bench_timer_t instantiation_timer;
  wasm_bench_callback_t instantiation_start;
  wasm_bench_callback_t instantiation_end;
  wasm_bench_timer_t execution_timer;
  wasm_bench_callback_t execution_start;
  wasm_bench_callback_t execution_end;
} wasm_bench_config_t;

/// API functions (TODO cite).

int wasm_bench_create(wasm_bench_config_t config, void** state_out_ptr);
void wasm_bench_free(void* state);
int wasm_bench_compile(void* state, char* wasm_bytes, size_t wasm_bytes_length);
int wasm_bench_instantiate(void* state);
int wasm_bench_execute(void* state);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // #ifdef __BENCH_API_H
