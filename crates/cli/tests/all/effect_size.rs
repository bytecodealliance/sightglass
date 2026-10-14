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
                predicate::str::contains("faster than")
                    .or(predicate::str::contains("No difference in performance.")),
            ),
        );
}

/// Like `sightglass-cli benchmark`, `effect-size` compares the engines' geometric
/// means across its input's benchmarks.
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

/// Compare the two engines in our test data in human-readable form.
fn effect_sizes_human_readable() -> String {
    let assert = sightglass_cli()
        .args([
            "effect-size",
            "-f",
            multi_engine_v38_json(),
            "-f",
            multi_engine_v38_epoch_json(),
        ])
        .assert()
        .success();

    String::from_utf8(assert.get_output().stdout.clone()).unwrap()
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
///
/// Structured output holds the per-benchmark comparisons alone: the geomean is
/// not a benchmark, and its confidence interval is multiplicative rather than a
/// half-width around a difference of means.
#[test]
fn effect_size_output_format_json() {
    let effects = effect_sizes_json();

    assert!(
        effects.iter().all(|e| e["wasm"] != "Geomean"),
        "the geomean should not appear as a benchmark: {effects:#?}"
    );

    // Each of the three benchmarks in the input data is compared, and none of
    // them more than once.
    let mut wasms: Vec<&str> = effects
        .iter()
        .filter(|e| e["phase"] == "Compilation")
        .map(|e| e["wasm"].as_str().unwrap())
        .collect();
    wasms.sort();
    assert_eq!(
        wasms,
        [
            "benchmarks/bz2/benchmark.wasm",
            "benchmarks/pulldown-cmark/benchmark.wasm",
            "benchmarks/spidermonkey/benchmark.wasm",
        ]
    );
}

/// Each engine's reported geomean is the geometric mean of that engine's
/// per-benchmark means.
///
/// That rules out the bug this guards against, pooling all the samples and
/// averaging them, which by AM-GM lands at or above that figure: it overshoots
/// by 1.10x to 5.09x on this fixture.
#[test]
fn effect_size_geomean_across_benchmarks() {
    let effects = effect_sizes_json();
    let human_readable = effect_sizes_human_readable();

    for phase in ["Compilation", "Instantiation", "Execution"] {
        let effects: Vec<_> = effects.iter().filter(|e| e["phase"] == phase).collect();
        assert_eq!(effects.len(), 3, "expected three benchmarks");

        for mean in ["a_mean", "b_mean"] {
            let means: Vec<f64> = effects.iter().map(|e| e[mean].as_f64().unwrap()).collect();
            let expected = (means.iter().map(|m| m.ln()).sum::<f64>() / means.len() as f64).exp();
            let expected = format!("{expected:.2}");

            assert!(
                human_readable.contains(&expected),
                "{phase} {mean}: the geometric mean of the benchmarks' means \
                 {expected} is not reported:\n{human_readable}"
            );
        }
    }
}

/// A benchmark that only one engine measured is left out of the geomean, with a
/// warning, so the two engines' geomeans stay comparable.
///
/// Including it would let the geomean contradict every benchmark beneath it:
/// dropping spidermonkey from one engine alone makes that engine's geomean omit
/// the most expensive benchmark, which looks like a large speed up.
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
        .args(["effect-size", "-f", input.to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("spidermonkey"));

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();

    // Both geomeans cover the two benchmarks that survive, and every one of
    // those has engine B slower, so the geomean must agree rather than report a
    // speed up from the missing benchmark.
    assert!(
        stdout.contains("Geomean (2 benchmarks)"),
        "spidermonkey should be left out of the geomean:\n{stdout}"
    );

    let execution = stdout
        .split("execution :: cycles :: Geomean")
        .nth(1)
        .unwrap_or_else(|| panic!("no execution geomean:\n{stdout}"));
    assert!(
        execution.contains("(no flags) is"),
        "engine A should be the faster one, as it is on every benchmark:\n{stdout}"
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
