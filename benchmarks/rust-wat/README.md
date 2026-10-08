# rust-wat

Assembles WebAssembly text into a binary with the [`wat`] crate (version
1.261.0, from [wasm-tools]), the text parser used by Wasmtime, `wasm-tools`, and
much of the Rust Wasm ecosystem. It exercises a lexer, a recursive-descent
parser, name resolution, and the binary encoder. Prints the size and FNV-1a hash
of the binary.

The input, `rust-wat.input.wat`, is the text form of sightglass's own
`shootout/shootout-ackermann.wasm` (built with wasi-sdk 28 from
`shootout/src/ackermann.c`, sightglass commit 550c4b7), with its DWARF sections
stripped and its `name` section kept:

```
wasm-tools strip --delete '\.debug.*' ../shootout/shootout-ackermann.wasm \
  | wasm-tools print > rust-wat.input.wat
```

That module is mostly wasi-libc. `rust-wat.iterations.input` sets how many times
the text is assembled.

The crate's `component-model` feature is disabled to keep the benchmark's code
size, and therefore its compile time, down.

## License

`wat` is Apache-2.0 WITH LLVM-exception. The input derives from code under the
Benchmarks Game's BSD license (`../shootout/src/LICENSE.md`) and from wasi-libc
(MIT and others). This benchmark is Apache-2.0 WITH LLVM-exception, or MIT, like
the rest of sightglass.

[`wat`]: https://crates.io/crates/wat/1.261.0
[wasm-tools]: https://github.com/bytecodealliance/wasm-tools
