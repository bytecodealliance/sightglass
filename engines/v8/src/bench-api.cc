#include "bench-api.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <iostream>
#include <map>
#include <optional>
#include <utility>
#include <vector>
#include "bench-state.hh"
#include "link.hh"
#include "wasm.hh"

extern "C" {

int wasm_bench_create(wasm_bench_config_t config, void** out_ptr) {
  BenchState* st = new BenchState(Config::make(config));
  st->engine = wasm::Engine::make();
  assert(st->engine != nullptr);
  st->store = wasm::Store::make(st->engine.get());
  assert(st->store != nullptr);

  assert(out_ptr != NULL);
  *out_ptr = (void*)st;
  return 0;
}

void wasm_bench_free(void* state_) {
  BenchState* state = (BenchState*)state_;
  delete state;
}

int wasm_bench_compile(void* state_,
                       char* wasm_bytes,
                       size_t wasm_bytes_length) {
  BenchState* st = (BenchState*)state_;
  assert(st != nullptr);
  assert(st->store != nullptr);
  auto wasm = wasm::vec<byte_t>::make(wasm_bytes_length, wasm_bytes);

  st->config.compilation.start();
  st->module = wasm::Module::make(st->store.get(), wasm);
  st->config.compilation.end();

  if (!st->module) {
    std::cerr << "> Error compiling module!" << std::endl;
    return 1;
  } else {
    return 0;
  }
}

int wasm_bench_instantiate(void* state_) {
  BenchState* st = (BenchState*)state_;
  assert(st != nullptr);
  assert(st->module != nullptr);
  assert(st->store != nullptr);

  auto imports = link(st->store.get(), st->module.get(), &st->config.execution);
  auto instance =
      wasm::Instance::make(st->store.get(), st->module.get(), &imports[0]);
  if (!instance) {
    std::cerr << "> Error instantiating module!" << std::endl;
    return 1;
  } else {
    return 0;
  }
  return 0;
}

int wasm_bench_execute(void* state_) {
  BenchState* st = (BenchState*)state_;
  assert(st != nullptr);
  assert(st->module != nullptr);
  assert(st->instance != nullptr);
  //   RETURN_ON_INVALID_POINTER(state, state);
  //   RETURN_ON_INVALID_POINTER(state->module, module);
  //   RETURN_ON_INVALID_POINTER(state->instance, instance);

  //   const wasm_func_t* run_func = find_start_fn(state->module,
  //   state->instance); RETURN_ON_INVALID_POINTER(run_func, run_func); if
  //   (wasm_func_call(run_func, NULL, NULL)) {
  //     printf("> Error calling function!\n");
  //     return 1;
  //   }

  return 0;
}

// wasm_func_t* find_start_fn(const wasm_module_t* module,
//                            const wasm_instance_t* instance) {
//   wasm_exporttype_vec_t export_types;
//   wasm_extern_vec_t exports;
//   wasm_module_exports(module, &export_types);
//   wasm_instance_exports(instance, &exports);
//   assert(exports.size == export_types.size);

//   for (size_t i = 0; i < exports.size; ++i) {
//     if (wasm_extern_kind(exports.data[i]) == WASM_EXTERN_FUNC) {
//       assert(wasm_extern_kind(exports.data[i]) ==
//              wasm_externtype_kind(wasm_exporttype_type(export_types.data[i])));
//       const wasm_name_t* fn_name =
//       wasm_exporttype_name(export_types.data[i]); int len = (fn_name->size >
//       6) ? fn_name->size : 6; if (strncmp("_start", fn_name->data, len) == 0)
//       {
//         return wasm_extern_as_func(exports.data[i]);
//       }
//     }
//   }
//   return NULL;
// }

}  // extern "C"
