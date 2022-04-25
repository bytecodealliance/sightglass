use anyhow::Result;
use sightglass_build::WasmBenchmark;
use std::path::PathBuf;
use structopt::StructOpt;

/// Check that a Wasm benchmark is runnable in this tool.
#[derive(StructOpt, Debug)]
#[structopt(name = "validate")]
pub struct ValidateCommand {
    /// The path to the WebAssembly benchmark module; this file should import `bench.start` and
    /// `bench.end`.
    #[structopt(
        index = 1,
        required = true,
        value_name = "WASMFILE",
        parse(from_os_str)
    )]
    benchmark: PathBuf,
}

impl ValidateCommand {
    pub fn execute(&self) -> Result<()> {
        WasmBenchmark::from(&self.benchmark).is_valid()?;
        log::info!("benchmark is valid");
        Ok(())
    }
}
