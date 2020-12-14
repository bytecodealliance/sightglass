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
    let buffer = [0 as u8; 64 << 10];
    eprintln!("[blake3] hashing a 64KB, zero-filled buffer");
    unsafe {
        start();
    }
    let hash = blake3::hash(&buffer);
    unsafe {
        end();
    }
    eprintln!("[blake3] returned {:?}", hash);
}
