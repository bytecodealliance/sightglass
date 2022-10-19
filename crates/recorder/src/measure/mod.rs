use sightglass_data::{Measurement, Phase};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    str::FromStr,
};

/// An in-progress collection of measurements that are currently being recorded.
pub struct Measurements<'a> {
    arch: &'a str,
    engine: &'a str,
    wasm: &'a str,
    process: u32,
    iteration: u32,
    measurements: Vec<Measurement<'a>>,
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
    pub fn add(&mut self, phase: Phase, event: Cow<'a, str>, count: u64) {
        self.measurements.push(Measurement {
            arch: self.arch.into(),
            engine: self.engine.into(),
            wasm: self.wasm.into(),
            process: self.process,
            iteration: self.iteration,
            phase,
            event,
            count,
        });
    }

    /// When all measurements have been recorded, call this method to get the
    /// underlying measurements data.
    pub fn finish(self) -> Vec<Measurement<'a>> {
        self.measurements
    }
}

/// Recording measurements.
///
/// This is primary trait for implementing different measurement mechanisms. The idea is that
/// instantiating a measurement may take some time so it should be done once in `new` and data is
/// collected by calling `start` and `end`. In a recording library like this one an error from
/// implementors of this should result in a panic--not much point in recording anything if our
/// measurement mechanism is broken. The same logic applies to misuse of the API (e.g. calling `end`
/// before `start`).
pub trait Measure: 'static {
    /// Start measuring.
    fn start(&mut self, phase: Phase);

    /// Finish measuring and add the measurements taken between `start` and
    /// `end` to `measurements`.
    fn end(&mut self, phase: Phase, measurements: &mut Measurements);
}

#[cfg(target_os = "linux")]
pub mod counters;
#[cfg(target_os = "linux")]
pub mod insts;

pub mod cycles;
pub mod noop;
pub mod vtune;

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

    /// Measure cycles using, e.g., `RDTSC`.
    Cycles,

    /// Measure using VTune; this will return `0` values.
    VTune,

    /// Measure a combination of HW counters using `perf_event_open`.
    #[cfg(target_os = "linux")]
    PerfCounters,

    /// Measure instructions retired.
    #[cfg(target_os = "linux")]
    InstsRetired,
}

impl fmt::Display for MeasureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeasureType::Noop => write!(f, "noop"),
            MeasureType::Cycles => write!(f, "cycles"),
            MeasureType::VTune => write!(f, "vtune"),
            #[cfg(target_os = "linux")]
            MeasureType::PerfCounters => write!(f, "perf-counters"),
            #[cfg(target_os = "linux")]
            MeasureType::InstsRetired => write!(f, "insts-retired"),
        }
    }
}

impl FromStr for MeasureType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            "cycles" => Ok(Self::Cycles),
            "vtune" => Ok(Self::VTune),
            #[cfg(target_os = "linux")]
            "perf-counters" => Ok(Self::PerfCounters),
            #[cfg(target_os = "linux")]
            "insts-retired" => Ok(Self::InstsRetired),
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
            Self::Cycles => Box::new(cycles::CycleMeasure::new()),
            Self::VTune => Box::new(vtune::VTuneMeasure::new()),
            #[cfg(target_os = "linux")]
            Self::PerfCounters => Box::new(counters::CounterMeasure::new()),
            #[cfg(target_os = "linux")]
            Self::InstsRetired => Box::new(insts::InstsRetiredMeasure::new()),
        }
    }
}

impl Measure for Box<dyn Measure> {
    fn start(&mut self, phase: Phase) {
        (**self).start(phase);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        (**self).end(phase, measurements)
    }
}
