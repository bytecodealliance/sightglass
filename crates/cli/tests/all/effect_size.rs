use super::util::sightglass_cli;
use assert_cmd::prelude::*;
use predicates::prelude::*;

fn multi_engine_v38_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/multi_engine_v38.json")
}

fn multi_engine_v38_epoch_json() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/multi_engine_v38_epoch.json"
    )
}

/// effect-size reads two-engine JSON and prints a human-readable comparison by default.
#[test]
fn effect_size_human_readable() {
    sightglass_cli()
        .args([
            "effect-size",
            "-f",
            multi_engine_v38_json(),
            "-f",
            multi_engine_v38_epoch_json(),
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("cycles").and(
                predicate::str::contains("Δ = ")
                    .or(predicate::str::contains("No difference in performance.")),
            ),
        );
}

/// effect-size with --output-format json produces parseable JSON.
#[test]
fn effect_size_output_format_json() {
    let assert = sightglass_cli()
        .args([
            "effect-size",
            "-f",
            multi_engine_v38_json(),
            "-f",
            multi_engine_v38_epoch_json(),
            "--output-format",
            "json",
        ])
        .assert()
        .success();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(
        serde_json::from_str::<serde_json::Value>(stdout).is_ok(),
        "stdout was not valid JSON: {stdout}"
    );
}

/// effect-size with --output-format csv produces a CSV header row.
#[test]
fn effect_size_output_format_csv() {
    sightglass_cli()
        .args([
            "effect-size",
            "-f",
            multi_engine_v38_json(),
            "-f",
            multi_engine_v38_epoch_json(),
            "--output-format",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("mean"));
}

/// effect-size with a nonexistent input file fails with an error.
#[test]
fn effect_size_missing_file_fails() {
    sightglass_cli()
        .args(["effect-size", "-f", "nonexistent_file_xyz.json"])
        .assert()
        .failure();
}

/// effect-size with a single-engine file (no comparison possible) exits non-zero.
#[test]
fn effect_size_single_engine_fails() {
    sightglass_cli()
        .args([
            "effect-size",
            "-f",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/results.json"),
        ])
        .assert()
        .failure();
}
