//! Multiplexing measurement in order to take multiple measures
//! from a single entry point.

use sightglass_data::Phase;

use super::{Measure, Measurements};

pub struct MultiMeasure {
    measures: Vec<Box<dyn Measure>>,
}

impl MultiMeasure {
    pub fn new<I>(measures: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Measure>>,
    {
        Self {
            measures: measures.into_iter().collect(),
        }
    }
}

impl Measure for MultiMeasure {
    fn start(&mut self, phase: Phase) {
        for measure in self.measures.iter_mut() {
            measure.start(phase)
        }
    }

    fn end(&mut self, phase: Phase, measurements: &mut Measurements) {
        for measure in self.measures.iter_mut() {
            measure.end(phase, measurements)
        }
    }
}
