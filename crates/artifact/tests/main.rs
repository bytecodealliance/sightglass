use anyhow::Result;
use log::info;
use pretty_env_logger;
use sightglass_artifact::{Artifact, Dockerfile, WasmBenchmark};
use std::env;
use std::path::PathBuf;

// This example tests the crate functionality from end to end.
#[test]
#[ignore]
fn e2e() -> Result<()> {
    pretty_env_logger::init();

    // Build a Wasm benchmark using its Dockerfile.
    let dockerfile = Dockerfile::from(PathBuf::from("./tests/Dockerfile"));
    let destination_wasm = env::temp_dir().join("benchmark.wasm");
    dockerfile.extract(WasmBenchmark::source(), &destination_wasm, None)?;
    let wasmfile = WasmBenchmark::from(destination_wasm);

    // Verify that the benchmark is a valid one.
    assert!(wasmfile.is_valid().is_ok());

    // Construct the artifact metadata.
    let artifact = Artifact::from(dockerfile, wasmfile);
    info!("Artifact created: {}", serde_json::to_string(&artifact)?);

    Ok(())
}
