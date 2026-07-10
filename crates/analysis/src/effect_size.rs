use crate::keys::KeyBuilder;
use anyhow::Result;
use sightglass_data::{EffectSize, Engine, Measurement, Phase, Summary};
use std::collections::BTreeSet;
use termcolor::{ColorSpec, WriteColor};

/// Find the effect size (and confidence interval) between different engines
/// (i.e. different commits of Wasmtime).
///
/// This allows us to justify statements like "we are 99% confident that the new
/// register allocator is 13.6% faster (± 1.7%) than the old register
/// allocator."
///
/// The `measurements` must contain results for two or more different engines.
/// Each returned [`EffectSize`] compares exactly two different engines: for
/// every group of measurements that share an architecture, benchmark, phase,
/// and event, one `EffectSize` is produced for each pair of engines present in
/// that group. If fewer than two different engines are represented in
/// `measurements` then an error is returned.
pub fn calculate<'a>(
    significance_level: f64,
    measurements: &[Measurement<'a>],
) -> Result<Vec<EffectSize<'a>>> {
    anyhow::ensure!(
        (0.0..=1.0).contains(&significance_level),
        "The significance_level must be between 0.0 and 1.0. \
             Typical values are 0.05 and 0.01 (i.e. 95% and 99% confidence). \
             Found {significance_level}."
    );

    // We need at least two different engines to have anything to compare.
    let all_engines: BTreeSet<_> = measurements.iter().map(|m| &m.engine).collect();
    anyhow::ensure!(
        all_engines.len() >= 2,
        "Comparing effect sizes requires two or more different engines. Found {} \
             different engines.",
        all_engines.len()
    );

    let keys = KeyBuilder::all()
        .engine(false)
        .engine_flags(false)
        .keys(measurements);
    let mut results = Vec::new();

    for key in keys {
        let key_measurements: Vec<_> = measurements.iter().filter(|m| key.matches(m)).collect();

        let engines: Vec<_> = key_measurements
            .iter()
            .map(|m| &m.engine)
            // Deduplicate. Use a `BTreeSet` to keep them sorted.
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        // Create an `EffectSize` comparing each pair of distinct engines within
        // this group of measurements.
        for (i, engine_a) in engines.iter().enumerate() {
            for engine_b in &engines[i + 1..] {
                let a: behrens_fisher::Stats = key_measurements
                    .iter()
                    .filter(|m| &m.engine == *engine_a)
                    .map(|m| m.count as f64)
                    .collect();
                let b: behrens_fisher::Stats = key_measurements
                    .iter()
                    .filter(|m| &m.engine == *engine_b)
                    .map(|m| m.count as f64)
                    .collect();

                let ci = behrens_fisher::confidence_interval(1.0 - significance_level, a, b)
                    .unwrap_or(0.0);
                results.push(EffectSize {
                    arch: key.arch.clone().unwrap(),
                    wasm: key.wasm.clone().unwrap(),
                    phase: key.phase.unwrap(),
                    event: key.event.clone().unwrap(),
                    a_engine: (*engine_a).clone(),
                    a_mean: a.mean,
                    b_engine: (*engine_b).clone(),
                    b_mean: b.mean,
                    significance_level,
                    half_width_confidence_interval: ci,
                });
            }
        }
    }

    Ok(results)
}

/// Write a vector of [EffectSize] structures to the passed `output_file` in human-readable form.
/// The `summaries` are needed
pub fn write(
    mut effect_sizes: Vec<EffectSize<'_>>,
    summaries: &[Summary<'_>],
    significance_level: f64,
    output_file: &mut dyn WriteColor,
) -> Result<()> {
    // Sort the effect sizes so that our "Sum Total" results come first, then we
    // focus on statistically significant results before insignificant results
    // and larger relative effect sizes before smaller relative effect sizes.
    effect_sizes.sort_by(|x, y| {
        (y.wasm == "Sum Total")
            .cmp(&(x.wasm == "Sum Total"))
            .then_with(|| y.is_significant().cmp(&x.is_significant()))
            .then_with(|| {
                let x_speedup = x.a_speed_up_over_b().0.max(x.b_speed_up_over_a().0);
                let y_speedup = y.a_speed_up_over_b().0.max(y.b_speed_up_over_a().0);
                y_speedup.partial_cmp(&x_speedup).unwrap()
            })
    });

    // Bold variants of the engine (blue) and ratio (green) colors, plus plain
    // bold, for the "faster than" message.
    let bold = {
        let mut spec = ColorSpec::new();
        spec.set_bold(true);
        spec
    };
    let bold_engine = {
        let mut spec = crate::engine_spec();
        spec.set_bold(true);
        spec
    };
    let bold_ratio = {
        let mut spec = crate::ratio_spec();
        spec.set_bold(true);
        spec
    };

    for effect_size in effect_sizes {
        writeln!(output_file)?;
        crate::write_in(
            output_file,
            &crate::phase_spec(),
            &effect_size.phase.to_string(),
        )?;
        write!(output_file, " :: ")?;
        crate::write_in(output_file, &crate::event_spec(), &effect_size.event)?;
        write!(output_file, " :: ")?;
        crate::write_in(
            output_file,
            &crate::benchmark_spec(),
            crate::benchmark_label(&effect_size.wasm),
        )?;
        writeln!(output_file)?;
        writeln!(output_file)?;

        // For readability, trim the shared prefix from our two engine names.
        //
        // Furthermore, there are a few special cases:
        // 1. If the engines are the same, show just the flags.
        // 2. If not, show the computed full label with common prefix removed.
        let (a_eng_label, b_eng_label) =
            effect_size.a_engine.relative_labels(&effect_size.b_engine);

        if effect_size.is_significant() {
            write!(output_file, "    ")?;
            crate::write_in(output_file, &crate::orange(), "Δ")?;
            write!(
                output_file,
                " = {:.2} ± {:.2} ",
                (effect_size.b_mean - effect_size.a_mean).abs(),
                effect_size.half_width_confidence_interval.abs(),
            )?;
            crate::write_in(
                output_file,
                &crate::stats_parenthetical_spec(),
                &format!(
                    "(confidence = {}%)",
                    (1.0 - significance_level) * 100.0,
                ),
            )?;
            writeln!(output_file)?;
            writeln!(output_file)?;

            // Whichever engine has the smaller mean is the faster one.
            let (faster, slower, ratio, ratio_ci) = if effect_size.a_mean < effect_size.b_mean {
                (
                    &a_eng_label,
                    &b_eng_label,
                    effect_size.b_mean / effect_size.a_mean,
                    effect_size.half_width_confidence_interval / effect_size.a_mean,
                )
            } else {
                (
                    &b_eng_label,
                    &a_eng_label,
                    effect_size.a_mean / effect_size.b_mean,
                    effect_size.half_width_confidence_interval / effect_size.b_mean,
                )
            };

            write!(output_file, "    ")?;
            crate::write_in(output_file, &bold_engine, faster)?;
            crate::write_in(output_file, &bold, " is ")?;
            crate::write_in(
                output_file,
                &bold_ratio,
                &format!("{:.2}x", ratio - ratio_ci),
            )?;
            crate::write_in(output_file, &bold, " to ")?;
            crate::write_in(
                output_file,
                &bold_ratio,
                &format!("{:.2}x", ratio + ratio_ci),
            )?;
            crate::write_in(output_file, &bold, " faster than ")?;
            crate::write_in(output_file, &bold_engine, slower)?;
            crate::write_in(output_file, &bold, "!")?;
            writeln!(output_file)?;
        } else {
            writeln!(output_file, "    No difference in performance.")?;
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
        let b_summary = get_summary(
            &effect_size.b_engine,
            &effect_size.wasm,
            effect_size.phase,
            &effect_size.event,
        );

        let rows = vec![
            crate::summary_row(a_summary, a_eng_label),
            crate::summary_row(b_summary, b_eng_label),
        ];
        crate::write_stats_table(output_file, "    ", &rows)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement<'a>(engine: &'a str, count: u64) -> Measurement<'a> {
        Measurement {
            arch: "x86_64".into(),
            engine: Engine {
                name: engine.into(),
                flags: None,
            },
            wasm: "bench.wasm".into(),
            process: 0,
            iteration: 0,
            phase: Phase::Execution,
            event: "cycles".into(),
            count,
        }
    }

    #[test]
    fn effect_size_for_each_pair_of_engines() -> Result<()> {
        // Three engines within a single (arch, wasm, phase, event) group should
        // yield one `EffectSize` per unordered pair of engines: (a, b), (a, c),
        // and (b, c).
        let mut measurements = vec![];
        for (engine, base) in [("a", 100), ("b", 200), ("c", 300)] {
            for i in 0..5 {
                measurements.push(measurement(engine, base + i));
            }
        }

        let effect_sizes = calculate(0.01, &measurements)?;
        assert_eq!(effect_sizes.len(), 3);

        let pairs: Vec<(String, String)> = effect_sizes
            .iter()
            .map(|e| (e.a_engine.name.to_string(), e.b_engine.name.to_string()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("a".to_string(), "b".to_string()),
                ("a".to_string(), "c".to_string()),
                ("b".to_string(), "c".to_string()),
            ]
        );

        // Every `EffectSize` compares two *different* engines.
        for e in &effect_sizes {
            assert_ne!(e.a_engine, e.b_engine);
        }

        Ok(())
    }

    #[test]
    fn effect_size_requires_at_least_two_engines() {
        let measurements = vec![measurement("only", 1), measurement("only", 2)];
        assert!(calculate(0.01, &measurements).is_err());
    }

    #[test]
    fn write_sorts_sum_total_first() -> Result<()> {
        fn pair<'a>(wasm: &'a str) -> (EffectSize<'a>, Vec<Summary<'a>>) {
            let a = Engine {
                name: "a".into(),
                flags: None,
            };
            let b = Engine {
                name: "b".into(),
                flags: None,
            };
            let effect_size = EffectSize {
                arch: "x86".into(),
                wasm: wasm.into(),
                phase: Phase::Execution,
                event: "cycles".into(),
                a_engine: a.clone(),
                a_mean: 100.0,
                b_engine: b.clone(),
                b_mean: 200.0,
                significance_level: 0.01,
                half_width_confidence_interval: 1.0,
            };
            let summary = |engine: Engine<'a>, mean: f64| Summary {
                arch: "x86".into(),
                engine,
                wasm: wasm.into(),
                phase: Phase::Execution,
                event: "cycles".into(),
                min: mean as u64,
                max: mean as u64,
                median: mean as u64,
                mean,
                mean_deviation: 0.0,
            };
            (effect_size, vec![summary(a, 100.0), summary(b, 200.0)])
        }

        // "Aaa" sorts before "Sum Total" and is passed first, so this exercises
        // the explicit total-first ordering rather than a sorting accident.
        let (es_aaa, mut summaries) = pair("Aaa");
        let (es_total, mut total_summaries) = pair("Sum Total");
        summaries.append(&mut total_summaries);

        let mut out = termcolor::NoColor::new(Vec::new());
        write(vec![es_aaa, es_total], &summaries, 0.01, &mut out)?;
        let out = String::from_utf8(out.into_inner())?;

        let total = out.find("Sum Total").unwrap();
        let aaa = out.find("Aaa").unwrap();
        assert!(total < aaa, "Sum Total should be first:\n{out}");
        Ok(())
    }
}
