### Build native engine for sightglass
cd <sightglass-base>/engines/native/libengine<br>
cargo build --release<br>
cp target/release/libnative_bench_api.so ../libengine.so<br>
cd -<br>
<br>

### Build the native binary for a single benchmark*
cd <sightglass-base>/path/to/benchmark<br>
cargo run<br>
cd -<br>
<br>

### To build the native binary for all shootout benchmarks*
cd <sightglass-base>/benchmarks/<br>
find shootout-* -maxdepth 0 -type d \( ! -name . \) -exec bash -c "cd '{}' && pwd && cargo run" \;<br>
cd -<br>
<br>

### Run a benchmark with the newly created library
cd <sightglass-base><br>
cargo build --release<br>
LD_LIBRARY_PATH=./engines/native/ cargo run benchmark --engine engines/native/libengine.so -- /path/to/benchmark.so<br>
<br>
**\* Note, not all benchmark support a native build**