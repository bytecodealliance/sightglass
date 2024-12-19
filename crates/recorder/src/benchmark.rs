use crate::bench_api::{Engine, Module};
use crate::measure::Measure;
use anyhow::Result;
use log::info;

/// Measure all phases of a Wasm module's lifetime: compilation, instantiation,
/// and execution.
pub fn all<'a, 'b, 'c, M: Measure>(
    engine: Engine<'a, 'b, 'c, M>,
    wasm_bytes: &[u8],
) -> Result<Engine<'a, 'b, 'c, M>> {
    #[cfg(target_os = "linux")]
    info!("Benchmark scheduled on CPU: {}", unsafe {
        libc::sched_getcpu()
    });

    // Measure the module compilation.
    let module = engine.compile(wasm_bytes);
    info!("Compiled successfully");

    // Measure the module instantiation.
    let instance = module.instantiate();
    info!("Instantiated successfully");

    let module = instance.execute();
    info!("Executed successfully");

    Ok(module.into_engine())
}

/// Measure just the compilation phase of a Wasm module's lifetime.
pub fn compilation<'a, 'b, 'c, M: Measure>(
    engine: Engine<'a, 'b, 'c, M>,
    wasm_bytes: &[u8],
) -> Result<Engine<'a, 'b, 'c, M>> {
    #[cfg(target_os = "linux")]
    info!("Benchmark scheduled on CPU: {}", unsafe {
        libc::sched_getcpu()
    });

    let module = engine.compile(wasm_bytes);
    info!("Compiled successfully");

    Ok(module.into_engine())
}

/// Measure just the instantiation phase of a Wasm module's lifetime.
pub fn instantiation<'a, 'b, 'c, M: Measure>(
    module: Module<'a, 'b, 'c, M>,
) -> Result<Module<'a, 'b, 'c, M>> {
    let instance = module.instantiate();
    info!("Instantiated successfully");

    Ok(instance.into_module())
}

/// Measure just the execution phase of a Wasm module's lifetime.
pub fn execution<'a, 'b, 'c, M: Measure>(
    module: Module<'a, 'b, 'c, M>,
) -> Result<Module<'a, 'b, 'c, M>> {
    let instance = module.instantiate();
    info!("Instantiated successfully");

    let module = instance.execute();
    info!("Executed successfully");

    Ok(module)
}
