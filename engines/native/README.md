# Build native engine for sightglass
cd <sightglass-base>/engines/native/libengine
cargo build --release
cp target/release/libnative_bench_api.so ../libengine.so
cd -

# Build the native binary for a single benchmark
# Note, currently only shootout-* benchmarks supported.
cd <sightglass-base>/path/to/benchmark
cargo run
cd -

or

# To build the native binary for all shootout benchmarks
cd <sightglass-base>/benchmarks/
find shootout-* -maxdepth 0 -type d \( ! -name . \) -exec bash -c "cd '{}' && pwd && cargo run" \;
cd -

# Run a benchmark with the newly created library
cd <sightglass-base>
cargo build --release
LD_LIBRARY_PATH=./engines/native/ cargo run benchmark --engine engines/native/libengine.so -- /path/to/benchmark.so
