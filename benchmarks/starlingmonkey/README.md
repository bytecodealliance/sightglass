# StarlingMonkey

The three `spidermonkey` JavaScript workloads (`json`, `markdown`, `regex`), run on
[StarlingMonkey](https://github.com/bytecodealliance/StarlingMonkey) v0.3.0 (commit
`9dda8ba7fcda2e17c6795d402f0478cf4c1f7f37`), the SpiderMonkey-based JS runtime for Wasm
components. `js/` is shared with `../spidermonkey/`, and stdout matches those benchmarks'
output on the same input.

Each `.wasm` is a component pre-initialized with Wizer: building it boots the engine and
evaluates the JS, ending with `registerBenchmark(inputPath, main)`. The resulting heap is
snapshotted. At run time, the `wasi:cli/run` export in `bench.cpp` reads the `.input`
file, then calls `main(input)` between `bench.start` and `bench.end`, then prints the
result. The top-level script is never parsed or evaluated at run time.

## Build notes

- `host-api/` is a custom StarlingMonkey host API: WASI 0.2.3 `wasi:cli/command` plus the
  `bench` import, without `wasi:http`, which sightglass does not provide. To match, the
  `fetch` builtins and the JS debugger are disabled.
- `bench.cpp` is added as a StarlingMonkey builtin through its `add_builtin` CMake API, and
  the components are produced by StarlingMonkey's own `componentize.sh` (`wasmtime wizer`
  plus `wasm-tools component new`).
- SpiderMonkey comes from StarlingMonkey's prebuilt release
  (`libspidermonkey_FIREFOX_147_0_4_RELEASE_STARLING`), checked by SHA-256.
- Changing anything under `js/` or `warmup/` requires a rebuild, since that code is in the
  snapshot. Changing a `.input` file does not.

## Instruction counts

All three are tuned to ~100M instructions with their `.input` files. `json` and `regex`
use the same inputs as `spidermonkey`.

`marked` compiles its inline-lexer regexes lazily on first use. That one-time cost (~215M
instructions for a 3-byte input) is the reason `spidermonkey-markdown` cannot reach
~100M. Here, `warmup/markdown.js` calls `main` once on a small document during
pre-initialization, so the compiled regexes land in the snapshot. That brings a 3-byte
input down to ~2M instructions. `starlingmonkey-markdown.input` is the first 5,178 bytes
of the CommonMark spec preamble (a longer prefix of the text `spidermonkey-markdown` uses).

## License

StarlingMonkey is Apache-2.0 WITH LLVM-exception. `marked` is MIT. The glue in this
directory is under sightglass's licenses (Apache-2.0 WITH LLVM-exception, or MIT).
