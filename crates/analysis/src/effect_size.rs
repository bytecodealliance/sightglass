use crate::geomean::{GeomeanEffectSize, GEOMEAN};
use crate::keys::KeyBuilder;
use crate::mann_whitney;
use crate::summarize::{mean, percentile_from_sorted};
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
///
/// Each comparison is a Mann-Whitney U test; see [`crate::mann_whitney`] for
/// the method. Note that this estimates a different quantity than the suite
/// geomean does: the median of the ratios between the two engines' samples,
/// rather than the ratio of their means. The geomean is an aggregate over a
/// fixed set of benchmarks, which the mean is the right summary for, while a
/// single benchmark's comparison is about where its samples sit relative to
/// each other.
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
                let counts = |engine: &Engine<'_>| -> Vec<u64> {
                    key_measurements
                        .iter()
                        .filter(|m| &m.engine == engine)
                        .map(|m| m.count)
                        .collect()
                };
                let a = counts(engine_a);
                let b = counts(engine_b);

                let Some(comparison) = mann_whitney::compare(significance_level, &a, &b) else {
                    continue;
                };

                // Sorted once per engine, so the percentiles and the extremes
                // all come off the same ordering.
                let (a_sorted, b_sorted) = (sorted(&a), sorted(&b));

                results.push(EffectSize {
                    arch: key.arch.clone().unwrap(),
                    wasm: key.wasm.clone().unwrap(),
                    phase: key.phase.unwrap(),
                    event: key.event.clone().unwrap(),
                    a_engine: (*engine_a).clone(),
                    a_mean: mean(&a),
                    a_median: percentile_from_sorted(&a_sorted, 50.0),
                    a_min: a_sorted[0],
                    a_max: a_sorted[a_sorted.len() - 1],
                    a_p25: percentile_from_sorted(&a_sorted, 25.0),
                    a_p75: percentile_from_sorted(&a_sorted, 75.0),
                    a_samples: a.len(),
                    b_engine: (*engine_b).clone(),
                    b_mean: mean(&b),
                    b_median: percentile_from_sorted(&b_sorted, 50.0),
                    b_min: b_sorted[0],
                    b_max: b_sorted[b_sorted.len() - 1],
                    b_p25: percentile_from_sorted(&b_sorted, 25.0),
                    b_p75: percentile_from_sorted(&b_sorted, 75.0),
                    b_samples: b.len(),
                    significance_level,
                    ratio: comparison.ratio,
                    confidence_interval: comparison.confidence_interval,
                    p_value: comparison.p_value,
                });
            }
        }
    }

    Ok(results)
}

/// A sorted copy of `counts`, for reading percentiles off.
fn sorted(counts: &[u64]) -> Vec<u64> {
    let mut sorted = counts.to_vec();
    sorted.sort();
    sorted
}

/// Write a vector of [EffectSize] structures to the passed `output_file` in
/// human-readable form, led by the suite `geomeans`.
///
/// The `summaries` must cover every engine, benchmark, phase, and event that the
/// `effect_sizes` compare.
pub fn write(
    mut effect_sizes: Vec<EffectSize<'_>>,
    geomeans: &[GeomeanEffectSize<'_>],
    summaries: &[Summary<'_>],
    significance_level: f64,
    output_file: &mut dyn WriteColor,
) -> Result<()> {
    // Sort the effect sizes so that we focus on statistically significant
    // results before insignificant results and larger relative effect sizes
    // before smaller relative effect sizes.
    effect_sizes.sort_by(|x, y| {
        y.is_significant().cmp(&x.is_significant()).then_with(|| {
            let x_speedup = x.a_speed_up_over_b().0.max(x.b_speed_up_over_a().0);
            let y_speedup = y.a_speed_up_over_b().0.max(y.b_speed_up_over_a().0);
            y_speedup.total_cmp(&x_speedup)
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

    write_geomeans(geomeans, &bold, &bold_engine, &bold_ratio, output_file)?;

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

        match effect_size.confidence_interval {
            // `ratio` is b's counts over a's, so a is the faster engine when it
            // exceeds 1. Inverting the ratio inverts its interval's bounds too.
            Some((low, high)) if effect_size.is_significant() => {
                let (faster, slower, low, high) = if effect_size.ratio > 1.0 {
                    (&a_eng_label, &b_eng_label, low, high)
                } else {
                    (&b_eng_label, &a_eng_label, 1.0 / high, 1.0 / low)
                };

                write!(output_file, "    ")?;
                crate::write_in(output_file, &bold_engine, faster)?;
                crate::write_in(output_file, &bold, " is ")?;
                crate::write_in(output_file, &bold_ratio, &format_ratio(low, high, low))?;
                crate::write_in(output_file, &bold, " to ")?;
                crate::write_in(output_file, &bold_ratio, &format_ratio(low, high, high))?;
                crate::write_in(output_file, &bold, " faster than ")?;
                crate::write_in(output_file, &bold_engine, slower)?;
                crate::write_in(output_file, &bold, "!")?;
                writeln!(output_file)?;

                write!(output_file, "    ")?;
                crate::write_in(
                    output_file,
                    &crate::stats_parenthetical_spec(),
                    &format!(
                        "(confidence = {}%, p {})",
                        (1.0 - significance_level) * 100.0,
                        format_p_value(effect_size.p_value),
                    ),
                )?;
                writeln!(output_file)?;
            }
            Some(_) => writeln!(output_file, "    No difference in performance.")?,

            // Three different things leave us without an interval, and they
            // call for different responses from the reader: more samples fixes
            // the first, and nothing fixes the other two.
            None if a_summary.min == 0 || b_summary.min == 0 => writeln!(
                output_file,
                "    No confidence interval: a count of zero has no ratio.",
            )?,
            None => {
                let floor = mann_whitney::smallest_attainable_p_value(
                    effect_size.a_samples,
                    effect_size.b_samples,
                );
                if floor > significance_level {
                    writeln!(
                        output_file,
                        "    Not enough samples to compare: no difference between {} \
                         and {} samples can reach p < {significance_level}, only \
                         p < {floor:.4}.",
                        effect_size.a_samples, effect_size.b_samples,
                    )?
                } else {
                    writeln!(
                        output_file,
                        "    No confidence interval: these counts are too alike to \
                         rank apart, so no ratio is ruled out.",
                    )?
                }
            }
        }
        writeln!(output_file)?;

        let rows = vec![
            crate::summary_row(a_summary, a_eng_label),
            crate::summary_row(b_summary, b_eng_label),
        ];
        crate::write_stats_table(output_file, "    ", &rows)?;
    }

    Ok(())
}

/// Write the suite `geomeans` as leading blocks, each shaped like the
/// per-benchmark comparisons that follow. A no-op if there are none.
fn write_geomeans(
    geomeans: &[GeomeanEffectSize<'_>],
    bold: &ColorSpec,
    bold_engine: &ColorSpec,
    bold_ratio: &ColorSpec,
    output_file: &mut dyn WriteColor,
) -> Result<()> {
    let mut geomeans: Vec<&GeomeanEffectSize<'_>> = geomeans.iter().collect();
    geomeans.sort_by(|x, y| {
        y.is_significant()
            .cmp(&x.is_significant())
            .then_with(|| x.phase.cmp(&y.phase))
            .then_with(|| x.event.cmp(&y.event))
    });

    for geomean in geomeans {
        writeln!(output_file)?;
        crate::write_in(
            output_file,
            &crate::phase_spec(),
            &geomean.phase.to_string(),
        )?;
        write!(output_file, " :: ")?;
        crate::write_in(output_file, &crate::event_spec(), &geomean.event)?;
        write!(output_file, " :: ")?;
        crate::write_in(
            output_file,
            &crate::benchmark_spec(),
            &format!("{GEOMEAN} ({} benchmarks)", geomean.benchmarks),
        )?;
        writeln!(output_file)?;
        writeln!(output_file)?;

        let (a_eng_label, b_eng_label) = geomean.a_engine.relative_labels(&geomean.b_engine);

        match geomean.confidence_interval {
            // `ratio` is b's geomean over a's, so a is the faster engine when it
            // exceeds 1. Inverting the ratio inverts its interval's bounds too.
            Some((low, high, dof)) if geomean.is_significant() => {
                let (faster, slower, low, high) = if geomean.ratio > 1.0 {
                    (&a_eng_label, &b_eng_label, low, high)
                } else {
                    (&b_eng_label, &a_eng_label, 1.0 / high, 1.0 / low)
                };

                write!(output_file, "    ")?;
                crate::write_in(output_file, bold_engine, faster)?;
                crate::write_in(output_file, bold, " is ")?;
                crate::write_in(output_file, bold_ratio, &format_ratio(low, high, low))?;
                crate::write_in(output_file, bold, " to ")?;
                crate::write_in(output_file, bold_ratio, &format_ratio(low, high, high))?;
                crate::write_in(output_file, bold, " faster than ")?;
                crate::write_in(output_file, bold_engine, slower)?;
                crate::write_in(output_file, bold, "!")?;
                writeln!(output_file)?;

                write!(output_file, "    ")?;
                crate::write_in(
                    output_file,
                    &crate::stats_parenthetical_spec(),
                    &format!(
                        "(confidence = {}%, df = {dof:.1})",
                        (1.0 - geomean.significance_level) * 100.0
                    ),
                )?;
                writeln!(output_file)?;
            }
            Some(_) => writeln!(output_file, "    No difference in performance.")?,
            None => {
                writeln!(
                    output_file,
                    "    No confidence interval: the samples do not vary enough to \
                     estimate one."
                )?;
            }
        }
        writeln!(output_file)?;

        let rows = vec![
            vec![
                format!("{:.2}", geomean.a_geomean),
                geomean.benchmarks.to_string(),
                a_eng_label,
            ],
            vec![
                format!("{:.2}", geomean.b_geomean),
                geomean.benchmarks.to_string(),
                b_eng_label,
            ],
        ];
        crate::write_geomean_table(output_file, "    ", &rows)?;
    }

    Ok(())
}

/// Format a p-value, which is often far below the precision worth printing.
fn format_p_value(p: f64) -> String {
    if p < 0.0001 {
        "< 0.0001".to_string()
    } else {
        format!("= {p:.4}")
    }
}

/// Format one bound of a speed up ratio, with just enough precision to tell the
/// interval's two bounds apart.
///
/// A suite ratio is diluted by the number of benchmarks, so a real win can leave
/// both bounds rounding to the same two decimals.
fn format_ratio(low: f64, high: f64, bound: f64) -> String {
    for precision in 2..=6 {
        if format!("{low:.precision$}") != format!("{high:.precision$}") {
            return format!("{bound:.precision$}x");
        }
    }
    format!("{bound:.6}x")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geomean::GeomeanEffectSize;

    fn measurement<'a>(engine: &'a str, count: u64) -> Measurement<'a> {
        Measurement {
            arch: "x86_64".into(),
            engine: Engine {
                name: engine.into(),
                flags: None,
            },
            wasm: "bench.wasm".into(),
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

    /// Samples too few to reach the significance level however they fall get
    /// no interval and are not significant.
    #[test]
    fn too_few_samples_are_not_significant() -> Result<()> {
        let mut measurements = vec![];
        for (engine, count) in [("a", 100), ("b", 99)] {
            for _ in 0..3 {
                measurements.push(measurement(engine, count));
            }
        }

        let effect_sizes = calculate(0.01, &measurements)?;
        assert_eq!(effect_sizes.len(), 1);

        let e = &effect_sizes[0];
        assert_eq!(e.confidence_interval, None);
        assert!(!e.is_significant());

        // Three samples each cannot beat `2 / C(6, 3)`.
        assert!(e.p_value >= 0.1, "{}", e.p_value);

        Ok(())
    }

    /// A comparison with plenty of well-separated samples is significant, and
    /// its interval excludes 1.
    #[test]
    fn a_clear_difference_is_significant() -> Result<()> {
        let mut measurements = vec![];
        for (engine, base) in [("a", 100), ("b", 200)] {
            for i in 0..10 {
                measurements.push(measurement(engine, base + i));
            }
        }

        let effect_sizes = calculate(0.01, &measurements)?;
        let e = &effect_sizes[0];

        assert!(e.is_significant(), "p = {}", e.p_value);
        assert!((e.ratio - 2.0).abs() < 0.1, "{}", e.ratio);

        let (low, high) = e.confidence_interval.unwrap();
        assert!(low > 1.0, "the interval should exclude 1: {low} to {high}");

        // `a` is the faster engine, and inverting the ratio swaps the
        // interval's bounds.
        let (ratio, interval) = e.a_speed_up_over_b();
        assert!(ratio < 1.0, "{ratio}");
        let (low_inverted, high_inverted) = interval.unwrap();
        assert!((low_inverted - 1.0 / high).abs() < 1e-12);
        assert!((high_inverted - 1.0 / low).abs() < 1e-12);

        Ok(())
    }

    #[test]
    fn effect_size_requires_at_least_two_engines() {
        let measurements = vec![measurement("only", 1), measurement("only", 2)];
        assert!(calculate(0.01, &measurements).is_err());
    }

    fn engines() -> (Engine<'static>, Engine<'static>) {
        (
            Engine {
                name: "a".into(),
                flags: None,
            },
            Engine {
                name: "b".into(),
                flags: None,
            },
        )
    }

    fn pair<'a>(wasm: &'a str) -> (EffectSize<'a>, Vec<Summary<'a>>) {
        let (a, b) = engines();
        let effect_size = EffectSize {
            arch: "x86".into(),
            wasm: wasm.into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            a_engine: a.clone(),
            a_mean: 100.0,
            a_median: 100.0,
            a_min: 98,
            a_max: 102,
            a_p25: 99.0,
            a_p75: 101.0,
            a_samples: 10,
            b_engine: b.clone(),
            b_mean: 200.0,
            b_median: 200.0,
            b_min: 196,
            b_max: 204,
            b_p25: 198.0,
            b_p75: 202.0,
            b_samples: 10,
            significance_level: 0.01,
            ratio: 2.0,
            confidence_interval: Some((1.9, 2.1)),
            p_value: 0.0001,
        };
        let summary = |engine: Engine<'a>, mean: f64| Summary {
            arch: "x86".into(),
            engine,
            wasm: wasm.into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            min: mean as u64,
            max: mean as u64,
            median: mean,
            mean,
            mean_deviation: 0.0,
            samples: 10,
            p25: mean,
            p75: mean,
        };
        (effect_size, vec![summary(a, 100.0), summary(b, 200.0)])
    }

    fn geomean(confidence_interval: Option<(f64, f64, f64)>) -> GeomeanEffectSize<'static> {
        let (a, b) = engines();
        GeomeanEffectSize {
            arch: "x86".into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            a_engine: a,
            a_geomean: 100.0,
            b_engine: b,
            b_geomean: 200.0,
            benchmarks: 2,
            significance_level: 0.01,
            ratio: 2.0,
            confidence_interval,
        }
    }

    #[test]
    fn write_puts_the_geomean_first() -> Result<()> {
        // "Aaa" sorts before "Geomean" and is passed first, so this exercises
        // the explicit ordering rather than a sorting accident.
        let (es_aaa, summaries) = pair("Aaa");

        let mut out = termcolor::NoColor::new(Vec::new());
        write(
            vec![es_aaa],
            &[geomean(Some((1.5, 2.5, 20.0)))],
            &summaries,
            0.01,
            &mut out,
        )?;
        let out = String::from_utf8(out.into_inner())?;

        let geomean = out.find("Geomean").unwrap();
        let aaa = out.find("Aaa").unwrap();
        assert!(geomean < aaa, "the geomean should be first:\n{out}");

        // `a` is the faster engine, since the ratio is `b` over `a`.
        assert!(out.contains("a is 1.50x to 2.50x faster than b!"), "{out}");
        assert!(out.contains("df = 20.0"), "{out}");
        Ok(())
    }

    #[test]
    fn write_inverts_the_interval_when_b_is_faster() -> Result<()> {
        let (es_aaa, summaries) = pair("Aaa");
        let mut geomean = geomean(Some((0.4, 0.5, 20.0)));
        geomean.ratio = 0.45;

        let mut out = termcolor::NoColor::new(Vec::new());
        write(vec![es_aaa], &[geomean], &summaries, 0.01, &mut out)?;
        let out = String::from_utf8(out.into_inner())?;

        // 1/0.5 to 1/0.4, with the bounds swapped so the interval stays ordered.
        assert!(out.contains("b is 2.00x to 2.50x faster than a!"), "{out}");
        Ok(())
    }

    #[test]
    fn write_reports_a_missing_confidence_interval() -> Result<()> {
        let (es_aaa, summaries) = pair("Aaa");

        let mut out = termcolor::NoColor::new(Vec::new());
        write(vec![es_aaa], &[geomean(None)], &summaries, 0.01, &mut out)?;
        let out = String::from_utf8(out.into_inner())?;

        // The geomean block says so and claims no speed up, though the
        // per-benchmark block beneath it still reports its own.
        let geomean_block = &out[..out.find("Aaa").unwrap()];
        assert!(geomean_block.contains("No confidence interval"), "{out}");
        assert!(!geomean_block.contains("faster than"), "{out}");
        Ok(())
    }

    /// A comparison with no interval has three possible causes, and the
    /// message has to name the right one: only the first is fixed by running
    /// more iterations.
    #[test]
    fn write_tells_apart_the_reasons_for_a_missing_interval() -> Result<()> {
        let message = |samples: usize, zero_count: bool| -> Result<String> {
            let (mut effect_size, mut summaries) = pair("Aaa");
            effect_size.confidence_interval = None;
            effect_size.a_samples = samples;
            effect_size.b_samples = samples;
            if zero_count {
                summaries[0].min = 0;
            }

            let mut out = termcolor::NoColor::new(Vec::new());
            write(vec![effect_size], &[], &summaries, 0.01, &mut out)?;
            Ok(String::from_utf8(out.into_inner())?)
        };

        // Three samples of each engine cannot reach 1% however they fall, and
        // more iterations is the answer.
        let out = message(3, false)?;
        assert!(out.contains("Not enough samples to compare"), "{out}");
        assert!(out.contains("only p < 0.1000"), "{out}");

        // Twenty can, so when they still bound nothing it is because the counts
        // are too alike to rank apart, and more iterations will not help.
        let out = message(20, false)?;
        assert!(out.contains("too alike to rank apart"), "{out}");
        assert!(!out.contains("Not enough samples"), "{out}");

        // A zero count has no ratio at any sample size, which outranks both.
        let out = message(20, true)?;
        assert!(out.contains("a count of zero has no ratio"), "{out}");
        assert!(!out.contains("Not enough samples"), "{out}");
        Ok(())
    }

    #[test]
    fn ratio_precision_tells_the_bounds_apart() {
        // Two decimals suffice for ordinary per-benchmark effects.
        assert_eq!(format_ratio(1.5, 2.5, 1.5), "1.50x");

        // A suite ratio is diluted by the benchmark count, so a real win can
        // leave both bounds rounding to the same two decimals. Both of these
        // are "1.00" at two, so a third is needed to tell them apart.
        assert_eq!(format_ratio(1.0041, 1.0048, 1.0041), "1.004x");
        assert_eq!(format_ratio(1.0041, 1.0048, 1.0048), "1.005x");

        // A yet narrower interval takes a fourth.
        assert_eq!(format_ratio(1.00041, 1.00048, 1.00041), "1.0004x");
    }
}
