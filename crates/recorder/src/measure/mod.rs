use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};

/// This is primary trait for implementing different measurement mechanisms. The idea is that
/// instantiating a measurement may take some time so it should be done once in `new` and data is
/// collected by calling `start` and `end`. In a recording library like this one an error from
/// implementors of this should result in a panic--not much point in recording anything if our
/// measurement mechanism is broken. The same logic applies to misuse of the API (e.g. calling `end`
/// before `start`).
pub trait Measure {
    /// Start measuring.
    fn start(&mut self);

    /// Finish measuring and return the measurement between `start` and `end`.
    fn end(&mut self) -> Measurement;
}

pub mod counters;
pub mod noop;
pub mod wall_cycles;

/// [MeasureType] enumerates the implementations of [Measure] and allows us to `build` an instance
/// from its name:
/// ```
/// use sightglass_recorder::measure::MeasureType;
/// let ty: MeasureType = "noop".parse().unwrap();
/// let measure = ty.build();
/// ```
#[derive(Debug, Clone, Copy)]
pub enum MeasureType {
    /// No measurement.
    Noop,
    /// Measure wall-clock time using, e.g., `RDTSC`.
    WallCycles,
    /// Measure a combination of HW counters using `perf_event_open`.
    PerfCounters,
}
impl FromStr for MeasureType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            "wall-cycles" => Ok(Self::WallCycles),
            "perf-counters" => Ok(Self::PerfCounters),
            _ => Err("unknown measure type"),
        }
    }
}
impl MeasureType {
    /// Build a dynamic instance of a [Measure]. The recording infrastructure does not need to know
    /// exactly what type of [Measure] we want to use, just that it can `start` and `end`
    /// measurements.
    pub fn build(&self) -> Box<dyn Measure> {
        match self {
            Self::Noop => Box::new(noop::NoopMeasure::new()),
            Self::WallCycles => Box::new(wall_cycles::WallCycleMeasure::new()),
            Self::PerfCounters => Box::new(counters::CounterMeasure::new()),
        }
    }
}

impl Measure for Box<dyn Measure> {
    fn start(&mut self) {
        (**self).start();
    }

    fn end(&mut self) -> Measurement {
        (**self).end()
    }
}

/// Enumerate the types of measurements returned by a [Measure]. This would also be possible with a
/// `Box<dyn Measurement>` where `Measurement` had super-traits `Debug`, `Serialize`, and
/// `Deserialize` but that would involve more complexity (e.g. using the `typetag` and
/// `erased_serde` crates to properly implement `Serialize` and `Deserialize`).
#[derive(Debug, Serialize, Deserialize)]
pub enum Measurement {
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "wall-cycles")]
    WallCycles(u64),
    #[serde(rename = "perf-counters")]
    PerfCounters(counters::PerfCounters),
}
