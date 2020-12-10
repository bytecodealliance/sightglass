//! Measure the wall-clock number of ticks/cycles elapsed (e.g. using RDTSC). This is a small
//! wrapper around the `precision` crate to adapt it to the [Measure] API.
use super::{Measure, Measurement};
use precision::{Config, Precision, Timestamp};

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

    fn end(&mut self) -> Measurement {
        let end = self.0.now();
        let elapsed = (end - self.1.expect("an existing timestamp")).ticks();
        Measurement::WallCycles(elapsed)
    }
}
