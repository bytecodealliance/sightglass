use crate::measure::{Measure, Measurements};
use anyhow::Result;
use log::info;
use sightglass_data::Phase;
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
    pub fn compile(
        self,
        wasm: &[u8],
        measure: &mut impl Measure,
        measurements: &mut Measurements,
    ) -> Module<'a, 'b> {
        measure.start();
        let result =
            unsafe { (self.bench_api.wasm_bench_compile)(self.engine, wasm.as_ptr(), wasm.len()) };
        measure.end(Phase::Compilation, measurements);
        assert_eq!(result, 0);
        Module { engine: self }
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
    pub fn instantiate(
        self,
        measure: &mut impl Measure,
        measurements: &mut Measurements,
    ) -> Instance<'a, 'b> {
        measure.start();
        let result = unsafe {
            (self.engine.bench_api.wasm_bench_instantiate)(
                self.engine.engine,
                static_measure::bench_start,
                static_measure::bench_end,
            )
        };
        measure.end(Phase::Instantiation, measurements);
        assert_eq!(result, 0);
        Instance {
            engine: self.engine,
        }
    }
}

/// An instantiated module from our `BenchApi`.
pub struct Instance<'a, 'b> {
    engine: Engine<'a, 'b>,
}

impl<'a, 'b> Instance<'a, 'b> {
    /// Execute this instance's code, returning the engine it was in.
    pub fn execute(self, measure: &mut impl Measure, measurements: &mut Measurements) {
        static_measure::with(measure, measurements, || {
            let result = unsafe { (self.engine.bench_api.wasm_bench_execute)(self.engine.engine) };
            assert_eq!(result, 0);
        });
    }
}

/// Measure various phases of a Wasm module's lifetime. This relies on `engine_lib_path`, the path
/// to a shared library that implements `wasm_bench_*` functions that allow for measurement of each
/// phase.
pub fn benchmark(
    bench_api: &mut BenchApi,
    wasm_bytes: &[u8],
    measure: &mut impl Measure,
    measurements: &mut Measurements,
) -> Result<()> {
    let engine = Engine::new(bench_api);

    // Measure the module compilation.
    let module = engine.compile(wasm_bytes, measure, measurements);
    info!("Compiled successfully");

    // Measure the module instantiation.
    let instance = module.instantiate(measure, measurements);
    info!("Instantiated successfully");

    // Measure the module execution; note that, because bench_start and bench_end are passed in to the Wasm module
    // as imports, we must retain the measurement state here, in the host code--see `static_measure`.
    instance.execute(measure, measurements);
    info!("Executed successfully");

    Ok(())
}

/// Because this recorder cedes control to the Wasm module during execution,
/// giving it the responsibility for calling `bench_start` and `bench_end` at
/// the appropriate times, we need to maintain any recording state here. This
/// Rust module does so using atomic pointers and internal unsafe code. A safe
/// interface is provided via the `with` function.
mod static_measure {
    use super::*;
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};

    static STATE: AtomicPtr<MeasureState> = AtomicPtr::new(ptr::null_mut());

    /// Call `f` with the static measurement state used by `bench_{start,end}`
    /// initialized to the given measure and measurements pair.
    ///
    /// # Panics
    ///
    /// Panics if the static measurment state is already initialized, which
    /// could only happen if this function were called on multiple threads
    /// concurrently, which is not allowed.
    pub(crate) fn with<T>(
        measure: &mut dyn Measure,
        measurements: &mut Measurements,
        f: impl FnOnce() -> T + std::panic::UnwindSafe,
    ) -> T {
        let measure: *mut dyn Measure = measure as _;
        let measure = ptr::NonNull::new(measure).unwrap();
        let measurements = ptr::NonNull::new(measurements).unwrap();
        let measurements = measurements.cast();

        let state = &mut MeasureState {
            measure,
            measurements,
        };

        let old_state = STATE.swap(state, Ordering::SeqCst);
        assert!(old_state.is_null());

        let result = std::panic::catch_unwind(f);

        let state_ptr = STATE.swap(ptr::null_mut(), Ordering::SeqCst);
        assert_eq!(state_ptr, state as *mut _);

        match result {
            Ok(x) => x,
            Err(e) => std::panic::resume_unwind(e),
        }
    }

    #[derive(Debug)]
    struct MeasureState {
        measure: ptr::NonNull<dyn Measure>,
        measurements: ptr::NonNull<Measurements<'static>>,
    }

    pub(crate) extern "C" fn bench_start() {
        info!("bench_start was called");
        unsafe {
            let state = STATE.load(Ordering::SeqCst);
            let state = state.as_mut().unwrap();
            state.measure.as_mut().start();
        }
    }

    pub(crate) extern "C" fn bench_end() {
        info!("bench_end was called");
        unsafe {
            let state = STATE.load(Ordering::SeqCst);
            let state = state.as_mut().unwrap();
            state
                .measure
                .as_mut()
                .end(Phase::Execution, state.measurements.as_mut());
        }
    }
}
