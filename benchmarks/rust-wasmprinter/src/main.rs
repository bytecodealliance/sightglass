//! Disassembles a WebAssembly binary into text with the `wasmprinter` crate.

wit_bindgen::generate!({
    inline: "
        package sightglass:bench;
        world rust-wasmprinter {
            import bench: interface {
                start: func();
                end: func();
            }
        }
    ",
});

/// Fallback for `./rust-wasmprinter.iterations.input`, tuned for ~100M Wasm
/// instructions.
const ITERATIONS: usize = 1;

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

fn main() {
    let binary = std::fs::read("./rust-wasmprinter.input.bin").expect("read input");
    let iterations = std::fs::read_to_string("./rust-wasmprinter.iterations.input")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(ITERATIONS);

    bench::start();
    let mut text = String::new();
    for _ in 0..iterations {
        text = wasmprinter::print_bytes(std::hint::black_box(&binary)).expect("valid wasm");
    }
    bench::end();

    println!("{} bytes, fnv1a {:016x}", text.len(), fnv1a(text.as_bytes()));
}
