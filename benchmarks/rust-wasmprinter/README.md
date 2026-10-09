# rust-wasmprinter

Disassembles a WebAssembly binary into text with the [`wasmprinter`] crate
(version 0.261.0, from [wasm-tools]), which backs `wasm-tools print` and
Wasmtime's text output. It exercises `wasmparser`'s binary decoding and
`wasmprinter`'s formatting and string building. Prints the length and FNV-1a
hash of the text.

The input, `rust-wasmprinter.input.bin`, is a copy of sightglass's own
`rust-protobuf/benchmark.wasm` (sightglass commit 0145307): a 140 KB
`wasm32-wasip1` Rust module built from the `prost` crate, with a `name` section.
`rust-wasmprinter.iterations.input` sets how many times it is printed.

The input uses a `.bin` extension because tooling such as
`../check-incomplete-suite.sh` treats every `*.wasm` file as a benchmark.

The crate's default features (component-model support and validation) are
disabled to keep the benchmark's code size, and therefore its compile time,
down.

## License

`wasmprinter` is Apache-2.0 WITH LLVM-exception. The input module is built from
sightglass sources and Rust crates under MIT/Apache-2.0. This benchmark is
Apache-2.0 WITH LLVM-exception, or MIT, like the rest of sightglass.

[`wasmprinter`]: https://crates.io/crates/wasmprinter/0.261.0
[wasm-tools]: https://github.com/bytecodealliance/wasm-tools
