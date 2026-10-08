# java-navier-stokes

A Java port of Octane's `navier-stokes` (Oliver Hunt's 2D fluid solver),
compiled to Wasm GC with [TeaVM](https://teavm.org/) and wrapped as a WASI
component.

The hot loops (the Gauss-Seidel relaxation in `linSolve`/`linSolve2`,
`advect`, `project`, `setBnd`) read and write `double[]` fields of the
solver object, which TeaVM lowers to `struct.get` plus `array.get`/`array.set`
on `(array (mut f64))`. The simulation does not allocate after setup, so this
benchmark exercises GC object accesses rather than allocation or collection.

`default.input` holds the number of simulation frames (128x128 grid, 20
solver iterations, as in Octane). The program prints a hash of the bits of the
final density field. It matches the original JavaScript bit for bit, and for 15
or more frames the program also runs Octane's own checksum.

## Origin

- `navier-stokes.js` from JetStream
  (<https://github.com/WebKit/JetStream>, commit
  `c603c04db8505477867974a69789309ded2cc948`, `Octane/navier-stokes.js`).
- `FluidField.java` is a direct translation. The JS allocates a `Field` wrapper
  per frame for the UI callback; the port allocates one per reset instead, and
  drops the unused display callback.

## Build

The `Dockerfile` drives TeaVM's Wasm GC backend through `tools/TeaVMBuild.java`
with `teavm-jso-impl` left off the classpath, so the module needs no JS string
builtins or JS exception interop. `Main.java` calls WASI preview1 directly
through `@Import`, using linear memory only for WASI argument buffers.

TeaVM's Wasm GC output still imports a few things from its JS loader:
`env.memory`, `teavmMemory.{heapOffset,maxSize,notifyHeapResized}`,
`teavmMath.sqrt`, and `teavmDate.currentTimeMillis`. The build redirects them
to `shim.wat` (which defines the memory and implements `sqrt` as `f64.sqrt`),
merges that in with `wasm-merge`, drops TeaVM's JS-facing exports, and runs
`wasm-opt -O3`. `wasm-tools component new` with the preview1 command adapter
then produces the component.

The module uses the Wasm GC and function-references proposals; it contains no
exception-handling instructions.

## License

The original is copyright 2013 the V8 project authors and 2009 Oliver Hunt,
under the MIT-style license in the header of `FluidField.java`. The port is
available under sightglass's licenses (Apache-2.0 WITH LLVM-exception, or MIT).
