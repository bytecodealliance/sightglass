# Wasmtime Engine

The `build.rs` script in this directory will build a Sightglass-compatible[^details] benchmarking
library using the Wasmtime engine. The script output is a shared library (e.g., `libengine.so`) that
can be used for running benchmarks; the output also includes a build metadata file (i.e.,
`.build-info`) that contains environment information for reproducing the build. `build.rs` is a Rust
script so that it can be used on any supported OS.

[^details]: The specific details of the `bench` API that must be exported by the
engine shared library are available in [the Wasmtime bench-api
documentation](https://github.com/bytecodealliance/wasmtime/blob/main/crates/bench-api/src/lib.rs).

### Use

To build the files and store them in the current working directory, run:

```
rustc build.rs
./build
```

The script can be configured in several ways:

```
[REPOSITORY=<repo url>] [REVISION=<hash|branch|tag>] ./build [<destination dir>]
```

All configuration is optional. The script responds to environment variables that change the Wasmtime
source code used; by default, the script will download the tip-of-`main` Wasmtime from the official
repository. If provided, the first CLI argument can override the destination directory at which to
place the built files.

### Contributing

Since this script is not part of the main CI it would be helpful to run the following commands
before upstreaming changes:

```
rustfmt build.rs
clippy-driver build.rs
```
