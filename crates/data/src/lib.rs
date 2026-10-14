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
        let mut idx_end_of_shared = self_str
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

        // Trim back to last path separator to keep meaningful context
        if idx_end_of_shared > 0 {
            if let Some(last_slash) = self_str[..idx_end_of_shared].rfind('/') {
                idx_end_of_shared = last_slash + 1;
            }
        }

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

/// Below this many samples of a benchmark, report the full range rather than
/// the interquartile range as its noise.
///
/// The quartiles are interpolated between neighboring samples, so with only a
/// handful of them the interquartile range is reading off two or three points
/// while the full range uses all of them. Above the threshold the balance
/// reverses: the interquartile range keeps a single interfered-with sample
/// from dominating, which the full range cannot do, since one outlier is one
/// of its two ends.
const FEW_SAMPLES: usize = 8;

/// How far a benchmark's counts spread, as a percentage of `median`.
///
/// See [`Summary::noise`] for what this measures and when it is comparable.
fn noise(samples: usize, min: f64, max: f64, p25: f64, p75: f64, median: f64) -> f64 {
    // A benchmark whose counts are all zero has no scale to be noisy relative
    // to, and dividing by its median would give infinity or NaN.
    if median == 0.0 {
        return 0.0;
    }

    let spread = if samples < FEW_SAMPLES {
        max - min
    } else {
        p75 - p25
    };

    spread / median * 100.0
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
    pub median: f64,

    /// The arithmetic mean of the `count` field.
    pub mean: f64,

    /// The mean deviation (note: not standard deviation) of the `count` field.
    pub mean_deviation: f64,

    /// How many measurements went into this summary.
    pub samples: usize,

    /// The 25th percentile of the `count` field.
    pub p25: f64,

    /// The 75th percentile of the `count` field.
    pub p75: f64,
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
    pub median: f64,
    pub mean: f64,
    pub mean_deviation: f64,
    pub samples: usize,
    pub p25: f64,
    pub p75: f64,
}

impl Summary<'_> {
    /// How much this benchmark's counts moved between samples, as a
    /// percentage of the median.
    ///
    /// When there are at least [`FEW_SAMPLES`], this is the interquartile range
    /// divided by the median (`(p75 - p25) / median`); otherwise, we don't have
    /// enough samples to ignore outliers, so this becomes `(max - min) /
    /// median`.
    pub fn noise(&self) -> f64 {
        noise(
            self.samples,
            self.min as f64,
            self.max as f64,
            self.p25,
            self.p75,
            self.median,
        )
    }
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
            samples: self.samples,
            p25: self.p25,
            p75: self.p75,
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
            samples: wire.samples,
            p25: wire.p25,
            p75: wire.p75,
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

    /// The first engine's result's median of the `count` field.
    pub a_median: f64,

    /// The first engine's result's minimum of the `count` field.
    pub a_min: u64,

    /// The first engine's result's maximum of the `count` field.
    pub a_max: u64,

    /// The first engine's result's 25th percentile of the `count` field.
    pub a_p25: f64,

    /// The first engine's result's 75th percentile of the `count` field.
    pub a_p75: f64,

    /// How many samples the first engine contributed.
    pub a_samples: usize,

    /// The second engine being compared.
    ///
    /// This is the file path of the wasmtime benchmark API shared library used
    /// to record this measurement.
    pub b_engine: Engine<'a>,

    /// The second engine's result's arithmetic mean of the `count` field.
    pub b_mean: f64,

    /// The second engine's result's median of the `count` field.
    pub b_median: f64,

    /// The second engine's result's minimum of the `count` field.
    pub b_min: u64,

    /// The second engine's result's maximum of the `count` field.
    pub b_max: u64,

    /// The second engine's result's 25th percentile of the `count` field.
    pub b_p25: f64,

    /// The second engine's result's 75th percentile of the `count` field.
    pub b_p75: f64,

    /// How many samples the second engine contributed.
    pub b_samples: usize,

    /// The significance level for the confidence interval.
    ///
    /// This is always between 0.0 and 1.0. Typical values are 0.01 and 0.05
    /// which correspond to 99% confidence and 95% confidence respectively.
    pub significance_level: f64,

    /// How much larger `b`'s counts are than `a`'s: the median of the ratios
    /// between every pair of their samples.
    pub ratio: f64,

    /// The confidence interval for `ratio`, as `(low, high)`.
    ///
    /// This is `None` when the samples cannot support an interval, as when
    /// there are too few of them to reach `significance_level` however they
    /// fall.
    pub confidence_interval: Option<(f64, f64)>,

    /// The probability of counts at least this far apart when the two engines
    /// do not differ.
    pub p_value: f64,
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
    a_median: f64,
    a_min: u64,
    a_max: u64,
    a_p25: f64,
    a_p75: f64,
    a_samples: usize,
    b_engine: Cow<'a, str>,
    b_engine_flags: Option<Cow<'a, str>>,
    b_mean: f64,
    b_median: f64,
    b_min: u64,
    b_max: u64,
    b_p25: f64,
    b_p75: f64,
    b_samples: usize,
    significance_level: f64,
    ratio: f64,
    // Flattened into two columns, since `csv` cannot serialize a tuple field.
    confidence_interval_low: Option<f64>,
    confidence_interval_high: Option<f64>,
    p_value: f64,
}

impl EffectSize<'_> {
    /// Did the two engines' counts differ by more than measurement noise?
    ///
    /// Samples too few to reach `significance_level` however they fall are not
    /// significant, rather than being treated as a difference with no
    /// uncertainty.
    pub fn is_significant(&self) -> bool {
        self.confidence_interval.is_some() && self.p_value <= self.significance_level
    }

    /// How much the first engine's counts moved between samples, as a
    /// percentage of its median.
    ///
    /// See [`Summary::noise`], which measures the same thing the same way. A
    /// comparison has one of these per engine rather than one overall, since
    /// which of the two scattered is the useful thing to know.
    pub fn a_noise(&self) -> f64 {
        noise(
            self.a_samples,
            self.a_min as f64,
            self.a_max as f64,
            self.a_p25,
            self.a_p75,
            self.a_median,
        )
    }

    /// How much the second engine's counts moved between samples, as a
    /// percentage of its median.
    ///
    /// See [`EffectSize::a_noise`].
    pub fn b_noise(&self) -> f64 {
        noise(
            self.b_samples,
            self.b_min as f64,
            self.b_max as f64,
            self.b_p25,
            self.b_p75,
            self.b_median,
        )
    }

    /// Return `b`'s speed up over `a` and the speed up's confidence interval.
    pub fn b_speed_up_over_a(&self) -> (f64, Option<(f64, f64)>) {
        (self.ratio, self.confidence_interval)
    }

    /// Return `a`'s speed up over `b` and the speed up's confidence interval.
    ///
    /// Inverting a ratio swaps which end of its interval is which.
    pub fn a_speed_up_over_b(&self) -> (f64, Option<(f64, f64)>) {
        (
            1.0 / self.ratio,
            self.confidence_interval
                .map(|(low, high)| (1.0 / high, 1.0 / low)),
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
            a_median: self.a_median,
            a_min: self.a_min,
            a_max: self.a_max,
            a_p25: self.a_p25,
            a_p75: self.a_p75,
            a_samples: self.a_samples,
            b_mean: self.b_mean,
            b_median: self.b_median,
            b_min: self.b_min,
            b_max: self.b_max,
            b_p25: self.b_p25,
            b_p75: self.b_p75,
            b_samples: self.b_samples,
            significance_level: self.significance_level,
            ratio: self.ratio,
            confidence_interval_low: self.confidence_interval.map(|(low, _)| low),
            confidence_interval_high: self.confidence_interval.map(|(_, high)| high),
            p_value: self.p_value,
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
            a_median: wire.a_median,
            a_min: wire.a_min,
            a_max: wire.a_max,
            a_p25: wire.a_p25,
            a_p75: wire.a_p75,
            a_samples: wire.a_samples,
            b_mean: wire.b_mean,
            b_median: wire.b_median,
            b_min: wire.b_min,
            b_max: wire.b_max,
            b_p25: wire.b_p25,
            b_p75: wire.b_p75,
            b_samples: wire.b_samples,
            significance_level: wire.significance_level,
            ratio: wire.ratio,
            confidence_interval: wire
                .confidence_interval_low
                .zip(wire.confidence_interval_high),
            p_value: wire.p_value,
        })
    }
}

/// Extract benchmark name from wasm file path.
///
/// This function handles various path formats commonly used in Sightglass:
/// - `./benchmarks/foo/benchmark.wasm` -> `foo`
/// - `benchmarks/bar/benchmark.wasm` -> `bar`
/// - `benchmarks/foo/bar.wasm` -> `foo/bar`
/// - `simple.wasm` -> `simple`
///
/// # Examples
///
/// ```
/// use sightglass_data::extract_benchmark_name;
///
/// assert_eq!(extract_benchmark_name("./benchmarks/foo/benchmark.wasm"), "foo");
/// assert_eq!(extract_benchmark_name("benchmarks/bar/benchmark.wasm"), "bar");
/// assert_eq!(extract_benchmark_name("benchmarks/foo/bar.wasm"), "foo/bar");
/// assert_eq!(extract_benchmark_name("simple.wasm"), "simple");
/// ```
pub fn extract_benchmark_name(wasm_path: &str) -> String {
    let mut path = wasm_path;

    // Remove prefix variations
    if let Some(stripped) = path.strip_prefix("./benchmarks/") {
        path = stripped;
    } else if let Some(stripped) = path.strip_prefix("benchmarks/") {
        path = stripped;
    }

    // Remove suffix variations
    if let Some(stripped) = path.strip_suffix("/benchmark.wasm") {
        path = stripped;
    } else if let Some(stripped) = path.strip_suffix(".wasm") {
        path = stripped;
    }

    path.to_string()
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

    #[test]
    fn test_extract_benchmark_name() {
        // Standard benchmark.wasm format with ./benchmarks/ prefix
        assert_eq!(
            extract_benchmark_name("./benchmarks/foo/benchmark.wasm"),
            "foo"
        );

        // Standard benchmark.wasm format without ./ prefix
        assert_eq!(
            extract_benchmark_name("benchmarks/bar/benchmark.wasm"),
            "bar"
        );

        // Direct .wasm file in benchmarks directory
        assert_eq!(extract_benchmark_name("benchmarks/simple.wasm"), "simple");
        assert_eq!(extract_benchmark_name("./benchmarks/simple.wasm"), "simple");

        // Nested paths with .wasm extension
        assert_eq!(extract_benchmark_name("benchmarks/foo/bar.wasm"), "foo/bar");
        assert_eq!(
            extract_benchmark_name("./benchmarks/nested/path/test.wasm"),
            "nested/path/test"
        );

        // Simple .wasm files without benchmarks prefix
        assert_eq!(extract_benchmark_name("simple.wasm"), "simple");
        assert_eq!(extract_benchmark_name("test.wasm"), "test");

        // Edge cases - no extensions or prefixes
        assert_eq!(extract_benchmark_name("somefile"), "somefile");
        assert_eq!(extract_benchmark_name("path/to/file"), "path/to/file");
    }

    #[test]
    fn test_relative_labels_same_name_different_flags() {
        let engine1 = Engine {
            name: "wasmtime".into(),
            flags: Some("-Wfoo=bar".into()),
        };
        let engine2 = Engine {
            name: "wasmtime".into(),
            flags: Some("-Wbaz=qux".into()),
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // When names match, should return just the flags
        assert_eq!(label1, "-Wfoo=bar");
        assert_eq!(label2, "-Wbaz=qux");
    }

    #[test]
    fn test_relative_labels_same_name_no_flags() {
        let engine1 = Engine {
            name: "wasmtime".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "wasmtime".into(),
            flags: Some("-Wfoo=bar".into()),
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        assert_eq!(label1, "(no flags)");
        assert_eq!(label2, "-Wfoo=bar");
    }

    #[test]
    fn test_relative_labels_different_names_simple() {
        let engine1 = Engine {
            name: "wasmtime".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "lucet".into(),
            flags: None,
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // No common prefix, should return full names
        assert_eq!(label1, "wasmtime");
        assert_eq!(label2, "lucet");
    }

    #[test]
    fn test_relative_labels_with_common_prefix() {
        let engine1 = Engine {
            name: "engines/wasmtime/v1.0/libengine.so".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "engines/wasmtime/v2.0/libengine.so".into(),
            flags: None,
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // Should trim to last path separator before the difference
        assert_eq!(label1, "v1.0/libengine.so");
        assert_eq!(label2, "v2.0/libengine.so");
    }

    #[test]
    fn test_relative_labels_with_path_and_flags() {
        let engine1 = Engine {
            name: "engines/wasmtime/v1.0/libengine.so".into(),
            flags: Some("-Wepoch=y".into()),
        };
        let engine2 = Engine {
            name: "engines/wasmtime/v2.0/libengine.so".into(),
            flags: Some("-Wepoch=y".into()),
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // Should include flags in the shortened label
        assert_eq!(label1, "v1.0/libengine.so (-Wepoch=y)");
        assert_eq!(label2, "v2.0/libengine.so (-Wepoch=y)");
    }

    #[test]
    fn test_relative_labels_long_common_prefix() {
        let engine1 = Engine {
            name: "/home/user/projects/engines/wasmtime/wasmtime-v38/libengine.dylib".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "/home/user/projects/engines/wasmtime/wasmtime-v39/libengine.dylib".into(),
            flags: None,
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // Should cut at the path separator before the version difference
        assert_eq!(label1, "wasmtime-v38/libengine.dylib");
        assert_eq!(label2, "wasmtime-v39/libengine.dylib");
    }

    #[test]
    fn test_relative_labels_no_path_separator_in_common_prefix() {
        let engine1 = Engine {
            name: "wasmtime-v1".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "wasmtime-v2".into(),
            flags: None,
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // No path separator, trims at the first difference (after 'wasmtime-v')
        assert_eq!(label1, "1");
        assert_eq!(label2, "2");
    }

    #[test]
    fn test_relative_labels_identical_engines() {
        let engine1 = Engine {
            name: "wasmtime".into(),
            flags: Some("-Wfoo=bar".into()),
        };
        let engine2 = Engine {
            name: "wasmtime".into(),
            flags: Some("-Wfoo=bar".into()),
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // Identical engines should return the flags
        assert_eq!(label1, "-Wfoo=bar");
        assert_eq!(label2, "-Wfoo=bar");
    }

    #[test]
    fn test_relative_labels_one_is_prefix_of_other() {
        let engine1 = Engine {
            name: "engines/wasmtime".into(),
            flags: None,
        };
        let engine2 = Engine {
            name: "engines/wasmtime/v2.0".into(),
            flags: None,
        };

        let (label1, label2) = engine1.relative_labels(&engine2);
        // When one is a prefix, there's no difference to detect, so returns full names
        assert_eq!(label1, "engines/wasmtime");
        assert_eq!(label2, "engines/wasmtime/v2.0");
    }

    /// A `Summary` whose counts are `counts`, with the fields `noise` reads
    /// filled in from them.
    fn summary_of(counts: &[u64]) -> Summary<'static> {
        let mut sorted = counts.to_vec();
        sorted.sort();
        let percentile = |p: f64| {
            let index = (p / 100.0) * (sorted.len() - 1) as f64;
            let (low, high) = (index.floor() as usize, index.ceil() as usize);
            if low == high {
                sorted[low] as f64
            } else {
                let weight = index - low as f64;
                sorted[low] as f64 * (1.0 - weight) + sorted[high] as f64 * weight
            }
        };

        Summary {
            arch: "x86_64".into(),
            engine: "wasmtime".into(),
            wasm: "benchmark.wasm".into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            min: sorted[0],
            max: sorted[sorted.len() - 1],
            median: percentile(50.0),
            mean: sorted.iter().sum::<u64>() as f64 / sorted.len() as f64,
            mean_deviation: 0.0,
            samples: sorted.len(),
            p25: percentile(25.0),
            p75: percentile(75.0),
        }
    }

    /// Which spread `noise` measures depends on how many samples there are,
    /// and the two give different answers on the same data.
    #[test]
    fn noise_switches_from_the_range_to_the_interquartile_range() {
        // One sample sits well above the rest, so the range and the
        // interquartile range disagree about how noisy this is.
        let spike = |n: usize| {
            let mut counts = vec![100; n];
            *counts.last_mut().unwrap() = 200;
            counts
        };

        // Below the threshold the full range is reported: 100 over a median of
        // 100.
        let few = summary_of(&spike(FEW_SAMPLES - 1));
        assert_eq!(few.samples, 7);
        assert!((few.noise() - 100.0).abs() < 1e-9, "{}", few.noise());

        // At the threshold the interquartile range takes over, and the lone
        // high sample no longer sets the answer.
        let enough = summary_of(&spike(FEW_SAMPLES));
        assert_eq!(enough.samples, 8);
        assert_eq!(enough.noise(), 0.0);
    }

    /// Resisting a single interfered-with sample is the reason the
    /// interquartile range takes over at all.
    #[test]
    fn an_outlier_moves_the_interquartile_range_far_less_than_the_range() {
        let clean: Vec<u64> = (100..120).collect();
        let mut spiked = clean.clone();
        *spiked.last_mut().unwrap() = 400;

        let before = summary_of(&clean).noise();
        let after = summary_of(&spiked).noise();
        assert!(
            (after - before).abs() < before,
            "one sample should not double it: {before} became {after}"
        );

        // The same data read as a range, which the outlier does dominate.
        let mut as_range = summary_of(&spiked);
        as_range.samples = FEW_SAMPLES - 1;
        assert!(
            as_range.noise() > 3.0 * after,
            "the range should be far wider: {} vs {after}",
            as_range.noise()
        );
    }

    /// Counts that are all zero have no scale to be noisy against, and must
    /// not divide by it.
    #[test]
    fn zero_counts_have_no_noise() {
        let zeros = summary_of(&[0, 0, 0]);
        assert_eq!(zeros.median, 0.0);
        assert_eq!(zeros.noise(), 0.0);

        // Identical non-zero counts did not move at all.
        assert_eq!(summary_of(&[1000; 10]).noise(), 0.0);
    }

    /// A comparison reports a figure per engine, since which engine scattered
    /// is the useful thing to know.
    #[test]
    fn an_effect_size_reports_noise_for_each_engine() {
        let (a, b) = (summary_of(&[100; 10]), summary_of(&[90, 95, 100, 105, 110]));

        let effect_size = EffectSize {
            arch: "x86_64".into(),
            wasm: "benchmark.wasm".into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            a_engine: "a.so".into(),
            a_mean: a.mean,
            a_median: a.median,
            a_min: a.min,
            a_max: a.max,
            a_p25: a.p25,
            a_p75: a.p75,
            a_samples: a.samples,
            b_engine: "b.so".into(),
            b_mean: b.mean,
            b_median: b.median,
            b_min: b.min,
            b_max: b.max,
            b_p25: b.p25,
            b_p75: b.p75,
            b_samples: b.samples,
            significance_level: 0.01,
            ratio: 1.0,
            confidence_interval: None,
            p_value: 1.0,
        };

        // `a` never moved; `b` did, and with five samples that is its range.
        assert_eq!(effect_size.a_noise(), 0.0);
        assert_eq!(effect_size.a_noise(), a.noise());
        assert_eq!(effect_size.b_noise(), b.noise());
        assert!(
            (effect_size.b_noise() - 20.0).abs() < 1e-9,
            "{}",
            effect_size.b_noise()
        );
    }
}
