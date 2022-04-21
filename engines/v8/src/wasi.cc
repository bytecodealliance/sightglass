#include "wasi.hh"
#include <cstdlib>
#include <iostream>

// WebAssembly type: (func (param i32))
auto proc_exit(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "proc_exit" << std::endl;
  // exit(args[0].i32()); // TODO this cannot actually exit here; should trap?
  return nullptr;
}

// WebAssembly type: (func (param i32 i32 i32 i32) (result i32))
auto fd_write(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "fd_write" << std::endl;
  // TODO actually write contents; needs access to linear memory.
  // results[0] = args[0].copy();
  results[0] = wasm::Val::i32(65553);
  return nullptr;
}

// WebAssembly type: (func (param i32 i64 i32 i32) (result i32))
auto fd_seek(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "fd_seek" << std::endl;
  // TODO actually seek contents; needs access to linear memory.
  results[0] = wasm::Val::i32(0);
  return nullptr;
}

// WebAssembly type: (func (param i32 i32) (result i32))
auto fd_fdstat_get(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "fd_fdstat_get" << std::endl;
  // TODO
  results[0] = wasm::Val::i32(0);
  return nullptr;
}

// WebAssembly type: (func (param i32) (result i32))
auto fd_close(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "fd_close" << std::endl;
  // TODO
  results[0] = wasm::Val::i32(0);
  return nullptr;
}
