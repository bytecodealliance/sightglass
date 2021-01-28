//! Common data definitions for sightglass.
//!
//! These are in one place, pulled out from the rest of the crates, so that many
//! different crates can serialize and deserialize data by using the same
//! definitions.

#![deny(missing_docs, missing_debug_implementations)]

mod format;
pub use format::Format;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A single measurement, for example instructions retired when compiling a Wasm
/// module.
///
/// This is often used with the `'static` lifetime when recording measurements,
/// where we can use string literals for various fields. When reading data, it
/// can be used with a non-static lifetime to avoid many small allocations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement<'a> {
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
