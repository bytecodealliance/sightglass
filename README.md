<div align="center">
  <h1><code>sightglass</code></h1>

  <p>
    <strong>A benchmarking suite and tooling for Wasmtime and Cranelift</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/sightglass/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/sightglass/workflows/sightglass/badge.svg" alt="build status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <img src="https://img.shields.io/badge/rustc-stable+-green.svg" alt="supported rustc stable" />
  </p>

  <h3>
    <a href="https://github.com/bytecodealliance/sightglass/blob/main/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime">Chat</a>
  </h3>
</div>

## About

This repository contains benchmarking infrastructure for Wasmtime and Cranelift,
as described in [this
RFC](https://github.com/bytecodealliance/rfcs/blob/main/accepted/benchmark-suite.md).
In particular, it has

* a benchmark suite of Wasm applications in `benchmarks/*`, and

* a benchmark runner CLI tool to record, analyze, and display benchmark results
  in `crates/cli/*`.

We plan to implement a server that periodically runs benchmarks as new commits
are pushed to Wasmtime and display the history of those benchmark results,
similar to Firefox's [Are We Fast Yet?](https://arewefastyet.com). However, this
work is not completed yet. See [issue
93](https://github.com/bytecodealliance/sightglass/issues/93) for details.

Results are always broken down by *phase* &mdash; compilation vs. instantiation
vs. execution &mdash; for each program in the suite. This allows us to reason
about, for example, compiler performance separately from its generated code
quality. How all this works together:

- each __benchmark__ is compiled to a `benchmark.wasm` module that calls two host functions,
  `bench.start` and `bench.end`, to notify Sightglass of the portion of the execution to measure
  (see the [benchmarks README])
- we build an __engine__ (e.g., Wasmtime) as a shared library that implements the [bench API]; the
  Sightglass infrastructure uses this to measure each phase (see an [engine README])
- the __`sightglass-cli`__ tool runs __benchmarks__ using the __engines__ and emits __measurements__
  for each phase; this is configurable, e.g., by various measurement mechanisms, various output
  formats, different aggregations, etc.

[benchmarks README]: benchmarks/README.md
[engine README]: engines/wasmtime/README.md

## This is *NOT* a General-Purpose WebAssembly Benchmark Suite

This benchmark suite and tooling is specifically designed for Wasmtime and
Cranelift, as explained in [the benchmarking suite
RFC](https://github.com/bytecodealliance/rfcs/blob/main/accepted/benchmark-suite.md#nongoal-creating-a-general-purpose-webassembly-benchmark-suite):

> It is also worth mentioning this explicit non-goal: we do not intend to
> develop a general-purpose WebAssembly benchmark suite, used to compare between
> different WebAssembly compilers and runtimes. We don't intend to trigger a
> WebAssembly benchmarking war, reminiscent of JavaScript benchmarking wars in
> Web browsers. Doing so would make the benchmark suite's design high stakes,
> because engineers would be incentivized to game the benchmarks, and would
> additionally impose cross-engine portability constraints on the benchmark
> runner. We only intend to compare the performance of various versions of
> Wasmtime and Cranelift, where we don't need the cross-engine portability in
> the benchmark runner, and where gaming the benchmarks isn't incentivized.
>
> Furthermore, general-purpose WebAssembly benchmarking must include WebAssembly
> on the Web. Doing that well requires including interactions with the rest of
> the Web browser: JavaScript, rendering, and the DOM. Building and integrating
> a full Web browser is overkill for our purposes, and represents significant
> additional complexity that we would prefer to avoid.

Even if someone did manage to get other Wasm engines hooked into this
benchmarking infrastructure, comparing results across engines would likely be
invalid. The `wasmtime-bench-api` intentionally does things that will likely
hurt its absolute performance numbers but which help us more easily get
statistically meaningful results, like randomizing the locations of heap
allocations. Without taking great care to level the playing field with respect
to these sorts of tweaks, as well as keeping an eye on all engine specific
configuration options, you'll end up comparing apples and oranges.

## Usage

You can always see all subcommands and options via

```
cargo run -- help
```

There are flags to control how many different processes we spawn and take
measurements from, how many iterations we perform for each process, etc...

That said, here are a couple of typical usage scenarios.

### Building the Runtime Engine for Wasmtime
```
$ cd engines/wasmtime && rustc build.rs && ./build && cd ../../
```

### Running the Full Benchmark Suite

```
$ cargo run -- benchmark --engine engines/wasmtime/libengine.so -- benchmarks/*/benchmark.wasm
```

The output will be a summary of each benchmark program's compilation,
instantiation, and execution times.

### Running a Single Wasm Benchmark

```
$ cargo run -- benchmark --engine engines/wasmtime/libengine.so -- path/to/benchmark.wasm
```

### Comparing a Feature Branch to Main

First, build `libwasmtime_bench_api.so` (or `.dylib` or `.dll` depending on your
OS) for the latest `main` branch:

```
$ cd ~/wasmtime
$ git checkout main
$ cargo build --release -p wasmtime-bench-api
$ cp target/release/libwasmtime_bench_api.so /tmp/wasmtime_main.so
```

Then, checkout your feature branch and build its `libwasmtime_bench_api.so`:

```
$ git checkout my-feature
$ cargo build --release -p wasmtime-bench-api
```

Finally, run the benchmarks and supply both versions of
`libwasmtime_bench_api.so` via repeated use of the `--engine` flag:

```
$ cd ~/sightglass
$ cargo run -- \
    benchmark \
    --engine /tmp/wasmtime_main.so \
    --engine ~/wasmtime/target/release/libwasmtime_bench_api.so \
    -- \
    benchmarks/*/benchmark.wasm
```

The output will show a comparison between the `main` branch's results and your
feature branch's results, giving you an effect size and confidence interval
(i.e. "we are 99% confident that `my-feature` is 1.32x to 1.37x faster than
`main`" or "there is no statistically significant difference in performance
between `my-feature` and `main`") for each benchmark Wasm program in the suite.

As you make further changes to your `my-feature` branch, you can execute this
command whenever you want new, updated benchmark results:

```
$ cargo build --manifest-path ~/wasmtime/Cargo.toml --release -p wasmtime-bench-api && \
    cargo run --manifest-path ~/sightglass/Cargo.toml -- \
      benchmark \
      --engine /tmp/wasmtime_main.so \
      --engine ~/wasmtime/target/release/libwasmtime_bench_api.so \
      -- \
      benchmarks/*/benchmark.wasm
```

### Collecting Different Kinds of Results

Sightglass comes enabled with several different kinds of measurement mechanisms
&mdash; a _measure_.  The default _measure_ is `cycles`, which simply measures
the elapsed duration of CPU cycles for each phase (e.g., using `RDTSC`). The
accuracy of this _measure_ is documented [here](crates/recorder/README.md) but
note that measuring using CPU cycles alone can be problematic (e.g., CPU
frequency changes, context switches, etc.).

Several _measures_ can be configured using the `--measure` option:
- `cycles`: the number of CPU cycles elapsed
- `perf-counters`: a selection of common `perf` counters (CPU cycles, instructions retired, cache
accesses, cache misses); only available on Linux
- `vtune`: record each phase as a VTune task for analysis; see [this help
  documentation](docs/vtune.md) for more details
- `noop`: no measurement is performed

For example, run:

```
$ cargo run -- benchmark --measure perf-counters ...
```

### Getting Raw JSON or CSV Results

If you don't want the results to be summarized and displayed in a human-readable
format, you can get raw JSON or CSV via the `--raw` flag:

```
$ cargo run -- benchmark --raw --output-format csv -- benchmarks/*/benchmark.wasm
```

Then you can use your own R/Python/spreadsheets/etc. to analyze and visualize the
benchmark results.

### Adding a New Benchmark

Add a Dockerfile under `benchmarks/<your benchmark>` building a Wasm file that brackets the work to
measure with the `bench.start` and `bench.end` host calls. See the [benchmarks README] for a fuller
set of requirements and the [`build.sh`] script for building this file.

[`build.sh`]: benchmarks/build.sh
