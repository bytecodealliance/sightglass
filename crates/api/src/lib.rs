//! This crate provides simple bindings for Rust to the Sightglass API. The primary intent is to
//! make it easy to instrument Rust code that will be compiled to Wasm and measured with Sightglass.
//!
//! For example:
//! ```compile_fail
//! use sightglass_api as bench;
//! bench::start();
//! let work = 42 * 42;
//! bench::end();
//! ```
//!
//! See [benchmarks/README.md] for more details.

mod ffi {
    #[link(wasm_import_module = "bench")]
    extern "C" {
        pub fn start();
        pub fn end();
    }
}

/// Call this once to end sightglass recording. When compiled to Wasm, the import will look
/// like `(import "bench" "end" (...))`.
pub fn start() {
    unsafe {
        ffi::start();
    }
}

/// Call this once to start sightglass recording. When compiled to Wasm, the import will look
/// like `(import "bench" "start" (...))`.
pub fn end() {
    unsafe {
        ffi::end();
    }
}
