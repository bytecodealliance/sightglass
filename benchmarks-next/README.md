# Candidate Benchmark Programs

This directory contains the candidate programs for the benchmark suite. They are
candidates, not officially part of the suite yet, because we [intend][rfc] to
record various metrics about the programs and then run a principal component
analysis to find a representative subset of candidates that doesn't contain
effectively duplicate workloads.

[rfc]: https://github.com/bytecodealliance/rfcs/pull/4

## Building

Build an individual benchmark program via:

```
$ cargo run -p sightglass-cli -- build-benchmark path/to/benchmark/Dockerfile
```

Build all benchmark programs by entering this directory
(`sightglass/benchmarks-next`) and running:

```
$ ./build.sh
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

* Provide reproducible builds via Docker.

* Be located at `sightglass/benchmarks-next/$BENCHMARK_NAME/benchmark.wasm`

* Input workloads must be files that lives in the same directory as the `.wasm`
  benchmark program. The benchmark program is run within the directory where it
  lives on the filesystem, with that directory propened in WASI. The workload
  must be read via a relative file path.

  If, for example, the benchmark processes JSON input, then its input workload
  should live at `sightglass/benchmarks-next/$BENCHMARK_NAME/input.json`, and it
  should open that file as `"./input.json"`.

* Define the expected `stdout` output in a `./stdout.expected` sibling file
  located next to the `benchmark.wasm` file. The runner will assert that the
  actual execution's output matches the expectation.

* Define the expected `stderr` output in a `./stderr.expected` sibling file
  located next to the `benchmark.wasm` file. The runner will assert that the
  actual execution's output matches the expectation.

## Additional Requirements

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

* A candidate program must have two associated input workloads: one small and
  one large. The small workload may be used by developers locally to get quick,
  ballpark numbers for whether further investment in an optimization is worth
  it, without waiting for the full, thorough benchmark suite to complete.

* Each workload must have an expected result, so that we can validate executions
  and avoid accepting "fast" but incorrect results.

* Compiling and instantiating the candidate program and then executing its
  workload should take *roughly* one to six seconds total.

  > Napkin math: We want the full benchmark to run in a reasonable amount of
  > time, say twenty to thirty minutes, and we want somewhere around ten to
  > twenty programs altogether in the benchmark suite to balance diversity,
  > simplicity, and time spent in execution versus compilation and
  > instantiation. Additionally, for good statistical analyses, we need *at
  > least* 30 samples (ideally more like 100) from each benchmark program. That
  > leaves an average of about one to six seconds for each benchmark program to
  > compile, instantiate, and execute the workload.

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
