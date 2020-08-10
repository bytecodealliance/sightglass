use std::collections::{BTreeMap, HashMap};

/// Describes the data model for a sequence of benchmark executions--a `History` and associated
/// sub-objects.

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct History {
    history: BTreeMap<Timestamp, Run>,
}

impl History {
    pub fn insert(&mut self, timestamp: Timestamp, run: Run) {
        self.history.insert(timestamp, run);
    }
}

/// Each entry in the `History` is indexed by its timestamp--when it was run--and details about the
/// run itself. The `Run` describes both the runtimes used for executing the benchmarks (see
/// `Metadata`) and the results themselves (see `BenchResults`).
pub type Timestamp = u64;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Run {
    pub meta: Metadata,
    results: BenchResults,
}

/// The `Metadata` serves to identify what code was run for each runtime (i.e. its Git reference)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// The time at which the benchmarks were run
    pub timestamp: Timestamp,
    suite: String,
    /// The server on which the benchmarks were run
    server: String,
    /// This identifies for the UI which runtime (of those listed in `runtimes`) is used as the
    /// reference runtime to compare against.
    reference_runtime: Runtime,
    runtimes: HashMap<Runtime, RuntimeMetadata>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RuntimeMetadata {
    /// A URL to the repository, e.g. https://github.com/CraneStation/wasmtime.
    repo: String,
    /// The Git branch used for building the runtime.
    branch: String,
    /// The Git commit
    commit: String,
    /// The Git commit author, message, and timestamp, for displaying these within the UI.
    author: String,
    message: String,
    timestamp: Timestamp,
}

/// `BenchResults` contain multiple `BenchResult`s (indexed by the benchmark name) which contain
/// multiple `Summary`s (indexed by runtime name).
pub type BenchResults = HashMap<Benchmark, BenchResult>;
pub type BenchResult = HashMap<Runtime, Summary>;
pub type Benchmark = String;
pub type Runtime = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    elapsed: Stat,
    cpu_cycles: Stat,
    instructions_retired: Stat,
    cache_accesses: Stat,
    cache_misses: Stat,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stat {
    mean: f64,
    median: f64,
    min: f64,
    max: f64,
    std_dev: f64,
}
