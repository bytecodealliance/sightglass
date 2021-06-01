# Contributing to Wasmtime and/or Cranelift

Wasmtime and Cranelift are [Bytecode Alliance] projects. They follow the
Bytecode Alliance's [Code of Conduct] and [Organizational Code of Conduct].

[Bytecode Alliance]: https://bytecodealliance.org/
[Code of Conduct]: CODE_OF_CONDUCT.md
[Organizational Code of Conduct]: ORG_CODE_OF_CONDUCT.md

## Building

```
$ cargo build --all
```

## Testing

```
$ cargo test --all
```

You can also specify that a particular `libwasmtime_bench_api.{so,dylib,dll}` is
used in the tests via the `SIGHTGLASS_TEST_ENGINE` environment variable. This
can be useful when developing changes to the `wasmtime-bench-api` ABI.

```
$ cd ~/wasmtime
$ cargo build --release -p wasmtime-bench-api
$ cd ~/sightglass
$ SIGHTGLASS_TEST_ENGINE=~/wasmtime/target/release/libwasmtime_bench_api.so \
    cargo test --all
```

## Finding Something to Work On

* [Milestones.](https://github.com/bytecodealliance/sightglass/milestones) Help
  us all move in the same direction! These should roughly match the incremental
  milestones [outlined in our
  RFC](https://github.com/bytecodealliance/rfcs/blob/main/accepted/benchmark-suite.md#incremental-milestones).

* [All open issues.](https://github.com/bytecodealliance/sightglass/issues) Find
  a bug to fix!

## Adding New Benchmarks

We would love to have more benchmark programs! In particular we want real,
widely used programs, or at least extracted kernels of such programs. These
programs are ideally taken from domains where Wasmtime and Cranelift are
currently used, or domains where they are intended to be a good fit
(e.g. serverless compute, game plugins, client Web applications, server Web
applications, audio plugins, etc.).

The benchmarking suite RFC has a [list of potential benchmark programs we'd like
to
add](https://github.com/bytecodealliance/rfcs/blob/main/accepted/benchmark-suite.md#initial-list-of-potential-candidates),
and we still haven't added most of them!

[See `benchmarks-next/README.md` for technical details on adding new benchmark
programs.](https://github.com/bytecodealliance/sightglass/blob/main/benchmarks-next/README.md)
