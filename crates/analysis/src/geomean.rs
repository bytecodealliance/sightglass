//! Synthetic measurements that take the geometric mean of a phase's counts
//! across all benchmarks.

use sightglass_data::{Engine, Measurement, Phase};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

/// The benchmark name given to the synthetic measurements produced by
/// [`calculate`].
pub const GEOMEAN: &str = "Geomean";

/// The fields measurements must agree on to be aggregated: we aggregate across
/// benchmarks, never across architectures, engines, phases, or events.
type GroupKey<'a> = (Cow<'a, str>, Engine<'a>, Phase, Cow<'a, str>);

/// One benchmark's samples within a group, which sort deterministically.
type Samples = Vec<u64>;

/// The benchmarks of a group, by name.
type Benchmarks<'a> = BTreeMap<Cow<'a, str>, Samples>;

/// For each `(arch, phase, event)`, the benchmark names every engine measured.
type SharedBenchmarks<'a> = BTreeMap<(Cow<'a, str>, Phase, Cow<'a, str>), BTreeSet<Cow<'a, str>>>;

/// Take the geometric mean of measurement `count`s across all benchmarks,
/// producing new "Geomean" measurements: one per sample index, each aggregating
/// the `i`th sample of every benchmark. A benchmark's `i`th sample is its `i`th
/// smallest count, since its samples are otherwise unordered.
///
/// A geometric mean weights every benchmark equally, where a sum is dominated by
/// whichever has the largest counts: spidermonkey's millions of cycles drown out
/// everything else, so a summed "speed up" is really just the speed up on the
/// biggest benchmark.
///
/// Only benchmarks that every engine measured are aggregated, and within those,
/// only the samples that *every* benchmark has. Any [`GEOMEAN`] measurements
/// already in `measurements` are ignored, so they are never aggregated a second
/// time. A group holding fewer than two benchmarks produces no row at all.
///
/// The rows are rescaled so that their arithmetic mean, which is what the
/// analyses consume, equals the geometric mean of the benchmarks' mean counts.
/// Without that, the raw per-sample geomeans average below that figure by a gap
/// that grows with the engine's own noise and does not cancel between two
/// engines, so a merely noisier engine would be reported as faster.
///
/// Emitting synthetic rows is what lets summaries and the `effect_size`
/// analysis treat this aggregate as an ordinary benchmark. Computing the
/// geomean and its confidence interval directly in log space would not need
/// rescaling, but would not be able to reuse our existing analysis pipeline.
pub fn calculate<'a>(measurements: &[Measurement<'a>]) -> Vec<Measurement<'a>> {
    // Samples carry nothing that pairs one benchmark's with another's: each is
    // taken in its own process, so we group by everything but the benchmark and
    // line the samples up by sorted order below.
    let mut groups: BTreeMap<GroupKey<'a>, Benchmarks<'a>> = BTreeMap::new();
    for m in measurements.iter().filter(|m| m.wasm != GEOMEAN) {
        groups
            .entry((m.arch.clone(), m.engine.clone(), m.phase, m.event.clone()))
            .or_default()
            .entry(m.wasm.clone())
            .or_default()
            .push(m.count);
    }

    let shared = shared_benchmarks(&groups);
    warn_about_excluded_benchmarks(&groups, &shared);
    let mut geomeans = Vec::new();

    for ((arch, engine, phase, event), benchmarks) in groups {
        // Aggregate only over benchmarks that every engine measured, so that two
        // engines' rows always summarize the same population.
        let shared = &shared[&(arch.clone(), phase, event.clone())];
        let counts: Vec<Vec<u64>> = benchmarks
            .into_iter()
            .filter(|(wasm, _)| shared.contains(wasm))
            // Line the samples up deterministically so we can aggregate the
            // `i`th of each.
            .map(|(_, mut samples)| {
                samples.sort_unstable();
                samples
            })
            .collect();

        // An aggregate over one benchmark is not an aggregate: it would
        // reproduce that benchmark exactly, under a name claiming otherwise.
        if counts.len() < 2 {
            continue;
        }

        // Only produce geomeans for the samples that every benchmark has.
        let samples = counts.iter().map(|c| c.len()).min().unwrap_or(0);
        let per_sample: Vec<f64> = (0..samples)
            .map(|i| geomean(counts.iter().map(|c| c[i] as f64)))
            .collect();

        // Rescale so the rows average to the geometric mean of the benchmarks'
        // mean counts. See the `calculate` doc comment.
        let target = geomean(
            counts
                .iter()
                .map(|c| c[..samples].iter().sum::<u64>() as f64 / samples as f64),
        );
        let scale = match mean(&per_sample) {
            m if m > 0.0 => target / m,
            _ => 1.0,
        };

        geomeans.extend(per_sample.iter().map(|g| Measurement {
            arch: arch.clone(),
            engine: engine.clone(),
            wasm: GEOMEAN.into(),
            phase,
            event: event.clone(),
            count: (g * scale).round() as u64,
        }));
    }

    geomeans
}

/// For each `(arch, phase, event)`, the benchmarks that *every* engine measured.
///
/// A geomean weights each benchmark equally, so one that only one engine ran
/// would shift that engine's row on its own. Comparing such rows measures the
/// difference in benchmark sets rather than the difference between the engines.
fn shared_benchmarks<'a>(groups: &BTreeMap<GroupKey<'a>, Benchmarks<'a>>) -> SharedBenchmarks<'a> {
    let mut shared: BTreeMap<_, BTreeSet<Cow<'a, str>>> = BTreeMap::new();
    for ((arch, _, phase, event), benchmarks) in groups {
        let key = (arch.clone(), *phase, event.clone());
        let names: BTreeSet<Cow<'a, str>> = benchmarks.keys().cloned().collect();
        shared
            .entry(key)
            .and_modify(|common| *common = common.intersection(&names).cloned().collect())
            .or_insert(names);
    }
    shared
}

/// Warn about benchmarks left out of the geomean because some engine did not
/// measure them, since the row then answers a narrower question than the user
/// asked.
fn warn_about_excluded_benchmarks<'a>(
    groups: &BTreeMap<GroupKey<'a>, Benchmarks<'a>>,
    shared: &SharedBenchmarks<'a>,
) {
    let mut excluded: BTreeSet<&str> = BTreeSet::new();
    for ((arch, _, phase, event), benchmarks) in groups {
        let shared = &shared[&(arch.clone(), *phase, event.clone())];
        excluded.extend(
            benchmarks
                .keys()
                .filter(|wasm| !shared.contains(*wasm))
                .map(|wasm| wasm.as_ref()),
        );
    }

    if !excluded.is_empty() {
        let names: Vec<&str> = excluded.into_iter().collect();
        eprintln!(
            "\nWarning: not every engine measured {}, so including {} would make \
             the \"{GEOMEAN}\" results compare different sets of benchmarks. \
             Left out of those results: {}\n",
            if names.len() == 1 {
                "one benchmark"
            } else {
                "some benchmarks"
            },
            if names.len() == 1 { "it" } else { "them" },
            names.join(", "),
        );
    }
}

/// The arithmetic mean of `xs`, or 0 if empty.
fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// The geometric mean of `counts`, or 0 if empty.
///
/// Zeroes are clamped up to one, since a single zero count would otherwise zero
/// out the whole row. Note that this is not a small distortion: against
/// neighbours around `ln(1e6)`, a clamped zero pulls the geomean down by a
/// factor of `1e6^(1/B)` over `B` benchmarks. It also makes the row sensitive to
/// tiny absolute differences, as a counter reading 0 for one engine and 3 for
/// another shifts the geomean by `3^(1/B)`. Counters that legitimately read zero
/// on small benchmarks, such as the cache misses that `sightglass_recorder`'s
/// callgrind measure records, are therefore best compared per-benchmark rather
/// than through this row.
fn geomean(counts: impl Iterator<Item = f64>) -> f64 {
    // Accumulate in log space: the product of even a handful of cycle counts
    // overflows an `f64`.
    let mut n = 0.0;
    let mut log_sum = 0.0;
    for c in counts {
        n += 1.0;
        log_sum += c.max(1.0).ln();
    }
    if n == 0.0 {
        return 0.0;
    }
    (log_sum / n).exp()
}

/// Augment `measurements` with the [`calculate`]d "Geomean" measurements, so that
/// analyses report a geometric mean alongside per-benchmark results.
///
/// Any "Geomean" measurements already present are replaced, so this is
/// idempotent.
pub fn add<'a>(measurements: &mut Vec<Measurement<'a>>) {
    measurements.retain(|m| m.wasm != GEOMEAN);
    let geomeans = calculate(measurements);
    measurements.extend(geomeans);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(wasm: &'static str, count: u64) -> Measurement<'static> {
        Measurement {
            arch: "x86_64".into(),
            engine: Engine {
                name: "e".into(),
                flags: None,
            },
            wasm: wasm.into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            count,
        }
    }

    /// Assert that `got` is `want`, to `f64` rounding.
    #[track_caller]
    fn assert_near(got: f64, want: f64) {
        let ok = if want == 0.0 {
            got == 0.0
        } else {
            (got - want).abs() / want < 1e-9
        };
        assert!(ok, "{got} is not approximately {want}");
    }

    /// The `count`s of the given measurements, sorted.
    fn counts(measurements: &[Measurement<'_>]) -> Vec<u64> {
        let mut counts: Vec<_> = measurements.iter().map(|m| m.count).collect();
        counts.sort();
        counts
    }

    #[test]
    fn geomean_of_counts() {
        // Counts throughout these tests have exact geometric means.
        assert_near(geomean([10.0, 1000.0].into_iter()), 100.0);
        assert_near(geomean([2.0, 8.0].into_iter()), 4.0);
        assert_near(geomean([9.0, 9.0, 9.0].into_iter()), 9.0);

        // Unlike an arithmetic mean, one huge value does not dominate.
        assert_near(geomean([1.0, 1_000_000.0].into_iter()), 1000.0);

        assert_near(geomean([].into_iter()), 0.0);
    }

    #[test]
    fn geomean_clamps_zero_counts() {
        // Clamped to one, so a benchmark that recorded no events cannot wipe
        // out the whole row.
        assert_near(geomean([0.0, 100.0].into_iter()), 10.0);
        assert_near(geomean([0.0, 0.0].into_iter()), 1.0);

        // The clamp is not a small distortion: 0 against 3 moves a three
        // benchmark geomean by 3^(1/3), about 44%.
        let zero = geomean([0.0, 1000.0, 1000.0].into_iter());
        let three = geomean([3.0, 1000.0, 1000.0].into_iter());
        assert_near(three / zero, 3f64.cbrt());
    }

    #[test]
    fn geomean_accumulates_in_log_space() {
        // Multiplying these would overflow to infinity. In log space the geomean
        // of n copies of a value is that value, to `f64`'s precision.
        let huge = (u64::MAX / 2) as f64;
        let got = geomean([huge; 8].into_iter());
        assert!(
            (got - huge).abs() / huge < 1e-12,
            "{got} is not approximately {huge}"
        );
    }

    #[test]
    fn geomeans_counts_across_benchmarks() {
        // Two benchmarks, each sampled by two processes.
        let measurements = vec![
            measurement("a.wasm", 10),
            measurement("b.wasm", 1000),
            measurement("a.wasm", 20),
            measurement("b.wasm", 2000),
        ];

        let geomeans = calculate(&measurements);
        assert_eq!(geomeans.len(), 2);
        for g in &geomeans {
            assert_eq!(g.wasm, GEOMEAN);
            assert_eq!(g.phase, Phase::Execution);
            assert_eq!(g.event, "cycles");
        }

        assert_eq!(counts(&geomeans), vec![100, 200]);
    }

    #[test]
    fn geomeans_across_benchmarks_measured_in_different_processes() {
        // Each sample comes from its own process, and no two benchmarks share
        // one, but the geomeans must still aggregate across them.
        let measurements = vec![
            measurement("a.wasm", 10),
            measurement("a.wasm", 20),
            measurement("a.wasm", 30),
            measurement("a.wasm", 40),
            measurement("b.wasm", 1000),
            measurement("b.wasm", 2000),
            measurement("b.wasm", 3000),
            measurement("b.wasm", 4000),
        ];

        // One geomean per sample, rather than one per benchmark per sample...
        let geomeans = calculate(&measurements);
        assert_eq!(geomeans.len(), 4);

        // ...each aggregating one sample from each benchmark, paired in sorted
        // order.
        assert_eq!(counts(&geomeans), vec![100, 200, 300, 400]);

        // Every geomean lies between the samples it aggregates; a sum would not.
        for g in &geomeans {
            assert!(g.count > 10 && g.count < 4000);
        }
    }

    #[test]
    fn only_aggregates_the_samples_that_every_benchmark_has() {
        // `b.wasm` was measured once, so only one sample covers every benchmark.
        // Aggregating `a.wasm`'s extra samples alone would leave `b.wasm` out.
        let measurements = vec![
            measurement("a.wasm", 10),
            measurement("a.wasm", 20),
            measurement("a.wasm", 30),
            measurement("b.wasm", 1000),
        ];

        let geomeans = calculate(&measurements);
        assert_eq!(counts(&geomeans), vec![100]);
    }

    #[test]
    fn measurements_are_only_aggregated_within_a_phase_and_event() {
        let compilation = |m: Measurement<'static>| Measurement {
            phase: Phase::Compilation,
            ..m
        };
        let nanoseconds = |m: Measurement<'static>| Measurement {
            event: "nanoseconds".into(),
            ..m
        };

        let measurements = vec![
            measurement("a.wasm", 10),
            measurement("b.wasm", 1000),
            compilation(measurement("a.wasm", 40)),
            compilation(measurement("b.wasm", 4000)),
            nanoseconds(measurement("a.wasm", 90)),
            nanoseconds(measurement("b.wasm", 9000)),
        ];

        let geomeans = calculate(&measurements);
        assert_eq!(counts(&geomeans), vec![100, 400, 900]);
    }

    #[test]
    fn rows_average_to_the_geomean_of_the_benchmarks_means() {
        // Without rescaling the rows average below this, by an amount that grows
        // with the engine's own noise.
        let measurements = vec![
            measurement("a.wasm", 100),
            measurement("a.wasm", 10_000),
            measurement("b.wasm", 500),
            measurement("b.wasm", 500),
        ];

        let geomeans = calculate(&measurements);
        let got = mean(&geomeans.iter().map(|g| g.count as f64).collect::<Vec<_>>());

        // a.wasm averages 5050 and b.wasm 500, so the target is sqrt(5050 * 500),
        // give or take the rounding of each row to a `u64`.
        let want = (5050.0f64 * 500.0).sqrt();
        assert!((got - want).abs() <= 1.0, "{got} is not about {want}");
    }

    #[test]
    fn a_noisier_engine_is_not_reported_as_faster() {
        // Both engines average 1000 on both benchmarks; `noisy` merely swings
        // around it, and its two benchmarks swing opposite ways. Averaging the
        // raw per-sample geomeans would put `noisy` at ~624, which
        // `effect_size` would report as 60% faster than `steady`.
        let engine = |name: &'static str, wasm: &'static str, count: u64| Measurement {
            engine: Engine {
                name: name.into(),
                flags: None,
            },
            ..measurement(wasm, count)
        };

        let mut measurements = vec![];
        for (a, b) in [(100u64, 1900u64), (1900, 100), (1000, 1000)] {
            measurements.push(engine("noisy", "a.wasm", a));
            measurements.push(engine("noisy", "b.wasm", b));
            measurements.push(engine("steady", "a.wasm", 1000));
            measurements.push(engine("steady", "b.wasm", 1000));
        }

        let geomeans = calculate(&measurements);
        let for_engine = |name: &str| {
            mean(
                &geomeans
                    .iter()
                    .filter(|g| g.engine.name == name)
                    .map(|g| g.count as f64)
                    .collect::<Vec<_>>(),
            )
        };

        assert_near(for_engine("steady"), 1000.0);
        assert!(
            (for_engine("noisy") - 1000.0).abs() <= 1.0,
            "the noisier engine reports {}, not 1000",
            for_engine("noisy")
        );
    }

    #[test]
    fn only_aggregates_benchmarks_that_every_engine_measured() {
        // `b.wasm` ran under `a` alone. Including it would make the two rows
        // summarize different populations, so each engine's row covers only
        // `a.wasm` and the rows stay comparable.
        let engine = |name: &'static str, wasm: &'static str, count: u64| Measurement {
            engine: Engine {
                name: name.into(),
                flags: None,
            },
            ..measurement(wasm, count)
        };

        let measurements = vec![
            engine("a", "a.wasm", 100),
            engine("a", "b.wasm", 1_000_000),
            engine("a", "shared.wasm", 400),
            engine("b", "a.wasm", 100),
            engine("b", "shared.wasm", 400),
        ];

        let geomeans = calculate(&measurements);
        assert_eq!(geomeans.len(), 2);
        for g in &geomeans {
            // sqrt(100 * 400), i.e. b.wasm left out of both.
            assert_near(g.count as f64, 200.0);
        }
    }

    #[test]
    fn a_single_benchmark_produces_no_row() {
        // The geomean of one benchmark is that benchmark, which would appear
        // twice under two names and skew the insignificance count.
        let measurements = vec![measurement("a.wasm", 10), measurement("a.wasm", 20)];
        assert!(calculate(&measurements).is_empty());
    }

    #[test]
    fn add_appends_geomeans_to_the_measurements() {
        let mut measurements = vec![measurement("a.wasm", 10), measurement("b.wasm", 1000)];
        add(&mut measurements);

        assert_eq!(measurements.len(), 3);
        let geomeans: Vec<_> = measurements
            .iter()
            .filter(|m| m.wasm == GEOMEAN)
            .cloned()
            .collect();
        assert_eq!(counts(&geomeans), vec![100]);
    }

    #[test]
    fn add_is_idempotent() {
        // Existing geomeans are recomputed rather than folded in, so analyzing
        // already-augmented measurements doesn't aggregate them twice.
        let mut measurements = vec![measurement("a.wasm", 10), measurement("b.wasm", 1000)];
        add(&mut measurements);
        let once = measurements.clone();
        add(&mut measurements);

        assert_eq!(measurements.len(), once.len());
        assert_eq!(counts(&measurements), counts(&once));
    }
}
