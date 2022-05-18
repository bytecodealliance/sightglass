use crate::bench_api::{BenchApi, Engine};
use crate::measure::{Measure, Measurements};
use anyhow::Result;
use log::info;
use sightglass_data::Phase;
use std::path::Path;

/// Measure various phases of a Wasm module's lifetime.
///
/// Provide paths to files created for logging the Wasm's `stdout` and `stderr`
/// and (optionally) a file read and piped into the Wasm execution as `stdin`.
///
/// Optionally stop after the given `stop_after_phase`, rather than running all
/// phases.
pub fn benchmark<'a, 'b, 'c>(
    bench_api: &'a mut BenchApi<'b>,
    working_dir: &Path,
    stdout_path: &Path,
    stderr_path: &Path,
    stdin_path: Option<&Path>,
    wasm_bytes: &[u8],
    stop_after_phase: Option<Phase>,
    measure: &'a mut impl Measure,
    measurements: &'a mut Measurements<'c>,
) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    info!("Benchmark scheduled on CPU: {}", unsafe {
        libc::sched_getcpu()
    });

    let engine = Engine::new(
        bench_api,
        working_dir,
        stdout_path,
        stderr_path,
        stdin_path,
        measurements,
        measure,
        None,
    );

    // Measure the module compilation.
    let module = engine.compile(wasm_bytes);
    info!("Compiled successfully");

    if stop_after_phase == Some(Phase::Compilation) {
        return Ok(());
    }

    // Measure the module instantiation.
    let instance = module.instantiate();
    info!("Instantiated successfully");

    if stop_after_phase == Some(Phase::Instantiation) {
        return Ok(());
    }

    instance.execute();
    info!("Executed successfully");

    Ok(())
}
