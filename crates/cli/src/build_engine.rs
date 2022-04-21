use anyhow::Result;
use sightglass_build::{engine::build_engine, path::get_engine_path_from_buildinfo, BuildInfo};
use structopt::StructOpt;

/// Build a Wasm engine from either a BUILD-INFO string or a Dockerfile and print the path to the
/// generated library.
#[derive(Debug, StructOpt)]
#[structopt(name = "build-engine")]
pub struct BuildEngineCommand {
    /// Force this tool to rebuild the benchmark.
    #[structopt(long, short)]
    force_rebuild: bool,

    /// A build-info string, `[engine name]?[variable]=[value]...`; e.g. `wasmtime` or
    /// `wasmtime?REVISION=92350bf2` or `wasmtime?REVISION=92350bf2+FLAGS='--release --features
    /// new-feature`. Valid engines are listed in `sightglass/engines/<engine>` and modifiable
    /// variables are Dockerfile `ARG`s.
    #[structopt(index = 1, required = true, value_name = "BUILD-INFO")]
    buildinfo: BuildInfo,
}

impl BuildEngineCommand {
    pub fn execute(&self) -> Result<()> {
        let engine_path = get_engine_path_from_buildinfo(&self.buildinfo)?;
        if !engine_path.exists() || self.force_rebuild {
            build_engine(&self.buildinfo, &engine_path)?;
        }
        println!("{}", engine_path.display());
        Ok(())
    }
}
