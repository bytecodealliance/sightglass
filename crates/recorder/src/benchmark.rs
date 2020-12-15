use crate::measure::{Measure, MeasureType, Measurement};
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;

/// An shared library that implements our in-process benchmarking API.
pub struct BenchApi<'a> {
    wasm_bench_create: libloading::Symbol<'a, unsafe extern "C" fn() -> *mut c_void>,
    wasm_bench_free: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void)>,
    wasm_bench_compile:
        libloading::Symbol<'a, unsafe extern "C" fn(*const c_void, *const u8, usize) -> i32>,
    wasm_bench_instantiate: libloading::Symbol<
        'a,
        unsafe extern "C" fn(*const c_void, extern "C" fn(), extern "C" fn()) -> i32,
    >,
    wasm_bench_execute: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void) -> i32>,
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
            wasm_bench_create: lib.get(b"wasm_bench_create")?,
            wasm_bench_free: lib.get(b"wasm_bench_free")?,
            wasm_bench_compile: lib.get(b"wasm_bench_compile")?,
            wasm_bench_instantiate: lib.get(b"wasm_bench_instantiate")?,
            wasm_bench_execute: lib.get(b"wasm_bench_execute")?,
        })
    }
}

/// An engine from a `BenchApi`.
pub struct Engine<'a, 'b> {
    bench_api: &'a mut BenchApi<'b>,
    engine: *mut c_void,
}

impl<'a, 'b> Engine<'a, 'b> {
    /// Construct a new engine from the given `BenchApi`.
    // NB: take a mutable reference to the `BenchApi` so that no one else can
    // call its API methods out of order.
    pub fn new(bench_api: &'a mut BenchApi<'b>) -> Self {
        let engine = unsafe { (bench_api.wasm_bench_create)() };
        Engine { bench_api, engine }
    }

    /// Compile the Wasm into a module.
    pub fn compile(self, wasm: &[u8], measure: &mut impl Measure) -> (Module<'a, 'b>, Measurement) {
        measure.start();
        let result =
            unsafe { (self.bench_api.wasm_bench_compile)(self.engine, wasm.as_ptr(), wasm.len()) };
        let measurement = measure.end();
        assert_eq!(result, 0);
        (Module { engine: self }, measurement)
    }
}

impl<'a, 'b> Drop for Engine<'a, 'b> {
    fn drop(&mut self) {
        unsafe {
            (self.bench_api.wasm_bench_free)(self.engine);
        }
    }
}

/// A compiled module from a `BenchApi`.
pub struct Module<'a, 'b> {
    engine: Engine<'a, 'b>,
}

impl<'a, 'b> Module<'a, 'b> {
    /// Instantiate this module, returning the resulting `Instance`.
    pub fn instantiate(self, measure: &mut impl Measure) -> (Instance<'a, 'b>, Measurement) {
        measure.start();
        let result = unsafe {
            (self.engine.bench_api.wasm_bench_instantiate)(
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
pub struct Instance<'a, 'b> {
    engine: Engine<'a, 'b>,
}

impl<'a, 'b> Instance<'a, 'b> {
    /// Execute this instance's code, returning the engine it was in.
    pub fn execute(self, measure: Box<dyn Measure>) -> Measurement {
        static_measure::setup(measure);
        let result = unsafe { (self.engine.bench_api.wasm_bench_execute)(self.engine.engine) };
        let execution = static_measure::result();
        assert_eq!(result, 0);
        execution
    }
}

/// Measure various phases of a Wasm module's lifetime. This relies on `engine_lib_path`, the path
/// to a shared library that implements `wasm_bench_*` functions that allow for measurement of each
/// phase.
pub fn benchmark(
    bench_api: &mut BenchApi,
    wasm_bytes: &[u8],
    measure_type: MeasureType,
) -> Result<BenchmarkMeasurements> {
    let mut measure = measure_type.build();
    let engine = Engine::new(bench_api);

    // Measure the module compilation.
    let (module, compilation) = engine.compile(wasm_bytes, &mut measure);
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
