This crate provides a way (on Linux) to run a subset of the Sightglass
benchmarks natively. This involves compiling two specially-crafted shared
libraries: (1) this engine and (2) the benchmark itself.

### Build the native engine

```console
$ cd <sightglass-base>/engines/native/libengine
$ cargo build --release
$ cp target/release/libnative_bench_api.so ../libengine.so
$ cd -
```

### Build the native binary for a single benchmark*

```console
$ <sightglass-base>/benchmarks/build-native.sh path/to/benchmark
```

See the benchmark-building [requirements](../../benchmarks/README.md) for more
details.

**\* Note: not all benchmarks support a native build**

### Run a benchmark with the newly-created library

```console
$ cd <sightglass-base>
$ cargo build --release
$ LD_LIBRARY_PATH=./engines/native/ benchmarks/run-native.sh \
    /path/to/benchmark.so
```

