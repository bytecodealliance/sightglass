use crate::keys::KeyBuilder;
use sightglass_data::{Measurement, Summary};

/// Summarize measurements grouped by: architecture, engine, benchmark file, phase and event.
pub fn summarize<'a>(measurements: &[Measurement<'a>]) -> Vec<Summary<'a>> {
    let mut summaries = Vec::new();
    for k in KeyBuilder::all().keys(&measurements) {
        let mut grouped_counts: Vec<_> = measurements
            .iter()
            .filter(|m| k.matches(m))
            .map(|m| m.count)
            .collect();
        summaries.push(Summary {
            arch: k.arch.unwrap(),
            engine: k.engine.unwrap(),
            wasm: k.wasm.unwrap(),
            phase: k.phase.unwrap(),
            event: k.event.unwrap(),
            min: grouped_counts
                .iter()
                .cloned()
                .min()
                .expect("at least one element"),
            max: grouped_counts
                .iter()
                .cloned()
                .max()
                .expect("at least one element"),
            mean: mean(&grouped_counts),
            mean_deviation: mean_deviation(&grouped_counts),
            median: median(grouped_counts.as_mut_slice()),
        })
    }
    summaries
}

/// Calculate the arithmetic mean of a slice of numbers.
fn mean(numbers: &[u64]) -> f64 {
    numbers.iter().sum::<u64>() as f64 / numbers.len() as f64
}

/// Calculate the mean deviation (note: not standard deviation) of a slice of numbers.
fn mean_deviation(numbers: &[u64]) -> f64 {
    let mean = mean(numbers);
    numbers
        .iter()
        .map(|&c| (mean - c as f64).abs())
        .sum::<f64>()
        / numbers.len() as f64
}

/// Returns the median value of a group.
fn median(numbers: &mut [u64]) -> u64 {
    numbers.sort();
    // Note this index is *the* right one for odd lengths (the median value among 2p+1 values is at
    // index p), and *a* right one for even lengths.
    numbers[numbers.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use sightglass_data::Phase;

    #[test]
    fn simple_statistics() {
        fn measurement<'a>(count: u64) -> Measurement<'a> {
            Measurement {
                arch: "x86".into(),
                engine: "wasmtime".into(),
                wasm: "bench.wasm".into(),
                process: 42,
                iteration: 0,
                phase: Phase::Compilation,
                event: "wall-cycles".into(),
                count,
            }
        }

        let measurements = vec![measurement(1), measurement(0), measurement(2)];

        assert_eq!(
            summarize(&measurements),
            vec![Summary {
                arch: "x86".into(),
                engine: "wasmtime".into(),
                wasm: "bench.wasm".into(),
                phase: Phase::Compilation,
                event: "wall-cycles".into(),
                mean: 1.0,
                min: 0,
                median: 1,
                max: 2,
                mean_deviation: 2f64 / 3f64,
            }]
        );
    }

    #[test]
    fn interleaving_phases() {
        fn measurement<'a>(phase: Phase, count: u64) -> Measurement<'a> {
            Measurement {
                arch: "x86".into(),
                engine: "wasmtime".into(),
                wasm: "bench.wasm".into(),
                process: 42,
                iteration: 0,
                phase,
                event: "wall-cycles".into(),
                count,
            }
        }
        let measurements = vec![
            measurement(Phase::Compilation, 0),
            measurement(Phase::Execution, 1),
            measurement(Phase::Compilation, 2),
        ];

        assert_eq!(summarize(&measurements).len(), 2);
    }
}
