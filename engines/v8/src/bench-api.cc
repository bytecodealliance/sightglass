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
  st->engine = wasm::Engine::make();  // TODO cannot instantiate an EngineImpl
                                      // multiple times.
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

  // Retrieve linked functions and extract their pointers here to avoid dropping
  // them too early.
  auto imports = link(st->store.get(), st->module.get(), &st->config.execution);
  const wasm::Extern* imports_as_extern[imports.size()];
  for (size_t i = 0; i < imports.size(); ++i) {
    imports_as_extern[i] = imports[i].get();
  }

  st->config.instantiation.start();
  st->instance = wasm::Instance::make(st->store.get(), st->module.get(),
                                      imports_as_extern);
  st->config.instantiation.end();

  if (!st->instance) {
    std::cerr << "> Error instantiating module!" << std::endl;
    return 1;
  } else {
    return 0;
  }
}

int wasm_bench_execute(void* state_) {
  BenchState* st = (BenchState*)state_;
  assert(st != nullptr);
  assert(st->module != nullptr);
  assert(st->instance != nullptr);

  // Find the _start function.
  auto start_fn = find_start_fn(st->module.get(), st->instance.get());
  if (!start_fn) {
    std::cerr << "> Unable to find the '_start' function!" << std::endl;
    return 1;
  }

  // Run the start function.
  if (!start_fn->func()->call()) {
    std::cerr << "> Error calling start function!" << std::endl;
    return 1;
  }

  return 0;
}

}  // extern "C"
