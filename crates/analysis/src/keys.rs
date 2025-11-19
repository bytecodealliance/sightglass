use sightglass_data::{Measurement, Phase};
use std::{borrow::Cow, collections::BTreeSet};

/// A builder for finding keys in a set of measurements.
#[derive(Copy, Clone)]
pub struct KeyBuilder {
    arch: bool,
    engine: bool,
    wasm: bool,
    phase: bool,
    event: bool,
}

impl KeyBuilder {
    /// Construct a new `KeyBuilder` that differentiates between all metrics.
    pub fn all() -> Self {
        KeyBuilder {
            arch: true,
            engine: true,
            wasm: true,
            phase: true,
            event: true,
        }
    }

    /// Construct a new `KeyBuilder` that does not differentiate between any metrics.
    pub fn none() -> Self {
        KeyBuilder {
            arch: false,
            engine: false,
            wasm: false,
            phase: false,
            event: false,
        }
    }

    /// Whether to group keys by architecture or not.
    pub fn arch(mut self, arch: bool) -> Self {
        self.arch = arch;
        self
    }

    /// Whether to group keys by engine or not.
    pub fn engine(mut self, engine: bool) -> Self {
        self.engine = engine;
        self
    }

    /// Whether to group keys by wasm or not.
    pub fn wasm(mut self, wasm: bool) -> Self {
        self.wasm = wasm;
        self
    }

    /// Whether to group keys by phase or not.
    pub fn phase(mut self, phase: bool) -> Self {
        self.phase = phase;
        self
    }

    /// Whether to group keys by event or not.
    pub fn event(mut self, event: bool) -> Self {
        self.event = event;
        self
    }

    /// Extract the keys for the groups of measurements to aggregate.
    pub fn keys<'a>(self, measurements: &[Measurement<'a>]) -> Vec<Key<'a>> {
        let set: BTreeSet<_> = measurements
            .iter()
            .cloned()
            .map(|m| Key {
                arch: if self.arch { Some(m.arch) } else { None },
                engine: if self.engine { Some(m.engine) } else { None },
                wasm: if self.wasm { Some(m.wasm) } else { None },
                phase: if self.phase { Some(m.phase) } else { None },
                event: if self.event { Some(m.event) } else { None },
            })
            .collect();
        set.into_iter().collect()
    }
}

/// A key for grouping measurements together.
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Key<'a> {
    pub arch: Option<Cow<'a, str>>,
    pub engine: Option<Cow<'a, str>>,
    pub wasm: Option<Cow<'a, str>>,
    pub phase: Option<Phase>,
    pub event: Option<Cow<'a, str>>,
}

impl Key<'_> {
    /// Does the given measurement match this key?
    pub fn matches(&self, m: &Measurement) -> bool {
        self.arch.as_ref().is_none_or(|x| *x == m.arch)
            && self.engine.as_ref().is_none_or(|x| *x == m.engine)
            && self.wasm.as_ref().is_none_or(|x| *x == m.wasm)
            && self.phase.as_ref().is_none_or(|x| *x == m.phase)
            && self.event.as_ref().is_none_or(|x| *x == m.event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sightglass_data::Phase;

    #[test]
    fn matching_fields() {
        let key = Key {
            arch: Some("x86".into()),
            engine: Some("wasmtime".into()),
            wasm: Some("bench.wasm".into()),
            phase: Some(Phase::Compilation),
            event: Some("cycles".into()),
        };

        // More test cases are needed, but this provides a sanity check for the matched key and
        // measurement fields.
        assert!(key.matches(&Measurement {
            arch: "x86".into(),
            engine: "wasmtime".into(),
            wasm: "bench.wasm".into(),
            process: 42,
            iteration: 0,
            phase: Phase::Compilation,
            event: "cycles".into(),
            count: 42,
        }));
    }
}
