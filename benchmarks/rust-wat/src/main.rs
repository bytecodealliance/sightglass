//! Assembles WebAssembly text into a binary with the `wat` crate.

wit_bindgen::generate!({
    inline: "
        package sightglass:bench;
        world rust-wat {
            import bench: interface {
                start: func();
                end: func();
            }
        }
    ",
});

/// Fallback for `./rust-wat.iterations.input`, tuned for ~100M Wasm instructions.
const ITERATIONS: usize = 1;

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

fn main() {
    let text = std::fs::read_to_string("./rust-wat.input.wat").expect("read input");
    let iterations = std::fs::read_to_string("./rust-wat.iterations.input")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(ITERATIONS);

    bench::start();
    let mut binary = Vec::new();
    for _ in 0..iterations {
        binary = wat::parse_str(std::hint::black_box(&text)).expect("valid wat");
    }
    bench::end();

    println!("{} bytes, fnv1a {:016x}", binary.len(), fnv1a(&binary));
}
