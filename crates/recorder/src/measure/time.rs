//! Measure time elapsed time for each benchmark; note that this is
//! often probably not the best measurement as, unlike cycle counts,
//! there are likely additional external factors that could skew
//! results (such as cpu frequency scaling, etc.).

use std::time::Instant;

use super::{Measure, Measurements};
use sightglass_data::Phase;

pub struct TimeMeasure(Option<Instant>);

impl Default for TimeMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeMeasure {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Measure for TimeMeasure {
    fn start(&mut self, _phase: Phase) {
        let start = Instant::now();
        self.0 = Some(start);
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        let elapsed = self.0.take().unwrap().elapsed();
        measurements.add(phase, "nanoseconds".into(), elapsed.as_nanos() as u64);
    }
}
