//! The geometric mean of a suite's benchmarks, and a test for whether it differs
//! between two engines.
//!
//! A geometric mean weights every benchmark equally, where a sum is dominated by
//! whichever has the largest counts: spidermonkey's millions of cycles drown out
//! everything else, so a summed "speed up" is really just the speed up on the
//! biggest benchmark.
//!
//! # Comparing two engines' geomeans
//!
//! We are asking whether *this* suite's geomean changed, and that question is
//! what picks the method:
//!
//! * The estimate has to be the figure we report, the geomean of the
//!   benchmarks' mean counts. An average of per-sample geomeans sits below that
//!   by a gap that grows with the engine's own noise and does not cancel
//!   between two engines, so a merely noisier engine would read as faster.
//!
//! * The suite is fixed, so the only randomness is measurement noise, and a win
//!   confined to a single benchmark still moves the geomean. A one-sample
//!   t-test over the `k` log-ratios would instead ask whether programs in
//!   general got faster, and would dismiss that win.
//!
//! * Noise has to widen the interval rather than move the estimate, so the
//!   benchmarks are weighted equally rather than by inverse variance. A noisy
//!   benchmark genuinely does make the suite geomean harder to pin down; the
//!   answer is more samples of it, not less weight.
//!
//! Hence the ratio of the two geomeans, taken in log space where it is a plain
//! average of per-benchmark log-ratios: a linear combination of `2 * k`
//! independently measured means, so their variances add, by the [delta method]
//! for `ln` and [Welch-Satterthwaite] for the degrees of freedom. Below, `a`
//! and `b` hold each engine's samples of each of the `k` benchmarks.
//!
//! ```text
//! log_ratio = sum(ln(mean(b[i])) - ln(mean(a[i])) for i in 0..k) / k
//! ratio     = exp(log_ratio)
//!
//! groups = [a[i] for i in 0..k] + [b[i] for i in 0..k]
//! u      = {g: var(g) / (len(g) * mean(g) ** 2) for g in groups}
//!
//! standard_error = sqrt(sum(u.values())) / k
//! dof            = sum(u.values()) ** 2 / sum(u[g] ** 2 / (len(g) - 1)
//!                                             for g in groups)
//! t              = students_t_inverse_cdf(1 - significance_level / 2, dof)
//!
//! confidence_interval = (exp(log_ratio - t * standard_error),
//!                        exp(log_ratio + t * standard_error))
//! ```
//!
//! where `var` is the sample variance, and the suite geomean changed
//! significantly when the interval excludes 1. This is [Welch's t-test] widened
//! from one difference of means to a weighted sum of `2 * k` of them, so it
//! assumes the engines' variances differ, as they do, and that every sample is
//! independent, which holds because every sample is taken in its own process
//! and those are interleaved across engines.
//!
//! Note that the suite effect is diluted by `k` while the standard error picks
//! up every benchmark's noise. A single benchmark's win moves the geomean, but
//! reaching significance here still takes a far larger effect than in that
//! benchmark's own comparison.
//!
//! [delta method]: https://en.wikipedia.org/wiki/Delta_method
//! [Welch-Satterthwaite]: https://en.wikipedia.org/wiki/Welch%E2%80%93Satterthwaite_equation
//! [Welch's t-test]: https://en.wikipedia.org/wiki/Welch%27s_t-test

use behrens_fisher::Stats;
use sightglass_data::{Engine, Measurement, Phase};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

/// The label given to geomean results in human-readable output.
pub const GEOMEAN: &str = "Geomean";

/// One engine's suite geomean for an architecture, phase, and event.
#[derive(Clone, Debug)]
pub struct Geomean<'a> {
    /// The CPU architecture on which the measurements were taken.
    pub arch: Cow<'a, str>,

    /// The engine whose benchmarks were aggregated.
    pub engine: Engine<'a>,

    /// The phase in a Wasm program's lifecycle that was measured.
    pub phase: Phase,

    /// The event that was measured.
    pub event: Cow<'a, str>,

    /// The geometric mean of the benchmarks' mean counts.
    pub geomean: f64,

    /// How many benchmarks were aggregated.
    pub benchmarks: usize,
}

/// Whether, and by how much, the suite geomean differs between two engines.
///
/// See the module documentation for the statistics.
#[derive(Clone, Debug)]
pub struct GeomeanEffectSize<'a> {
    /// The CPU architecture on which the measurements were taken.
    pub arch: Cow<'a, str>,

    /// The phase in a Wasm program's lifecycle that was measured.
    pub phase: Phase,

    /// The event that was measured.
    pub event: Cow<'a, str>,

    /// The first engine being compared.
    pub a_engine: Engine<'a>,

    /// The first engine's suite geomean.
    pub a_geomean: f64,

    /// The second engine being compared.
    pub b_engine: Engine<'a>,

    /// The second engine's suite geomean.
    pub b_geomean: f64,

    /// How many benchmarks were aggregated into each geomean.
    pub benchmarks: usize,

    /// The significance level for the confidence interval, between 0.0 and 1.0.
    pub significance_level: f64,

    /// The ratio `b_geomean / a_geomean`.
    pub ratio: f64,

    /// The confidence interval for `ratio` as `(low, high, degrees_of_freedom)`.
    ///
    /// This is `None` when the samples cannot support an interval, as when every
    /// group has zero variance.
    pub confidence_interval: Option<(f64, f64, f64)>,
}

impl GeomeanEffectSize<'_> {
    /// Did the suite geomean differ between the two engines?
    ///
    /// It did when the confidence interval for the ratio excludes 1. Without an
    /// interval there is no evidence either way, which is not a difference.
    pub fn is_significant(&self) -> bool {
        match self.confidence_interval {
            Some((low, high, _)) => low > 1.0 || high < 1.0,
            None => false,
        }
    }
}

/// The architecture, phase, and event a geomean aggregates within: we aggregate
/// across benchmarks and compare across engines, never across these.
type GroupKey<'a> = (Cow<'a, str>, Phase, Cow<'a, str>);

/// Each engine's per-benchmark sample statistics within one group.
type Group<'a> = BTreeMap<Engine<'a>, Benchmarks<'a, Stats>>;

/// One group's benchmarks, by name, each holding a `T`.
type Benchmarks<'a, T> = BTreeMap<Cow<'a, str>, T>;

/// Take the geometric mean of each engine's benchmarks' mean counts.
///
/// Only benchmarks usable under every engine are aggregated; see
/// `shared_benchmarks`. A group left holding fewer than two benchmarks produces
/// no result at all, since an aggregate over one benchmark is just that
/// benchmark under a name claiming otherwise.
pub fn summarize<'a>(measurements: &[Measurement<'a>]) -> Vec<Geomean<'a>> {
    let mut geomeans = Vec::new();

    for ((arch, phase, event), engines) in group(measurements) {
        for (engine, benchmarks) in engines {
            if benchmarks.len() < 2 {
                continue;
            }
            geomeans.push(Geomean {
                arch: arch.clone(),
                engine,
                phase,
                event: event.clone(),
                geomean: geomean(benchmarks.values().map(|s| s.mean)),
                benchmarks: benchmarks.len(),
            });
        }
    }

    geomeans
}

/// Compare the suite geomeans of each pair of engines in `measurements`.
///
/// One [`GeomeanEffectSize`] is produced for each pair of engines present within
/// an architecture, phase, and event, mirroring the per-benchmark comparisons
/// that [`crate::effect_size::calculate`] produces. The same benchmarks are
/// aggregated as in [`summarize`].
pub fn effect_size<'a>(
    significance_level: f64,
    measurements: &[Measurement<'a>],
) -> Vec<GeomeanEffectSize<'a>> {
    let mut results = Vec::new();

    for ((arch, phase, event), engines) in group(measurements) {
        // `BTreeMap` keeps the engines sorted, so `a` and `b` are assigned as in
        // the per-benchmark comparisons.
        let engines: Vec<_> = engines.into_iter().collect();

        for (i, (a_engine, a)) in engines.iter().enumerate() {
            for (b_engine, b) in &engines[i + 1..] {
                if a.len() < 2 {
                    continue;
                }
                results.push(compare(
                    arch.clone(),
                    phase,
                    event.clone(),
                    significance_level,
                    (a_engine.clone(), a),
                    (b_engine.clone(), b),
                ));
            }
        }
    }

    results
}

/// Compare one pair of engines over the benchmarks they share.
///
/// Both maps hold the same benchmarks, since `group` intersects them.
fn compare<'a>(
    arch: Cow<'a, str>,
    phase: Phase,
    event: Cow<'a, str>,
    significance_level: f64,
    (a_engine, a): (Engine<'a>, &Benchmarks<'a, Stats>),
    (b_engine, b): (Engine<'a>, &Benchmarks<'a, Stats>),
) -> GeomeanEffectSize<'a> {
    let k = a.len() as f64;

    // One pass over the `2 * k` groups, accumulating every sum the module
    // documentation's formulas need.
    let mut log_ratio_sum = 0.0;
    let mut u_total = 0.0;
    let mut u_over_dof = 0.0;

    for (wasm, x) in a {
        let y = &b[wasm];
        log_ratio_sum += y.mean.ln() - x.mean.ln();

        for s in [x, y] {
            let u = s.var / (s.count as f64 * s.mean * s.mean);
            u_total += u;
            u_over_dof += u * u / (s.count - 1) as f64;
        }
    }

    let log_ratio = log_ratio_sum / k;
    let standard_error = u_total.sqrt() / k;
    let dof = u_total * u_total / u_over_dof;

    // `inv_cdf` asserts that its degrees of freedom are normal rather than
    // returning an error, so check before calling it. They are degenerate
    // whenever every group has zero variance, as the deterministic counts from
    // callgrind do, or whenever some benchmark was measured only once.
    let confidence_interval = if dof.is_finite() && dof >= 1.0 && standard_error.is_finite() {
        let t = behrens_fisher::student_t::inv_cdf(1.0 - significance_level / 2.0, dof);
        Some((
            (log_ratio - t * standard_error).exp(),
            (log_ratio + t * standard_error).exp(),
            dof,
        ))
    } else {
        None
    };

    GeomeanEffectSize {
        arch,
        phase,
        event,
        a_engine,
        a_geomean: geomean(a.values().map(|s| s.mean)),
        b_engine,
        b_geomean: geomean(b.values().map(|s| s.mean)),
        benchmarks: a.len(),
        significance_level,
        ratio: log_ratio.exp(),
        confidence_interval,
    }
}

/// Group `measurements` by architecture, phase, and event, then by engine and
/// benchmark, summarizing each benchmark's samples.
///
/// Benchmarks not usable under every engine are dropped from every engine, and
/// warned about.
fn group<'a>(measurements: &[Measurement<'a>]) -> BTreeMap<GroupKey<'a>, Group<'a>> {
    let mut counts: BTreeMap<GroupKey<'a>, BTreeMap<Engine<'a>, Benchmarks<'a, Vec<f64>>>> =
        BTreeMap::new();
    for m in measurements {
        counts
            .entry((m.arch.clone(), m.phase, m.event.clone()))
            .or_default()
            .entry(m.engine.clone())
            .or_default()
            .entry(m.wasm.clone())
            .or_default()
            .push(m.count as f64);
    }

    let mut excluded = BTreeSet::new();
    let mut groups = BTreeMap::new();

    for (key, engines) in counts {
        let mut engines: Group<'a> = engines
            .into_iter()
            .map(|(engine, benchmarks)| {
                let benchmarks = benchmarks
                    .into_iter()
                    .map(|(wasm, counts)| (wasm, counts.into_iter().collect::<Stats>()))
                    .collect();
                (engine, benchmarks)
            })
            .collect();

        let shared = shared_benchmarks(&engines);
        for benchmarks in engines.values() {
            excluded.extend(
                benchmarks
                    .keys()
                    .filter(|wasm| !shared.contains(*wasm))
                    .cloned(),
            );
        }
        for benchmarks in engines.values_mut() {
            benchmarks.retain(|wasm, _| shared.contains(wasm));
        }

        groups.insert(key, engines);
    }

    warn_about_excluded_benchmarks(&excluded);
    groups
}

/// The benchmarks every engine measured with a non-zero mean count.
///
/// A benchmark only one engine ran would shift that engine's geomean on its own,
/// so comparing the two would measure the difference in benchmark sets rather
/// than the difference between the engines. A mean of zero has no logarithm;
/// counters that legitimately read zero on small benchmarks, such as the cache
/// misses that `sightglass_recorder`'s callgrind measure records, are better
/// compared per-benchmark anyway.
fn shared_benchmarks<'a>(engines: &Group<'a>) -> BTreeSet<Cow<'a, str>> {
    let mut shared: Option<BTreeSet<Cow<'a, str>>> = None;

    for benchmarks in engines.values() {
        let usable: BTreeSet<Cow<'a, str>> = benchmarks
            .iter()
            .filter(|(_, stats)| stats.mean > 0.0)
            .map(|(wasm, _)| wasm.clone())
            .collect();
        shared = Some(match shared {
            Some(common) => common.intersection(&usable).cloned().collect(),
            None => usable,
        });
    }

    shared.unwrap_or_default()
}

/// Warn about benchmarks left out of the geomean, since it then answers a
/// narrower question than the user asked.
fn warn_about_excluded_benchmarks(excluded: &BTreeSet<Cow<'_, str>>) {
    if excluded.is_empty() {
        return;
    }

    let names: Vec<&str> = excluded.iter().map(|wasm| wasm.as_ref()).collect();
    eprintln!(
        "\nWarning: the \"{GEOMEAN}\" results leave out {}. A benchmark is left \
         out when some engine did not measure it, since the results would then \
         compare different sets of benchmarks, or when its counts average zero, \
         which has no logarithm.\n",
        names.join(", "),
    );
}

/// The geometric mean of `means`, or 0 if empty.
fn geomean(means: impl Iterator<Item = f64>) -> f64 {
    // Accumulate in log space: the product of even a handful of cycle counts
    // overflows an `f64`.
    let mut n = 0.0;
    let mut log_sum = 0.0;
    for m in means {
        n += 1.0;
        log_sum += m.ln();
    }
    if n == 0.0 {
        return 0.0;
    }
    (log_sum / n).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(engine: &'static str, wasm: &'static str, count: u64) -> Measurement<'static> {
        Measurement {
            arch: "x86_64".into(),
            engine: Engine {
                name: engine.into(),
                flags: None,
            },
            wasm: wasm.into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            count,
        }
    }

    /// One measurement per count, for each `(engine, benchmark, counts)` triple.
    fn measurements(
        groups: &[(&'static str, &'static str, &'static [u64])],
    ) -> Vec<Measurement<'static>> {
        let mut measurements = vec![];
        for (engine, wasm, counts) in groups {
            for count in counts.iter() {
                measurements.push(measurement(engine, wasm, *count));
            }
        }
        measurements
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

    #[test]
    fn geomean_of_means() {
        // These have exact geometric means.
        assert_near(geomean([10.0, 1000.0].into_iter()), 100.0);
        assert_near(geomean([9.0, 9.0, 9.0].into_iter()), 9.0);

        // Unlike an arithmetic mean, one huge value does not dominate.
        assert_near(geomean([1.0, 1_000_000.0].into_iter()), 1000.0);

        assert_near(geomean([].into_iter()), 0.0);
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
    fn summarizes_the_geomean_of_the_benchmarks_means() {
        // a.wasm averages 10 and b.wasm 1000, so the geomean is 100.
        let measurements =
            measurements(&[("e", "a.wasm", &[5, 15]), ("e", "b.wasm", &[900, 1100])]);

        let geomeans = summarize(&measurements);
        assert_eq!(geomeans.len(), 1);
        assert_eq!(geomeans[0].benchmarks, 2);
        assert_eq!(geomeans[0].phase, Phase::Execution);
        assert_eq!(geomeans[0].event, "cycles");
        assert_near(geomeans[0].geomean, 100.0);
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

        let mut ms = measurements(&[("e", "a.wasm", &[10]), ("e", "b.wasm", &[1000])]);
        ms.extend(
            measurements(&[("e", "a.wasm", &[40]), ("e", "b.wasm", &[4000])])
                .into_iter()
                .map(compilation),
        );
        ms.extend(
            measurements(&[("e", "a.wasm", &[90]), ("e", "b.wasm", &[9000])])
                .into_iter()
                .map(nanoseconds),
        );

        let mut geomeans: Vec<f64> = summarize(&ms).iter().map(|g| g.geomean).collect();
        geomeans.sort_by(f64::total_cmp);
        assert_eq!(geomeans.len(), 3);
        assert_near(geomeans[0], 100.0);
        assert_near(geomeans[1], 400.0);
        assert_near(geomeans[2], 900.0);
    }

    #[test]
    fn a_single_benchmark_produces_no_geomean() {
        // The geomean of one benchmark is that benchmark, under a name claiming
        // it aggregates across benchmarks.
        let one_engine = measurements(&[("e", "a.wasm", &[10, 20])]);
        assert!(summarize(&one_engine).is_empty());

        let two_engines = measurements(&[("a", "a.wasm", &[10, 20]), ("b", "a.wasm", &[10, 20])]);
        assert!(effect_size(0.01, &two_engines).is_empty());
    }

    #[test]
    fn only_aggregates_benchmarks_that_every_engine_measured() {
        // `b.wasm` ran under `a` alone. Including it would make the two geomeans
        // summarize different populations, so both cover only the shared
        // benchmarks and stay comparable.
        let measurements = measurements(&[
            ("a", "a.wasm", &[100]),
            ("a", "b.wasm", &[1_000_000]),
            ("a", "shared.wasm", &[400]),
            ("b", "a.wasm", &[100]),
            ("b", "shared.wasm", &[400]),
        ]);

        let geomeans = summarize(&measurements);
        assert_eq!(geomeans.len(), 2);
        for g in &geomeans {
            assert_eq!(g.benchmarks, 2);
            // sqrt(100 * 400), i.e. b.wasm left out of both.
            assert_near(g.geomean, 200.0);
        }
    }

    #[test]
    fn engines_sharing_no_benchmarks_produce_nothing() {
        // With nothing in common there is no comparable population at all, so
        // there is nothing to compare rather than something to compare wrongly.
        let measurements = measurements(&[
            ("a", "a.wasm", &[100, 101]),
            ("a", "b.wasm", &[200, 201]),
            ("b", "c.wasm", &[100, 101]),
            ("b", "d.wasm", &[200, 201]),
        ]);

        assert!(summarize(&measurements).is_empty());
        assert!(effect_size(0.01, &measurements).is_empty());
    }

    #[test]
    fn benchmarks_averaging_zero_are_excluded() {
        // A mean of zero has no logarithm, so `zero.wasm` drops out of both
        // engines rather than being clamped up to one, which would pull the
        // geomean down by a factor of its neighbours' magnitude.
        let measurements = measurements(&[
            ("a", "a.wasm", &[100]),
            ("a", "b.wasm", &[400]),
            ("a", "zero.wasm", &[0]),
            ("b", "a.wasm", &[100]),
            ("b", "b.wasm", &[400]),
            ("b", "zero.wasm", &[3]),
        ]);

        let geomeans = summarize(&measurements);
        assert_eq!(geomeans.len(), 2);
        for g in &geomeans {
            assert_eq!(g.benchmarks, 2);
            assert_near(g.geomean, 200.0);
        }
    }

    #[test]
    fn compares_two_engines_suite_geomeans() {
        // Engine `b` is 10% faster on both benchmarks, with little noise, so the
        // suite ratio is 0.9 and its interval excludes 1.
        //
        // These values come from an independent implementation of the method in
        // the module documentation:
        //
        //     mean(a) = [102, 1020],  mean(b) = [92, 920]
        //     geomean(a) = 322.5523213371745,  geomean(b) = 290.9295447354911
        //     ratio = 0.901960784313726,  dof = 15.832619637866742
        //     99% confidence interval = (0.8828579148372148, 0.9214769928067469)
        let measurements = measurements(&[
            ("a", "a.wasm", &[100, 101, 102, 103, 104]),
            ("a", "b.wasm", &[1000, 1010, 1020, 1030, 1040]),
            ("b", "a.wasm", &[90, 91, 92, 93, 94]),
            ("b", "b.wasm", &[900, 910, 920, 930, 940]),
        ]);

        let effects = effect_size(0.01, &measurements);
        assert_eq!(effects.len(), 1);
        let e = &effects[0];

        assert_eq!(e.a_engine.name, "a");
        assert_eq!(e.b_engine.name, "b");
        assert_eq!(e.benchmarks, 2);
        assert_near(e.a_geomean, 322.5523213371745);
        assert_near(e.b_geomean, 290.9295447354911);
        assert_near(e.ratio, 0.901960784313726);

        let (low, high, dof) = e.confidence_interval.unwrap();
        // The t-quantile is computed slightly differently in each
        // implementation, so the interval's bounds agree less closely than the
        // ratio and degrees of freedom, which are computed in closed form.
        assert!((low - 0.8828579148372148).abs() < 1e-6, "low is {low}");
        assert!((high - 0.9214769928067469).abs() < 1e-6, "high is {high}");
        assert_near(dof, 15.832619637866742);

        assert!(e.is_significant());
    }

    #[test]
    fn one_effect_size_per_pair_of_engines() {
        let measurements = measurements(&[
            ("a", "a.wasm", &[100, 101]),
            ("a", "b.wasm", &[200, 201]),
            ("b", "a.wasm", &[110, 111]),
            ("b", "b.wasm", &[210, 211]),
            ("c", "a.wasm", &[120, 121]),
            ("c", "b.wasm", &[220, 221]),
        ]);

        let effects = effect_size(0.01, &measurements);
        let pairs: Vec<(&str, &str)> = effects
            .iter()
            .map(|e| (e.a_engine.name.as_ref(), e.b_engine.name.as_ref()))
            .collect();
        assert_eq!(pairs, vec![("a", "b"), ("a", "c"), ("b", "c")]);
    }

    #[test]
    fn a_noisier_engine_is_not_reported_as_faster() {
        // Both engines average 1000 on both benchmarks; `noisy` merely swings
        // around it, and its two benchmarks swing opposite ways. Its noise must
        // widen the interval rather than move the estimate.
        let measurements = measurements(&[
            ("noisy", "a.wasm", &[100, 1900, 1000, 100, 1900, 1000]),
            ("noisy", "b.wasm", &[1900, 100, 1000, 1900, 100, 1000]),
            ("steady", "a.wasm", &[999, 1000, 1001, 999, 1000, 1001]),
            ("steady", "b.wasm", &[999, 1000, 1001, 999, 1000, 1001]),
        ]);

        let effects = effect_size(0.01, &measurements);
        assert_eq!(effects.len(), 1);
        let e = &effects[0];

        assert_near(e.a_geomean, 1000.0);
        assert_near(e.b_geomean, 1000.0);
        assert_near(e.ratio, 1.0);
        assert!(
            !e.is_significant(),
            "a merely noisier engine was reported as different: {:?}",
            e.confidence_interval
        );
    }

    #[test]
    fn a_single_benchmarks_win_moves_the_suite_geomean() {
        // `a.wasm` is 20% faster under `b` and the other three benchmarks are
        // unchanged, so the suite ratio is `0.8 ** (1 / 4)` and, with this
        // little noise, significant.
        const STEADY: &[u64] = &[1000, 1001, 999, 1000, 1001, 999];
        let measurements = measurements(&[
            ("a", "a.wasm", STEADY),
            ("a", "b.wasm", STEADY),
            ("a", "c.wasm", STEADY),
            ("a", "d.wasm", STEADY),
            ("b", "a.wasm", &[800, 801, 799, 800, 801, 799]),
            ("b", "b.wasm", STEADY),
            ("b", "c.wasm", STEADY),
            ("b", "d.wasm", STEADY),
        ]);

        let effects = effect_size(0.01, &measurements);
        let e = &effects[0];

        assert_eq!(e.benchmarks, 4);
        assert_near(e.ratio, 0.8f64.powf(0.25));
        assert!(e.is_significant(), "{:?}", e.confidence_interval);
    }

    #[test]
    fn zero_variance_samples_get_no_confidence_interval() {
        // Deterministic counts, as callgrind records, leave the degrees of
        // freedom undefined. Reporting no interval is better than asserting
        // inside the t-distribution's inverse CDF.
        let measurements = measurements(&[
            ("a", "a.wasm", &[100, 100]),
            ("a", "b.wasm", &[400, 400]),
            ("b", "a.wasm", &[50, 50]),
            ("b", "b.wasm", &[200, 200]),
        ]);

        let effects = effect_size(0.01, &measurements);
        assert_eq!(effects.len(), 1);
        assert_near(effects[0].ratio, 0.5);
        assert!(effects[0].confidence_interval.is_none());
        assert!(!effects[0].is_significant());
    }

    #[test]
    fn a_single_sample_gets_no_confidence_interval() {
        // One sample has no sample variance, so the degrees of freedom are
        // undefined just as for zero-variance samples.
        let measurements = measurements(&[
            ("a", "a.wasm", &[100]),
            ("a", "b.wasm", &[400]),
            ("b", "a.wasm", &[50]),
            ("b", "b.wasm", &[200]),
        ]);

        let effects = effect_size(0.01, &measurements);
        assert_eq!(effects.len(), 1);
        assert!(effects[0].confidence_interval.is_none());
    }
}
