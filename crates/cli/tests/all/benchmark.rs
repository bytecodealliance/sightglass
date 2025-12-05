use super::util::{benchmark, sightglass_cli, sightglass_cli_benchmark, test_engine};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_data::Measurement;
use std::path::PathBuf;

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
                .and(predicate::str::contains(benchmark("noop")))
                .and(predicate::str::is_match(r#"\[\d+ \d+\.\d+ \d+\]"#).unwrap())
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
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains(format!("compilation :: cycles :: {}", benchmark("noop")))
                .and(predicate::str::contains(format!(
                    "instantiation :: cycles :: {}",
                    benchmark("noop")
                )))
                .and(predicate::str::contains(format!(
                    "execution :: cycles :: {}",
                    benchmark("noop")
                )))
                .and(predicate::str::is_match(r#"\[\d+ \d+\.\d+ \d+\]"#).unwrap())
                .and(
                    predicate::str::contains("Δ = ")
                        .or(predicate::str::contains("No difference in performance.")),
                ),
        );

    Ok(())
}
