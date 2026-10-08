# dotnet-raytracer

The .NET workload from JetStream 3: a ray tracer rendering one frame of the
`TwoPlanes` scene (`Vector128<float>` math, virtual dispatch over scene
objects, recursive reflection/refraction), preceded by JetStream's exception
throw/catch micro-tasks (plain `catch`, `catch ... when` filters, rethrow out
of a non-matching filter, `try`/`finally`). It is compiled ahead of time from
C# with NativeAOT-LLVM into a WASIp2 component, via
[componentize-dotnet](https://github.com/bytecodealliance/componentize-dotnet).

This is the suite's C#-exceptions benchmark: about 40% of the measured
instructions are in throw/catch work and the rest is ray tracing.

## Origin

* Source: <https://github.com/WebKit/JetStream/tree/c603c04db8505477867974a69789309ded2cc948/wasm/dotnet/src/dotnet>
  (`RayTracer/` and `Benchmarks/{BenchTask,Exceptions}.cs`).

Changes from upstream:

* `RunIteration` and the JS interop (`MainJS.cs`) are replaced by
  `src/Program.cs`, which runs the same sequence synchronously: every
  exception measurement for `batch` batches, then one `width`x`height` render.
* `Camera.RenderScene` renders its stripes sequentially instead of with
  `Task`s, since WASI has no threads (JetStream also uses one stripe).
* `BenchTask.cs` keeps only the synchronous batch loop. The catch blocks in
  `Exceptions.cs` count the exceptions they catch, so the output depends on
  them.
* The JSON and String micro-tasks are left out.

`default.input` holds `batch width height`. The output is the count of caught
exceptions plus an FNV-1a hash of the rendered RGBA buffer.

## Build notes

* `WasmEnableExceptionHandling` selects NativeAOT-LLVM's Wasm EH model (the
  WASI default is "emulated": a global exception flag checked after every
  call). A custom target also passes `-mllvm -wasm-use-legacy-eh=false` to
  clang so the output uses the standard exnref instructions (`try_table`,
  `throw`, `throw_ref`) rather than legacy `try`/`catch`/`delegate`/`rethrow`.
  The .NET runtime still does two-pass dispatch (filters run before unwinding)
  in managed code over a shadow stack of virtual unwind frames. Wasm EH only
  carries the native unwind between frames.
* The Wasm needs the exceptions proposal. It does not use Wasm GC or SIMD:
  .NET's collector runs in linear memory, and NativeAOT-LLVM lowers
  `Vector128<T>` to scalar code.
* Even an empty NativeAOT program is about 540 KB of code (GC, type system,
  CoreLib startup), so Wasmtime compile time is several times execution time.
* componentize-dotnet always downloads the x86_64 wasi-sdk. The Dockerfile
  pre-installs the host's wasi-sdk at the path it checks, so the build also
  runs natively on aarch64.

## License

`BenchTask.cs` and `Exceptions.cs` are MIT-licensed by the .NET Foundation
(see their headers). The `RayTracer/` sources carry no header and are
distributed as part of JetStream under its BSD 2-clause license. This port is made
available under the same licenses as the rest of sightglass (Apache-2.0 WITH
LLVM-exception, or MIT).
