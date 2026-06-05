//! Callgrind-backed measurement and dump parsing.
//!
//! This measure is active only when the benchmark child is already running
//! under Valgrind Callgrind. At each phase boundary it zeroes the current
//! counters, triggers a labeled dump, reads the matching `callgrind.out` part
//! file, and converts the recorded events into Sightglass measurements.

use super::{Measure, Measurements};
use anyhow::{anyhow, ensure, Context, Result};
use sightglass_data::Phase;
use std::{
    ffi::CString,
    fs,
    path::{Path, PathBuf},
};
use valgrind_requests::{callgrind, valgrind};

/// Environment variable used by the CLI parent to tell the child where
/// Callgrind dump files will be written.
pub const CALLGRIND_OUT_DIR_ENV_VAR: &str = "SIGHTGLASS_CALLGRIND_OUT_DIR";

const CLIENT_REQUEST_PREFIX: &str = "Client Request: ";
const EVENT_MAPPINGS: &[(&str, &str)] = &[
    ("Ir", "instructions-retired"),
    ("Dr", "data-reads"),
    ("Dw", "data-writes"),
    ("I1mr", "l1-icache-misses"),
    ("D1mr", "l1-dcache-read-misses"),
    ("D1mw", "l1-dcache-write-misses"),
    ("ILmr", "ll-icache-misses"),
    ("DLmr", "ll-dcache-read-misses"),
    ("DLmw", "ll-dcache-write-misses"),
    ("Bc", "conditional-branches"),
    ("Bcm", "conditional-branch-misses"),
    ("Bi", "indirect-branches"),
    ("Bim", "indirect-branch-misses"),
];

/// A `Measure` implementation that uses Callgrind to get low-noise
/// measurements.
pub struct CallgrindMeasure {
    output_dir: Option<PathBuf>,
    next_dump_part: u32,
}

impl Default for CallgrindMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl CallgrindMeasure {
    /// Create a new callgrind measure for the current process.
    pub fn new() -> Self {
        Self {
            output_dir: std::env::var_os(CALLGRIND_OUT_DIR_ENV_VAR).map(PathBuf::from),
            next_dump_part: 1,
        }
    }

    fn running_under_valgrind(&self) -> bool {
        let running_under_valgrind = valgrind::running_on_valgrind() > 0;
        assert!(
            running_under_valgrind || self.output_dir.is_none(),
            "callgrind measure requested but benchmark process is not running under Valgrind",
        );
        running_under_valgrind
    }

    fn parse_dump_for_phase(
        &self,
        phase: Phase,
        iteration: u32,
        part: u32,
    ) -> Result<ParsedCallgrindDump> {
        let output_dir = self.output_dir.as_ref().ok_or_else(|| {
            anyhow!(
                "callgrind output directory is not configured; expected {CALLGRIND_OUT_DIR_ENV_VAR}"
            )
        })?;
        let dump_path = output_dir.join(format!("callgrind.out.{}.{}", std::process::id(), part));
        ensure!(
            dump_path.exists(),
            "no callgrind output found at {} — is valgrind installed and on PATH?",
            dump_path.display()
        );

        let dump = parse_callgrind_dump_file(&dump_path)?;
        ensure!(
            dump.pid == std::process::id(),
            "callgrind dump pid mismatch: expected {}, found {} in {}",
            std::process::id(),
            dump.pid,
            dump_path.display()
        );
        ensure!(
            dump.part == part,
            "callgrind dump part mismatch: expected {part}, found {} in {}",
            dump.part,
            dump_path.display()
        );

        let expected_label = format!("{phase}/{iteration}");
        let actual_label = dump.label.as_deref().ok_or_else(|| {
            anyhow!(
                "callgrind dump {} is missing a client-request label",
                dump_path.display()
            )
        })?;
        ensure!(
            actual_label == expected_label,
            "callgrind dump mismatch: expected {expected_label}, got {actual_label}"
        );

        Ok(dump)
    }
}

impl Measure for CallgrindMeasure {
    fn start(&mut self, _phase: Phase) {
        if !self.running_under_valgrind() {
            return;
        }

        callgrind::start_instrumentation();
        callgrind::zero_stats();
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        if !self.running_under_valgrind() {
            return;
        }

        let label = CString::new(format!("{phase}/{}", measurements.iteration())).unwrap();
        callgrind::dump_stats_at(label.as_c_str());

        let dump = self
            .parse_dump_for_phase(phase, measurements.iteration(), self.next_dump_part)
            .unwrap_or_else(|error| panic!("failed to read callgrind dump: {error:#}"));
        self.next_dump_part += 1;

        measurements.reserve(dump.counts.len());
        for event in dump.counts {
            measurements.add(phase, event.name.into(), event.count);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedCallgrindDump {
    pid: u32,
    part: u32,
    label: Option<String>,
    counts: Vec<CallgrindEventCount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CallgrindEventCount {
    name: &'static str,
    count: u64,
}

fn parse_callgrind_dump_file(path: &Path) -> Result<ParsedCallgrindDump> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read callgrind dump {}", path.display()))?;
    parse_callgrind_dump(&contents)
}

fn parse_callgrind_dump(contents: &str) -> Result<ParsedCallgrindDump> {
    let mut events = None;
    let mut summary = None;
    let mut totals = None;
    let mut pid = None;
    let mut part = None;
    let mut label = None;

    for line in contents.lines() {
        if let Some(raw_events) = line.strip_prefix("events: ") {
            events = Some(raw_events.split_whitespace().collect::<Vec<_>>());
        } else if let Some(raw_counts) = line.strip_prefix("summary: ") {
            summary = Some(parse_counts(raw_counts)?);
        } else if let Some(raw_counts) = line.strip_prefix("totals: ") {
            totals = Some(parse_counts(raw_counts)?);
        } else if let Some(raw_pid) = line.strip_prefix("pid: ") {
            pid = Some(raw_pid.trim().parse().context("invalid callgrind pid")?);
        } else if let Some(raw_part) = line.strip_prefix("part: ") {
            part = Some(raw_part.trim().parse().context("invalid callgrind part")?);
        } else if let Some(trigger) = line.strip_prefix("desc: Trigger: ") {
            label = trigger
                .strip_prefix(CLIENT_REQUEST_PREFIX)
                .map(ToOwned::to_owned);
        }
    }

    let events = events.ok_or_else(|| anyhow!("callgrind dump is missing an events header"))?;
    let counts = summary
        .or(totals)
        .ok_or_else(|| anyhow!("callgrind dump is missing a summary/totals line"))?;
    ensure!(
        events.len() == counts.len(),
        "callgrind events/count mismatch: {} events, {} counts",
        events.len(),
        counts.len()
    );

    let mut parsed_counts = Vec::with_capacity(EVENT_MAPPINGS.len());
    for (event, count) in events.into_iter().zip(counts) {
        if let Some(name) = event_name(event) {
            parsed_counts.push(CallgrindEventCount { name, count });
        }
    }

    Ok(ParsedCallgrindDump {
        pid: pid.ok_or_else(|| anyhow!("callgrind dump is missing pid"))?,
        part: part.ok_or_else(|| anyhow!("callgrind dump is missing part"))?,
        label,
        counts: parsed_counts,
    })
}

fn parse_counts(raw_counts: &str) -> Result<Vec<u64>> {
    raw_counts
        .split_whitespace()
        .map(|count| {
            count
                .parse::<u64>()
                .with_context(|| format!("invalid callgrind count: {count}"))
        })
        .collect()
}

fn event_name(raw_event: &str) -> Option<&'static str> {
    EVENT_MAPPINGS
        .iter()
        .find_map(|(event, name)| (*event == raw_event).then_some(*name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_client_request_dump() -> Result<()> {
        let dump = parse_callgrind_dump(include_str!(
            "../../../../test/fixtures/callgrind.client-request.out"
        ))?;

        assert_eq!(dump.pid, 773940);
        assert_eq!(dump.part, 1);
        assert_eq!(dump.label.as_deref(), Some("execution/7"));
        assert_eq!(
            dump.counts,
            vec![
                CallgrindEventCount {
                    name: "instructions-retired",
                    count: 790139,
                },
                CallgrindEventCount {
                    name: "data-reads",
                    count: 230038,
                },
                CallgrindEventCount {
                    name: "data-writes",
                    count: 320058,
                },
                CallgrindEventCount {
                    name: "l1-icache-misses",
                    count: 17,
                },
                CallgrindEventCount {
                    name: "l1-dcache-read-misses",
                    count: 0,
                },
                CallgrindEventCount {
                    name: "l1-dcache-write-misses",
                    count: 8,
                },
                CallgrindEventCount {
                    name: "ll-icache-misses",
                    count: 17,
                },
                CallgrindEventCount {
                    name: "ll-dcache-read-misses",
                    count: 0,
                },
                CallgrindEventCount {
                    name: "ll-dcache-write-misses",
                    count: 8,
                },
                CallgrindEventCount {
                    name: "conditional-branches",
                    count: 30004,
                },
                CallgrindEventCount {
                    name: "conditional-branch-misses",
                    count: 8,
                },
                CallgrindEventCount {
                    name: "indirect-branches",
                    count: 1,
                },
                CallgrindEventCount {
                    name: "indirect-branch-misses",
                    count: 1,
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn ignores_program_termination_label() -> Result<()> {
        let dump = parse_callgrind_dump(include_str!(
            "../../../../test/fixtures/callgrind.program-termination.out"
        ))?;

        assert_eq!(dump.label, None);
        assert_eq!(dump.part, 2);
        Ok(())
    }
}
