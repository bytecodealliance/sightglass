use anyhow::Result;
use sightglass_build::{Dockerfile, WasmBenchmark};
use std::path::PathBuf;
use structopt::StructOpt;

/// Build a Wasm benchmark from a Dockerfile.
#[derive(StructOpt, Debug)]
#[structopt(name = "build-benchmark")]
pub struct BuildBenchmarkCommand {
    /// The location at which to store the generated Wasm benchmark.
    #[structopt(long, short, value_name = "WASMFILE", parse(from_os_str))]
    destination: Option<PathBuf>,

    /// If enabled, emit a WebAssembly Text (WAT) version of the Wasm benchmark in the same
    /// directory as the `destination` with the `.wat` suffix; e.g. if `destination` is
    /// `/usr/src/benchmark.wasm`, the WAT file will be created at `/usr/src/benchmark.wat`.
    #[structopt(long, short = "w")]
    emit_wat: bool,

    /// The path to a Dockerfile that will build a WebAssembly benchmark module.
    #[structopt(
        index = 1,
        required = true,
        value_name = "DOCKERFILE",
        parse(from_os_str)
    )]
    dockerfile: PathBuf,
}

impl BuildBenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        let dockerfile = Dockerfile::from(self.dockerfile.clone());
        let destination = match self.destination.clone() {
            None => dockerfile.parent_dir().join("benchmark.wasm"),
            Some(p) => p,
        };

        dockerfile.extract(WasmBenchmark::source(), &destination, None)?;
        let wasmfile = WasmBenchmark::from(destination);
        if self.emit_wat {
            wasmfile.emit_wat()?;
        }
        wasmfile.is_valid()?;
        Ok(())
    }
}
