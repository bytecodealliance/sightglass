use super::util::{benchmark, sightglass_cli};
use assert_cmd::prelude::*;

/// validate accepts a valid benchmark .wasm file and exits successfully.
#[test]
fn validate_valid_benchmark() {
    sightglass_cli()
        .arg("validate")
        .arg(benchmark("noop"))
        .assert()
        .success();
}

/// validate rejects a non-wasm file (e.g. a JSON fixture).
#[test]
fn validate_non_wasm_file_fails() {
    sightglass_cli()
        .args([
            "validate",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/results.json"),
        ])
        .assert()
        .failure();
}
