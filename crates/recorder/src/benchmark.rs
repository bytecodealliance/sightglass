use crate::bench_api::Engine;
use crate::measure::Measure;
use anyhow::Result;
use log::info;
use sightglass_data::Phase;

/// Measure various phases of a Wasm module's lifetime.
///
/// Provide paths to files created for logging the Wasm's `stdout` and `stderr`
/// and (optionally) a file read and piped into the Wasm execution as `stdin`.
///
/// Optionally stop after the given `stop_after_phase`, rather than running all
/// phases.
pub fn benchmark<'a, 'b, 'c, M: Measure>(
    engine: Engine<'a, 'b, 'c, M>,
    wasm_bytes: &[u8],
    stop_after_phase: Option<Phase>,
) -> Result<Engine<'a, 'b, 'c, M>> {
    #[cfg(target_os = "linux")]
    info!("Benchmark scheduled on CPU: {}", unsafe {
        libc::sched_getcpu()
    });

    // Measure the module compilation.
    let module = engine.compile(wasm_bytes);
    info!("Compiled successfully");

    if stop_after_phase == Some(Phase::Compilation) {
        return Ok(module.into_engine());
    }

    // Measure the module instantiation.
    let instance = module.instantiate();
    info!("Instantiated successfully");

    if stop_after_phase == Some(Phase::Instantiation) {
        return Ok(instance.into_module().into_engine());
    }

    let module = instance.execute();
    info!("Executed successfully");

    Ok(module.into_engine())
}
