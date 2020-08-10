use super::config::*;
use super::errors::*;
use super::out;
use super::perf::PerfCounterCollection;
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
    pub summary: SummarySet,
}

/// The outcome of a test
#[derive(Clone)]
struct TestResult {
    name: String,
    grand_summary: SummarySet,
    bodies_summary: Vec<TestBodySummary>,
}

/// The outcome of a test, without the name of the test
pub struct AnonymousTestResult {
    pub grand_summary: SummarySet,
    pub bodies_summary: Vec<TestBodySummary>,
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
    ctx: TestCtx,
    bodies: Vec<unsafe extern "C" fn(TestCtx)>,
}

#[derive(Default, Debug, Clone)]
pub struct Samples<T>(Vec<T>);

impl<T> Samples<T> {
    pub fn empty() -> Self {
        Samples(vec![])
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

pub struct AdaptiveRunner<'a> {
    round_size: usize,
    min_sample_size: usize,
    min_run_time_ms: u64,
    max_run_time_ms: u64,
    counters: &'a mut PerfCounterCollection,
}

/// A recording of time and performance counter information. `Sample::default()` provides a useful
/// zero to accumulate into.
#[derive(Clone, Copy, Default)]
pub struct Sample {
    /// The wall clock time that passed while measuring this sample.
    pub clock_time: Elapsed,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cpu_cycles: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub instructions_retired: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cache_accesses: u64,
    /// Measured by performance counter. May be 0, in which case the counter is almost certainly
    /// disabled.
    pub cache_misses: u64,
}

impl std::ops::Div<u64> for Sample {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Sample {
            clock_time: Elapsed::from_ticks(self.clock_time.ticks() / rhs),
            cpu_cycles: self.cpu_cycles / rhs,
            instructions_retired: self.instructions_retired / rhs,
            cache_accesses: self.cache_accesses / rhs,
            cache_misses: self.cache_misses / rhs,
        }
    }
}

impl std::ops::Add<Sample> for Sample {
    type Output = Self;

    fn add(self, rhs: Sample) -> Self::Output {
        Sample {
            clock_time: self.clock_time + rhs.clock_time,
            cpu_cycles: self.cpu_cycles + rhs.cpu_cycles,
            instructions_retired: self.instructions_retired + rhs.instructions_retired,
            cache_accesses: self.cache_accesses + rhs.cache_accesses,
            cache_misses: self.cache_misses + rhs.cache_misses,
        }
    }
}

impl std::ops::AddAssign<Sample> for Sample {
    fn add_assign(&mut self, rhs: Sample) {
        self.clock_time += rhs.clock_time;
        self.cpu_cycles += rhs.cpu_cycles;
        self.instructions_retired += rhs.instructions_retired;
        self.cache_accesses += rhs.cache_accesses;
        self.cache_misses += rhs.cache_misses;
    }
}

#[derive(Clone)]
pub struct RunnerResult {
    pub summaries: Vec<SummarySet>,
    pub grand_summary: SummarySet,
}

// A collection of summaries
#[derive(Clone)]
pub struct SummarySet {
    pub elapsed: Summary,
    pub cpu_cycles: Summary,
    pub instructions_retired: Summary,
    pub cache_accesses: Summary,
    pub cache_misses: Summary,
}

impl SummarySet {
    pub fn new(samples: Vec<Sample>, precision: &Precision) -> Self {
        let mut transformed_samples: [Vec<f64>; 5] = [
            Vec::with_capacity(samples.len()),
            Vec::with_capacity(samples.len()),
            Vec::with_capacity(samples.len()),
            Vec::with_capacity(samples.len()),
            Vec::with_capacity(samples.len()),
        ];
        for sample in samples.into_iter() {
            transformed_samples[0].push(sample.clock_time.as_ns(&precision) as f64);
            transformed_samples[1].push(sample.cpu_cycles as f64);
            transformed_samples[2].push(sample.instructions_retired as f64);
            transformed_samples[3].push(sample.cache_accesses as f64);
            transformed_samples[4].push(sample.cache_misses as f64);
        }
        SummarySet {
            elapsed: Summary::new(&transformed_samples[0]),
            cpu_cycles: Summary::new(&transformed_samples[1]),
            instructions_retired: Summary::new(&transformed_samples[2]),
            cache_accesses: Summary::new(&transformed_samples[3]),
            cache_misses: Summary::new(&transformed_samples[4]),
        }
    }
}

impl<'a> AdaptiveRunner<'a> {
    pub fn new(
        initial_round_size: usize,
        min_sample_size: usize,
        min_run_time_ms: u64,
        max_run_time_ms: u64,
        counters: &'a mut PerfCounterCollection,
    ) -> Self {
        AdaptiveRunner {
            round_size: initial_round_size,
            min_sample_size,
            min_run_time_ms,
            max_run_time_ms,
            counters,
        }
    }

    pub fn bench<Target, Ret>(&mut self, target: &mut Target) -> RunnerResult
    where
        Target: Runnable<Ret>,
    {
        let mut sample_for_all_runs: Samples<Sample> = Samples::empty();
        let mut samples: Vec<Samples<Sample>> = vec![];
        let bodies = target.bodies();
        samples.resize(bodies.len(), Samples::empty());
        let mut round_size = self.round_size;
        let ts_bench_start = self.counters.now();
        let mut sample_id = 0;
        loop {
            let mut bodies_sample_vec: Vec<Sample> = Vec::with_capacity(bodies.len());
            bodies_sample_vec.resize_with(bodies.len(), || Sample::default());
            for _ in 0..round_size {
                for (body_id, body) in bodies.iter().enumerate() {
                    bodies_sample_vec[body_id] = self.counters.sample(|| {
                        body(body_id);
                    });
                }
            }
            let mut sample_for_all_bodies = Sample::default();
            for (body_id, sample) in bodies_sample_vec.into_iter().enumerate() {
                samples[body_id].0.push(sample / round_size as u64);
                sample_for_all_bodies += sample;
            }
            sample_for_all_runs
                .0
                .push(sample_for_all_bodies / round_size as u64);

            let elapsed_total = (self.counters.precision.now() - ts_bench_start)
                .as_millis(&self.counters.precision);
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
        let summaries: Vec<SummarySet> = samples
            .into_iter()
            .map(|sample| SummarySet::new(sample.0, &self.counters.precision))
            .collect();
        let grand_summary = SummarySet::new(sample_for_all_runs.0, &self.counters.precision);
        RunnerResult {
            summaries,
            grand_summary,
        }
    }
}

/// Run an individual test
fn run_test(
    config: &Config,
    counters: &mut PerfCounterCollection,
    global_ctx: TestCtx,
    test: &Test,
) -> Result<TestResult, BenchError> {
    let mut ctx: TestCtx = ptr::null_mut();
    if let Some(setup) = (*test).setup_fn {
        unsafe { setup(global_ctx, &mut ctx) }
    }

    let mut bench_runner = AdaptiveRunner::new(
        config
            .initial_round_size
            .unwrap_or(DEFAULT_INITIAL_ROUND_SIZE),
        config.min_sample_size.unwrap_or(DEFAULT_MIN_SAMPLE_SIZE),
        config.min_run_time_ms.unwrap_or(DEFAULT_MIN_RUN_TIME_MS),
        config.max_run_time_ms.unwrap_or(DEFAULT_MAX_RUN_TIME_MS),
        counters,
    );
    let mut test_bodies_bench = TestBodiesBench {
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

    let mut counters = PerfCounterCollection::new(config.use_perf_counters())?;

    for test in tests {
        eprintln!("  - {}", test.name);
        let test_result = run_test(config, &mut counters, global_ctx, &test)?;
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
