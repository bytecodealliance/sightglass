use crate::KeyBuilder;
use anyhow::Result;
use sightglass_data::{EffectSize, Measurement};
use std::collections::BTreeSet;

/// Find the effect size (and confidence interval) of between two different
/// engines (i.e. two different commits of Wasmtime).
///
/// This allows us to justify statements like "we are 99% confident that the new
/// register allocator is 13.6% faster (± 1.7%) than the old register
/// allocator."
///
/// This can only test differences between the results for exactly two different
/// engines. If there aren't exactly two different engines represented in
/// `measurements` then an error is returned.
pub fn effect_size<'a>(
    significance_level: f64,
    measurements: &[Measurement<'a>],
) -> Result<Vec<EffectSize<'a>>> {
    anyhow::ensure!(
        0.0 <= significance_level && significance_level <= 1.0,
        "The significance_level must be between 0.0 and 1.0. \
             Typical values are 0.05 and 0.01 (i.e. 95% and 99% confidence). \
             Found {}.",
        significance_level,
    );

    let keys = KeyBuilder::all().engine(false).keys(measurements);
    let mut results = Vec::with_capacity(keys.len());

    for key in keys {
        let key_measurements: Vec<_> = measurements.iter().filter(|m| key.matches(m)).collect();

        // NB: `BTreeSet` so they're always sorted.
        let engines: BTreeSet<_> = key_measurements.iter().map(|m| &m.engine).collect();
        anyhow::ensure!(
            engines.len() == 2,
            "Can only test significance between exactly two different engines. Found {} \
                 different engines.",
            engines.len()
        );

        let mut engines = engines.into_iter();
        let engine_a = engines.next().unwrap();
        let engine_b = engines.next().unwrap();

        let a: behrens_fisher::Stats = key_measurements
            .iter()
            .filter(|m| m.engine.as_ref() == engine_a)
            .map(|m| m.count as f64)
            .collect();
        let b: behrens_fisher::Stats = key_measurements
            .iter()
            .filter(|m| m.engine.as_ref() == engine_b)
            .map(|m| m.count as f64)
            .collect();

        let ci = behrens_fisher::confidence_interval(significance_level, a, b)?;
        results.push(EffectSize {
            arch: key.arch.unwrap(),
            wasm: key.wasm.unwrap(),
            phase: key.phase.unwrap(),
            event: key.event.unwrap(),
            a_engine: engine_a.clone(),
            a_mean: a.mean,
            b_engine: engine_b.clone(),
            b_mean: b.mean,
            significance_level,
            half_width_confidence_interval: ci,
        });
    }

    Ok(results)
}
