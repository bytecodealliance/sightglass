//! Measure each Sightglass phase using Valgrind's Callgrind. When this [Measure] is selected,
//! `sightglass-cli` launches each benchmark process under `valgrind --tool=callgrind` itself (see
//! [`command_prefix`]); collection is toggled on at the start of each phase and off at the end,
//! and the statistics for each phase are dumped to a separate file annotated with the phase name
//! (visible as the "trigger" in the dump's header):
//!
//! ```text
//! sightglass-cli benchmark <path to>/benchmark.wasm \
//!     --measure callgrind --processes 1 --iterations-per-process 1
//! ```
//!
//! Each phase's event totals (e.g., `callgrind-ir` for instructions executed) are parsed from the
//! dump files and recorded as measurements. The dump files themselves are left in place for deeper
//! inspection with, e.g., KCachegrind.
//!
//! Running `sightglass-cli` under Valgrind manually is also supported (the launcher detects this
//! and does not wrap again), in which case Callgrind's default output file naming is required to
//! locate the dumps, so do not pass `--callgrind-out-file`, and use `--processes 1` (Valgrind does
//! not follow child processes by default).
//!
//! Callgrind's instrumentation slows execution down considerably, but the instruction counts it
//! measures are deterministic, so a single process and iteration is usually sufficient.

use super::{Measure, Measurements};
use anyhow::{anyhow, ensure, Context, Result};
use sightglass_data::Phase;
use std::ffi::{CString, OsString};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use valgrind_requests::{callgrind, valgrind};

/// The environment variable through which `sightglass-cli` tells a benchmark process it launched
/// under Valgrind which `--callgrind-out-file` base path the dumps will be written to.
pub const OUT_FILE_ENV: &str = "SIGHTGLASS_CALLGRIND_OUT_FILE";

/// The number of dumps this process has written so far. Callgrind numbers a process' dump files
/// with sequential part suffixes starting at `1`, regardless of which `CallgrindMeasure` instance
/// requested the dump, so this counter must be process-global.
static PARTS: AtomicU32 = AtomicU32::new(0);

/// Was this binary built with Valgrind client-request support (i.e., were the Valgrind headers
/// found at build time)? If so, is this process running under Valgrind?
///
/// `valgrind-requests` compiles its client requests to runtime aborts when the Valgrind headers
/// are not found at build time, so probe it once under a panic guard (with the panic output
/// suppressed) rather than crashing the harness.
pub fn valgrind_support() -> Option<bool> {
    static SUPPORT: std::sync::OnceLock<Option<bool>> = std::sync::OnceLock::new();
    *SUPPORT.get_or_init(|| {
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(|| valgrind::running_on_valgrind() > 0).ok();
        std::panic::set_hook(previous_hook);
        result
    })
}

/// Is this process running under Valgrind?
pub fn running_under_valgrind() -> bool {
    valgrind_support() == Some(true)
}

/// Prepare to launch the `spawn_index`th benchmark subprocess of a run under Callgrind, which
/// will write `expected_dumps` per-phase dump files: choose the dump file base path, remove any
/// stale dump files a previous run could have left at that path (reachable when the OS recycles
/// this process id), and return the command prefix and environment variables with which to spawn
/// the subprocess.
pub fn process_wrapper(spawn_index: usize, expected_dumps: u32) -> Result<super::ProcessWrapper> {
    let out_file = PathBuf::from(format!(
        "callgrind.out.{}-{spawn_index}",
        std::process::id()
    ));
    remove_stale_dumps(&out_file, expected_dumps)?;
    Ok((
        command_prefix(&out_file),
        vec![(OUT_FILE_ENV, out_file.into())],
    ))
}

/// The command prefix with which to launch a benchmark process under Callgrind, writing its dumps
/// to `out_file` (and `out_file.<n>`, one per phase measurement).
fn command_prefix(out_file: &Path) -> Vec<OsString> {
    let mut out_file_flag = OsString::from("--callgrind-out-file=");
    out_file_flag.push(out_file);
    vec![
        "valgrind".into(),
        "--quiet".into(),
        "--tool=callgrind".into(),
        // Callgrind only collects between this measure's start/end toggles.
        "--collect-atstart=no".into(),
        // Without cache and branch simulation, Callgrind just counts instructions, which is both
        // faster and deterministic.
        "--cache-sim=no".into(),
        "--branch-sim=no".into(),
        out_file_flag,
        "--".into(),
    ]
}

/// The dump file Callgrind writes for the `part`th `DUMP_STATS` of a process writing to the
/// `--callgrind-out-file` base path `base`.
fn dump_file(base: &Path, part: u32) -> PathBuf {
    let mut path = base.to_path_buf().into_os_string();
    path.push(format!(".{part}"));
    path.into()
}

/// Remove the dump files a previous run could have written to the `out_file` base path: the base
/// file (Callgrind's final dump at process exit) plus the `expected_dumps` per-phase part files.
fn remove_stale_dumps(out_file: &Path, expected_dumps: u32) -> Result<()> {
    let paths = std::iter::once(out_file.to_path_buf())
        .chain((1..=expected_dumps).map(|part| dump_file(out_file, part)));
    for path in paths {
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).context(format!(
                    "unable to remove the stale dump file `{}`",
                    path.display()
                ))
            }
        }
    }
    Ok(())
}

pub struct CallgrindMeasure {
    /// Whether this process is running under Valgrind; the client requests are no-ops (and no dump
    /// files are written) otherwise.
    under_valgrind: bool,
    /// The `--callgrind-out-file` base path communicated by the parent `sightglass-cli` process
    /// via [OUT_FILE_ENV], if any; otherwise dumps are located by Callgrind's default
    /// `callgrind.out.<pid>` naming.
    out_file: Option<PathBuf>,
}

impl Default for CallgrindMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl CallgrindMeasure {
    pub fn new() -> Self {
        assert!(
            valgrind_support().is_some(),
            "this binary was built without Valgrind support (the Valgrind headers were not found \
             at build time); install Valgrind and rebuild"
        );
        let under_valgrind = running_under_valgrind();
        let out_file = std::env::var_os(OUT_FILE_ENV).map(PathBuf::from);
        assert!(
            out_file.is_none() || under_valgrind,
            "sightglass-cli launched this benchmark process under Valgrind ({OUT_FILE_ENV} is \
             set) but the process is not running under Valgrind"
        );
        if !under_valgrind {
            log::warn!(
                "the callgrind measure is enabled but this process is not running under Valgrind, \
                 so no instruction counts will be recorded"
            );
        }
        Self {
            under_valgrind,
            out_file,
        }
    }

    /// The dump file Callgrind writes for the `part`th `DUMP_STATS` of this process.
    fn dump_path(&self, part: u32) -> PathBuf {
        match &self.out_file {
            Some(base) => dump_file(base, part),
            None => PathBuf::from(format!("callgrind.out.{}.{}", std::process::id(), part)),
        }
    }
}

impl Measure for CallgrindMeasure {
    fn start(&mut self, _phase: Phase) {
        callgrind::zero_stats();
        callgrind::toggle_collect();
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        callgrind::toggle_collect();
        let reason = CString::new(phase.to_string()).expect("phase names do not contain nul bytes");
        callgrind::dump_stats_at(reason);

        if !self.under_valgrind {
            return;
        }

        // `DUMP_STATS` is synchronous, so the dump file is complete once `dump_stats_at` returns.
        let part = PARTS.fetch_add(1, Ordering::SeqCst) + 1;
        let path = self.dump_path(part);
        // Per the `Measure` contract, a broken measurement mechanism is a panic, not a silently
        // emptier result set.
        let events = parse_dump_summary(&path).unwrap_or_else(|error| {
            panic!(
                "unable to record callgrind measurements from the dump file `{}`{}: {:#}",
                path.display(),
                if self.out_file.is_none() {
                    " (was `--callgrind-out-file` or another dump-affecting option passed to \
                     valgrind?)"
                } else {
                    ""
                },
                error
            )
        });
        for (event, count) in events {
            measurements.add(
                phase,
                format!("callgrind-{}", event.to_lowercase()).into(),
                count,
            );
        }
    }
}

/// Parse the `summary:` totals of a Callgrind dump file, returning each recorded event (named by
/// the dump's `events:` header, e.g. `Ir`) and its total count.
fn parse_dump_summary(path: &Path) -> Result<Vec<(String, u64)>> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("unable to read `{}`", path.display()))?;
    parse_summary(&contents)
}

fn parse_summary(contents: &str) -> Result<Vec<(String, u64)>> {
    let mut events: Option<Vec<&str>> = None;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("events:") {
            events = Some(rest.split_whitespace().collect());
        } else if let Some(rest) = line.strip_prefix("summary:") {
            let events = events.ok_or_else(|| anyhow!("missing an `events:` header line"))?;
            let counts = rest
                .split_whitespace()
                .map(|v| v.parse::<u64>().context("invalid `summary:` count"))
                .collect::<Result<Vec<_>>>()?;
            ensure!(
                events.len() == counts.len(),
                "the `events:` header names {} events but the `summary:` line has {} counts",
                events.len(),
                counts.len()
            );
            return Ok(events.into_iter().map(str::to_string).zip(counts).collect());
        }
    }
    Err(anyhow!("missing a `summary:` line"))
}

#[cfg(test)]
mod tests {
    use super::parse_summary;

    #[test]
    fn parse_single_event_summary() {
        let dump = "# callgrind format\nversion: 1\ncreator: callgrind-3.24.0\npid: 36\n\
                    desc: Trigger: Client Request: execution\nevents: Ir\n\
                    fn=(1) main\n1 42\n\nsummary: 160763380\n";
        assert_eq!(
            parse_summary(dump).unwrap(),
            vec![("Ir".to_string(), 160763380)]
        );
    }

    #[test]
    fn parse_multi_event_summary() {
        let dump = "events: Ir Dr Dw I1mr\nsummary: 100 200 300 400\n";
        assert_eq!(
            parse_summary(dump).unwrap(),
            vec![
                ("Ir".to_string(), 100),
                ("Dr".to_string(), 200),
                ("Dw".to_string(), 300),
                ("I1mr".to_string(), 400),
            ]
        );
    }

    #[test]
    fn parse_errors() {
        assert!(parse_summary("").is_err());
        assert!(parse_summary("summary: 1 2 3\n").is_err());
        assert!(parse_summary("events: Ir\nsummary: 1 2\n").is_err());
    }
}
