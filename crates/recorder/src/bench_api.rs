use crate::measure::{Measure, Measurements};
use anyhow::Result;
use sightglass_data::Phase;
use std::path::Path;
use std::ptr;
use std::{cell::UnsafeCell, ffi::c_void};

/// NB: Keep this in sync with the version defined in
/// `wasmtime-bench-api`!
#[repr(C)]
struct WasmBenchConfig {
    working_dir_ptr: *const u8,
    working_dir_len: usize,

    stdout_path_ptr: *const u8,
    stdout_path_len: usize,

    stderr_path_ptr: *const u8,
    stderr_path_len: usize,

    stdin_path_ptr: *const u8,
    stdin_path_len: usize,

    compilation_timer: *mut u8,
    compilation_start: extern "C" fn(*mut u8),
    compilation_end: extern "C" fn(*mut u8),

    instantiation_timer: *mut u8,
    instantiation_start: extern "C" fn(*mut u8),
    instantiation_end: extern "C" fn(*mut u8),

    execution_timer: *mut u8,
    execution_start: extern "C" fn(*mut u8),
    execution_end: extern "C" fn(*mut u8),
}

/// An shared library that implements our in-process benchmarking API.
pub struct BenchApi<'a> {
    wasm_bench_create:
        libloading::Symbol<'a, unsafe extern "C" fn(WasmBenchConfig, *mut *mut c_void) -> i32>,
    wasm_bench_free: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void)>,
    wasm_bench_compile:
        libloading::Symbol<'a, unsafe extern "C" fn(*const c_void, *const u8, usize) -> i32>,
    wasm_bench_instantiate: libloading::Symbol<'a, unsafe extern "C" fn(*const c_void) -> i32>,
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
pub struct Engine<'a, 'b, 'c, M> {
    bench_api: &'a mut BenchApi<'b>,
    measurement_data: *mut UnsafeCell<(&'a mut M, &'a mut Measurements<'c>)>,
    engine: *mut c_void,
}

impl<'a, 'b, 'c, M> Engine<'a, 'b, 'c, M>
where
    M: Measure,
{
    /// Construct a new engine from the given `BenchApi`.
    // NB: take a mutable reference to the `BenchApi` so that no one else can
    // call its API methods out of order.
    pub fn new(
        bench_api: &'a mut BenchApi<'b>,
        working_dir: &Path,
        stdout_path: &Path,
        stderr_path: &Path,
        stdin_path: Option<&Path>,
        measurements: &'a mut Measurements<'c>,
        measure: &'a mut M,
    ) -> Self {
        let working_dir = working_dir.display().to_string();
        let stdout_path = stdout_path.display().to_string();
        let stderr_path = stderr_path.display().to_string();
        let stdin_path = stdin_path.map(|p| p.display().to_string());

        let measurement_data = Box::new(UnsafeCell::new((measure, measurements)));
        let measurement_data = Box::into_raw(measurement_data);

        let config = WasmBenchConfig {
            working_dir_ptr: working_dir.as_ptr(),
            working_dir_len: working_dir.len(),
            stdout_path_ptr: stdout_path.as_ptr(),
            stdout_path_len: stdout_path.len(),
            stderr_path_ptr: stderr_path.as_ptr(),
            stderr_path_len: stderr_path.len(),
            stdin_path_ptr: stdin_path.as_ref().map_or(ptr::null(), |p| p.as_ptr()),
            stdin_path_len: stdin_path.as_ref().map_or(0, |p| p.len()),
            compilation_timer: measurement_data as *mut u8,
            compilation_start: Self::start,
            compilation_end: Self::compilation_end,
            instantiation_timer: measurement_data as *mut u8,
            instantiation_start: Self::start,
            instantiation_end: Self::instantiation_end,
            execution_timer: measurement_data as *mut u8,
            execution_start: Self::start,
            execution_end: Self::execution_end,
        };

        let mut engine = ptr::null_mut();
        unsafe {
            let result = (bench_api.wasm_bench_create)(config, &mut engine);
            assert_eq!(result, 0);
            assert!(!engine.is_null());
        };
        Engine {
            bench_api,
            measurement_data,
            engine,
        }
    }

    /// Compile the Wasm into a module.
    pub fn compile(self, wasm: &[u8]) -> Module<'a, 'b, 'c, M> {
        let result =
            unsafe { (self.bench_api.wasm_bench_compile)(self.engine, wasm.as_ptr(), wasm.len()) };
        assert_eq!(result, 0);
        Module { engine: self }
    }

    /// Shared bench API callback for the start of
    /// compilation/instantiation/execution.
    extern "C" fn start(data: *mut u8) {
        log::info!("Starting new measurement");
        let data = data as *mut (*mut M, *mut Measurements<'b>);
        let measure = unsafe { data.as_mut().unwrap().0.as_mut().unwrap() };
        measure.start();
    }

    /// Bench API callback for the end of compilation.
    extern "C" fn compilation_end(data: *mut u8) {
        let data = data as *mut (*mut M, *mut Measurements<'b>);
        let (measure, measurements) = unsafe {
            let data = data.as_mut().unwrap();
            (data.0.as_mut().unwrap(), data.1.as_mut().unwrap())
        };
        measure.end(Phase::Compilation, measurements);
        log::info!("Finished measuring compilation");
    }

    /// Bench API callback for the end of instantiation.
    extern "C" fn instantiation_end(data: *mut u8) {
        let data = data as *mut (*mut M, *mut Measurements<'b>);
        let (measure, measurements) = unsafe {
            let data = data.as_mut().unwrap();
            (data.0.as_mut().unwrap(), data.1.as_mut().unwrap())
        };
        measure.end(Phase::Instantiation, measurements);
        log::info!("Finished measuring instantiation");
    }

    /// Bench API callback for the end of execution.
    extern "C" fn execution_end(data: *mut u8) {
        let data = data as *mut (*mut M, *mut Measurements<'b>);
        let (measure, measurements) = unsafe {
            let data = data.as_mut().unwrap();
            (data.0.as_mut().unwrap(), data.1.as_mut().unwrap())
        };
        measure.end(Phase::Execution, measurements);
        log::info!("Finished measuring execution");
    }
}

impl<'a, 'b, 'c, M> Drop for Engine<'a, 'b, 'c, M> {
    fn drop(&mut self) {
        unsafe {
            (self.bench_api.wasm_bench_free)(self.engine);
            drop(Box::from_raw(self.measurement_data));
        }
    }
}

/// A compiled module from a `BenchApi`.
pub struct Module<'a, 'b, 'c, M> {
    engine: Engine<'a, 'b, 'c, M>,
}

impl<'a, 'b, 'c, M> Module<'a, 'b, 'c, M> {
    /// Instantiate this module, returning the resulting `Instance`.
    pub fn instantiate(self) -> Instance<'a, 'b, 'c, M> {
        let result = unsafe { (self.engine.bench_api.wasm_bench_instantiate)(self.engine.engine) };
        assert_eq!(result, 0);
        Instance {
            engine: self.engine,
        }
    }
}

/// An instantiated module from our `BenchApi`.
pub struct Instance<'a, 'b, 'c, M> {
    engine: Engine<'a, 'b, 'c, M>,
}

impl<'a, 'b, 'c, M> Instance<'a, 'b, 'c, M> {
    /// Turn this instance back into a module.
    ///
    /// This allows you to instantiate a module multiple times without executing
    /// each instance.
    pub fn into_module(self) -> Module<'a, 'b, 'c, M> {
        Module {
            engine: self.engine,
        }
    }

    /// Execute this instance's code, returning the engine it was in.
    ///
    /// This instance is consumed, but you get its `Module` back, which can then
    /// be instantiated and executed again.
    pub fn execute(self) -> Module<'a, 'b, 'c, M> {
        let result = unsafe { (self.engine.bench_api.wasm_bench_execute)(self.engine.engine) };
        assert_eq!(result, 0);
        Module {
            engine: self.engine,
        }
    }
}
