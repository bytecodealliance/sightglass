//! Track completion of benchmark iterations.
//!
//! In order to give users feedback about when Sightglass will finish running
//! benchmarks, we need to keep track of how many iterations will run. This is
//! complicated by the fact that benchmarks can be run both in a single process
//! (the current one) or across many processes. [`SingleProcess`] and
//! [`MultiProcess`] are completely separate implementations of counters in
//! order to account for this.
//!
//! Because a multi-process run delegates the running of benchmarks to a single
//! process, we need a way to communicate what iteration the single process
//! starts at. This is accomplished here by an internal environment variable
//! protocol:
//! - [`MultiProcess::to_environ_vars`] sets some environment variables with the
//!   `completed` and `total` number of iterations
//! - [`SingleProcess::new`] reads the environment to retrieve `completed` and
//!   `total`, which it uses when incrementing and printing its state to
//!   `stderr`.

use anyhow::Result;
use once_cell::sync::Lazy;
use std::env;
use std::ffi::OsString;

const ENV_COMPLETED: &str = "SIGHTGLASS_COUNTER_COMPLETED_ITERATIONS";
const ENV_TOTAL: &str = "SIGHTGLASS_COUNTER_TOTAL_ITERATIONS";

/// Calculate once whether the terminal has the ability to emit [escape
/// codes](https://en.wikipedia.org/wiki/ANSI_escape_code). If so, we can avoid
/// printing repeated `"Iterations completed: ..."` lines. This is loosely based
/// on this [StackOverflow
/// discussion](https://stackoverflow.com/questions/7445658).
static HAS_TERMINAL_ESCAPE_CODES: Lazy<bool> = Lazy::new(|| {
    let is_atty = atty::is(atty::Stream::Stderr);
    let is_supported_platform = if env::consts::OS == "windows" {
        env::var_os("TERM") == Some(OsString::from("ANSI"))
    } else {
        true
    };
    let is_rust_logging_enabled = env::var_os("RUST_LOG").is_some();

    is_atty && is_supported_platform && !is_rust_logging_enabled
});

/// Track iterations completed in a single process.
pub struct SingleProcess {
    completed: usize,
    total: usize,
}

impl SingleProcess {
    /// Construct the initial state by either reading the start state from the
    /// environment (i.e., part of a multi-process run) or using the known total iterations
    /// (i.e., single-process run).
    pub fn new(iterations_per_process: usize) -> Result<Self> {
        let total = if let Ok(total) = env::var(ENV_TOTAL) {
            total.parse()?
        } else {
            iterations_per_process
        };
        let completed = if let Ok(completed) = env::var(ENV_COMPLETED) {
            completed.parse()?
        } else {
            0
        };
        Ok(Self { completed, total })
    }
    pub fn is_empty(&self) -> bool {
        self.completed == 0
    }
    pub fn increment(&mut self) {
        self.completed += 1;
    }
    /// Print the current number of iterations completed to `stderr`: e.g.,
    /// `"Iterations completed: 5/10"`.
    ///
    /// If the terminal supports ANSI terminal escape codes, this will attempt
    /// to overwrite the current line, avoiding excessive output. See escape
    /// sequences for "escape", "cursor up", "carriage return"
    /// [here](https://en.wikipedia.org/wiki/ANSI_escape_code).
    ///
    /// __Important__: this function (due to `HAS_TERMINAL_ESCAPE_CODES`) has an
    /// implicit assumption that it will be used with `eprintln!` (e.g.).
    pub fn display_to_stderr(&self) -> String {
        let reset_line = if self.completed > 0 && *HAS_TERMINAL_ESCAPE_CODES {
            "\x1B[A\r"
        } else {
            ""
        };
        format!(
            "{}Iterations completed: {}/{}",
            reset_line, self.completed, self.total
        )
    }
}

/// Track iterations completed over a multi-process run.
pub struct MultiProcess {
    completed: usize,
    total: usize,
}

impl MultiProcess {
    pub fn new(total: usize) -> Self {
        Self {
            completed: 0,
            total,
        }
    }
    /// Communicate the internal counter state to a single-process run; this
    /// must match up with [`SingleProcess::new`].
    pub fn to_environ_vars(&self) -> impl IntoIterator<Item = (&str, String)> {
        vec![
            (ENV_COMPLETED, self.completed.to_string()),
            (ENV_TOTAL, self.total.to_string()),
        ]
    }
    pub fn increment_by(&mut self, iterations: usize) {
        self.completed += iterations;
        assert!(self.completed <= self.total)
    }
}
