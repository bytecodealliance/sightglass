//! Provides [UploadMeasurement], a data type to convert to before uploading
//! Sightglass [Measurement] objects to a server.
use serde::{Deserialize, Serialize};
use sightglass_data::{Measurement, Phase};
use sightglass_fingerprint::{Benchmark, Engine, Machine};
use std::{borrow::Cow, collections::HashMap};

/// A conversion of a [Measurement], with fields replaced by fingerprinting.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadMeasurement<'a> {
    /// The ID of the machine on which this measurement was taken; this relies
    /// on `upload` to insert the data for this ID.
    pub machine: Cow<'a, str>,

    /// The ID of the engine in which this measurement was taken; this relies on
    /// `upload` to insert the data for this ID.
    pub engine: Cow<'a, str>,

    /// The ID of the benchmark with which this measurement was taken; this
    /// relies on `upload` to insert the data for this ID.
    pub benchmark: Cow<'a, str>,

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

    /// When the measurement was collected into a package (not necessarily when
    /// it was measured).
    pub datetime: Cow<'a, str>,
}

impl<'a> UploadMeasurement<'a> {
    pub fn convert(
        machine: &'a str,
        engine: &'a str,
        benchmark: &'a str,
        datetime: &'a str,
        measurement: &'a Measurement,
    ) -> Self {
        Self {
            machine: Cow::Borrowed(machine),
            engine: Cow::Borrowed(engine),
            benchmark: Cow::Borrowed(benchmark),
            process: measurement.process,
            iteration: measurement.iteration,
            phase: measurement.phase,
            event: Cow::Borrowed(measurement.event.as_ref()),
            count: measurement.count,
            datetime: Cow::Borrowed(datetime),
        }
    }

    pub fn map_and_convert(
        machine: &'a str,
        engines: &'a HashMap<Cow<'_, str>, String>,
        benchmarks: &'a HashMap<Cow<'_, str>, String>,
        datetime: &'a str,
        measurement: &'a Measurement,
    ) -> Self {
        let engine = engines
            .get(measurement.engine.name.as_ref())
            .unwrap()
            .as_ref();
        let benchmark = benchmarks.get(measurement.wasm.as_ref()).unwrap().as_ref();
        Self::convert(machine, engine, benchmark, datetime, measurement)
    }
}

/// This container captures all of the measurement data from a single machine in
/// a format that can be exported and later uploaded.
#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementPackage<'a> {
    /// Store all the original measurements for upload.
    pub measurements: Vec<Measurement<'a>>,
    /// Map each engine path to its fingerprinted data.
    pub engines: HashMap<Cow<'a, str>, Engine>,
    /// Map each benchmark path to its fingerprinted data.
    pub benchmarks: HashMap<Cow<'a, str>, Benchmark>,
    /// Collect the machine fingerprint.
    pub machine: Machine,
    /// When the measurements were collected into a package (not necessarily
    /// when they were measured).
    pub datetime: Cow<'a, str>,
}
