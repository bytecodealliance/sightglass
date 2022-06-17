Sightglass (next)
=================

The `sightglass` next test runner measures benchmarks that are compiled into Wasm files and makes
use of a separate shared library to control the Wasm engine. This project, an iteration in the
spirit of the original sightglass, contains separate crates for running Wasm benchmarks:
 - [`sightglass-cli`](crates/cli) contains the CLI for creating and running benchmarks
 - [`sightglass-build`](crates/build) provides tools for building the Wasm benchmark artifacts,
   e.g., from a `Dockerfile`
 - [`sightglass-recorder`](crates/recorder) is a measurement tool for running a Wasm benchmark, see
   _Conventions_ below
 - TODO Eventually, other crates may be included: `sightglass-analysis`,
   `sightglass-build-server`, `sightglass-result-server`

### Conventions

The artifacts and measurement rely on several conventions:
 - the Wasm benchmark artifacts must have a start function, import `bench.start` and `bench.end`,
   and call these functions before and after the code section to measure (i.e. exclude the code
   doing setup and teardown)
 - the Wasm engines must currently implement the following C API in a shared library:
   - `void* engine_create(char *wasm_bytes, size_t wasm_bytes_len)`
   - `int engine_compile_module(void *engine)`
   - `int engine_instantiate_module(void *engine, void (*bench_start)(), void (*bench_end)())`
   - `int engine_execute_module(void *engine)`
   - `void engine_free(void *engine)`
 - the measurement tool loads the shared library and controls the engine, measuring each phase
   separately (i.e. compilation, instantiation, execution); note that the execution measurement is
   initiated by the benchmark itself, by calling the imported `bench.start` and `bench.end`
   functions imported by `engine_instantiate_module`.
- TODO the recorder measurements data format

> TODO: how will we record benchmarks that do not fit the engine/Wasm file pattern? E.g., lucet
> likely wants an `.so` as an artifact and if we want to compare to native code there is no engine,
> just a binary.

### Build and Test

```
RUST_LOG=trace cargo +nightly test --all
```

Note that this requires the `docker` executable to be present on the system path. This defaults to
`docker`; to specify a specific binary or alternate application (e.g. `podman`), run the command
above with `DOCKER=path/to/alternate/binary`.

### Use

Build a Dockerfile into a Wasm benchmark:

```
benchmarks/build.sh path/to/benchmark/directory/
```

The [benchmarks/build-all.sh](../benchmarks/build-all.sh) script iterates over the
Dockerfiles in `benchmarks` to build all of the included benchmarks.

Then, measure how long an engine takes to compile, instantiate, and execute a Wasm benchmark:

```
cargo run -- benchmark path/to/generated/benchmark.wasm --engine path/to/engine.so
```

The [benchmarks/run-all.sh](../benchmarks/run-all.sh) script runs all of the benchmarks in
the `benchmarks` directory.
