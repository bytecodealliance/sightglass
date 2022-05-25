mod wasm;

pub use wasm::WasmBenchmark;

/// Calculate the library name for a sightglass library on the target operating system: e.g.
/// `engine.dll`, `libengine.so`.
pub fn get_engine_filename() -> String {
    format!(
        "{}engine{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    )
}
