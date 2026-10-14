//! A distribution-free comparison of two engines' samples of one benchmark.
//!
//! Benchmark timings are right-skewed by construction: code cannot run faster
//! than its critical path, but there is no limit to how much interference can
//! slow it down. A t-test assumes normality that these samples do not have, so
//! the comparison here ranks the two samples against each other instead and
//! assumes nothing about their shape.
//!
//! # The test
//!
//! The [Mann-Whitney U test] counts how often a sample of `b` exceeds a sample
//! of `a`. Under the null hypothesis that the two engines draw from the same
//! distribution, every way of splitting the pooled samples between them is
//! equally likely, so the null distribution of that count is obtained by
//! enumerating the splits.
//!
//! Semi-deterministic counters, like the ones callgrind records, frequently
//! have ties, so we enumerate them exactly by dynamic programming over the
//! pooled midranks, which makes ties ordinary rather than a special case.
//!
//! The exact distribution keeps results for small samples valid. With `n`
//! samples of each engine there are only `C(2n, n)` splits, so no comparison
//! can report a two-sided p-value below `2 / C(2n, n)` however far apart the
//! samples lie: 0.1 at `n = 3`, and 0.0022 at `n = 6`. A run too small to reach
//! the chosen significance level reports that rather than a result.
//!
//! # The effect size
//!
//! The [Hodges-Lehmann estimator] for a ratio is the median of the `n_a * n_b`
//! pairwise ratios `b_j / a_i`. Taking the ratios directly is the same
//! estimator as Hodges-Lehmann on log counts, since the median commutes with
//! `ln`.
//!
//! Its interval comes from inverting the test: a ratio `c` belongs in the
//! interval when rescaling `a` by `c` leaves the two samples
//! indistinguishable. Only the pairwise ratios are worth trying, since the
//! ranks change only as `c` crosses one of them, so the interval's ends are two
//! of the ratios themselves and no delta-method approximation is needed.
//!
//! Inverting the same test the p-value comes from is what keeps the two from
//! contradicting each other: the interval contains 1 exactly when the test
//! fails to reject. Samples that accept even their most extreme ratio do not
//! bound the ratio at all, and report no interval rather than the span they
//! happen to cover.
//!
//! [Mann-Whitney U test]: https://en.wikipedia.org/wiki/Mann%E2%80%93Whitney_U_test
//! [Hodges-Lehmann estimator]: https://en.wikipedia.org/wiki/Hodges%E2%80%93Lehmann_estimator

/// How `b`'s samples compare to `a`'s.
///
/// See the module documentation for the method.
#[derive(Clone, Debug, PartialEq)]
pub struct Comparison {
    /// The two-sided p-value: the probability of samples at least this far
    /// apart when the two engines do not differ.
    pub p_value: f64,

    /// The Hodges-Lehmann estimate of `b` over `a`, i.e. the median of the
    /// pairwise ratios.
    pub ratio: f64,

    /// The confidence interval for `ratio` at the requested significance level.
    ///
    /// This is `None` when the samples do not bound the ratio, as when they are
    /// too few or too scattered to rule out any ratio at all, and when either
    /// holds a count of zero, which has no ratio. Use
    /// [`smallest_attainable_p_value`] to tell the first case from the second.
    pub confidence_interval: Option<(f64, f64)>,
}

/// Compare two engines' samples of one benchmark.
///
/// Returns `None` if either sample is empty. The `significance_level` is
/// between 0.0 and 1.0; 0.01 and 0.05 are typical.
pub fn compare(significance_level: f64, a: &[u64], b: &[u64]) -> Option<Comparison> {
    if a.is_empty() || b.is_empty() {
        return None;
    }

    // Both results are read from one null distribution, which is the expensive
    // part of the comparison and the only thing that keeps them consistent.
    let null = Null::of(a, b);
    let p_value = null.p_value(u_statistic(a, b));
    let (ratio, confidence_interval) = effect_size(&null, significance_level, a, b);

    Some(Comparison {
        p_value,
        ratio,
        confidence_interval,
    })
}

/// Enumerating the null distribution costs time cubic in the sample sizes, so
/// above this many samples of either engine we use the large-sample
/// approximation instead.
///
/// The approximation is close but not conservative: it lands either side of
/// the exact p-value, so a borderline result just above the limit is worth
/// neither more nor less trust than one just below. How close depends on how
/// many distinct counts there are rather than on how many samples. Over
/// samples at the limit, the two stay within 0.003 of each other near the
/// usual significance levels when the counts take twenty or more distinct
/// values, which ordinary timings do; when they take only two or three, as the
/// semi-deterministic counters can, the difference reaches 0.12 and a
/// borderline verdict can go either way.
///
/// Raising the limit would buy exactness at a steep price: the enumeration is
/// cubic, costing 1.1ms per comparison at twenty samples of each engine, 40ms
/// at fifty, and 640ms at the hundred that `--iters` defaults to. Callgrind,
/// whose counters tie the most, is also what runs with few enough samples to
/// be enumerated exactly, so the case that needs the exact test is the case
/// that gets it.
const EXACT_SAMPLE_LIMIT: usize = 20;

/// The null distribution of the U statistic: what both the p-value and the
/// confidence interval are read from.
///
/// Reading them from one distribution is what keeps the two consistent. They
/// are the same question asked twice, so a report that rejects the null while
/// showing an interval that covers "no difference" is contradicting itself, and
/// deriving both from one distribution is what rules that out.
enum Null {
    /// Every way of splitting the pooled samples, counted exactly.
    Exact {
        /// How many splits give each rank sum, indexed by its doubled value.
        counts: Vec<f64>,

        /// How many splits there are in all.
        total: f64,

        /// The doubled rank sum that a U statistic of zero sits at.
        offset: usize,
    },

    /// The normal approximation, for samples too many to enumerate.
    Approximate { mean: f64, std_dev: f64 },
}

impl Null {
    /// The distribution for samples of these sizes, enumerated exactly when
    /// that is affordable and approximated when it is not.
    fn of(a: &[u64], b: &[u64]) -> Null {
        if a.len() > EXACT_SAMPLE_LIMIT || b.len() > EXACT_SAMPLE_LIMIT {
            Null::approximate(a, b)
        } else {
            Null::exact(a, b)
        }
    }

    /// The exact distribution, by dynamic programming over the pooled
    /// midranks.
    fn exact(a: &[u64], b: &[u64]) -> Null {
        // Midranks are integers or halves, so doubling them keeps the null
        // distribution on a whole-number lattice and the arithmetic exact.
        let doubled: Vec<usize> = midranks(a, b)
            .iter()
            .map(|r| (r * 2.0).round() as usize)
            .collect();

        let counts = rank_sum_distribution(&doubled, b.len());
        Null::Exact {
            total: counts.iter().sum(),
            counts,
            offset: b.len() * (b.len() + 1),
        }
    }

    /// The normal approximation, with the tie correction.
    fn approximate(a: &[u64], b: &[u64]) -> Null {
        let (n_a, n_b) = (a.len() as f64, b.len() as f64);
        let n = n_a + n_b;

        // Ties shrink the variance, since a rank shared between the samples
        // cannot tell them apart.
        let variance = n_a * n_b / 12.0 * ((n + 1.0) - tie_correction(a, b) / (n * (n - 1.0)));

        Null::Approximate {
            mean: n_a * n_b / 2.0,
            std_dev: variance.max(0.0).sqrt(),
        }
    }

    /// The two-sided p-value of an observed `u`.
    fn p_value(&self, u: f64) -> f64 {
        match self {
            Null::Exact {
                counts,
                total,
                offset,
            } => {
                let d = (2.0 * u).round() as usize + offset;
                let at_most: f64 = counts[..=d].iter().sum::<f64>() / total;
                let at_least: f64 = counts[d..].iter().sum::<f64>() / total;
                (2.0 * at_most.min(at_least)).min(1.0)
            }
            Null::Approximate { mean, std_dev } => {
                if *std_dev <= 0.0 {
                    return 1.0;
                }
                // The continuity correction accounts for reading a lattice of
                // whole and half steps off a continuous curve.
                let z = ((u - mean).abs() - 0.5).max(0.0) / std_dev;
                (2.0 * normal_survival(z)).min(1.0)
            }
        }
    }

    /// The largest `u` that falls in the test's lower rejection tail.
    ///
    /// This is `None` when the tail is empty: with samples this few, or ties
    /// this pervasive, even the most lopsided split is ordinary enough that the
    /// test rejects nothing and the samples bound nothing.
    fn lower_critical(&self, significance_level: f64) -> Option<f64> {
        let half = significance_level / 2.0;
        match self {
            Null::Exact {
                counts,
                total,
                offset,
            } => {
                let mut tail = 0.0;
                let mut critical = None;
                for (d, count) in counts.iter().enumerate().skip(*offset) {
                    tail += count / total;
                    if tail > half {
                        break;
                    }
                    critical = Some((d - offset) as f64 / 2.0);
                }
                critical
            }
            Null::Approximate { mean, std_dev } => {
                let u = mean - normal_quantile(1.0 - half) * std_dev;
                // Round down onto the lattice `u` lives on, which errs towards
                // rejecting less than the exact test would.
                let u = (u * 2.0).floor() / 2.0;
                (u >= 0.0).then_some(u)
            }
        }
    }
}

/// The observed U statistic: `b`'s rank sum less the smallest it could be.
///
/// Equivalently, how many of the `n_a * n_b` pairs have `b`'s sample above
/// `a`'s, counting a tie as half a pair.
fn u_statistic(a: &[u64], b: &[u64]) -> f64 {
    let ranks = midranks(a, b);
    let b_rank_sum: f64 = ranks[a.len()..].iter().sum();
    b_rank_sum - (b.len() * (b.len() + 1)) as f64 / 2.0
}

/// The smallest two-sided p-value samples of these sizes can reach, however far
/// apart they fall.
///
/// There are only `C(n_a + n_b, n_b)` ways to split the pooled samples, and the
/// two most lopsided carry all the probability the tails can hold, so no
/// comparison reports a p-value below `2 / C(n_a + n_b, n_b)`: 0.1 at three
/// samples of each engine, and 0.0022 at six. A run whose floor is above the
/// significance level cannot report a difference whatever it measures, so it is
/// worth knowing before the run rather than after.
pub fn smallest_attainable_p_value(n_a: usize, n_b: usize) -> f64 {
    // `C(n_a + n_b, n_b)` as a product of ratios, which keeps the intermediate
    // values near one rather than overflowing through the factorials.
    let mut splits = 1.0;
    for i in 1..=n_b {
        splits *= (n_a + i) as f64 / i as f64;
    }

    (2.0 / splits).min(1.0)
}

/// `sum(t^3 - t)` over the sizes `t` of the pooled samples' groups of ties.
fn tie_correction(a: &[u64], b: &[u64]) -> f64 {
    let mut pooled: Vec<u64> = a.iter().chain(b).copied().collect();
    pooled.sort();

    let mut correction = 0.0;
    let mut i = 0;
    while i < pooled.len() {
        let mut j = i;
        while j + 1 < pooled.len() && pooled[j + 1] == pooled[i] {
            j += 1;
        }
        let t = (j - i + 1) as f64;
        correction += t * t * t - t;
        i = j + 1;
    }

    correction
}

/// `P(Z > z)` for a standard normal `Z`.
fn normal_survival(z: f64) -> f64 {
    // `erfc` is not in `std`, so use its relation to the normal CDF through an
    // Abramowitz and Stegun rational approximation, good to about 1e-7.
    let t = 1.0 / (1.0 + 0.231_641_9 * z.abs());
    let poly = t
        * (0.319_381_530
            + t * (-0.356_563_782
                + t * (1.781_477_937 + t * (-1.821_255_978 + t * 1.330_274_429))));
    let tail = (-z * z / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt() * poly;
    if z >= 0.0 {
        tail
    } else {
        1.0 - tail
    }
}

/// The pooled midranks of `a` followed by `b`, averaging the ranks of ties.
fn midranks(a: &[u64], b: &[u64]) -> Vec<f64> {
    let pooled: Vec<u64> = a.iter().chain(b).copied().collect();

    let mut order: Vec<usize> = (0..pooled.len()).collect();
    order.sort_by_key(|&i| pooled[i]);

    let mut ranks = vec![0.0; pooled.len()];
    let mut i = 0;
    while i < order.len() {
        let mut j = i;
        while j + 1 < order.len() && pooled[order[j + 1]] == pooled[order[i]] {
            j += 1;
        }
        let rank = (i + j) as f64 / 2.0 + 1.0;
        for &k in &order[i..=j] {
            ranks[k] = rank;
        }
        i = j + 1;
    }

    ranks
}

/// How many of the ways to choose `n` of the `ranks` give each possible sum.
///
/// This is the null distribution of `b`'s rank sum: under the null hypothesis
/// every such choice is equally likely. The returned vector is indexed by the
/// sum.
fn rank_sum_distribution(ranks: &[usize], n: usize) -> Vec<f64> {
    let total: usize = ranks.iter().sum();

    // `counts[c][s]` is the number of ways to choose `c` ranks summing to `s`.
    // Each rank is offered to the table once, and taking the sizes in
    // descending order lets each use the smaller sizes from before this rank
    // was offered.
    let mut counts = vec![vec![0.0; total + 1]; n + 1];
    counts[0][0] = 1.0;
    for &rank in ranks {
        for c in (1..=n).rev() {
            for s in (rank..=total).rev() {
                counts[c][s] += counts[c - 1][s - rank];
            }
        }
    }

    counts.swap_remove(n)
}

/// The Hodges-Lehmann ratio of `b` over `a` and its confidence interval.
fn effect_size(
    null: &Null,
    significance_level: f64,
    a: &[u64],
    b: &[u64],
) -> (f64, Option<(f64, f64)>) {
    // A count of zero has no ratio. These are legitimate on the counters that
    // read zero for small benchmarks, so fall back to a ratio of the medians
    // and offer no interval.
    if a.contains(&0) || b.contains(&0) {
        return (median_ratio(a, b), None);
    }

    let mut ratios: Vec<f64> = Vec::with_capacity(a.len() * b.len());
    for &x in a {
        for &y in b {
            ratios.push(y as f64 / x as f64);
        }
    }
    ratios.sort_by(f64::total_cmp);

    let ratio = median_of_sorted(&ratios);

    // Inverting the test puts the `k`th smallest and `k`th largest ratios at
    // the interval's ends, where `k` counts the ratios the test rejects at each
    // end: a U statistic of `k - 1` or below is the lower tail.
    let interval = null.lower_critical(significance_level).and_then(|u| {
        let k = u as usize + 1;
        // The two tails have to leave something between them. They do not when
        // the test rejects nothing, which `lower_critical` has already ruled
        // out, nor when the ratios are too few to have a `k`th from both ends.
        (2 * k <= ratios.len()).then(|| (ratios[k - 1], ratios[ratios.len() - k]))
    });

    (ratio, interval)
}

/// The `z` with `P(Z < z) = p` for a standard normal `Z`.
fn normal_quantile(p: f64) -> f64 {
    // `normal_survival` is monotonic, so bisect it rather than carry a second
    // approximation. The range covers every significance level we accept.
    let (mut low, mut high) = (-10.0, 10.0);
    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        if 1.0 - normal_survival(mid) < p {
            low = mid;
        } else {
            high = mid;
        }
    }
    (low + high) / 2.0
}

/// The median of already-sorted values.
fn median_of_sorted(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

/// The ratio of the two samples' medians, used when a zero count rules out the
/// pairwise ratios.
fn median_ratio(a: &[u64], b: &[u64]) -> f64 {
    let median = |xs: &[u64]| {
        let mut xs: Vec<f64> = xs.iter().map(|&x| x as f64).collect();
        xs.sort_by(f64::total_cmp);
        median_of_sorted(&xs)
    };
    let (a, b) = (median(a), median(b));
    if a == 0.0 {
        1.0
    } else {
        b / a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact two-sided p-value, which `compare` uses below the sample
    /// limit.
    fn p_value(a: &[u64], b: &[u64]) -> f64 {
        Null::exact(a, b).p_value(u_statistic(a, b))
    }

    /// The approximate two-sided p-value, which `compare` uses above it.
    fn approximate_p_value(a: &[u64], b: &[u64]) -> f64 {
        Null::approximate(a, b).p_value(u_statistic(a, b))
    }

    /// Assert that `got` is `want`, to `f64` rounding.
    #[track_caller]
    fn assert_near(got: f64, want: f64) {
        assert!(
            (got - want).abs() <= want.abs() * 1e-12,
            "{got} is not approximately {want}"
        );
    }

    /// The p-values below come from an independent implementation of the test,
    /// itself checked against an exhaustive enumeration of every way to split
    /// the pooled samples between the two engines.
    #[test]
    fn exact_p_values() {
        // The worked example from the `behrens-fisher` crate's documentation,
        // where the two samples share a 3.
        assert_near(
            p_value(&[1, 2, 3, 4], &[3, 5, 7, 9, 11]),
            0.047_619_047_619_047_616,
        );

        // Samples that do not overlap at all, which is as far apart as ranks
        // can see.
        assert_near(
            p_value(&[100, 101, 102, 103, 104], &[90, 91, 92, 93, 94]),
            0.007_936_507_936_507_936,
        );

        // Identical samples are as close together as ranks can see.
        assert_near(p_value(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]), 1.0);
    }

    /// Semi-deterministic counters, as callgrind records, are almost entirely
    /// ties. The test must rank them rather than divide by their zero variance,
    /// and the three samples it is given by default cannot reach any useful
    /// significance level however far apart they lie.
    #[test]
    fn deterministic_counts_are_not_significant() {
        let c = compare(0.05, &[100, 100, 100], &[50, 50, 50]).unwrap();

        assert_near(c.p_value, 0.1);
        assert_near(c.ratio, 0.5);

        // Three samples of each engine cannot reject anything at this level, so
        // they bound nothing either.
        assert_eq!(c.confidence_interval, None);
        assert!(smallest_attainable_p_value(3, 3) > 0.05);
    }

    /// No p-value below `2 / C(n_a + n_b, n_b)` is attainable, since that is
    /// how few ways there are to split the samples.
    #[test]
    fn small_samples_have_a_floor_on_significance() {
        let floor = |n: u64| {
            let a: Vec<u64> = (0..n).map(|i| 100 + i).collect();
            let b: Vec<u64> = (0..n).map(|i| 200 + i).collect();
            compare(0.01, &a, &b).unwrap().p_value
        };

        // 2 / C(4, 2), 2 / C(6, 3), and 2 / C(12, 6).
        assert_near(floor(2), 1.0 / 3.0);
        assert_near(floor(3), 0.1);
        assert_near(floor(6), 0.002_164_502_164_502_164_4);

        // `smallest_attainable_p_value` predicts that floor without the samples
        // in hand, which is what lets a run warn before it measures anything.
        for n in 2..=12 {
            assert_near(smallest_attainable_p_value(n, n), floor(n as u64));
        }

        // Uneven sample counts land between the even ones either side.
        assert!(
            smallest_attainable_p_value(6, 6) < smallest_attainable_p_value(5, 6)
                && smallest_attainable_p_value(5, 6) < smallest_attainable_p_value(5, 5)
        );

        // A single sample of one engine rules nothing out.
        assert_near(smallest_attainable_p_value(1, 1), 1.0);
    }

    #[test]
    fn hodges_lehmann_ratio_and_interval() {
        // Every sample of `b` is 10 below its counterpart in `a`, so the
        // pairwise ratios center just under 0.9.
        let c = compare(0.01, &[100, 101, 102, 103, 104], &[90, 91, 92, 93, 94]).unwrap();
        assert_near(c.ratio, 0.901_960_784_313_725_5);

        let (low, high) = c.confidence_interval.unwrap();
        assert_near(low, 0.865_384_615_384_615_4);
        assert_near(high, 0.94);
        assert!(high < 1.0, "the interval should exclude 1: {low} to {high}");

        // Identical samples center on 1, which their interval contains.
        let c = compare(0.05, &[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]).unwrap();
        assert_near(c.ratio, 1.0);
        let (low, high) = c.confidence_interval.unwrap();
        assert!(low < 1.0 && high > 1.0, "{low} to {high}");
    }

    /// A tighter significance level gives a wider interval, never a narrower
    /// one.
    #[test]
    fn a_tighter_significance_level_widens_the_interval() {
        let a: Vec<u64> = (100..110).collect();
        let b: Vec<u64> = (120..130).collect();

        let (l95, h95) = compare(0.05, &a, &b).unwrap().confidence_interval.unwrap();
        let (l99, h99) = compare(0.01, &a, &b).unwrap().confidence_interval.unwrap();

        assert!(
            l99 <= l95 && h99 >= h95,
            "99%: {l99}..{h99}, 95%: {l95}..{h95}"
        );
    }

    /// Counts of zero are legitimate for the counters that read zero on small
    /// benchmarks, and they have no ratio.
    #[test]
    fn zero_counts_get_no_interval() {
        let c = compare(0.05, &[0, 1, 2, 3, 4], &[10, 11, 12, 13, 14]).unwrap();

        // The ranks are unaffected, so the test still runs.
        assert_near(c.p_value, 0.007_936_507_936_507_936);
        assert_eq!(c.confidence_interval, None);
        assert!(c.ratio.is_finite(), "{}", c.ratio);
    }

    /// Above `EXACT_SAMPLE_LIMIT` the large-sample approximation takes over,
    /// so the two have to agree at the boundary for a result not to turn on
    /// which side of it the run landed.
    ///
    /// They agree on counts that are mostly distinct, which is what timings
    /// are, and disagree on counts that are mostly ties, which is what the
    /// deterministic counters are. Both halves are pinned here, since the
    /// second is the reason the limit cannot simply be lowered.
    #[test]
    fn the_approximation_agrees_with_the_exact_test_on_untied_counts() {
        for offset in [1, 3, 5, 8] {
            let a: Vec<u64> = (0..EXACT_SAMPLE_LIMIT as u64).map(|i| 100 + i).collect();
            let b: Vec<u64> = a.iter().map(|x| x + offset).collect();

            let exact = p_value(&a, &b);
            let approximate = approximate_p_value(&a, &b);

            // The approximation is not conservative, only close, so bound the
            // difference in both directions rather than its sign.
            assert!(
                (approximate - exact).abs() < 0.01,
                "offset {offset}: {approximate} and {exact} disagree"
            );
        }

        // Ties are what pull the two apart, and the counters that tie hardest
        // take only a few distinct values. Search for the worst disagreement
        // among those, so the limit cannot be raised on the belief that the two
        // always agree closely.
        let mut seed = 0xCAFE_F00D_DEAD_BEEF_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        let mut worst: f64 = 0.0;
        for _ in 0..500 {
            let n = EXACT_SAMPLE_LIMIT - (next() as usize) % 5;
            let a: Vec<u64> = (0..n).map(|_| 1000 + next() % 2).collect();
            let b: Vec<u64> = (0..n).map(|_| 1000 + next() % 2).collect();

            let (exact, approximate) = (p_value(&a, &b), approximate_p_value(&a, &b));
            // Only p-values near the levels anyone tests at matter here.
            if exact < 0.2 || approximate < 0.2 {
                worst = worst.max((approximate - exact).abs());
            }
        }
        assert!(
            worst > 0.01,
            "two distinct counts should pull the two apart, but the worst \
             disagreement was only {worst}"
        );

        // The same for the critical value the interval is read from, which must
        // not reject more than the exact test does.
        for n in [EXACT_SAMPLE_LIMIT, EXACT_SAMPLE_LIMIT - 5] {
            for significance_level in [0.01, 0.05] {
                let a: Vec<u64> = (0..n as u64).collect();
                let b: Vec<u64> = a.iter().map(|x| x + 1).collect();

                let exact = Null::exact(&a, &b)
                    .lower_critical(significance_level)
                    .unwrap();
                let approximate = Null::approximate(&a, &b)
                    .lower_critical(significance_level)
                    .unwrap();
                assert!(
                    approximate <= exact && exact - approximate <= 2.0,
                    "n = {n}, alpha = {significance_level}: exact {exact}, \
                     approximate {approximate}"
                );
            }
        }
    }

    /// The test and the interval are one question asked twice, so a rejected
    /// null and an interval covering 1 cannot both be right.
    ///
    /// Deriving the interval from the untied null distribution while the
    /// p-value used the tied one broke this: the tied distribution is the
    /// narrower of the two, so the test rejected while the interval still
    /// covered 1.
    #[test]
    fn the_interval_contains_one_exactly_when_the_test_does_not_reject() {
        // Both of these rejected at their level while reporting an interval
        // that covered 1, which the report then read out as a speed up.
        let cases: [(&[u64], &[u64], f64); 2] = [
            (&[2, 2, 4, 4, 4], &[1, 1, 1, 3, 3], 0.05),
            (
                &[1001, 1001, 1002, 1002, 1002, 1002, 1002],
                &[1000, 1000, 1000, 1001, 1001, 1001, 1001],
                0.01,
            ),
        ];
        for (a, b, significance_level) in cases {
            let c = compare(significance_level, a, b).unwrap();
            let (low, high) = c.confidence_interval.unwrap();
            assert!(
                c.p_value <= significance_level,
                "{a:?} vs {b:?}: p = {} should reject",
                c.p_value
            );
            assert!(
                high < 1.0 || low > 1.0,
                "{a:?} vs {b:?}: p = {} rejects, so {low} to {high} must not \
                 hold 1",
                c.p_value
            );
        }

        // The same agreement over tied samples generally, which is where the
        // two distributions came apart.
        let mut seed = 0x2545_f491_4f6c_dd1d_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for significance_level in [0.01, 0.05] {
            for n in [5, 8, 12] {
                for _ in 0..200 {
                    // A narrow range of counts, as the semi-deterministic
                    // counters give, so that ties are the rule.
                    let a: Vec<u64> = (0..n).map(|_| 1000 + next() % 4).collect();
                    let b: Vec<u64> = (0..n).map(|_| 1000 + next() % 4).collect();

                    let c = compare(significance_level, &a, &b).unwrap();
                    let Some((low, high)) = c.confidence_interval else {
                        continue;
                    };
                    // Whole counts put a pairwise ratio of exactly 1 on the
                    // lattice the interval's ends are drawn from, so a rejected
                    // null can leave 1 at an end. What it cannot do is leave it
                    // inside.
                    if c.p_value <= significance_level {
                        assert!(
                            !(low < 1.0 && 1.0 < high),
                            "alpha = {significance_level}, {a:?} vs {b:?}: \
                             p = {} rejects, so {low} to {high} must not hold 1",
                            c.p_value
                        );
                    } else {
                        assert!(
                            low <= 1.0 && 1.0 <= high,
                            "alpha = {significance_level}, {a:?} vs {b:?}: \
                             p = {} does not reject, so {low} to {high} must \
                             hold 1",
                            c.p_value
                        );
                    }
                }
            }
        }
    }

    /// The interval has to cover the true ratio as often as it claims, which is
    /// the property that makes it a confidence interval at all.
    ///
    /// Both samples are drawn from one distribution, so the ratio being
    /// estimated is exactly 1 and coverage is just how often the interval holds
    /// it. Ties push coverage above the nominal level rather than below, since
    /// the discrete lattice of ratios cannot land on the level exactly.
    #[test]
    fn the_interval_covers_the_true_ratio_as_often_as_it_claims() {
        let mut seed = 0x243F_6A88_85A3_08D3_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        for significance_level in [0.05, 0.01] {
            for spread in [3, 400] {
                let (mut covered, mut trials) = (0, 0);
                for _ in 0..2000 {
                    let a: Vec<u64> = (0..10).map(|_| 10_000 + next() % spread).collect();
                    let b: Vec<u64> = (0..10).map(|_| 10_000 + next() % spread).collect();

                    if let Some((low, high)) = compare(significance_level, &a, &b)
                        .unwrap()
                        .confidence_interval
                    {
                        trials += 1;
                        if low <= 1.0 && 1.0 <= high {
                            covered += 1;
                        }
                    }
                }

                // Allow three standard errors of sampling noise, so that the
                // test fails on a broken interval rather than on an unlucky
                // seed.
                let want = 1.0 - significance_level;
                let tolerance = 3.0 * (want * significance_level / trials as f64).sqrt();

                let coverage = covered as f64 / trials as f64;
                assert!(
                    coverage >= want - tolerance,
                    "alpha = {significance_level}, spread = {spread}: covered \
                     {coverage} of {trials}, want {want} give or take {tolerance:.4}"
                );
            }
        }
    }

    #[test]
    fn empty_samples_do_not_compare() {
        assert_eq!(compare(0.05, &[], &[1, 2, 3]), None);
        assert_eq!(compare(0.05, &[1, 2, 3], &[]), None);
    }
}
