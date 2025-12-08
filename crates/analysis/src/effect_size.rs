use crate::keys::KeyBuilder;
use anyhow::Result;
use sightglass_data::{EffectSize, Engine, Measurement, Phase, Summary};
use std::{collections::BTreeSet, io::Write};

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
pub fn calculate<'a>(
    significance_level: f64,
    measurements: &[Measurement<'a>],
) -> Result<Vec<EffectSize<'a>>> {
    anyhow::ensure!(
        (0.0..=1.0).contains(&significance_level),
        "The significance_level must be between 0.0 and 1.0. \
             Typical values are 0.05 and 0.01 (i.e. 95% and 99% confidence). \
             Found {}.",
        significance_level,
    );

    let keys = KeyBuilder::all()
        .engine(false)
        .engine_flags(false)
        .keys(measurements);
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
            .filter(|m| &m.engine == engine_a)
            .map(|m| m.count as f64)
            .collect();
        let b: behrens_fisher::Stats = key_measurements
            .iter()
            .filter(|m| &m.engine == engine_b)
            .map(|m| m.count as f64)
            .collect();

        let ci = behrens_fisher::confidence_interval(1.0 - significance_level, a, b)?;
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

/// Write a vector of [EffectSize] structures to the passed `output_file` in human-readable form.
/// The `summaries` are needed
pub fn write(
    mut effect_sizes: Vec<EffectSize<'_>>,
    summaries: &[Summary<'_>],
    significance_level: f64,
    output_file: &mut dyn Write,
) -> Result<()> {
    // Sort the effect sizes so that we focus on statistically significant results before
    // insignificant results and larger relative effect sizes before smaller relative effect sizes.
    effect_sizes.sort_by(|x, y| {
        y.is_significant().cmp(&x.is_significant()).then_with(|| {
            let x_speedup = x.a_speed_up_over_b().0.max(x.b_speed_up_over_a().0);
            let y_speedup = y.a_speed_up_over_b().0.max(y.b_speed_up_over_a().0);
            y_speedup.partial_cmp(&x_speedup).unwrap()
        })
    });

    for effect_size in effect_sizes {
        writeln!(output_file)?;
        writeln!(
            output_file,
            "{} :: {} :: {}",
            effect_size.phase, effect_size.event, effect_size.wasm
        )?;
        writeln!(output_file)?;

        // For readability, trim the shared prefix from our two engine names.
        //
        // Furthermore, there are a few special cases:
        // 1. If the engines are the same, show just the flags.
        // 2. If not, show the computed full label with common prefix removed.
        let (a_eng_label, b_eng_label) =
            effect_size.a_engine.relative_labels(&effect_size.b_engine);

        if effect_size.is_significant() {
            writeln!(
                output_file,
                "  Δ = {:.2} ± {:.2} (confidence = {}%)",
                (effect_size.b_mean - effect_size.a_mean).abs(),
                effect_size.half_width_confidence_interval.abs(),
                (1.0 - significance_level) * 100.0,
            )?;
            writeln!(output_file)?;

            if effect_size.a_mean < effect_size.b_mean {
                let ratio = effect_size.b_mean / effect_size.a_mean;
                let ratio_ci = effect_size.half_width_confidence_interval / effect_size.a_mean;
                writeln!(
                    output_file,
                    "  {a_eng_label} is {ratio_min:.2}x to {ratio_max:.2}x faster than {b_eng_label}!",
                    ratio_min = ratio - ratio_ci,
                    ratio_max = ratio + ratio_ci,
                )?;
            } else {
                let ratio = effect_size.a_mean / effect_size.b_mean;
                let ratio_ci = effect_size.half_width_confidence_interval / effect_size.b_mean;
                writeln!(
                    output_file,
                    "  {b_eng_label} is {ratio_min:.2}x to {ratio_max:.2}x faster than {a_eng_label}!",
                    ratio_min = ratio - ratio_ci,
                    ratio_max = ratio + ratio_ci,
                )?;
            }
        } else {
            writeln!(output_file, "  No difference in performance.")?;
        }
        writeln!(output_file)?;

        let get_summary = |engine: &Engine, wasm: &str, phase: Phase, event: &str| {
            // TODO this sorting is not using `arch` which is not guaranteed to be the same in
            // result sets; potentially this could re-use `Key` functionality.
            summaries
                .iter()
                .find(|s| {
                    &s.engine == engine && s.wasm == wasm && s.phase == phase && s.event == event
                })
                .unwrap()
        };

        let a_summary = get_summary(
            &effect_size.a_engine,
            &effect_size.wasm,
            effect_size.phase,
            &effect_size.event,
        );
        writeln!(
            output_file,
            "  [{} {:.2} {}] {}",
            a_summary.min, a_summary.mean, a_summary.max, a_eng_label,
        )?;

        let b_summary = get_summary(
            &effect_size.b_engine,
            &effect_size.wasm,
            effect_size.phase,
            &effect_size.event,
        );
        writeln!(
            output_file,
            "  [{} {:.2} {}] {}",
            b_summary.min, b_summary.mean, b_summary.max, b_eng_label,
        )?;
    }

    Ok(())
}
