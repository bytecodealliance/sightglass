//! Measure the number of ticks/cycles elapsed (e.g. using RDTSC). This is a
//! small wrapper around the `precision` crate to adapt it to the [Measure] API.

use super::{Measure, Measurements};
use lazy_static::lazy_static;
use precision::{Config, Precision, Timestamp};
use sightglass_data::Phase;

lazy_static! {
    static ref PRECISION: Precision = {
        // NB: Disable wall-time measurement, as that requires calibrating the
        // CPU frequency, which adds ~5 seconds on start up time per
        // benchmarking process, and we only care about cycles anyways (less
        // affected by CPU frequency monitors).
        let config = Config::default().wall_time(false);
        Precision::new(config).unwrap()
    };
}

pub struct CycleMeasure(Option<Timestamp>);

impl Default for CycleMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl CycleMeasure {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Measure for CycleMeasure {
    fn start(&mut self, _phase: Phase) {
        let start = PRECISION.now();
        self.0 = Some(start);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        // Deref the lazy-static just once.
        let precision = &*PRECISION;

        let end = precision.now();
        let elapsed = end - self.0.take().expect("must call start before end");

        measurements.add(phase, "cycles".into(), elapsed.ticks());
    }
}
