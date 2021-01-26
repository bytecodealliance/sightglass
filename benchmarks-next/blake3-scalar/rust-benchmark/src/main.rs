// This syntax informs the Rust-to-Wasm compiler toolchain that we want to import the `bench.start`
// and `bench.end` functions.
#[link(wasm_import_module = "bench")]
extern "C" {
    fn start();
}
#[link(wasm_import_module = "bench")]
extern "C" {
    fn end();
}

fn main() {
    let buffer = if std::env::var("WASM_BENCH_USE_SMALL_WORKLOAD").is_ok() {
        eprintln!("[blake3] hashing ./small.input");
        std::fs::read("./small.input").unwrap()
    } else {
        eprintln!("[blake3] hashing ./default.input");
        std::fs::read("./default.input").unwrap()
    };
    eprintln!("[blake3] input size = {}", buffer.len());
    unsafe {
        start();
    }
    let hash = blake3::hash(&buffer);
    unsafe {
        end();
    }
    eprintln!("[blake3] returned {:?}", hash);
}
