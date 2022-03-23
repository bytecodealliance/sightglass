//! Measure the wall-clock number of ticks/cycles elapsed (e.g. using RDTSC). This is a small
//! wrapper around the `precision` crate to adapt it to the [Measure] API.

use super::{Measure, Measurements};
use lazy_static::lazy_static;
use precision::{Config, Precision, Timestamp};
use sightglass_data::Phase;

lazy_static! {
    static ref PRECISION: Precision = Precision::new(Config::default()).unwrap();
}

pub struct WallCycleMeasure(Option<Timestamp>);

impl WallCycleMeasure {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Measure for WallCycleMeasure {
    fn start(&mut self, _phase: Phase) {
        let start = PRECISION.now();
        self.0 = Some(start);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        let precision = &*PRECISION; // deref the lazy-static just once.
        let end = precision.now();
        let elapsed = end - self.0.take().expect("an existing timestamp");
        measurements.add(phase, "cycles".into(), elapsed.ticks());
        measurements.add(phase, "nanoseconds".into(), elapsed.as_ns(precision));
    }
}
