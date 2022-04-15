#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "bench-api.h"

int read_file(char* path, char** out_str);
wasm_bench_config_t empty_config();
#define RETURN_ON_INVALID_POINTER(pointer, name) \
  if (!pointer) {                                \
    printf("Invalid pointer: " #name "\n");      \
    return 1;                                    \
  }
#define EXIT_ON_INVALID_POINTER(pointer, name) \
  if (!pointer) {                              \
    printf("Invalid pointer: " #name "\n");    \
    exit(1);                                   \
  }

int main(int argc, char* argv[]) {
  void* state = NULL;
  int create_result = wasm_bench_create(empty_config(), (void**)&state);
  assert(create_result == 0);
  EXIT_ON_INVALID_POINTER(state, state);
  printf("> created engine state: %p\n", state);

  // Get first argument; expects a path to a .wasm file.
  assert(argc > 1);
  char* wasm_path = argv[1];

  // Compile.
  char* wasm_bytes = NULL;
  int wasm_bytes_len = read_file(wasm_path, &wasm_bytes);
  EXIT_ON_INVALID_POINTER(wasm_bytes, wasm_bytes);
  int compile_result = wasm_bench_compile(state, wasm_bytes, wasm_bytes_len);
  assert(compile_result == 0);
  printf("> compiled %d bytes\n", wasm_bytes_len);

  // Instantiate.
  int instantiate_result = wasm_bench_instantiate(state);
  assert(instantiate_result == 0);
  printf("> instantiated module\n");

  // Execute
  int execution_result = wasm_bench_execute(state);
  assert(execution_result == 0);
  printf("> executed module\n");

  return 0;
}

wasm_bench_config_t empty_config() {
  wasm_bench_config_t config = {NULL, 0,    NULL, 0,    NULL, 0,
                                NULL, 0,    NULL, NULL, NULL, NULL,
                                NULL, NULL, NULL, NULL, NULL};
  return config;
}

int read_file(char* filename, char** out_str) {
  char* buffer = NULL;
  long length;
  FILE* file = fopen(filename, "rb");
  if (file) {
    fseek(file, 0, SEEK_END);
    length = ftell(file);
    fseek(file, 0, SEEK_SET);
    buffer = malloc(length);
    if (buffer) {
      fread(buffer, 1, length, file);
    }
    fclose(file);
    *out_str = buffer;
    return length;
  } else {
    return 0;
  }
}
