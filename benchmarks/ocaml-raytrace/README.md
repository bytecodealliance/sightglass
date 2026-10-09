# ocaml-raytrace

An OCaml port of Octane's `raytrace` benchmark, compiled to a WasmGC component
with wasm_of_ocaml's WASI target. It renders a small scene (two spheres and a
checkerboard plane, two lights) with diffuse lighting, shadows, Phong
highlights, and reflections, repeated `default.input` times (24 by default).

It exercises float-heavy code, many short-lived small allocations (vectors,
colors, intersection records), pattern-match dispatch, and recursion. The
OCaml runtime it runs on relies on the WasmGC, exception-handling (`exnref`),
and tail-call proposals.

## Origin

* Source: `Octane/raytrace.js` from JetStream
  (<https://github.com/WebKit/JetStream>, commit
  `c603c04db8505477867974a69789309ded2cc948`). The ray tracer was written by
  Adam Burmister and adapted by Google for Octane.

`raytrace.ml` follows the JS structure closely. Its JS subclasses (the
`Material` and `Shape` hierarchies) become OCaml records with a variant field,
and virtual calls become matches. Each render checks the JS's `checkNumber`
(2321). The output also includes a checksum over every pixel's brightness.
Both values match the JS run under Node.

## Build notes

* `wasm_of_ocaml compile --enable wasi` emits a core module that imports only
  `wasi_snapshot_preview1` and the `bench` hooks. `bench.wat` turns those hooks
  into OCaml primitives. The Dockerfile then wraps the module as a component
  using the preview1 command adapter.
* The program reads input through `open_in`, not `In_channel`/`Printf`, which
  roughly halves the code size.
* Compile time has a floor of about 47 ms (a hello-world from the same
  toolchain), which keeps the compile/exec ratio at about 2.5. `wasm_of_ocaml`'s
  WASI runtime has a large constant global initializer (its errno message table,
  77 strings in one constant expression), and Wasmtime compiles all global
  initializers into a single module-init function. That global alone takes about
  37 ms to compile; the ray tracer adds little.

## License

`raytrace.js` is distributed with Octane under the BSD 3-clause license (see
`Octane/LICENSE.txt` in JetStream). This port is available under the same
licenses as the rest of sightglass (Apache-2.0 WITH LLVM-exception, or MIT).
The compiled binary includes the OCaml standard library and the wasm_of_ocaml
runtime, both LGPL-2.1 with the OCaml linking exception.
