# Build native engine for sightglass
cd libengine
# wget https://raw.githubusercontent.com/bytecodealliance/wasmtime/dbc6db0cfb114f1cfa30866f602eff7c9025a597/crates/bench-api/src/lib.rs
# wget https://raw.githubusercontent.com/bytecodealliance/wasmtime/dbc6db0cfb114f1cfa30866f602eff7c9025a597/crates/bench-api/src/unsafe_send_sync.rs
cargo build --release
cp target/release/libnative_bench_api.so ../libengine.so
cd -