# dart-deltablue

A Dart port of Octane's DeltaBlue, an incremental constraint-hierarchy solver
(Freeman-Benson and Maloney, CACM 1990), compiled with dart2wasm to a WasmGC
component.

Each iteration runs the JS benchmark's `deltaBlue()`: a 100-variable chain of
equality constraints and a 100-pair projection through scale/offset
constraints, each repeatedly re-planned and executed. It exercises small-object
allocation, virtual dispatch over a constraint class hierarchy, growable lists,
and pointer chasing, plus the WasmGC and exception-handling proposals.

`default.input` holds the iteration count. The output reports the sum of the
planners' final marks, and "verified" if no solver check failed.

## Origin

`src/deltablue.dart` is a class-for-class translation of `Octane/deltablue.js`
from JetStream (<https://github.com/WebKit/JetStream>, commit
`c603c04db8505477867974a69789309ded2cc948`), keeping its quirks (e.g.
`Strength.nextWeaker`, double-valued variables). The Dart team's DeltaBlue
(`pkg/front_end/testcases/general/DeltaBlue.dart` in
<https://github.com/dart-lang/sdk>, GPL-2.0-or-later) was consulted for Dart
idioms only.

## Build

* Dart SDK 3.13.5 `dart compile wasm --standalone -O4` targets WasmGC without
  JavaScript. `-E --enable-experimental-ffi` allows `@Native` functions, which
  become core-module imports: `main.dart` uses this to import
  `wasi_snapshot_preview1` (to read `default.input`) and `bench`.
* Standalone mode still imports a few `dart.*` embedder functions (strings,
  `print`, stack traces). `src/embedder.wat` implements them in Wasm, with
  `print` writing to stdout via `fd_write`. Binaryen's `wasm-merge` links it in.
  The build fails if the Dart code starts using an import it does not provide.
* `wasm-opt --translate-to-exnref` turns dart2wasm's legacy exception handling
  into the standard exnref form that Wasmtime implements.
* `wasm-tools component new` with the WASI preview1 command adapter produces
  the component.

## License

`deltablue.js` is copyright 2008 the V8 project authors and 1996 John Maloney
and Mario Wolczko, under GPL-2.0-or-later. This port is a derivative work and
is distributed under the same license. `main.dart`, `embedder.wat`, and the
build files are available under sightglass's licenses (Apache-2.0 WITH
LLVM-exception, or MIT).
