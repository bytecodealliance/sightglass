use crate::measure::{Measure, MeasureType, Measurement};
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, OsStr};

/// An shared library that implements our in-process benchmarking API.
pub struct BenchApi<'a> {
    engine_create: libloading::Symbol<'a, unsafe extern "C" fn(*const u8, usize) -> *mut c_void>,
    engine_free: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void)>,
    engine_compile_module: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void) -> i32>,
    engine_instantiate_module: libloading::Symbol<
        'a,
        unsafe extern "C" fn(*const c_void, extern "C" fn(), extern "C" fn()) -> i32,
    >,
    engine_execute_module: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void) -> i32>,
}

impl<'a> BenchApi<'a> {
    /// Create a new `BenchApi` from the given shared library.
    ///
    /// # Safety
    ///
    /// The given shared library must export bench API functions with the
    /// correct signatures, since we have no way to check that it gets them
    /// right.
    pub unsafe fn new(lib: &'a libloading::Library) -> Result<Self> {
        Ok(BenchApi {
            engine_create: lib.get(b"engine_create")?,
            engine_free: lib.get(b"engine_free")?,
            engine_compile_module: lib.get(b"engine_compile_module")?,
            engine_instantiate_module: lib.get(b"engine_instantiate_module")?,
            engine_execute_module: lib.get(b"engine_execute_module")?,
        })
    }
}

/// An engine from a `BenchApi`.
pub struct Engine<'a> {
    bench_api: &'a BenchApi<'a>,
    engine: *mut c_void,
}

impl<'a> Engine<'a> {
    /// Construct a new engine from the given `BenchApi`.
    pub fn new(bench_api: &'a BenchApi<'a>, wasm: &[u8]) -> Self {
        let engine = unsafe { (bench_api.engine_create)(wasm.as_ptr(), wasm.len()) };
        Engine { bench_api, engine }
    }

    /// Compile the Wasm into a module.
    pub fn compile(self, measure: &mut impl Measure) -> (Module<'a>, Measurement) {
        measure.start();
        let result = unsafe { (self.bench_api.engine_compile_module)(self.engine) };
        let measurement = measure.end();
        assert_eq!(result, 0);
        (Module { engine: self }, measurement)
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {
        unsafe {
            (self.bench_api.engine_free)(self.engine);
        }
    }
}

/// A compiled module from a `BenchApi`.
pub struct Module<'a> {
    engine: Engine<'a>,
}

impl<'a> Module<'a> {
    /// Instantiate this module, returning the resulting `Instance`.
    pub fn instantiate(self, measure: &mut impl Measure) -> (Instance<'a>, Measurement) {
        measure.start();
        let result = unsafe {
            (self.engine.bench_api.engine_instantiate_module)(
                self.engine.engine,
                static_measure::bench_start,
                static_measure::bench_end,
            )
        };
        let measurement = measure.end();
        assert_eq!(result, 0);
        (
            Instance {
                engine: self.engine,
            },
            measurement,
        )
    }
}

/// An instantiated module from our `BenchApi`.
pub struct Instance<'a> {
    engine: Engine<'a>,
}

impl<'a> Instance<'a> {
    /// Execute this instance's code, returning the engine it was in.
    pub fn execute(self, measure: Box<dyn Measure>) -> Measurement {
        static_measure::setup(measure);
        let result = unsafe { (self.engine.bench_api.engine_execute_module)(self.engine.engine) };
        let execution = static_measure::result();
        assert_eq!(result, 0);
        execution
    }
}

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
    let lib = libloading::Library::new(engine_lib_path)?;
    let bench_api = unsafe { BenchApi::new(&lib)? };

    let mut measure = measure_type.build();
    let engine = Engine::new(&bench_api, wasm_bytes);

    // Measure the module compilation.
    let (module, compilation) = engine.compile(&mut measure);
    info!("Compiled successfully: {:?}", compilation);

    // Measure the module instantiation.
    let (instance, instantiation) = module.instantiate(&mut measure);
    info!("Instantiated successfully: {:?}", instantiation);

    // Measure the module execution; note that, because bench_start and bench_end are passed in to the Wasm module
    // as imports, we must retain the measurement state here, in the host code--see `static_measure`.
    let execution = instance.execute(measure);
    info!("Executed successfully (bench time): {:?}", execution);

    Ok(BenchmarkMeasurements {
        compilation,
        instantiation,
        execution,
    })
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
