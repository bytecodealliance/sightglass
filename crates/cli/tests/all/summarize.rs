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

/// Like `sightglass-cli benchmark`, `summarize` reports the geometric mean
/// across its input's benchmarks.
#[test]
fn summarize_geomean() {
    sightglass_cli()
        .args(["summarize", "-f", multi_engine_v38_json()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Geomean"));
}

/// Summarize our multi-engine test data in human-readable form.
fn summaries_human_readable() -> String {
    let assert = sightglass_cli()
        .args(["summarize", "-f", multi_engine_v38_json()])
        .assert()
        .success();

    String::from_utf8(assert.get_output().stdout.clone()).unwrap()
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

/// Structured output holds the per-benchmark summaries alone: the geomean is
/// not a benchmark, and has no min, max, or median to report.
#[test]
fn summarize_geomean_is_not_in_structured_output() {
    let summaries = summaries_json();

    assert!(
        summaries.iter().all(|s| s["wasm"] != "Geomean"),
        "the geomean should not appear as a benchmark: {summaries:#?}"
    );

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let summaries: Vec<_> = summaries.iter().filter(|s| s["phase"] == phase).collect();
        assert_eq!(summaries.len(), 3, "expected three benchmarks");
    }
}

/// The reported geomean is the geometric mean of the benchmarks' means, the
/// conventional SPEC-style summary number.
#[test]
fn summarize_geomean_is_geomean_of_means() {
    let summaries = summaries_json();
    let human_readable = summaries_human_readable();

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let means: Vec<f64> = summaries
            .iter()
            .filter(|s| s["phase"] == phase)
            .map(|s| s["mean"].as_f64().unwrap())
            .collect();
        let expected = (means.iter().map(|m| m.ln()).sum::<f64>() / means.len() as f64).exp();
        let expected = format!("{expected:.2}");

        assert!(
            human_readable.contains(&expected),
            "{phase}: the benchmarks' means' geometric mean {expected} is not \
             reported:\n{human_readable}"
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
