//! Measure instructions retired via perf events.

use super::Measure;
use crate::measure::Measurements;
use perf_event::{events::Hardware, Builder, Counter};
use sightglass_data::Phase;

/// Measure CPU counters.
pub struct InstsRetiredMeasure(Counter);

impl InstsRetiredMeasure {
    pub fn new() -> Self {
        let counter = Builder::new().kind(Hardware::INSTRUCTIONS).build().expect(
            "Unable to create INSTRUCTIONS hardware counter. Does this system actually \
             have such a counter? If it does, your kernel may not fully support this \
             processor.",
        );
        InstsRetiredMeasure(counter)
    }
}

impl Measure for InstsRetiredMeasure {
    fn start(&mut self, _phase: Phase) {
        self.0.reset().unwrap();
        self.0.enable().unwrap()
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        self.0.disable().unwrap();
        let count = self.0.read().unwrap();
        measurements.add(phase, "instructions-retired".into(), count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn sanity() {
        if env::var("CI").is_ok() {
            // If we find ourselves in a CI environment, skip this test: not all CI environments
            // have perf events available (e.g. https://github.community/t/149164). Since this
            // short-circuiting could be problematic in the future, we attempt to warn the user
            // (hoping they have `--nocapture` set).
            println!(
                "Skipping the perf counters sanity test; it seems that this test is running \
                 in a CI environment"
            );
            return;
        }

        let mut measurements = Measurements::new("arch".into(), "engine".into(), "wasm".into());
        let mut measure = InstsRetiredMeasure::new();
        measure.start(Phase::Compilation);
        eprintln!("test test test...");
        measure.end(Phase::Compilation, &mut measurements);
        let measurements = measurements.finish();
        println!("Measurements: {:?}", measurements);
    }
}
