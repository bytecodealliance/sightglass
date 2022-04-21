# Sightglass Engines

In order to get comparable results between environments, it is important that the Sightglass engines
can be reliably rebuilt. This is related to [reproducible builds](https://reproducible-builds.org),
and although the mechanism described here is not completely deterministic, it approaches this goal.

First, each engine here is built using a Dockerfile to fix some of the build environment. The Docker
image is expected to contain a shared library at a certain location upon completion:
`/libengine.so`. The specific details of the `bench` API that must be exported by the engine shared
library are available in [the Wasmtime bench-api
documentation](https://github.com/bytecodealliance/wasmtime/blob/main/crates/bench-api/src/lib.rs).

Secondly, engines are expected to output an accompanying `.build-info` file (i.e., `/.build-info`)
with variable-value pairs describing the build configuration necessary to rebuild the engine. E.g.:

```
BUILD='cargo build -p wasmtime-bench-api'
FLAGS=--release
NAME=wasmtime
REPOSITORY=https://github.com/bytecodealliance/wasmtime/
REVISION=main
_CARGO='cargo 1.54.0 (5ae8d74b3 2021-06-22)'
_COMMIT=d9dfc44c32d47c9627858075dfa02e3190e8b876
_RUSTC='rustc 1.54.0 (a178d0322 2021-07-26)'
```

The `_`-prefixed pairs are ignored for now[^prefix], but the other pairs correspond to `ARG` statements
in the engine's Dockerfile. `sightglass-cli build-engine` uses a URI-like string of this
configuration to pass build arguments to the Dockerfile, reproducing the built engine shared
library.

[^prefix]: the `_COMMIT` setting is actually used for overwriting the revision to avoid
 situations where, e.g., a branch is used and additional commits are added between builds. Also,
 `_RUSTC` could be used to alter the compiler version.

To modify the construction of an engine, use the `.build-info` string

```
sightglass-cli build-engine wasmtime?REVISION=v0.33.1+FLAGS='--release --features new-feature'
```
