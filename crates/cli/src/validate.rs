use anyhow::Result;
use clap::Parser;
use sightglass_build::WasmBenchmark;
use std::path::PathBuf;

/// Check that a Wasm benchmark is runnable in this tool.
#[derive(Parser, Debug)]
#[command(name = "validate")]
pub struct ValidateCommand {
    /// The path to the WebAssembly benchmark module; this file should import `bench.start` and
    /// `bench.end`.
    #[arg(required = true, value_name = "WASMFILE")]
    benchmark: PathBuf,
}

impl ValidateCommand {
    pub fn execute(&self) -> Result<()> {
        WasmBenchmark::from(&self.benchmark).is_valid()?;
        log::info!("benchmark is valid");
        Ok(())
    }
}
