use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_data::Measurement;
use std::process::Command;

/// Get a `Command` for this crate's `sightglass-cli` executable.
fn sightglass_cli() -> Command {
    drop(env_logger::try_init());
    Command::cargo_bin("sightglass-cli").unwrap()
}

fn sightglass_cli_benchmark() -> Command {
    let mut cmd = sightglass_cli();
    cmd.arg("benchmark");

    if let Ok(engine) = std::env::var("SIGHTGLASS_TEST_ENGINE") {
        // Use the engine specified by the environment variable. We use this to
        // cache built `libwasmtime_bench_api.so`s in CI.
        cmd.arg("--engine").arg(engine);
    } else {
        // Make sure we only ever build Wasmtime once, and don't have N threads
        // build it in parallel and race to be the one to save it onto the file
        // system.
        static BUILD_WASMTIME: std::sync::Once = std::sync::Once::new();
        BUILD_WASMTIME.call_once(|| {
            let status = Command::cargo_bin("sightglass-cli")
                .unwrap()
                .current_dir("../..") // Run in the root of the repo.
                .arg("build-engine")
                .arg("wasmtime")
                .status()
                .expect("failed to run `sightglass-cli build-engine`");
            assert!(status.success());
        });
    }

    cmd
}

/// Get the benchmark path for the benchmark with the given name.
fn benchmark(benchmark_name: &str) -> String {
    format!("../../benchmarks-next/{}/benchmark.wasm", benchmark_name).into()
}

#[test]
fn help() {
    sightglass_cli().arg("help").assert().success();
}

#[test]
fn benchmark_json() {
    let assert = sightglass_cli_benchmark()
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
    eprintln!("=== stdout ===\n{}\n===========", stdout);
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
    eprintln!("=== stdout ===\n{}\n===========", stdout);
    let mut reader = csv::Reader::from_reader(stdout.as_bytes());
    for measurement in reader.deserialize::<Measurement<'_>>() {
        drop(measurement.unwrap());
    }

    assert
        .stdout(
            predicate::str::starts_with("arch,engine,wasm,process,iteration,phase,event,count\n")
                .and(predicate::str::contains(benchmark("noop")))
                .and(predicate::str::contains("Compilation"))
                .and(predicate::str::contains("Instantiation"))
                .and(predicate::str::contains("Execution")),
        )
        .success();
}
