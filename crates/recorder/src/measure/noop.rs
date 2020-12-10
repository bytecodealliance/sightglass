use super::{Measure, Measurement};

/// For users that may want to record measurements on their own, this mechanism allows the tool to
/// be used without the overhead of any measurement activity. TODO document example using `perf` and
/// `start`/`end` (how to reference `NoopMeasure::start`?)
pub struct NoopMeasure;
impl NoopMeasure {
    pub fn new() -> Self {
        Self
    }
}

impl Measure for NoopMeasure {
    fn start(&mut self) {}
    fn end(&mut self) -> Measurement {
        Measurement::Noop
    }
}
