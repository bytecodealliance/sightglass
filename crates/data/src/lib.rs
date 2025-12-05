//! Common data definitions for sightglass.
//!
//! These are in one place, pulled out from the rest of the crates, so that many
//! different crates can serialize and deserialize data by using the same
//! definitions.

#![deny(missing_docs, missing_debug_implementations)]

mod format;
pub use format::Format;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, fmt::Display, str::FromStr};

/// Composite of data about an engine and its configuration
#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct Engine<'a> {
    /// The file path of the wasmtime benchmark API shared library used to
    /// record this measurement or a user provided name.
    pub name: Cow<'a, str>,

    /// The engine-specific flags used when recording this measurement.
    /// This allows disambiguation when the same engine is used with different
    /// configurations.
    pub flags: Option<Cow<'a, str>>,
}

// this is mostly out of convenience for tests and the like
impl<'a, T> From<T> for Engine<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(name: T) -> Self {
        Self {
            name: name.into(),
            flags: None,
        }
    }
}

impl<'a> Engine<'a> {
    /// Given two engines, provide potentially shorter labels for each.
    ///
    /// Often, the names for both engines may share significant overlap.
    /// If the engines are named the same, fallback to displaying only
    /// the flags that differentiate engine invocations.  If not, remove
    /// the common prefix from the two engines paths/names.
    ///
    /// Returns a two-tuple with labels for self followed by other.
    pub fn relative_labels<'b>(&self, other: &Engine<'b>) -> (String, String) {
        if self.name == other.name {
            let self_flags = self
                .flags
                .as_ref()
                .map(|ref ef| ef.to_string())
                .unwrap_or_else(|| "(no flags)".into())
                .to_string();
            let other_flags = other
                .flags
                .as_ref()
                .map(|ref ef| ef.to_string())
                .unwrap_or_else(|| "(no flags)".into())
                .to_string();
            return (self_flags, other_flags);
        }

        // The engine names are distinct, compute the combined
        // name/flags and remove any common prefix.
        let self_str = format!("{self}");
        let other_str = format!("{other}");
        let idx_end_of_shared = self_str
            .char_indices()
            .zip(other_str.char_indices())
            .find_map(|((i, a), (j, b))| {
                if a == b {
                    None
                } else {
                    debug_assert_eq!(i, j);
                    Some(i)
                }
            })
            .unwrap_or(0);

        (
            self_str[idx_end_of_shared..].into(),
            other_str[idx_end_of_shared..].into(),
        )
    }
}

impl<'a> Display for Engine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref flags) = self.flags {
            write!(f, "{} ({flags})", self.name)
        } else {
            f.write_str(&self.name)
        }
    }
}

/// A single measurement, for example instructions retired when compiling a Wasm
/// module.
///
/// This is often used with the `'static` lifetime when recording measurements,
/// where we can use string literals for various fields. When reading data, it
/// can be used with a non-static lifetime to avoid many small allocations.
#[derive(Clone, Debug)]
pub struct Measurement<'a> {
    /// The CPU architecture on which this measurement was taken, for example
    /// "aarch64" or "x86_64".
    pub arch: Cow<'a, str>,

    /// Composite of data about the engine and its configuration
    ///
    /// This is currently flattened into:
    /// - engine: path to engine library used for measurement
    /// - engine_flags: if present, string representing engine-flags used for the measurement.
    pub engine: Engine<'a>,

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

#[derive(Serialize, Deserialize)]
struct MeasurementWire<'a> {
    arch: Cow<'a, str>,
    engine: Cow<'a, str>,               // engine.name
    engine_flags: Option<Cow<'a, str>>, // engine.flags
    wasm: Cow<'a, str>,
    process: u32,
    iteration: u32,
    phase: Phase,
    event: Cow<'a, str>,
    count: u64,
}

impl<'a> Serialize for Measurement<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // this isn't optimal but convenient to work around rust-csv restrictions.
        let wire = MeasurementWire {
            arch: self.arch.clone(),
            engine: self.engine.name.clone(),
            engine_flags: self.engine.flags.clone(),
            wasm: self.wasm.clone(),
            process: self.process,
            iteration: self.iteration,
            phase: self.phase,
            event: self.event.clone(),
            count: self.count,
        };
        wire.serialize(serializer)
    }
}

impl<'a, 'de> Deserialize<'de> for Measurement<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MeasurementWire::deserialize(deserializer)?;
        Ok(Measurement {
            arch: wire.arch,
            engine: Engine {
                name: wire.engine,
                flags: wire.engine_flags,
            },
            wasm: wire.wasm,
            process: wire.process,
            iteration: wire.iteration,
            phase: wire.phase,
            event: wire.event,
            count: wire.count,
        })
    }
}

/// A phase in a Wasm program's lifecycle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
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

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Phase::Compilation => write!(f, "compilation"),
            Phase::Instantiation => write!(f, "instantiation"),
            Phase::Execution => write!(f, "execution"),
        }
    }
}

impl FromStr for Phase {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "compilation" => Ok(Self::Compilation),
            "instantiation" => Ok(Self::Instantiation),
            "execution" => Ok(Self::Execution),
            _ => Err("invalid phase".into()),
        }
    }
}

/// A summary of grouped measurements.
#[derive(Clone, Debug, PartialEq)]
pub struct Summary<'a> {
    /// The CPU architecture on which this measurement was taken, for example
    /// "aarch64" or "x86_64".
    pub arch: Cow<'a, str>,

    /// Information about the Engine used to record measurement.  This includes
    /// the engine name/path and flags used to configure the engine.
    pub engine: Engine<'a>,

    /// The file path of the Wasm benchmark program.
    pub wasm: Cow<'a, str>,

    /// The phase in a Wasm program's lifecycle that was measured: compilation,
    /// instantiation, or execution.
    pub phase: Phase,

    /// The event that was measured: micro seconds of wall time, CPU cycles
    /// executed, instructions retired, cache misses, etc.
    pub event: Cow<'a, str>,

    /// The minimum value of the `count` field.
    pub min: u64,

    /// The maximum value of the `count` field.
    pub max: u64,

    /// The median value of the `count` field.
    pub median: u64,

    /// The arithmetic mean of the `count` field.
    pub mean: f64,

    /// The mean deviation (note: not standard deviation) of the `count` field.
    pub mean_deviation: f64,
}

/// Version of [`Summary`] for serialization purposes.
///
/// This is a workaround for limitations of rust-csv
/// related to (flattened) struct types.
#[derive(Debug, Serialize, Deserialize)]
struct SummaryWire<'a> {
    pub arch: Cow<'a, str>,
    pub engine: Cow<'a, str>,               // engine.name
    pub engine_flags: Option<Cow<'a, str>>, // engine.flags
    pub wasm: Cow<'a, str>,
    pub phase: Phase,
    pub event: Cow<'a, str>,
    pub min: u64,
    pub max: u64,
    pub median: u64,
    pub mean: f64,
    pub mean_deviation: f64,
}

impl<'a> Serialize for Summary<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // this isn't optimal but convenient to work around rust-csv restrictions.
        let wire = SummaryWire {
            arch: self.arch.clone(),
            engine: self.engine.name.clone(),
            engine_flags: self.engine.flags.clone(),
            wasm: self.wasm.clone(),
            phase: self.phase,
            event: self.event.clone(),
            min: self.min,
            max: self.max,
            median: self.median,
            mean: self.mean,
            mean_deviation: self.mean_deviation,
        };
        wire.serialize(serializer)
    }
}

impl<'a, 'de> Deserialize<'de> for Summary<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SummaryWire::deserialize(deserializer)?;
        Ok(Summary {
            arch: wire.arch,
            engine: Engine {
                name: wire.engine,
                flags: wire.engine_flags,
            },
            wasm: wire.wasm,
            phase: wire.phase,
            event: wire.event,
            min: wire.min,
            max: wire.max,
            median: wire.median,
            mean: wire.mean,
            mean_deviation: wire.mean_deviation,
        })
    }
}

/// The effect size (and confidence interval) between two different engines
/// (i.e. two different commits of Wasmtime).
///
/// This allows us to justify statements like "we are 99% confident that the new
/// register allocator is 13.6% faster (± 1.7%) than the old register
/// allocator."
#[derive(Clone, Debug)]
pub struct EffectSize<'a> {
    /// The CPU architecture on which this measurement was taken, for example
    /// "aarch64" or "x86_64".
    pub arch: Cow<'a, str>,

    /// The file path of the Wasm benchmark program.
    pub wasm: Cow<'a, str>,

    /// The phase in a Wasm program's lifecycle that was measured: compilation,
    /// instantiation, or execution.
    pub phase: Phase,

    /// The event that was measured: micro seconds of wall time, CPU cycles
    /// executed, instructions retired, cache misses, etc.
    pub event: Cow<'a, str>,

    /// The first engine being compared.
    ///
    /// This is the file path of the wasmtime benchmark API shared library used
    /// to record this measurement.
    pub a_engine: Engine<'a>,

    /// The first engine's result's arithmetic mean of the `count` field.
    pub a_mean: f64,

    /// The second engine being compared.
    ///
    /// This is the file path of the wasmtime benchmark API shared library used
    /// to record this measurement.
    pub b_engine: Engine<'a>,

    /// The second engine's result's arithmetic mean of the `count` field.
    pub b_mean: f64,

    /// The significance level for the confidence interval.
    ///
    /// This is always between 0.0 and 1.0. Typical values are 0.01 and 0.05
    /// which correspond to 99% confidence and 95% confidence respectively.
    pub significance_level: f64,

    /// The half-width confidence interval, i.e. the `i` in
    ///
    /// ```text
    /// b_mean - a_mean ± i
    /// ```
    pub half_width_confidence_interval: f64,
}

#[derive(Serialize, Deserialize)]
struct EffectSizeWire<'a> {
    arch: Cow<'a, str>,
    wasm: Cow<'a, str>,
    phase: Phase,
    event: Cow<'a, str>,
    a_engine: Cow<'a, str>,               // a_engine.name
    a_engine_flags: Option<Cow<'a, str>>, // a_engine.flags
    a_mean: f64,
    b_engine: Cow<'a, str>,
    b_engine_flags: Option<Cow<'a, str>>,
    b_mean: f64,
    significance_level: f64,
    half_width_confidence_interval: f64,
}

impl EffectSize<'_> {
    /// Is the difference between `self.a_mean` and `self.b_mean` statistically
    /// significant?
    pub fn is_significant(&self) -> bool {
        (self.a_mean - self.b_mean).abs() > self.half_width_confidence_interval.abs()
    }

    /// Return `b`'s speedup over `a` and the speedup's confidence interval.
    pub fn b_speed_up_over_a(&self) -> (f64, f64) {
        (
            self.b_mean / self.a_mean,
            self.half_width_confidence_interval / self.a_mean,
        )
    }

    /// Return `a`'s speed up over `b` and the speed up's confidence interval.
    pub fn a_speed_up_over_b(&self) -> (f64, f64) {
        (
            self.a_mean / self.b_mean,
            self.half_width_confidence_interval / self.b_mean,
        )
    }
}

impl<'a> Serialize for EffectSize<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // this isn't optimal but convenient to work around rust-csv restrictions.
        let wire = EffectSizeWire {
            arch: self.arch.clone(),
            a_engine: self.a_engine.name.clone(),
            a_engine_flags: self.a_engine.flags.clone(),
            wasm: self.wasm.clone(),
            phase: self.phase,
            event: self.event.clone(),
            b_engine: self.b_engine.name.clone(),
            b_engine_flags: self.b_engine.flags.clone(),
            a_mean: self.a_mean,
            b_mean: self.b_mean,
            significance_level: self.significance_level,
            half_width_confidence_interval: self.half_width_confidence_interval,
        };
        wire.serialize(serializer)
    }
}

impl<'a, 'de> Deserialize<'de> for EffectSize<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EffectSizeWire::deserialize(deserializer)?;
        Ok(EffectSize {
            arch: wire.arch,
            a_engine: Engine {
                name: wire.a_engine,
                flags: wire.a_engine_flags,
            },
            b_engine: Engine {
                name: wire.b_engine,
                flags: wire.b_engine_flags,
            },
            wasm: wire.wasm,
            phase: wire.phase,
            event: wire.event,
            a_mean: wire.a_mean,
            b_mean: wire.b_mean,
            significance_level: wire.significance_level,
            half_width_confidence_interval: wire.half_width_confidence_interval,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_measurement<'a>() -> Measurement<'a> {
        Measurement {
            arch: "x86_64".into(),
            engine: Engine {
                name: "wasmtime".into(),
                flags: Some("-Wfoo=bar".into()),
            },
            wasm: "benchmark.wasm".into(),
            process: 101,
            iteration: 1,
            phase: Phase::Execution,
            event: "instructions".into(),
            count: 5000,
        }
    }

    #[test]
    fn test_csv_round_trip_with_flags() -> anyhow::Result<()> {
        let measurement = base_measurement();
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&measurement)?;
        let data = String::from_utf8(wtr.into_inner()?)?;

        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        for result in rdr.deserialize() {
            let record: Measurement = result?;
            assert_eq!(record.arch, "x86_64");
            assert_eq!(record.engine.name, "wasmtime");
            assert_eq!(record.engine.flags, Some(Cow::Borrowed("-Wfoo=bar")));
            assert_eq!(record.count, 5000);
        }
        Ok(())
    }

    #[test]
    fn test_csv_round_trip_no_flags() -> anyhow::Result<()> {
        let mut measurement = base_measurement();
        measurement.engine.flags = None;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&measurement)?;
        let data = String::from_utf8(wtr.into_inner()?)?;

        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        for result in rdr.deserialize() {
            let record: Measurement = result?;
            assert_eq!(record.arch, "x86_64");
            assert_eq!(record.engine.name, "wasmtime");
            assert_eq!(record.engine.flags, None);
            assert_eq!(record.count, 5000);
        }
        Ok(())
    }
}
