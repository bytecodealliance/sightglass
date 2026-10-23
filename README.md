<div align="center">
  <h1><code>sightglass</code></h1>

  <p>
    <strong>Benchmarking for Wasmtime and Cranelift</strong>
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

This repository contains benchmarking infrastructure for Wasmtime and
Cranelift. In particular, it has

* a suite of Wasm benchmark programs in `benchmarks/*`, and

* a CLI tool to record, analyze, compare, and display benchmark results in
  `crates/cli/*`.

Continuous benchmarking, showing historical performance over time and filing
issues for performance regressions, is built on top of this infrastructure in
the [`bytecodealliance/are-we-fast-yet`] repository.

[`bytecodealliance/are-we-fast-yet`]: https://github.com/bytecodealliance/are-we-fast-yet

Analyses use sound and rigorous statistical methods: Mann-Whitney U-tests for
significance (because benchmark results are not normally distributed), reporting
effect sizes only for statistically significant results, etc...

Results are always broken down by *phase* &mdash; compilation vs. instantiation
vs. execution &mdash; for each program in the suite. This allows us to reason
about, for example, compiler performance separately from its generated code
quality.

More details:

* [`benchmarks/README.md`]: Wasm benchmark programs and adding new ones.
* [`engines/wasmtime/README.md`]: Building Wasmtime for benchmarking with Sightglass.

[`benchmarks/README.md`]: ./benchmarks/README.md
[`engines/wasmtime/README.md`]: ./engines/wasmtime/README.md

## Quickstart

Compare the performance between a baseline commit and a feature commit:

```bash
rustc engines/wasmtime/build.rs

# Build the baseline engine.
mkdir baseline
REPOSITORY=/path/to/my/wasmtime REVISION=my-baseline-branch-or-sha ./build baseline/

# Build the feature engine.
mkdir feature
REPOSITORY=/path/to/my/wasmtime REVISION=my-feature-branch-or-sha ./build feature/

# Run the benchmarks in both engines and compare their performance!
cargo run -- benchmark \
    -e baseline/libengine.so \     # or `.dylib` on macos; `.dll` on windows
    -e feature/libengine.so
```

## Usage

You can always see all usage options via:

```
cargo run -- --help
```

### Running the Default Benchmark Suite

```
cargo run -- benchmark --engine engines/wasmtime/libengine.so
```

This runs all benchmarks listed in [`default.suite`](benchmarks/default.suite).

When one engine is given, the output will be a summary of each benchmark's
compilation, instantiation, and execution times. When multiple engines are
given, each pair of engines' results are compared and tested for statistical
significance.

### Running Particular Wasm Benchmarks

```
cargo run -- benchmark --engine engines/wasmtime/libengine.so -- path/to/benchmark.wasm
```

Append multiple `*.wasm` paths to the end of that command to run multiple
benchmarks.

See also "Which Benchmark Programs Should I Run?" in [`benchmarks/README.md`].

### Running Alternative Benchmark Suites

```
$ cargo run -- benchmark --engine engines/wasmtime/libengine.so -- path/to/the.suite
```

`*.suite` files contain relative paths of a list of benchmarks to run. This is a
convenience for organizing benchmarks but is functionally equivalent to listing
all `*.wasm` paths at the end of the `benchmark` command. There are a handful of
alternative suites defined in `./benchmarks/*.suite` files.

See also "Which Benchmark Programs Should I Run?" in [`benchmarks/README.md`].

### Comparing a Feature Branch to Main

Build one `libengine.so` for `main` and another for your `my-feature` branch and
pass both to `cargo run -- benchmark`:

```
rustc engines/wasmtime/build.rs

# Build the engine for `main`.
mkdir main
REPOSITORY=/path/to/my/wasmtime REVISION=main ./build main/

# Build the engine for the `my-feature` branch.
mkdir my-feature
REPOSITORY=/path/to/my/wasmtime REVISION=my-feature ./build my-feature/

# Pass all the engines to `benchmark` to compare their performance.
cargo run -- benchmark \
    --engine main/libengine.so \       # or `.dylib` on macos; `.dll` on windows
    --engine my-feature/libengine.so
```

You may pass in a third or fourth or etc... `--engine` and each pair of engines
will be compared.

### Collecting Different Measurements

You can change what measures are recorded via the `-m`/`--measure` CLI flag:

```
cargo run -- benchmark -e path/to/libengine.so -m insts-retired
```

Available measures:

* `cycles`: CPU cycles elapsed. This is the default.

* `insts-retired`: Instructions retired.

* `perf-counters`: A selection of common `perf` counters (CPU cycles,
  instructions retired, cache accesses, cache misses). Only available on Linux.

* `callgrind`: Uses Valgrind's Callgrind to count instructions retired and
  simulate caches and branch prediction. Mostly deterministic and very low
  noise. Only available on Linux and when built with `--features callgrind`.

* `vtune`: Record each phase as a VTune task for analysis; see
  [`docs/vtune.md`](./docs/vtune.md) for details.

See also [`crates/recorder/README.md`](crates/recorder/README.md).

### Re-analyzing a Previous Run's Results

Benchmark runs are expensive, so every run saves a copy of its raw results to
`sightglass-data.csv` in the current directory in addition to printing the
human-readable analysis. If there was already a `sightglass-data.csv`, it is
first moved to `sightglass-data.old.csv`, overwriting any file that was already
there.

This means you can analyze a run's results a different way without having to
re-run the benchmarks:

```
$ cargo run -- summarize --input-format csv -f sightglass-data.csv
$ cargo run -- effect-size --input-format csv -f sightglass-data.csv \
    --significance-level 0.05
```

The data file is not saved in `--raw` mode, where you are already choosing what
to do with the raw results yourself.

### Getting Raw JSON or CSV Results

If you don't want the results to be summarized and displayed in a human-readable
format, you can get raw JSON or CSV via the `--raw` and `--output-format` CLI
flags:

```
$ cargo run -- benchmark --raw --output-format json -- benchmark.wasm
```

Then you can use your own R/Python/spreadsheets/etc... to analyze and visualize
the benchmark results.

### HTML Report Generation

For raw results collected in the JSON format, sightglass includes a built-in
`report` subcommand that generates comprehensive HTML reports with statistical
analysis and interactive visualizations:

```bash
# Generate HTML report comparing multiple measurement files
$ cargo run -- report --output-file report.html baseline.json experiment1.json experiment2.json

# Specify which engine serves as the baseline for comparisons
$ cargo run -- report --baseline-engine wasmtime-baseline --output-file report.html baseline.json experiment.json
```

### Building the Engine(s) to Run Benchmarks Within

```shell-session
rustc engines/wasmtime/build.rs
./build
```

The resulting engine will be at `libengine.so` (or `libengine.dll` on windows
and `libengine.dylib` on macos). You may pass a directory path as an argument to
`./build` and the engine will be placed there instead of the current directory.

By default this builds the latest commit on `main` from the canonical
`bytecodealliance/wasmtime` repository.

You can set the following environment variables while running `./build` to
configure the engine's build:

* `REPOSITORY`: the URL or file path of the Wasmtime repository to be built. If
  the version of Wasmtime you'd like to benchmark is not merged upstream yet,
  you can set this to your fork. Defaults to
  `https://github.com/bytecodealliance/wasmtime.git`.
* `REVISION`: the git branch, commit, tag, etc... that should be built. Defaults
  to `main`.

For example:

```
REPOSITORY=https://github.com/fitzgen/wasmtime.git \
REVISION=add-cool-new-optimization \
    ./build
```
