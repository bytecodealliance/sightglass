use super::{Measure, Measurements};
use sightglass_data::Phase;

/// For users that may want to record measurements on their own, this mechanism allows the tool to
/// be used without the overhead of any measurement activity. TODO document example using `perf` and
/// `start`/`end` (how to reference `NoopMeasure::start`?)
pub struct NoopMeasure;

impl Default for NoopMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopMeasure {
    pub fn new() -> Self {
        Self
    }
}

impl Measure for NoopMeasure {
    fn start(&mut self, _phase: Phase) {}
    fn end(&mut self, _phase: Phase, _measurements: &mut Measurements) {}
}
