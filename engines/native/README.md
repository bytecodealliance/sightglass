# Build native engine for sightglass
cd <sightglass-base>/engines/native/libengine
cargo build --release
cp target/release/libnative_bench_api.so ../libengine.so
cd -

# To run a benchmark with the newly created library
cd <sightglass-base>
cargo build --release
LD_LIBRARY_PATH=./engines/native/ ./target/release/sightglass-cli benchmark ./benchmarks/shootout-base64/benchmark.wasm --engine engines/native/libengine.so  --processes=1 --raw --output-format csv