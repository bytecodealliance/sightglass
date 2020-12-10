use crate::measure::{Measure, MeasureType, Measurement};
use anyhow::Result;
use libloading as lib;
use log::info;
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, OsStr};

/// Measure various phases of a Wasm module's lifetime. This relies on `engine_lib_path`, the path
/// to a shared library that implements `engine_*` functions that allow for measurement of each
/// phase.
pub fn benchmark(
    wasm_bytes: impl AsRef<[u8]>,
    engine_lib_path: impl AsRef<OsStr>,
    measure_type: MeasureType,
) -> Result<BenchmarkMeasurements> {
    let wasm_bytes = wasm_bytes.as_ref();
    info!(
        "Attempting to run benchmark of {} bytes with engine at {:?}",
        wasm_bytes.len(),
        engine_lib_path.as_ref()
    );

    // Retrieve the functions used for benchmarking from the shared library at `engine_lib_path`. We
    // do this prior to benchmarking so that library resolution does not interfere with our results.
    let lib = lib::Library::new(engine_lib_path)?;
    let engine_create: lib::Symbol<'_, unsafe extern "C" fn(*const u8, usize) -> *mut c_void> =
        unsafe { lib.get(b"engine_create")? };
    let engine_free: lib::Symbol<'_, unsafe extern "C" fn(*const c_void)> =
        unsafe { lib.get(b"engine_free")? };
    let engine_compile_module: lib::Symbol<'_, unsafe extern "C" fn(*const c_void) -> i32> =
        unsafe { lib.get(b"engine_compile_module")? };
    let engine_instantiate_module: lib::Symbol<
        '_,
        unsafe extern "C" fn(*const c_void, extern "C" fn(), extern "C" fn()) -> i32,
    > = unsafe { lib.get(b"engine_instantiate_module")? };
    let engine_execute_module: lib::Symbol<'_, unsafe extern "C" fn(*const c_void) -> i32> =
        unsafe { lib.get(b"engine_execute_module")? };

    let mut measure = measure_type.build();
    unsafe {
        let engine = engine_create(wasm_bytes.as_ptr(), wasm_bytes.len());

        // Measure the module compilation.
        measure.start();
        let result = engine_compile_module(engine);
        let compilation = measure.end();
        assert_eq!(
            result, 0,
            "`engine_compile_module` did not return a success exit code"
        );
        info!("Compiled successfully: {:?}", compilation);

        // Measure the module instantiation.
        measure.start();
        let result = engine_instantiate_module(
            engine,
            static_measure::bench_start,
            static_measure::bench_end,
        );
        let instantiation = measure.end();
        assert_eq!(
            result, 0,
            "`engine_instantiate_module` did not return a success exit code"
        );
        info!("Instantiated successfully: {:?}", instantiation);

        // Measure the module execution; note that, because bench_start and bench_end are passed in to the Wasm module
        // as imports, we must retain the measurement state here, in the host code--see `static_measure`.
        static_measure::setup(measure);
        let result = engine_execute_module(engine);
        let execution = static_measure::result();
        assert_eq!(
            result, 0,
            "`engine_execute_module` did not return a success exit code"
        );
        info!("Executed successfully (bench time): {:?}", execution);

        engine_free(engine);
        Ok(BenchmarkMeasurements {
            compilation,
            instantiation,
            execution,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkMeasurements {
    compilation: Measurement,
    instantiation: Measurement,
    execution: Measurement,
}

// Because this recorder cedes control to the Wasm module during execution, giving it the responsibility for calling
// `bench_start` and `bench_end` at the appropriate times, we need to maintain any recording state here. This Rust
// module does so using static variables, naturally resulting in unsafe behavior. This module should be safe if used
// correctly:
//  - `setup()` the static values
//  - call `bench_start()` and then `bench_end()`
//  - retrieve the measurement with `result()`
mod static_measure {
    use super::*;
    static mut MEASURE: Option<Box<dyn Measure>> = None;
    static mut MEASUREMENT: Option<Measurement> = None;
    pub(crate) fn setup(measure: Box<dyn Measure>) {
        unsafe {
            MEASURE = Some(measure);
            MEASUREMENT = None;
        }
    }
    pub(crate) extern "C" fn bench_start() {
        info!("bench_start was called");
        unsafe {
            MEASURE.as_mut().unwrap().start();
        }
    }
    pub(crate) extern "C" fn bench_end() {
        info!("bench_end was called");
        unsafe {
            MEASUREMENT = Some(MEASURE.as_mut().unwrap().end());
        }
    }
    pub(crate) fn result() -> Measurement {
        unsafe { MEASUREMENT.take().unwrap() }
    }
}
