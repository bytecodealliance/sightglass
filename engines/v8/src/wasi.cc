#include "wasi.hh"
#include <cstdlib>
#include <iostream>

auto proc_exit(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "proc_exit" << std::endl;
  exit(args[0].i32());
  return nullptr;
}

auto fd_write(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap> {
  std::cerr << "fd_write" << std::endl;
  // TODO actually write contents; needs access to linear memory.
  results[0] = args[0].copy();
  return nullptr;
}
