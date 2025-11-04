//! Measure each Sightglass phase using VTune. When using this [Measure] and running a benchmark
//! inside VTune, VTune will mark the phases in the timeline and allow for filtering based on the
//! Sightglass phase. For example:
//!
//! ```text
//! vtune -collect hotspots sightglass-cli benchmark <path to>/benchmark.wasm --measure vtune
//! ```

use super::{Measure, Measurements};
use ittapi::{Domain, Task};
use lazy_static::lazy_static;
use sightglass_data::Phase;

lazy_static! {
    static ref DOMAIN: Domain = Domain::new("sightglass");
}

pub struct VTuneMeasure(Option<Task<'static>>);

impl Default for VTuneMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl VTuneMeasure {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Measure for VTuneMeasure {
    fn start(&mut self, phase: Phase) {
        let previous = self
            .0
            .replace(Task::begin(&DOMAIN, phase.to_string().as_str()));
        assert!(previous.is_none(), "no other VTune task should be running");
    }

    fn end(&mut self, _phase: Phase, _measurements: &mut Measurements) {
        let task = self.0.take();
        let task = task.expect("the VTune Task to be started by Measure::start");
        task.end();
    }
}
