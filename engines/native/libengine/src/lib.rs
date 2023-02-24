//! A C API for benchmarking Wasmtime's WebAssembly compilation, instantiation,
//! and execution.
//!
//! The API expects calls that match the following state machine:
//!
//! ```text
//!               |
//!               |
//!               V
//! .---> wasm_bench_create
//! |        |        |
//! |        |        |
//! |        |        V
//! |        |   wasm_bench_compile
//! |        |     |            |
//! |        |     |            |     .----.
//! |        |     |            |     |    |
//! |        |     |            V     V    |
//! |        |     |     wasm_bench_instantiate <------.
//! |        |     |            |        |             |
//! |        |     |            |        |             |
//! |        |     |            |        |             |
//! |        |     |     .------'        '-----> wasm_bench_execute
//! |        |     |     |                             |
//! |        |     |     |                             |
//! |        V     V     V                             |
//! '------ wasm_bench_free <--------------------------'
//!               |
//!               |
//!               V
//! ```
//!
//! All API calls must happen on the same thread.
//!
//! Functions which return pointers use null as an error value. Function which
//! return `int` use `0` as OK and non-zero as an error value.
//!
//! # Example
//!
//! ```
//! use std::ptr;
//! use wasmtime_bench_api::*;
//!
//! let working_dir = std::env::current_dir().unwrap().display().to_string();
//! let stdout_path = "./stdout.log";
//! let stderr_path = "./stderr.log";
//!
//! // Functions to start/end timers for compilation.
//! //
//! // The `compilation_timer` pointer configured in the `WasmBenchConfig` is
//! // passed through.
//! extern "C" fn compilation_start(timer: *mut u8) {
//!     // Start your compilation timer here.
//! }
//! extern "C" fn compilation_end(timer: *mut u8) {
//!     // End your compilation timer here.
//! }
//!
//! // Similar for instantiation.
//! extern "C" fn instantiation_start(timer: *mut u8) {
//!     // Start your instantiation timer here.
//! }
//! extern "C" fn instantiation_end(timer: *mut u8) {
//!     // End your instantiation timer here.
//! }
//!
//! // Similar for execution.
//! extern "C" fn execution_start(timer: *mut u8) {
//!     // Start your execution timer here.
//! }
//! extern "C" fn execution_end(timer: *mut u8) {
//!     // End your execution timer here.
//! }
//!
//! let config = WasmBenchConfig {
//!     working_dir_ptr: working_dir.as_ptr(),
//!     working_dir_len: working_dir.len(),
//!     stdout_path_ptr: stdout_path.as_ptr(),
//!     stdout_path_len: stdout_path.len(),
//!     stderr_path_ptr: stderr_path.as_ptr(),
//!     stderr_path_len: stderr_path.len(),
//!     stdin_path_ptr: ptr::null(),
//!     stdin_path_len: 0,
//!     compilation_timer: ptr::null_mut(),
//!     compilation_start,
//!     compilation_end,
//!     instantiation_timer: ptr::null_mut(),
//!     instantiation_start,
//!     instantiation_end,
//!     execution_timer: ptr::null_mut(),
//!     execution_start,
//!     execution_end,
//!     execution_flags_ptr: ptr::null(),
//!     execution_flags_len: 0,
//! };
//!
//! let mut bench_api = ptr::null_mut();
//! unsafe {
//!     let code = wasm_bench_create(config, &mut bench_api);
//!     assert_eq!(code, OK);
//!     assert!(!bench_api.is_null());
//! };
//!
//! let wasm = wat::parse_bytes(br#"
//!     (module
//!         (func $bench_start (import "bench" "start"))
//!         (func $bench_end (import "bench" "end"))
//!         (func $start (export "_start")
//!             call $bench_start
//!             i32.const 1
//!             i32.const 2
//!             i32.add
//!             drop
//!             call $bench_end
//!         )
//!     )
//! "#).unwrap();
//!
//! // This will call the `compilation_{start,end}` timing functions on success.
//! let code = unsafe { wasm_bench_compile(bench_api, wasm.as_ptr(), wasm.len()) };
//! assert_eq!(code, OK);
//!
//! // This will call the `instantiation_{start,end}` timing functions on success.
//! let code = unsafe { wasm_bench_instantiate(bench_api) };
//! assert_eq!(code, OK);
//!
//! // This will call the `execution_{start,end}` timing functions on success.
//! let code = unsafe { wasm_bench_execute(bench_api) };
//! assert_eq!(code, OK);
//!
//! unsafe {
//!     wasm_bench_free(bench_api);
//! }
//! ```

mod unsafe_send_sync;

use crate::unsafe_send_sync::UnsafeSendSync;
use anyhow::{Context, Result};
use std::os::raw::{c_int, c_void};
use std::slice;
use std::{env, path::PathBuf};
use target_lexicon::Triple;

pub type ExitCode = c_int;
pub const OK: ExitCode = 0;
pub const ERR: ExitCode = -1;

/// Configuration options for the benchmark.
#[repr(C)]
pub struct WasmBenchConfig {
    /// The working directory where benchmarks should be executed.
    pub working_dir_ptr: *const u8,
    pub working_dir_len: usize,

    /// The file path that should be created and used as `stdout`.
    pub stdout_path_ptr: *const u8,
    pub stdout_path_len: usize,

    /// The file path that should be created and used as `stderr`.
    pub stderr_path_ptr: *const u8,
    pub stderr_path_len: usize,

    /// The (optional) file path that should be opened and used as `stdin`. If
    /// not provided, then the WASI context will not have a `stdin` initialized.
    pub stdin_path_ptr: *const u8,
    pub stdin_path_len: usize,

    /// The functions to start and stop performance timers/counters during Wasm
    /// compilation.
    pub compilation_timer: *mut u8,
    pub compilation_start: extern "C" fn(*mut u8),
    pub compilation_end: extern "C" fn(*mut u8),

    /// The functions to start and stop performance timers/counters during Wasm
    /// instantiation.
    pub instantiation_timer: *mut u8,
    pub instantiation_start: extern "C" fn(*mut u8),
    pub instantiation_end: extern "C" fn(*mut u8),

    /// The functions to start and stop performance timers/counters during Wasm
    /// execution.
    pub execution_timer: *mut u8,
    pub execution_start: extern "C" fn(*mut u8),
    pub execution_end: extern "C" fn(*mut u8),
}

impl WasmBenchConfig {
    fn working_dir(&self) -> Result<PathBuf> {
        let working_dir =
            unsafe { std::slice::from_raw_parts(self.working_dir_ptr, self.working_dir_len) };
        let working_dir = std::str::from_utf8(working_dir)
            .context("given working directory is not valid UTF-8")?;
        Ok(working_dir.into())
    }

    fn stdout_path(&self) -> Result<PathBuf> {
        let stdout_path =
            unsafe { std::slice::from_raw_parts(self.stdout_path_ptr, self.stdout_path_len) };
        let stdout_path =
            std::str::from_utf8(stdout_path).context("given stdout path is not valid UTF-8")?;
        Ok(stdout_path.into())
    }

    fn stderr_path(&self) -> Result<PathBuf> {
        let stderr_path =
            unsafe { std::slice::from_raw_parts(self.stderr_path_ptr, self.stderr_path_len) };
        let stderr_path =
            std::str::from_utf8(stderr_path).context("given stderr path is not valid UTF-8")?;
        Ok(stderr_path.into())
    }

    fn stdin_path(&self) -> Result<Option<PathBuf>> {
        if self.stdin_path_ptr.is_null() {
            return Ok(None);
        }

        let stdin_path =
            unsafe { std::slice::from_raw_parts(self.stdin_path_ptr, self.stdin_path_len) };
        let stdin_path =
            std::str::from_utf8(stdin_path).context("given stdin path is not valid UTF-8")?;
        Ok(Some(stdin_path.into()))
    }
}

/// Exposes a C-compatible way of creating the engine from the bytes of a single
/// Wasm module.
///
/// On success, the `out_bench_ptr` is initialized to a pointer to a structure
/// that contains the engine's initialized state, and `0` is returned. On
/// failure, a non-zero status code is returned and `out_bench_ptr` is left
/// untouched.
#[no_mangle]
pub extern "C" fn wasm_bench_create(
    config: WasmBenchConfig,
    out_bench_ptr: *mut *mut c_void,
) -> ExitCode {
    let result = (|| -> Result<_> {
        let working_dir = config.working_dir()?;
        println!("HIIIIIIIII");
        eprintln!("This is going to standard error!, {}", "awesome");
        /*
        let working_dir =
            cap_std::fs::Dir::open_ambient_dir(&working_dir, cap_std::ambient_authority())
                .with_context(|| {
                    format!(
                        "failed to preopen the working directory: {}",
                        working_dir.display(),
                    )
                })?;
*/
        let stdout_path = config.stdout_path()?;
        let stderr_path = config.stderr_path()?;
        let stdin_path = config.stdin_path()?;

        let state = Box::new(BenchState::new(
            config.compilation_timer,
            config.compilation_start,
            config.compilation_end,
            config.instantiation_timer,
            config.instantiation_start,
            config.instantiation_end,
            config.execution_timer,
            config.execution_start,
            config.execution_end,
        )?);
        println!("HOOOOOOOOO");
        Ok(Box::into_raw(state) as _)
    })();

    if let Ok(bench_ptr) = result {
        unsafe {
            assert!(!out_bench_ptr.is_null());
            *out_bench_ptr = bench_ptr;
        }
    }

    to_exit_code(result.map(|_| ()))
}

/// Free the engine state allocated by this library.
#[no_mangle]
pub extern "C" fn wasm_bench_free(state: *mut c_void) {
    assert!(!state.is_null());
    unsafe {
        drop(Box::from_raw(state as *mut BenchState));
    }
}

/// Compile the Wasm benchmark module.
#[no_mangle]
pub extern "C" fn wasm_bench_compile(
    state: *mut c_void,
    wasm_bytes: *const u8,
    wasm_bytes_length: usize,
) -> ExitCode {
    println!("HEEEEEEEEEEE");
    let state = unsafe { (state as *mut BenchState).as_mut().unwrap() };
    let wasm_bytes = unsafe { slice::from_raw_parts(wasm_bytes, wasm_bytes_length) };
    let result = state.compile(wasm_bytes).context("failed to compile");
    println!("HEEEEEEEAAAA");
    to_exit_code(result)
}

/// Instantiate the Wasm benchmark module.
#[no_mangle]
pub extern "C" fn wasm_bench_instantiate(state: *mut c_void) -> ExitCode {
    let state = unsafe { (state as *mut BenchState).as_mut().unwrap() };
    let result = state.instantiate().context("failed to instantiate");
    println!("HEEEEEEEBBBB");
    to_exit_code(result)
}

/// Execute the Wasm benchmark module.
#[no_mangle]
pub extern "C" fn wasm_bench_execute(state: *mut c_void) -> ExitCode {
    let state = unsafe { (state as *mut BenchState).as_mut().unwrap() };
    let result = state.execute().context("failed to execute");
    println!("HEEEEEEECCCC");
    to_exit_code(result)
}

/// Helper function for converting a Rust result to a C error code.
///
/// This will print an error indicating some information regarding the failure.
fn to_exit_code<T>(result: impl Into<Result<T>>) -> ExitCode {
    match result.into() {
        Ok(_) => OK,
        Err(error) => {
            eprintln!("{:?}", error);
            ERR
        }
    }
}

/// This structure contains the actual Rust implementation of the state required
/// to manage the Wasmtime engine between calls.
struct BenchState {
    compilation_timer: *mut u8,
    compilation_start: extern "C" fn(*mut u8),
    compilation_end: extern "C" fn(*mut u8),
    instantiation_timer: *mut u8,
    instantiation_start: extern "C" fn(*mut u8),
    instantiation_end: extern "C" fn(*mut u8),
    execution_timer: *mut u8,
    execution_start: extern "C" fn(*mut u8),
    execution_end: extern "C" fn(*mut u8),
    compiled_code_location: Option<PathBuf>,
}

static mut NATIVE_EXECUTION_TIMER: Option<*mut u8> = None;
static mut NATIVE_EXECUTION_START: Option<extern "C" fn(*mut u8)> = None;
static mut NATIVE_EXECUTION_END: Option<extern "C" fn(*mut u8)> = None;

#[no_mangle]
pub extern "C" fn bench_start() {
    println!("About to call native bench start");
    let timer = unsafe { NATIVE_EXECUTION_TIMER.unwrap() };
    unsafe { NATIVE_EXECUTION_START.unwrap()(timer) };
    println!("Done call native bench start.");
}

#[no_mangle]
pub extern "C" fn bench_end() {
    println!("About to call native bench end.");
    let timer = unsafe { NATIVE_EXECUTION_TIMER.unwrap() };
    unsafe { NATIVE_EXECUTION_END.unwrap()(timer) };
    println!("Done call native bench end");
}

impl BenchState {
    fn new(
        compilation_timer: *mut u8,
        compilation_start: extern "C" fn(*mut u8),
        compilation_end: extern "C" fn(*mut u8),
        instantiation_timer: *mut u8,
        instantiation_start: extern "C" fn(*mut u8),
        instantiation_end: extern "C" fn(*mut u8),
        execution_timer: *mut u8,
        execution_start: extern "C" fn(*mut u8),
        execution_end: extern "C" fn(*mut u8),
    ) -> Result<Self> {
        Ok(Self {
            compilation_timer,
            compilation_start,
            compilation_end,
            instantiation_timer,
            instantiation_start,
            instantiation_end,
            execution_timer,
            execution_start,
            execution_end,
            compiled_code_location: None,
        })
    }

    fn compile(&mut self, _bytes: &[u8]) -> Result<()> {
        (self.compilation_start)(self.compilation_timer);
         // TODO .. Compile the code
        // set self.compile code to some path.
        //self.compiled_code_location = PathBuf
        println!("HEEEEEEECCCCompile");
        self.compiled_code_location = Some(PathBuf::from("./benchmark.so"));
        // if clang compilation fails we should return an error
        // When we compile this ... link in the sightglass recorder
        //Command::new("${CC} -fPIC ${COMMON_CFLAGS} "-DWORKLOAD_LOCATION=${SCRIPT_LOC}/benchmark" -shared -o implementation.so ../wrapper.c -fPIC")
        (self.compilation_end)(self.compilation_timer);
        Ok(())
    }

    fn instantiate(&mut self) -> Result<()> {
        Ok(())
    }

    fn execute(&mut self) -> Result<()> {
        unsafe {
            NATIVE_EXECUTION_TIMER = Some(self.execution_timer);
            NATIVE_EXECUTION_START = Some(self.execution_start);
            NATIVE_EXECUTION_END = Some(self.execution_end);
        }

        let code_location_path = &self.compiled_code_location.as_ref().unwrap();
        log::info!(
            "Using native compiled code location: {}",
            code_location_path.display()
        );
        //let native_compiled_code_lib = unsafe { libloading::Library::new(&self.compiled_code_location.unwrap())? };
        // TODO: instead of doing new ... do an open and pass in RTL_NOW and RTL_GLOBAL
        let native_compiled_code_lib = unsafe { libloading::Library::new(code_location_path)? };
        //let native_entry: libloading::Symbol<'a, unsafe extern "C" fn() -> i32> =
        //    native_compiled_code_lib.get(b"native_entry");
        let native_entry: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
            unsafe { native_compiled_code_lib.get(b"native_entry").unwrap() };
        println!("BLohhhhhhh\n");
        let result = unsafe { (native_entry)() };
        println!("BLASSSSSSSSSSSSSSSSSSSSSSSSSSSSSS\n");
        println!("Result {:?}", result);
        Ok(())
        // TODO Error checking .. if result is not zero maybe?
        // Replace with a system call to run the executable at compiled code location
    }
}
