#include "wasm.hh"

auto proc_exit(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
auto fd_write(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
