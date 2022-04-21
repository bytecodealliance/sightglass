use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_data::Measurement;
use std::path::PathBuf;
use std::process::Command;

/// Get a `Command` for this crate's `sightglass-cli` executable.
fn sightglass_cli() -> Command {
    drop(env_logger::try_init());
    Command::cargo_bin("sightglass-cli").unwrap()
}

/// Get the path to the engine we are testing with.
fn test_engine() -> PathBuf {
    if let Ok(engine) = std::env::var("SIGHTGLASS_TEST_ENGINE") {
        // Use the engine specified by the environment variable. We use this to
        // cache built `libwasmtime_bench_api.so`s in CI.
        engine.into()
    } else {
        // Make sure we only ever build Wasmtime once, and don't have N threads
        // build it in parallel and race to be the one to save it onto the file
        // system.
        static BUILD_WASMTIME: std::sync::Once = std::sync::Once::new();
        BUILD_WASMTIME.call_once(|| {
            if sightglass_build::path::get_known_engine_path("wasmtime")
                .unwrap()
                .is_file()
            {
                // A wasmtime engine is already built!
                return;
            }

            // Use this instead of `eprintln!` to avoid `cargo test`'s stdio
            // capturing.
            use std::io::Write;
            drop(writeln!(
                std::io::stderr(),
                "**************************************************************\n\
                 *** Building Wasmtime engine; this may take a few minutes. ***\n\
                 **************************************************************"
            ));

            let status = Command::cargo_bin("sightglass-cli")
                .unwrap()
                .current_dir("../..") // Run in the root of the repo.
                .arg("build-engine")
                .arg("wasmtime")
                .status()
                .expect("failed to run `sightglass-cli build-engine`");
            assert!(status.success());
        });
        sightglass_build::path::get_known_engine_path("wasmtime").unwrap()
    }
}

/// Get a `sightglass-cli benchmark` command that is configured to use our test
/// engine.
fn sightglass_cli_benchmark() -> Command {
    let mut cmd = sightglass_cli();
    cmd.arg("benchmark").arg("--engine").arg(test_engine());
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
fn benchmark_stop_after_compilation() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--stop-after")
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
fn benchmark_stop_after_instantiation() {
    sightglass_cli_benchmark()
        .arg("--raw")
        .arg("--processes")
        .arg("2")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--stop-after")
        .arg("instantiation")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Compilation")
                .and(predicate::str::contains("Instantiation"))
                .and(predicate::str::contains("Execution").not()),
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
fn benchmark_effect_size() -> anyhow::Result<()> {
    // Create a temporary copy of the test engine.
    let test_engine = test_engine();
    let alt_engine = tempfile::NamedTempFile::new()?;
    let alt_engine_path: PathBuf = alt_engine.path().into();
    alt_engine.close()?;
    std::fs::copy(&test_engine, &alt_engine_path)?;

    sightglass_cli()
        .arg("benchmark")
        .arg("--engine")
        .arg(&test_engine)
        .arg("--engine")
        .arg(&alt_engine_path)
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("3")
        .arg(benchmark("noop"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains(&format!("compilation :: cycles :: {}", benchmark("noop")))
                .and(predicate::str::contains(&format!(
                    "instantiation :: cycles :: {}",
                    benchmark("noop")
                )))
                .and(predicate::str::contains(&format!(
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
