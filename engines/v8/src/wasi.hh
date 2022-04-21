#include "wasm.hh"

auto proc_exit(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
auto fd_write(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
auto fd_seek(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
auto fd_fdstat_get(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
auto fd_close(const wasm::Val args[], wasm::Val results[])
    -> wasm::own<wasm::Trap>;
