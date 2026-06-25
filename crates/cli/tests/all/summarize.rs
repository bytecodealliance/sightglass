use super::util::sightglass_cli;
use assert_cmd::prelude::*;
use predicates::prelude::*;

fn results_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/results.json")
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

/// summarize with a nonexistent input file fails.
#[test]
fn summarize_missing_file_fails() {
    sightglass_cli()
        .args(["summarize", "-f", "nonexistent_xyz.json"])
        .assert()
        .failure();
}
