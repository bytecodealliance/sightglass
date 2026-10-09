use super::util::sightglass_cli;
use assert_cmd::prelude::*;
use predicates::prelude::*;

fn results_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/results.json")
}

/// Results for three benchmarks, each measured in its own two processes.
fn multi_engine_v38_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/multi_engine_v38.json")
}

/// summarize reads raw JSON and prints a human-readable table by default.
#[test]
fn summarize_human_readable() {
    sightglass_cli()
        .args(["summarize", "-f", results_json()])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("compilation")
                .or(predicate::str::contains("Compilation"))
                .and(predicate::str::contains("cycles")),
        );
}

/// summarize --output-format json produces parseable JSON.
#[test]
fn summarize_output_format_json() {
    let assert = sightglass_cli()
        .args(["summarize", "-f", results_json(), "--output-format", "json"])
        .assert()
        .success();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(
        serde_json::from_str::<serde_json::Value>(stdout).is_ok(),
        "stdout was not valid JSON: {stdout}"
    );
}

/// summarize --output-format csv produces a CSV header row.
#[test]
fn summarize_output_format_csv() {
    sightglass_cli()
        .args(["summarize", "-f", results_json(), "--output-format", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mean"));
}

/// Like `sightglass-cli benchmark`, `summarize` reports a synthetic "Geomean"
/// benchmark aggregating each sample's counts across its input's benchmarks.
#[test]
fn summarize_geomean() {
    sightglass_cli()
        .args(["summarize", "-f", multi_engine_v38_json()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Geomean"));
}

/// Summarize our multi-engine test data, returning the parsed summaries.
fn summaries_json() -> Vec<serde_json::Value> {
    let assert = sightglass_cli()
        .args([
            "summarize",
            "-f",
            multi_engine_v38_json(),
            "--output-format",
            "json",
        ])
        .assert()
        .success();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("stdout was not valid JSON: {e}"))
}

/// The "Geomean" summary aggregates across benchmarks, even though each was
/// measured in its own processes.
///
/// This guards against pooling all the benchmarks' samples and summarizing
/// those. The row's min and max give it away: aggregating one sample per
/// benchmark keeps the range strictly inside the underlying samples', whereas a
/// pooled summary reports the global extremes exactly. Means cannot make the
/// distinction, since for Instantiation the pooled mean is nearer than the
/// geomean's own Jensen gap (see `summarize_geomean_is_geomean_of_means`).
#[test]
fn summarize_geomean_across_benchmarks() {
    let summaries = summaries_json();

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let summaries: Vec<_> = summaries.iter().filter(|s| s["phase"] == phase).collect();
        assert_eq!(
            summaries.len(),
            4,
            "expected three benchmarks and a geomean"
        );

        let geomean = summaries.iter().find(|s| s["wasm"] == "Geomean").unwrap();
        let benchmarks: Vec<_> = summaries
            .iter()
            .filter(|s| s["wasm"] != "Geomean")
            .collect();

        // A pooled summary would report exactly these.
        let pooled_min = benchmarks
            .iter()
            .map(|s| s["min"].as_f64().unwrap())
            .fold(f64::INFINITY, f64::min);
        let pooled_max = benchmarks
            .iter()
            .map(|s| s["max"].as_f64().unwrap())
            .fold(f64::NEG_INFINITY, f64::max);

        let min = geomean["min"].as_f64().unwrap();
        let max = geomean["max"].as_f64().unwrap();
        assert!(
            pooled_min < min && max < pooled_max,
            "{phase}: the geomean's range {min}..{max} is not strictly inside \
             the benchmarks' samples' range {pooled_min}..{pooled_max}, which \
             suggests the samples were pooled rather than aggregated across \
             benchmarks"
        );

        // A central value, not a total: a sum would exceed every sample.
        let mean = geomean["mean"].as_f64().unwrap();
        assert!(
            pooled_min < mean && mean < pooled_max,
            "{phase}: the geomean's mean {mean} is outside the samples' range"
        );
    }
}

/// The "Geomean" row equals the geometric mean of the benchmarks' means, the
/// conventional SPEC-style summary number.
///
/// The rows themselves are per-sample geomeans, whose raw average sits below
/// that figure by an amount that grows with the engine's own noise. They are
/// rescaled so this holds exactly; see the `Statistics` section on
/// `sightglass_analysis::geomean::calculate`. The tolerance here is for `u64`
/// rounding of each row, not for that gap.
#[test]
fn summarize_geomean_is_geomean_of_means() {
    let summaries = summaries_json();

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let summaries: Vec<_> = summaries.iter().filter(|s| s["phase"] == phase).collect();

        let actual = summaries.iter().find(|s| s["wasm"] == "Geomean").unwrap()["mean"]
            .as_f64()
            .unwrap();

        let means: Vec<f64> = summaries
            .iter()
            .filter(|s| s["wasm"] != "Geomean")
            .map(|s| s["mean"].as_f64().unwrap())
            .collect();
        let expected = (means.iter().map(|m| m.ln()).sum::<f64>() / means.len() as f64).exp();

        assert!(
            (actual - expected).abs() < 1.0,
            "{phase}: the geomean is {actual}, not the benchmarks' means' \
             geometric mean {expected}"
        );
    }
}

/// summarize with a nonexistent input file fails.
#[test]
fn summarize_missing_file_fails() {
    sightglass_cli()
        .args(["summarize", "-f", "nonexistent_xyz.json"])
        .assert()
        .failure();
}
