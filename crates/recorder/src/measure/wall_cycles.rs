//! Measure the wall-clock number of ticks/cycles elapsed (e.g. using RDTSC). This is a small
//! wrapper around the `precision` crate to adapt it to the [Measure] API.

use super::{Measure, Measurements};
use precision::{Config, Precision, Timestamp};
use sightglass_data::Phase;

pub struct WallCycleMeasure(Precision, Option<Timestamp>);

impl WallCycleMeasure {
    pub fn new() -> Self {
        let precision = Precision::new(Config::default()).unwrap();
        Self(precision, None)
    }
}

impl Measure for WallCycleMeasure {
    fn start(&mut self) {
        self.1 = Some(self.0.now());
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        let end = self.0.now();
        let elapsed = end - self.1.take().expect("an existing timestamp");
        measurements.add(phase, "cycles".into(), elapsed.ticks());
        measurements.add(phase, "nanoseconds".into(), elapsed.as_ns(&self.0));
    }
}
