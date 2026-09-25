//! Test `sightglass-cli fingerprint`.

use super::util::{benchmark, sightglass_cli, test_engine};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use sightglass_fingerprint::{Benchmark, Engine, Machine};

#[test]
fn fingerprint_machine() {
    let assert = sightglass_cli()
        .arg("fingerprint")
        .arg("--kind")
        .arg("machine")
        .assert();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{stdout}\n===========");
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
    eprintln!("=== stdout ===\n{stdout}\n===========");
    let mut reader = csv::Reader::from_reader(stdout.as_bytes());
    for measurement in reader.deserialize::<Benchmark>() {
        drop(measurement.unwrap());
    }

    let benchmark_subpath = format!("noop{}benchmark.wasm", std::path::MAIN_SEPARATOR);
    assert
        .stdout(
            predicate::str::starts_with("id,name,path,hash,size\n")
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
        .assert()
        .success();

    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    eprintln!("=== stdout ===\n{stdout}\n===========");
    let fingerprint: Engine = serde_json::from_str(stdout).unwrap();
    assert!(fingerprint.id.starts_with("wasmtime-"));
    assert_eq!(fingerprint.name.as_deref(), Some("wasmtime"));
    assert_eq!(
        fingerprint.path,
        engine_path.canonicalize().unwrap().to_string_lossy()
    );
    assert!(fingerprint
        .buildinfo
        .as_deref()
        .is_some_and(|buildinfo| buildinfo.starts_with("NAME=wasmtime")));
}
