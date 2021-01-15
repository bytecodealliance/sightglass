//! Common data definitions for sightglass.
//!
//! These are in one place, pulled out from the rest of the crates, so that many
//! different crates can serialize and deserialize data by using the same
//! definitions.

#![deny(missing_docs, missing_debug_implementations)]

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A single measurement, for example instructions retired when compiling a Wasm
/// module.
///
/// This is often used with the `'static` lifetime when recording measurements,
/// where we can use string literals for various fields. When reading data, it
/// can be used with a non-static lifetime to avoid many small allocations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedMeasurement<'a> {
    /// The CPU architecture on which this measurement was taken, for example
    /// "aarch64" or "x86_64".
    pub arch: Cow<'a, str>,

    /// The file path of the wasmtime benchmark API shared library used to
    /// record this measurement.
    pub engine: Cow<'a, str>,

    /// The file path of the Wasm benchmark program.
    pub wasm: Cow<'a, str>,

    /// The id of the process within which this measurement was taken.
    pub process: u32,

    /// This measurement was the `n`th measurement of this phase taken within a
    /// process.
    pub iteration: u32,

    /// The phase in a Wasm program's lifecycle that was measured: compilation,
    /// instantiation, or execution.
    pub phase: Phase,

    /// The event that was measured: micro seconds of wall time, CPU cycles
    /// executed, instructions retired, cache misses, etc.
    pub event: Cow<'a, str>,

    /// The event counts.
    ///
    /// The meaning and units depend on what the `event` is: it might be a count
    /// of microseconds if the event is wall time, or it might be a count of
    /// instructions if the event is instructions retired.
    pub count: u64,
}

/// A phase in a Wasm program's lifecycle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Phase {
    /// The compilation phase, where Wasm bytes are translated into native
    /// machine code.
    Compilation,
    /// The instantiation phase, where imports are provided and memories,
    /// globals, and tables are initialized.
    Instantiation,
    /// The execution phase, where functions are called and instructions are
    /// executed.
    Execution,
}

/// An in-progress collection of measurements that are currently being recorded.
#[derive(Debug)]
pub struct Measurements<'a> {
    arch: &'a str,
    engine: &'a str,
    wasm: &'a str,
    process: u32,
    iteration: u32,
    measurements: Vec<ParsedMeasurement<'a>>,
}

impl<'a> Measurements<'a> {
    /// Construct a new `Measurements`.
    pub fn new(arch: &'a str, engine: &'a str, wasm: &'a str) -> Self {
        Measurements {
            arch,
            engine,
            wasm,
            process: std::process::id(),
            iteration: 0,
            measurements: vec![],
        }
    }

    /// Advance the iteration counter.
    pub fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    /// Reserve additional capacity for more measurements internally.
    pub fn reserve(&mut self, capacity: usize) {
        self.measurements.reserve(capacity);
    }

    /// Add a measurement of the given event for the given phase to this
    /// `Measurements` collection.
    pub fn add(&mut self, phase: Phase, events: impl Into<Vec<Event<'a>>>) {
        for event in events.into() {
            self.measurements.push(ParsedMeasurement {
                arch: self.arch.into(),
                engine: self.engine.into(),
                wasm: self.wasm.into(),
                process: self.process,
                iteration: self.iteration,
                phase,
                event: event.name,
                count: event.count,
            });
        }
    }

    /// When all measurements have been recorded, call this method to get the
    /// underlying measurements data.
    pub fn finish(self) -> Vec<ParsedMeasurement<'a>> {
        self.measurements
    }
}

/// TODO
#[derive(Debug)]
pub struct Event<'a> {
    name: Cow<'a, str>,
    count: u64,
}
