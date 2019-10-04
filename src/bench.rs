use super::config::*;
use super::errors::*;
use super::out;
use super::symbols;
use bencher::stats::Summary;
use libc::c_void;
#[cfg(unix)]
use libc::{RTLD_GLOBAL, RTLD_LAZY, RTLD_LOCAL, RTLD_NOW};
#[cfg(unix)]
use libloading;
use libloading::{Library, Symbol};
use precision::{self, Elapsed, Precision};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::ptr;

const TEST_ABI_VERSION: u64 = 0x01;
const TEST_LIBRARIES_TABLE_SYMBOL: &[u8] = b"tests_config";

pub type TestCtx = *mut c_void;
pub type TestSetupFn = unsafe extern "C" fn(TestCtx, *mut TestCtx);
pub type TestBodyFn = unsafe extern "C" fn(TestCtx);
pub type TestTeardownFn = unsafe extern "C" fn(TestCtx);

/// C structure to set up a global context for the test suite
#[repr(C)]
struct TestsConfig {
    global_setup: Option<unsafe extern "C" fn(*mut TestCtx)>,
    global_teardown: Option<unsafe extern "C" fn(TestCtx)>,
    version: u64,
}

/// A named test body function
#[derive(Clone, Debug)]
pub struct TestBody {
    pub name: String,
    pub body_fn: TestBodyFn,
}

/// An individual test, with function pointers for each step
#[derive(Clone, Debug)]
pub struct Test {
    pub name: String,
    pub setup_fn: Option<TestSetupFn>,
    pub bodies: Vec<TestBody>,
    pub teardown_fn: Option<TestTeardownFn>,
}

/// Measurements for a "body" of a test
#[derive(Clone)]
pub struct TestBodySummary {
    pub name: String,
    pub summary: Summary,
}

/// The outcome of a test
#[derive(Clone)]
struct TestResult {
    name: String,
    grand_summary: Summary,
    bodies_summary: Vec<TestBodySummary>,
}

/// The outcome of a test, without the name of the test
pub struct AnonymousTestResult {
    pub grand_summary: Summary,
    pub bodies_summary: Vec<TestBodySummary>,
}

impl Default for AnonymousTestResult {
    fn default() -> Self {
        Self {
            grand_summary: Summary::new(&[0.0]),
            bodies_summary: vec![],
        }
    }
}

impl From<TestResult> for AnonymousTestResult {
    fn from(test_result: TestResult) -> Self {
        AnonymousTestResult {
            grand_summary: test_result.grand_summary,
            bodies_summary: test_result.bodies_summary,
        }
    }
}

/// Environment for a single test
#[derive(Clone)]
struct TestBodiesBench {
    precision: Precision,
    ctx: TestCtx,
    bodies: Vec<unsafe extern "C" fn(TestCtx)>,
}

#[derive(Default, Debug, Clone)]
pub struct Sample<T>(Vec<T>);

impl<T> Sample<T> {
    pub fn empty() -> Self {
        Sample(vec![])
    }
}

pub trait Runnable<Ret> {
    fn setup(&mut self) {}
    fn teardown(&mut self) {}
    fn bodies<'t>(&'t mut self) -> Vec<Box<dyn Fn(usize) -> Ret + 't>>;
}

impl TestBodiesBench {
    #[inline]
    fn body(&self, body_id: usize) {
        unsafe { (self.bodies[body_id])(self.ctx) }
    }
}

impl Runnable<()> for TestBodiesBench {
    fn bodies<'t>(&'t mut self) -> Vec<Box<dyn Fn(usize) -> () + 't>> {
        let mut fns: Vec<Box<dyn Fn(usize) -> () + 't>> = vec![];
        for _ in 0..self.bodies.len() {
            let this = self.clone();
            fns.push(Box::new(move |body_id| this.body(body_id)))
        }
        fns
    }
}

pub struct AdaptiveRunner {
    round_size: usize,
    min_sample_size: usize,
    min_run_time_ms: u64,
    max_run_time_ms: u64,
    precision: Precision,
}

#[derive(Clone)]
pub struct RunnerResult {
    pub summaries: Vec<Summary>,
    pub grand_summary: Summary,
}

impl AdaptiveRunner {
    pub fn new(
        initial_round_size: usize,
        min_sample_size: usize,
        min_run_time_ms: u64,
        max_run_time_ms: u64,
        precision: &Precision,
    ) -> Self {
        AdaptiveRunner {
            round_size: initial_round_size,
            min_sample_size,
            min_run_time_ms,
            max_run_time_ms,
            precision: precision.clone(),
        }
    }

    pub fn bench<Target, Ret>(&self, target: &mut Target) -> RunnerResult
    where
        Target: Runnable<Ret>,
    {
        let mut sample_for_all_bodies: Sample<Elapsed> = Sample::empty();
        let mut samples: Vec<Sample<Elapsed>> = vec![];
        let bodies = target.bodies();
        samples.resize(bodies.len(), Sample::empty());
        let mut round_size = self.round_size;
        let ts_bench_start = self.precision.now();
        let mut sample_id = 0;
        loop {
            let mut elapsed_vec: Vec<Elapsed> = vec![];
            elapsed_vec.resize(bodies.len(), Elapsed::new());
            for _ in 0..round_size {
                for (body_id, body) in bodies.iter().enumerate() {
                    let ts_start = self.precision.now();
                    body(body_id);
                    let ts_end = self.precision.now();
                    elapsed_vec[body_id] += ts_end - ts_start;
                }
            }
            let mut elapsed_for_all_bodies = Elapsed::new();
            for (body_id, elapsed) in elapsed_vec.into_iter().enumerate() {
                samples[body_id]
                    .0
                    .push(Elapsed::from_ticks(elapsed.ticks() / round_size as u64));
                elapsed_for_all_bodies += elapsed;
            }
            sample_for_all_bodies.0.push(Elapsed::from_ticks(
                elapsed_for_all_bodies.ticks() / round_size as u64,
            ));

            let elapsed_total = (self.precision.now() - ts_bench_start).as_millis(&self.precision);
            if elapsed_total < self.min_run_time_ms {
                round_size = round_size.saturating_add(round_size);
                continue;
            }
            if elapsed_total > self.max_run_time_ms {
                break;
            }
            sample_id += 1;
            if sample_id >= self.min_sample_size {
                break;
            }
        }
        let summaries: Vec<_> = samples
            .into_iter()
            .map(|sample| {
                Summary::new(
                    sample
                        .0
                        .into_iter()
                        .map(|elapsed| elapsed.as_ns(&self.precision) as f64)
                        .collect::<Vec<f64>>()
                        .as_slice(),
                )
            })
            .collect();
        let grand_summary = Summary::new(
            sample_for_all_bodies
                .0
                .into_iter()
                .map(|elapsed| elapsed.as_ns(&self.precision) as f64)
                .collect::<Vec<f64>>()
                .as_slice(),
        );
        RunnerResult {
            summaries,
            grand_summary,
        }
    }
}

/// Run an individual test
fn run_test(
    config: &Config,
    precision: &Precision,
    global_ctx: TestCtx,
    test: &Test,
) -> Result<TestResult, BenchError> {
    let mut ctx: TestCtx = ptr::null_mut();
    if let Some(setup) = (*test).setup_fn {
        unsafe { setup(global_ctx, &mut ctx) }
    }

    let bench_runner = AdaptiveRunner::new(
        config
            .initial_round_size
            .unwrap_or(DEFAULT_INITIAL_ROUND_SIZE),
        config.min_sample_size.unwrap_or(DEFAULT_MIN_SAMPLE_SIZE),
        config.min_run_time_ms.unwrap_or(DEFAULT_MIN_RUN_TIME_MS),
        config.max_run_time_ms.unwrap_or(DEFAULT_MAX_RUN_TIME_MS),
        precision,
    );
    let mut test_bodies_bench = TestBodiesBench {
        precision: precision.clone(),
        ctx,
        bodies: (*test)
            .bodies
            .clone()
            .iter()
            .map(|body| body.body_fn)
            .collect(),
    };
    let bench_result = bench_runner.bench(&mut test_bodies_bench);
    let mut bodies_summary = vec![];
    for (body_id, body) in (*test).bodies.iter().enumerate() {
        let test_body_summary = TestBodySummary {
            name: body.name.clone(),
            summary: bench_result.summaries[body_id].clone(),
        };
        bodies_summary.push(test_body_summary);
    }

    unsafe { (*test).teardown_fn.map(|teardown_fn| teardown_fn(ctx)) };

    let grand_summary = bench_result.grand_summary;
    let name = test.name.clone();
    Ok(TestResult {
        name,
        grand_summary,
        bodies_summary,
    })
}

/// Run a sequence of tests
fn run_tests(
    config: &Config,
    global_ctx: TestCtx,
    tests: Vec<Test>,
) -> Result<Vec<TestResult>, BenchError> {
    let mut test_results: Vec<TestResult> = vec![];
    let precision = Precision::new(precision::Config::default())?;
    for test in tests {
        eprintln!("  - {}", test.name);
        let test_result = run_test(config, &precision, global_ctx, &test)?;
        test_results.push(test_result);
    }
    Ok(test_results)
}

#[cfg(unix)]
fn load_library<P: AsRef<OsStr>>(
    library_path: P,
    rtld_lazy: bool,
    rtld_global: bool,
) -> Result<Library, BenchError> {
    let mut flags = 0;
    if rtld_lazy {
        flags |= RTLD_LAZY;
    } else {
        flags |= RTLD_NOW;
    }
    if rtld_global {
        flags |= RTLD_GLOBAL;
    } else {
        flags |= RTLD_LOCAL;
    }
    let library = libloading::os::unix::Library::open(Some(library_path), flags)?.into();
    Ok(library)
}

#[cfg(not(unix))]
fn load_library<P: AsRef<OsStr>>(
    library_path: P,
    _rtld_lazy: bool,
    _rtld_global: bool,
) -> Result<Library, BenchError> {
    Ok(Library::new(library_path)?)
}

/// Bench all functions contained in a shared library
fn bench_library(config: &Config, library_path: &Path) -> Result<Vec<TestResult>, BenchError> {
    let tests_symbols = symbols::extract_tests_symbols(library_path)?;
    let library = load_library(library_path, false, true)?;
    let tests_runner: Symbol<'_, &TestsConfig> =
        unsafe { library.get(TEST_LIBRARIES_TABLE_SYMBOL) }.map_err(BenchError::from)?;
    if tests_runner.version != TEST_ABI_VERSION {
        xbail!(BenchError::ABIError("Incompatible ABI version"));
    }
    let tests = symbols::resolve(&tests_symbols, &library);
    let mut global_ctx: TestCtx = ptr::null_mut();
    if let Some(global_setup) = tests_runner.global_setup {
        unsafe { global_setup(&mut global_ctx) }
    }
    let test_results = run_tests(config, global_ctx, tests)?;
    if let Some(global_teardown) = tests_runner.global_teardown {
        unsafe { global_teardown(global_ctx) }
    }
    Ok(test_results)
}

/// Run an optional guard command
/// Returns `false` on success (return code = `0`), `true` on failure
fn disabled_due_to_guard(guard: &[String]) -> bool {
    match Command::new(&guard[0]).args(&guard[1..]).status() {
        Err(e) => {
            eprintln!(
                "Cannot run the [{}] guard script: [{}]",
                &guard[0],
                e.to_string()
            );
            true
        }
        Ok(status) => !status.success(),
    }
}

/// Entry point to run benchmarks according to a given configuration
pub fn bench(config: &Config) -> Result<(), BenchError> {
    let mut test_suites_results: HashMap<String, HashMap<String, AnonymousTestResult>> =
        HashMap::new();
    for test_suite in &config.test_suites {
        if let Some(guard) = &test_suite.guard {
            if !guard.is_empty() && disabled_due_to_guard(guard) {
                continue;
            }
        }
        eprintln!("{}:", test_suite.name);
        let library_path = &test_suite.library_path;
        let test_results = bench_library(&config, Path::new(library_path))?;
        for test_result in test_results {
            let test_name_key = test_result.name.clone();
            let anonymous_test_result = test_result.into();
            if !test_suites_results.contains_key(&test_name_key) {
                test_suites_results.insert(test_name_key.clone(), HashMap::new());
            }
            let results_for_test_name = test_suites_results.get_mut(&test_name_key).unwrap();
            results_for_test_name.insert(test_suite.name.clone(), anonymous_test_result);
        }
    }
    out::Out::new(test_suites_results).out_vec(&config.output)
}
