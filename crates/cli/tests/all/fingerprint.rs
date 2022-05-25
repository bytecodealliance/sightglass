//! Test `sightglass-cli fingerprint`.

use super::util::{benchmark, sightglass_cli, test_engine};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_fingerprint::{Benchmark, Machine};

#[test]
fn fingerprint_machine() {
    let assert = sightglass_cli()
        .arg("fingerprint")
        .arg("--kind")
        .arg("machine")
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{}\n===========", stdout);
    assert!(serde_json::from_str::<Machine>(stdout).is_ok());
}

#[test]
fn fingerprint_benchmark() {
    let assert = sightglass_cli()
        .arg("fingerprint")
        .arg("--kind")
        .arg("benchmark")
        .arg("--output-format")
        .arg("csv")
        .arg(benchmark("noop"))
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{}\n===========", stdout);
    let mut reader = csv::Reader::from_reader(stdout.as_bytes());
    for measurement in reader.deserialize::<Benchmark>() {
        drop(measurement.unwrap());
    }

    let benchmark_subpath = format!("noop{}benchmark.wasm", std::path::MAIN_SEPARATOR);
    assert
        .stdout(
            predicate::str::starts_with("name,path,hash,size\n")
                .and(predicate::str::contains(benchmark_subpath)),
        )
        .success();
}

#[test]
fn fingerprint_engine() {
    let engine_path = test_engine();
    let assert = sightglass_cli()
        .arg("fingerprint")
        .arg("--kind")
        .arg("engine")
        .arg("--output-format")
        .arg("json")
        .arg(&engine_path)
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{}\n===========", stdout);
    let mut reader = csv::Reader::from_reader(stdout.as_bytes());
    for measurement in reader.deserialize::<Benchmark>() {
        drop(measurement.unwrap());
    }

    use predicate::str::*;
    assert
        .stdout(
            starts_with("{")
                .and(contains(r#""name":"wasmtime-"#))
                .and(contains(format!(r#""path":"{}""#, engine_path.display())))
                .and(contains(r#""buildinfo":"NAME=wasmtime"#))
                .and(ends_with("}")),
        )
        .success();
}
