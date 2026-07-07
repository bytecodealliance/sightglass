use super::util::{benchmark, sightglass_cli, sightglass_cli_benchmark, test_engine};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_data::Measurement;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn benchmark_output_format_requires_raw() {
    sightglass_cli()
        .args(["benchmark", "--output-format", "csv", "dummy.wasm"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--raw"));
}

#[test]
fn benchmark_phase_compilation() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--benchmark-phase")
        .arg("compilation")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Compilation")
                .and(predicate::str::contains("Instantiation").not())
                .and(predicate::str::contains("Execution").not()),
        );
}

#[test]
fn benchmark_phase_instantiation() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--benchmark-phase")
        .arg("instantiation")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Compilation")
                .not()
                .and(predicate::str::contains("Instantiation"))
                .and(predicate::str::contains("Execution").not()),
        );
}

#[test]
fn benchmark_phase_execution() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--benchmark-phase")
        .arg("execution")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Compilation")
                .not()
                .and(predicate::str::contains("Instantiation").not())
                .and(predicate::str::contains("Execution")),
        );
}

#[test]
fn benchmark_json() {
    let assert = sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("2")
        .arg("--output-format")
        .arg("json")
        .arg("--")
        .arg(benchmark("noop"))
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{stdout}\n===========");
    assert!(serde_json::from_str::<serde_json::Value>(stdout).is_ok());

    assert
        .stdout(
            predicate::str::contains(benchmark("noop"))
                .and(predicate::str::contains("Compilation"))
                .and(predicate::str::contains("Instantiation"))
                .and(predicate::str::contains("Execution")),
        )
        .success();
}

#[test]
fn benchmark_csv() {
    let assert = sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("2")
        .arg("--output-format")
        .arg("csv")
        .arg("--")
        .arg(benchmark("noop"))
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{stdout}\n===========");
    let mut reader = csv::Reader::from_reader(stdout.as_bytes());
    for measurement in reader.deserialize::<Measurement<'_>>() {
        drop(measurement.unwrap());
    }

    assert
        .stdout(
            predicate::str::starts_with(
                "arch,engine,engine_flags,wasm,process,iteration,phase,event,count\n",
            )
            .and(predicate::str::contains(benchmark("noop")))
            .and(predicate::str::contains("Compilation"))
            .and(predicate::str::contains("Instantiation"))
            .and(predicate::str::contains("Execution")),
        )
        .success();
}

#[test]
fn benchmark_summary() {
    sightglass_cli_benchmark()
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("3")
        .arg("--")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("compilation")
                .and(predicate::str::contains("instantiation"))
                .and(predicate::str::contains("execution"))
                // The benchmark is displayed by its short label, not its path.
                .and(predicate::str::contains("noop"))
                // Statistics are rendered as a box-drawing table.
                .and(predicate::str::contains("Median"))
                .and(predicate::str::is_match(r#"│ \d+ +│"#).unwrap())
                .and(predicate::str::contains(
                    test_engine().display().to_string(),
                )),
        );
}

#[test]
#[cfg_attr(target_os = "windows", ignore)] // TODO: https://github.com/bytecodealliance/sightglass/issues/178
fn benchmark_effect_size() -> anyhow::Result<()> {
    // Create a temporary copy of the test engine.
    let test_engine = test_engine();
    let alt_engine = tempfile::NamedTempFile::new()?;
    let alt_engine_path: PathBuf = alt_engine.path().into();
    alt_engine.close()?;
    std::fs::copy(&test_engine, &alt_engine_path)?;
    assert!(alt_engine_path.exists());

    sightglass_cli()
        .arg("benchmark")
        .arg("--engine")
        .arg(&test_engine)
        .arg("--engine")
        .arg(&alt_engine_path)
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("200")
        // The two engines are identical copies, so their differences are not
        // significant; show them anyway to exercise the output format.
        .arg("--show-insignificant")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            // The benchmark is displayed by its short label ("noop"), not its path.
            predicate::str::contains("compilation :: cycles :: noop")
                // Instantiation is filtered out of effect-size output by default.
                .and(predicate::str::contains("instantiation").not())
                .and(predicate::str::contains("execution :: cycles :: noop"))
                // Statistics are rendered as a box-drawing table.
                .and(predicate::str::contains("Median"))
                .and(predicate::str::is_match(r#"│ \d+ +│"#).unwrap())
                .and(
                    predicate::str::contains("Δ = ")
                        .or(predicate::str::contains("No difference in performance.")),
                ),
        );

    Ok(())
}

/// Make a second, distinct on-disk copy of the test engine so we can benchmark
/// multiple engines.
fn alt_test_engine() -> PathBuf {
    let engine = test_engine();
    let alt = tempfile::NamedTempFile::new().unwrap();
    let alt_path: PathBuf = alt.path().into();
    alt.close().unwrap();
    std::fs::copy(&engine, &alt_path).unwrap();
    assert!(alt_path.exists());
    alt_path
}

/// A single engine (passed once) can be compared against itself with different
/// flags: no flags vs. fuel vs. epoch interruption.
#[test]
#[cfg_attr(target_os = "windows", ignore)] // TODO: https://github.com/bytecodealliance/sightglass/issues/178
fn benchmark_single_engine_multiple_flags() {
    let engine = test_engine();
    sightglass_cli()
        .arg("benchmark")
        .arg("-e")
        .arg(&engine)
        .arg("--engine-flags")
        .arg("")
        .arg("--engine-flags")
        .arg("-W fuel=99999999")
        .arg("--engine-flags")
        .arg("-W epoch-interruption=y")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("5")
        .arg("--show-insignificant")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            // The engine is compared against itself, so the configurations are
            // labeled by their flags.
            predicate::str::contains("compilation :: cycles :: noop")
                .and(predicate::str::contains("fuel=99999999"))
                .and(predicate::str::contains("epoch-interruption=y")),
        );
}

/// A single set of engine flags applies to every engine.
#[test]
#[cfg_attr(target_os = "windows", ignore)] // TODO: https://github.com/bytecodealliance/sightglass/issues/178
fn benchmark_multiple_engines_single_flags() {
    let engine = test_engine();
    let alt = alt_test_engine();
    sightglass_cli()
        .arg("benchmark")
        .arg("-e")
        .arg(&engine)
        .arg("-e")
        .arg(&alt)
        .arg("--engine-flags")
        .arg("-W fuel=99999999")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("5")
        .arg("--show-insignificant")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(predicate::str::contains("compilation :: cycles :: noop"));
}

/// Multiple engines can each be paired with their own flags.
#[test]
#[cfg_attr(target_os = "windows", ignore)] // TODO: https://github.com/bytecodealliance/sightglass/issues/178
fn benchmark_multiple_engines_multiple_flags() {
    let engine = test_engine();
    let alt = alt_test_engine();
    sightglass_cli()
        .arg("benchmark")
        .arg("-e")
        .arg(&engine)
        .arg("-e")
        .arg(&alt)
        .arg("--engine-flags")
        .arg("-W fuel=99999999")
        .arg("--engine-flags")
        .arg("-W epoch-interruption=y")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("5")
        .arg("--show-insignificant")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(predicate::str::contains("compilation :: cycles :: noop"));
}

/// With multiple benchmarks and non-raw (summary) output, a "Sum Total" row is
/// added that sums each sample's counts across the benchmarks. Using a single
/// process makes the per-sample sums span both benchmarks.
#[test]
fn benchmark_sum_total() {
    sightglass_cli_benchmark()
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("2")
        .arg("--")
        .arg(benchmark("noop"))
        .arg(benchmark("pulldown-cmark"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Sum Total"));
}

/// --output-file writes raw JSON to a file rather than stdout.
#[test]
fn benchmark_output_file() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let out = dir.path().join("out.json");
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--output-file")
        .arg(&out)
        .arg(benchmark("noop"))
        .assert()
        .success();
    assert!(out.exists(), "output file was not created");
    let content = std::fs::read_to_string(&out)?;
    assert!(
        serde_json::from_str::<serde_json::Value>(&content).is_ok(),
        "output file is not valid JSON"
    );
    Ok(())
}

/// --name overrides the engine name in the output.
#[test]
fn benchmark_name_override() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--name")
        .arg("my-custom-engine")
        .arg("--")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(predicate::str::contains("my-custom-engine"));
}

/// --measure time produces output with nanosecond events.
#[test]
fn benchmark_measure_noop() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--measure")
        .arg("time")
        .arg("--")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(predicate::str::contains("nanoseconds"));
}
