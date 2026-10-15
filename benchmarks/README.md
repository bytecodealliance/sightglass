# Candidate Benchmark Programs

This directory contains our various benchmark programs. Not all are included in
a benchmark suite.

## Which Benchmark Programs Should I Run?

**99% of the time, you want to run the `default.suite`.**

The default suite is derived from a [principal-component analysis
(PCA)](https://github.com/bytecodealliance/sightglass/blob/main/scripts/pca.R)
to maximize the amount of benchmark variation covered while avoiding
effectively-redundant workloads (those that exercise the same kinds of code that
an existing benchmark already exercises) and conserving developer
time. `default.suite` is an alias of `pca-20-min.suite`, and completes a full
run with default options in an estimated ~20 minutes.

For fast iteration locally, but not as a replacement for a full `default.suite`
run, you can also use one of the smaller PCA-based subsets of `default.suite`:

* `pca-5-min.suite`: completes in an estimated ~5 minutes
* `pca-10-min.suite`: completes in an estimated ~10 minutes

If you are specifically interested in a certain kind of program, you can also
try one of these suites:

* `gc.suite`: the set of benchmarks that use Wasm GC
* `simd.suite`: the set of benchmarks that use SIMD

Finally, while you *can* also run `all.suite` -- which contains every benchmark
program in this directory -- be aware that it contains many
effectively-redundant workloads (e.g. many of the libsodium benchmarks exercise
basically the same kind of code) and therefore its geomean is lopsided, weighing
these redundant workloads heavier than workloads that appear only once in the
corpus, and running it takes an exceedingly long time. We do not recommend
benchmarking `all.suite`.

## Building

Build an individual benchmark program via:

```
$ ./build.sh path/to/benchmark/dir/
```

Build all benchmark programs by running:

```
$ ./build-all.sh
```

## Minimal Technical Requirements

In order for the benchmark runner to successfully execute a Wasm program and
record its execution, it must:

* Export a `_start` function of type `[] -> []`.

* Import `bench.start` and `bench.end` functions, both of type `[] -> []`.

* Call `bench.start` exactly once during the execution of its `_start`
  function. This is when the benchmark runner will start recording execution
  time and performance counters.

* Call `bench.end` exactly once during execution of its `_start` function, after
  `bench.start` has already been called. This is when the benchmark runner will
  stop recording execution time and performance counters.

* Provide reproducible builds via Docker (see [`build.sh`](./build.sh)).

* Be located in a `sightglass/benchmarks/$BENCHMARK_NAME` directory. Typically
  the benchmark is named `benchmark.wasm`, but benchmarks with multiple files
  should use names like `<benchmark name>-<subtest name>.wasm` (e.g.,
  `libsodium-chacha20.wasm`).

* Input workloads must be files that live in the same directory as the `.wasm`
  benchmark program. The benchmark program is run within the directory where it
  lives on the filesystem, with that directory pre-opened in WASI. The workload
  must be read via a relative file path.

  If, for example, the benchmark processes JSON input, then its input workload
  should live at `sightglass/benchmarks/$BENCHMARK_NAME/input.json`, and it
  should open that file as `"./input.json"`.

* Define the expected `stdout` output in a `./<benchmark name>.stdout.expected`
  sibling file located next to the `benchmark.wasm` file (e.g.,
  `benchmark.stdout.expected`). The runner will assert that the actual
  execution's output matches the expectation.

* Define the expected `stderr` output in a `./<benchmark name>.stderr.expected`
  sibling file located next to the `benchmark.wasm` file. The runner will assert
  that the actual execution's output matches the expectation.

* The benchmark should dynamically execute ~100,000,000 wasm instructions per
  execution. You can check this via `cargo run -- pca-metrics path/to/benchmark`
  and looking at the `dynamic_total_inst_count` column of the resulting CSV
  output.

  Anything within 80,000,000-120,000,000 counts as on target. This matches
  `TARGET_INST_COUNT` in [`scripts/pca.R`](../scripts/pca.R), which also drops
  any benchmark measuring below `MIN_DYNAMIC_INST_COUNT` (half the target)
  outright. Benchmarks that are far over the target dominate the wall-clock time
  of a suite run; benchmarks under the floor disappear from the PCA entirely.

* The knob that sets the size of the workload should be read from a sibling
  input file at run time rather than hard-coded as a constant in the source, so
  that the benchmark can be retuned without a Docker rebuild. Read it *before*
  `bench_start()` so that the I/O is not measured, and keep a compiled-in
  fallback for when the file is absent.

  C and C++ benchmarks can use `bench_read_long()` from
  [`include/sightglass.h`](../include/sightglass.h):

  ```c
  /* Fallback tuned so that this benchmark executes ~100M Wasm instructions. */
  #define ITERATIONS 293

  int iterations = (int) bench_read_long("./shootout-base64.iterations.input",
                                         ITERATIONS);
  bench_start();
  for (int i = 0; i < iterations; i++) { ... }
  bench_end();
  ```

  Name the file after the benchmark and the knob (e.g.
  `shootout-base64.iterations.input`) when a directory holds several benchmarks,
  or just `default.input` when it holds one. Keep the compiled-in fallback in
  sync with the checked-in input file.

  Two cautions when adding a knob:

  * If the benchmark prints anything derived from the knob, its
    `.stdout.expected` must be regenerated.

  * Check that the compiler has not folded the workload away. Scaling a loop
    down can let LLVM collapse it into a constant; `shootout-nestedloop`
    executed *zero* instructions for exactly this reason. Verify the count
    responds to the knob, and use the `BLACK_BOX()` macro from
    `include/sightglass.h` (or a `volatile` local) to keep the work opaque.

Many of the above requirements can be checked by running the `.wasm` file
through the `validate` command:

```
$ cargo run -- validate path/to/benchmark.wasm
```

## Additional Desiderata

> Note: these requirements are lifted directly from the [the benchmarking
> RFC][rfc].

In addition to the minimal technical requirements, for a benchmark program to be
useful to Wasmtime and Cranelift developers, it should additionally meet the
following requirements:

* Candidates should be real, widely used programs, or at least extracted kernels
  of such programs. These programs are ideally taken from domains where Wasmtime
  and Cranelift are currently used, or domains where they are intended to be a
  good fit (e.g. serverless compute, game plugins, client Web applications,
  server Web applications, audio plugins, etc.).

* A candidate program must be deterministic (modulo Wasm nondeterminism like
  `memory.grow` failure).

* Inputs should be given through I/O and results reported through I/O. This
  ensures that the compiler cannot optimize the benchmark program away.

* Candidate programs should only import WASI functions. They should not depend
  on any other non-standard imports, hooks, or runtime environment.

* Candidate programs must be open source under a license that allows
  redistributing, modifying and redistributing modified versions. This makes
  distributing the benchmark easy, allows us to rebuild Wasm binaries as new
  versions are released, and lets us do source-level analysis of benchmark
  programs when necessary.

* Repeated executions of a candidate program must yield independent samples
  (ignoring priming Wasmtime's code cache). If the execution times keep taking
  longer and longer, or exhibit harmonics, they are not independent and this can
  invalidate any statistical analyses of the results we perform. We can easily
  check for this property with either [the chi-squared
  test](https://en.wikipedia.org/wiki/Chi-squared_test) or [Fisher's exact
  test](https://en.wikipedia.org/wiki/Fisher%27s_exact_test).

* The corpus of candidates should include programs that use a variety of
  languages, compilers, and toolchains.

## Benchmarks that cannot hit the ~100M instructions target

A few benchmarks are deliberately out of band because the region they measure is
a single indivisible operation whose smallest legal size still costs far more
than ~100M instructions. Each is documented in its own `README.md`:

| benchmark | instructions | why it cannot be reduced |
| --- | --- | --- |
| [`tract-onnx-image-classification`](./tract-onnx-image-classification/README.md) | ~6.6G | One MobileNetV2 forward pass; the model's input shape is fixed at 224x224. |
| [`sqlite3`](./sqlite3/README.md) | ~459M | speedtest1's `szTest` is already at its minimum of 1. |
| [`spidermonkey-markdown`](./spidermonkey/README.md) | ~289M | `marked`'s lazy inline-lexer regex compilation costs ~248M inside the measured region no matter how small the input is; `spidermonkey` is also in `build-all.sh`'s skip list, so only its input file can change. |
| [`blind-sig`](./blind-sig/README.md) | ~245M | One RSA blind signature; the crate rejects moduli below 2048 bits. |

Most of the `libsodium` subtests are also out of band, in both directions; see
[`libsodium/README.md`](./libsodium/README.md). They are all built from one
upstream test suite with a single per-test iteration count as the only knob, so
a test whose body already costs more than 120M cannot be scaled down, and a few
whose bodies are empty on Wasm cannot be scaled up.

`noop` executes 0 instructions by design and is intended for measuring harness
overhead.
