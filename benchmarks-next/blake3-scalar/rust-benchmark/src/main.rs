use sightglass_api as bench;

fn main() {
    let buffer = if std::env::var("WASM_BENCH_USE_SMALL_WORKLOAD").is_ok() {
        eprintln!("[blake3] hashing ./small.input");
        std::fs::read("./small.input").unwrap()
    } else {
        eprintln!("[blake3] hashing ./default.input");
        std::fs::read("./default.input").unwrap()
    };
    eprintln!("[blake3] input size = {}", buffer.len());

    bench::start();
    let hash = blake3::hash(&buffer);
    bench::end();

    eprintln!("[blake3] returned {:?}", hash);
}
