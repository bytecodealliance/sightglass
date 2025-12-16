use crate::keys::KeyBuilder;
use anyhow::Result;
use sightglass_data::{Measurement, Summary};
use std::io::Write;

/// Summarize measurements grouped by: architecture, engine, flags, benchmark file, phase and event.
pub fn calculate<'a>(measurements: &[Measurement<'a>]) -> Vec<Summary<'a>> {
    let mut summaries = Vec::new();
    for k in KeyBuilder::all().keys(measurements) {
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
pub fn mean(numbers: &[u64]) -> f64 {
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

/// Calculate the standard deviation of a slice of numbers.
pub fn std_deviation(numbers: &[u64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let mean = mean(numbers);
    let variance = numbers
        .iter()
        .map(|&c| {
            let diff = c as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / numbers.len() as f64;
    variance.sqrt()
}

/// Calculate a specific percentile from already-sorted data.
pub fn percentile_from_sorted(sorted_numbers: &[u64], p: f64) -> f64 {
    if sorted_numbers.is_empty() {
        return 0.0;
    }

    let index = (p / 100.0) * (sorted_numbers.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        sorted_numbers[lower] as f64
    } else {
        let weight = index - lower as f64;
        (sorted_numbers[lower] as f64) * (1.0 - weight) + (sorted_numbers[upper] as f64) * weight
    }
}

/// Calculate the coefficient of variation (CV) as a percentage.
pub fn coefficient_of_variation(numbers: &[u64]) -> f64 {
    let mean_val = mean(numbers);
    if mean_val == 0.0 {
        0.0
    } else {
        (std_deviation(numbers) / mean_val) * 100.0
    }
}

/// Returns the median value of a group.
fn median(numbers: &mut [u64]) -> u64 {
    numbers.sort();
    // Note this index is *the* right one for odd lengths (the median value among 2p+1 values is at
    // index p), and *a* right one for even lengths.
    numbers[numbers.len() / 2]
}

/// Write a vector of [Summary] structures to the passed `output_file` in human-readable form.
pub fn write(mut summaries: Vec<Summary<'_>>, output_file: &mut dyn Write) -> Result<()> {
    // TODO this sorting is not using `arch` which is not guaranteed to be the same in result sets;
    // potentially this could re-use `Key` functionality.
    summaries.sort_by(|x, y| {
        x.phase
            .cmp(&y.phase)
            .then_with(|| x.wasm.cmp(&y.wasm))
            .then_with(|| x.event.cmp(&y.event))
            .then_with(|| x.engine.cmp(&y.engine))
    });

    let mut last_phase = None;
    let mut last_wasm = None;
    let mut last_event = None;
    for summary in summaries {
        if last_phase != Some(summary.phase) {
            last_phase = Some(summary.phase);
            last_wasm = None;
            last_event = None;
            writeln!(output_file, "{}", summary.phase)?;
        }

        if last_wasm.as_ref() != Some(&summary.wasm) {
            last_wasm = Some(summary.wasm.clone());
            last_event = None;
            writeln!(output_file, "  {}", summary.wasm)?;
        }

        if last_event.as_ref() != Some(&summary.event) {
            last_event = Some(summary.event.clone());
            writeln!(output_file, "    {}", summary.event)?;
        }

        writeln!(
            output_file,
            "      [{} {:.2} {}] {}",
            summary.min, summary.mean, summary.max, summary.engine,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sightglass_data::{Engine, Phase};

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
                event: "cycles".into(),
                count,
            }
        }

        let measurements = vec![measurement(1), measurement(0), measurement(2)];

        assert_eq!(
            calculate(&measurements),
            vec![Summary {
                arch: "x86".into(),
                engine: "wasmtime".into(),
                wasm: "bench.wasm".into(),
                phase: Phase::Compilation,
                event: "cycles".into(),
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
                event: "cycles".into(),
                count,
            }
        }
        let measurements = vec![
            measurement(Phase::Compilation, 0),
            measurement(Phase::Execution, 1),
            measurement(Phase::Compilation, 2),
        ];

        assert_eq!(calculate(&measurements).len(), 2);
    }

    #[test]
    fn differing_engine_flags() {
        use std::borrow::Cow;

        fn measurement<'a>(flags: Option<Cow<'a, str>>, count: u64) -> Measurement<'a> {
            Measurement {
                arch: "x86".into(),
                engine: Engine {
                    name: "wasmtime".into(),
                    flags,
                },
                wasm: "bench.wasm".into(),
                process: 42,
                iteration: 0,
                phase: Phase::Execution,
                event: "cycles".into(),
                count,
            }
        }
        let measurements = vec![
            measurement(Some("-Wfoo=bar".into()), 0),
            measurement(Some("-Wfoo=bar".into()), 1),
            measurement(Some("-Wdead=beeef".into()), 2),
            measurement(None, 3),
        ];

        assert_eq!(calculate(&measurements).len(), 3);
    }

    #[test]
    fn test_std_deviation() {
        let numbers = vec![2, 4, 4, 4, 5, 5, 7, 9];
        let std_dev = std_deviation(&numbers);
        assert!((std_dev - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_percentile_from_sorted() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        numbers.sort();
        assert_eq!(percentile_from_sorted(&numbers, 50.0), 5.5);
        assert_eq!(percentile_from_sorted(&numbers, 25.0), 3.25);
        assert_eq!(percentile_from_sorted(&numbers, 75.0), 7.75);
    }

    #[test]
    fn test_coefficient_of_variation() {
        let numbers = vec![2, 4, 4, 4, 5, 5, 7, 9];
        let cv = coefficient_of_variation(&numbers);
        assert!(cv > 0.0);
        assert!(cv < 100.0); // Should be a reasonable CV percentage
    }
}
