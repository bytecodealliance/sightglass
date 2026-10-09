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

/// Like `sightglass-cli benchmark`, `effect-size` compares a synthetic "Geomean"
/// benchmark aggregating each sample's counts across its input's benchmarks.
#[test]
fn effect_size_geomean() {
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
        .stdout(predicate::str::contains("Geomean"));
}

/// Compare the two engines in our test data, returning the parsed effect sizes.
fn effect_sizes_json() -> Vec<serde_json::Value> {
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
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("stdout was not valid JSON: {e}"))
}

/// effect-size with --output-format json produces parseable JSON.
#[test]
fn effect_size_output_format_json() {
    let effects = effect_sizes_json();

    // Each of the three benchmarks in the input data is compared, as is our
    // synthetic "Geomean", and none of them are compared more than once.
    let mut wasms: Vec<&str> = effects
        .iter()
        .filter(|e| e["phase"] == "Compilation")
        .map(|e| e["wasm"].as_str().unwrap())
        .collect();
    wasms.sort();
    assert_eq!(
        wasms,
        [
            "Geomean",
            "benchmarks/bz2/benchmark.wasm",
            "benchmarks/pulldown-cmark/benchmark.wasm",
            "benchmarks/spidermonkey/benchmark.wasm",
        ]
    );
}

/// The "Geomean" comparison aggregates across benchmarks, even though each was
/// measured in its own processes.
///
/// Each engine's row must equal the geometric mean of that engine's
/// per-benchmark means. That also rules out the bug this guards against,
/// pooling all the samples and averaging them, which by AM-GM lands at or above
/// that figure: it overshoots by 1.10x to 5.09x on this fixture.
#[test]
fn effect_size_geomean_across_benchmarks() {
    let effects = effect_sizes_json();

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let effects: Vec<_> = effects.iter().filter(|e| e["phase"] == phase).collect();
        assert_eq!(effects.len(), 4, "expected three benchmarks and a geomean");

        let geomean = effects.iter().find(|e| e["wasm"] == "Geomean").unwrap();
        for mean in ["a_mean", "b_mean"] {
            let means: Vec<f64> = effects
                .iter()
                .filter(|e| e["wasm"] != "Geomean")
                .map(|e| e[mean].as_f64().unwrap())
                .collect();
            let expected = (means.iter().map(|m| m.ln()).sum::<f64>() / means.len() as f64).exp();
            let actual = geomean[mean].as_f64().unwrap();

            assert!(
                (actual - expected).abs() < 1.0,
                "{phase} {mean}: the geomean is {actual}, not the geometric mean \
                 of the benchmarks' means {expected}"
            );
        }
    }
}

/// A benchmark that only one engine measured is left out of the "Geomean"
/// rows, with a warning, so the rows stay comparable.
///
/// Including it would let the row contradict every benchmark under it: dropping
/// spidermonkey from one engine alone makes that engine's geomean omit the
/// most expensive benchmark, which looks like a large speed up.
#[test]
fn effect_size_geomean_excludes_benchmarks_missing_from_an_engine() {
    let dir = tempfile::TempDir::new().unwrap();
    let input = dir.path().join("partial.json");

    let mut measurements: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(multi_engine_v38_json()).unwrap()).unwrap();
    let epoch: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(multi_engine_v38_epoch_json()).unwrap())
            .unwrap();
    measurements.extend(
        epoch
            .into_iter()
            .filter(|m| !m["wasm"].as_str().unwrap().contains("spidermonkey")),
    );
    std::fs::write(&input, serde_json::to_string(&measurements).unwrap()).unwrap();

    let assert = sightglass_cli()
        .args([
            "effect-size",
            "-f",
            input.to_str().unwrap(),
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("spidermonkey"));

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    let effects: Vec<serde_json::Value> = serde_json::from_str(stdout).unwrap();

    let execution: Vec<_> = effects
        .iter()
        .filter(|e| e["phase"] == "Execution")
        .collect();
    let geomean = execution.iter().find(|e| e["wasm"] == "Geomean").unwrap();
    let ratio = geomean["a_mean"].as_f64().unwrap() / geomean["b_mean"].as_f64().unwrap();

    // Every remaining benchmark has engine B slower, so the row must agree.
    for e in execution.iter().filter(|e| e["wasm"] != "Geomean") {
        let r = e["a_mean"].as_f64().unwrap() / e["b_mean"].as_f64().unwrap();
        assert!(
            (ratio - r).abs() < 0.1,
            "the geomean ratio {ratio} disagrees with {}'s {r}",
            e["wasm"]
        );
    }
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
